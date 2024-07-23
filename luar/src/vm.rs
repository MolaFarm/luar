use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::Read;
use std::panic::{RefUnwindSafe, UnwindSafe};
use crate::bytecode::ByteCode;
use crate::parse::ParseProto;
use crate::value::Value;
use crate::lualib::baselib::lua_print;

#[derive(Debug)]
pub struct LuaState{
    pub globals: HashMap<String, Value>,
    pub stack: Vec::<Value>,
    // pub rootproto: Vec::<ParseProto>,
    // pub rootproto: Option<Rc<RefCell<ParseProto<T>>>>,
    pub func_index: usize,
}

impl UnwindSafe for LuaState {}
impl RefUnwindSafe for LuaState {}

impl LuaState{

    pub fn new() -> LuaState {
        let mut globals = HashMap::new();
        globals.insert("print".into(), Value::Function(lua_print)); // register print func

        LuaState {
            globals,
            stack: Vec::new(),
            // rootproto: None,
            func_index: 0,
        }
    }

    #[deprecated(
        since = "0.1.0",
        note = "use ParseProto::load, LuaState::run(proto) instead"
    )]
    pub fn load(&mut self) -> & Self {
        // let proto = ParseProto::load(input);
        // self.rootproto = Some(Rc::new(RefCell::new(proto)));

        self
    }

    #[allow(unused_variables)]
    pub fn run<T:Read>(&mut self,proto: &ParseProto<T>,narg:u8,nres:u8,base:u8) -> i32 {
        // let proto = match & self.rootproto {
        //     Some(proto) => Rc::clone(proto),
        //     None => panic!("no rootproto"),
        // };

        // let ref_proto = proto.borrow();
        let ref_proto = proto;
        for code in ref_proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name_idx) => {
                    let name: &Value = &ref_proto.constants[name_idx as usize];
                    let name: &str = (&ref_proto.constants[name_idx as usize]).into();
                    let v = self.globals.get(name).unwrap_or(&Value::Nil).clone();
                    self.set_stack(dst.into(), v);
                }
                ByteCode::SetGlobal(name_idx, src_idx) => {
                    let name = &proto.constants[name_idx as usize];
                    let value = self.stack[src_idx as usize].clone();
                    self.globals.insert(name.into(), value);
                }
                ByteCode::SetGlobalConst(name_idx, src_idx) => {
                    let name = &proto.constants[name_idx as usize];
                    let value = proto.constants[src_idx as usize].clone();
                    self.globals.insert(name.into(), value);
                }
                ByteCode::SetGlobalGlobal(name_idx, src_idx) => {
                    let name = &ref_proto.constants[name_idx as usize];
                    let value = ref_proto.constants[src_idx as usize].clone();
                    self.globals.insert(name.into(), value);
                }
                ByteCode::LoadConst(dst, c) => {
                    let v = ref_proto.constants[c as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::LoadNil(dst) => {
                    self.set_stack(dst, Value::Nil);
                }
                ByteCode::LoadBool(dst, b) => {
                    self.set_stack(dst, Value::Boolean(b));
                }
                ByteCode::LoadInt(dst, i) => {
                    self.set_stack(dst, Value::Integer(i as i64));
                }
                ByteCode::Move(dst, src) => {
                    let v = self.stack[src as usize].clone();
                    self.set_stack(dst, v);
                }
                ByteCode::Call(func, _) => {
                    self.func_index = func as usize;
                    let func = &self.stack[self.func_index];
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