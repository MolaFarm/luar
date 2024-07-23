use std::{fmt, hash::Hasher, mem};
use crate::vm::LuaState;
use std::rc::Rc;
use core::hash::Hash;

const SHORT_STR_MAX: usize = 14; // sizeof(Value) - 1(tag) - 1(len)
const MID_STR_MAX: usize = 48 - 1;

#[derive(Clone)]
pub enum Value {
    Nil,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    ShortStr(u8, [u8; SHORT_STR_MAX]), // for literal string shorter than 14, directly stored on stack
    MidStr(Rc<(u8, [u8; MID_STR_MAX])>), // for literal string longer than 14 but shorter than 48
    LongStr(Rc<Vec<u8>>), // for mutable string use Vec<u8>
    Function(fn (&mut LuaState) -> i32),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(n) => write!(f, "{n:?}"),
            Value::ShortStr(len, buf) => write!(f, "{}", String::from_utf8_lossy(&buf[..*len as usize])),
            Value::MidStr(s) => write!(f, "{}", String::from_utf8_lossy(&s.1[..s.0 as usize])),
            Value::LongStr(s) => write!(f, "{}", String::from_utf8_lossy(&s)),
            Value::Function(_) => write!(f, "func"),
        }

    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        // TODO compare Integer vs Float (123 == 123.0 => True)
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(b1), Value::Boolean(b2)) => *b1 == *b2,
            (Value::Integer(i1), Value::Integer(i2)) => *i1 == *i2,
            (Value::Float(f1), Value::Float(f2)) => *f1 == *f2,
            (Value::ShortStr(len1, s1), Value::ShortStr(len2, s2)) => s1[..*len1 as usize] == s2[..*len2 as usize],
            (Value::MidStr(s1), Value::MidStr(s2)) => s1.1[..s1.0 as usize] == s2.1[..s2.0 as usize],
            (Value::LongStr(s1), Value::LongStr(s2)) => s1 == s2,
            (Value::Function(f1), Value::Function(f2)) => std::ptr::eq(f1, f2),
            (_, _) => false,
        }
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => (),
            Value::Boolean(b) => b.hash(state),
            Value::Integer(i) => i.hash(state),
            Value::Float(f) => // TODO try to convert to integer
                unsafe {
                    mem::transmute::<f64, i64>(*f).hash(state)
                }
            Value::ShortStr(len, buf) => buf[..*len as usize].hash(state),
            Value::MidStr(s) => s.1[..s.0 as usize].hash(state),
            Value::LongStr(s) => s.hash(state),
            Value::Function(f) => (*f as *const usize).hash(state),
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Nil
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Float(n)
    }
}

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Value::Integer(n)
    }
}

// convert &[u8], Vec<u8>, &str and String into Value
impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        vec_to_str_value(v).unwrap_or(Value::LongStr(Rc::new(v.to_vec())))
    }
}
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        s.as_bytes().into() // &[u8]
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        vec_to_str_value(&v).unwrap_or(Value::LongStr(Rc::new(v)))
    }
}
impl From<String> for Value {
    fn from(s: String) -> Self {
        s.into_bytes().into() // Vec<u8>
    }
}

fn vec_to_str_value(v: &[u8]) -> Option<Value> {
    match v.len() {
        len if len <= SHORT_STR_MAX => {
            let mut buf = [0; SHORT_STR_MAX];
            buf[..len].copy_from_slice(&v);
            Some(Value::ShortStr(len as u8, buf))
        },
        len if len <= MID_STR_MAX => {
            let mut buf = [0; MID_STR_MAX];
            buf[..len].copy_from_slice(&v);
            Some(Value::MidStr(Rc::new((len as u8, buf))))
        },
        _ => None
    }
}



impl<'a> From<&'a Value> for &'a [u8] {
    fn from(v: &'a Value) -> Self {
        match v {
            Value::ShortStr(len, buf) => &buf[..*len as usize],
            Value::MidStr(s) => &s.1[..s.0 as usize],
            Value::LongStr(s) => s,
            _ => panic!("invalid string Value"),
        }
    }
}

impl<'a> From<&'a Value> for &'a str {
    fn from(v: &'a Value) -> Self {
        std::str::from_utf8(v.into()).unwrap()
    }
}

impl From<&Value> for String {
    fn from(v: &Value) -> Self {
        String::from_utf8_lossy(v.into()).to_string()
    }
}