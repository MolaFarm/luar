use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::rc::Rc;
use crate::bytecode::ByteCode;
use crate::parse::ParseProto;
use crate::value::Value;
use crate::lualib::baselib::lua_print;
use crate::parse;


pub struct LuaState {
    pub globals: HashMap<String, Value>,
    pub stack: Vec::<Value>,
    // pub rootproto: Vec::<ParseProto>,
    pub rootproto: Option<Rc<RefCell<ParseProto>>>,
}

impl LuaState {

    pub fn new() -> LuaState {
        let mut globals = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lua_print));

        LuaState {
            globals,
            stack: Vec::new(),
            rootproto: None,
        }
    }

    pub fn load(&mut self,f:File) -> & Self {
        self.rootproto = Some(Rc::new(RefCell::new(parse::load(f))));
        self
    }

    pub fn run(&mut self,narg:u8,nres:u8,base:u8) -> i32 {
        let proto = match & self.rootproto {
            Some(proto) => Rc::clone(proto),
            None => panic!("no rootproto"),
        };

        let tmp_proto = proto.borrow();
        for code in tmp_proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name) => {
                    let name = &tmp_proto.constants[name as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = tmp_proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Call(func, _) => {
                    let func = &self.stack[func as usize];
                    if let Value::Function(f) = func {
                        f(self);
                    } else {
                        panic!("invalid function: {func:?}");
                    }
                }
            }
        }

        0
    }

    fn set_stack(&mut self, dst: u8, v: Value) {
        let dst = dst as usize;
        match dst.cmp(&self.stack.len()) {
            Ordering::Equal => self.stack.push(v),
            Ordering::Less => self.stack[dst] = v,
            Ordering::Greater => panic!("fail in set_stack"),
        }
    }
}