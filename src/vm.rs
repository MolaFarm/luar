use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::rc::Rc;
use crate::bytecode::ByteCode;
use crate::parse::ParseProto;
use crate::value::Value;
use crate::lualib::baselib::lua_print;


pub struct LuaState {
    pub globals: HashMap<String, Value>,
    pub stack: Vec::<Value>,
    // pub rootproto: Vec::<ParseProto>,
    pub rootproto: Option<Rc<RefCell<ParseProto>>>,
    pub func_index: usize,
}

impl LuaState {

    pub fn new() -> LuaState {
        let mut globals = HashMap::new();
        globals.insert(String::from("print"), Value::Function(lua_print)); // register print func

        LuaState {
            globals,
            stack: Vec::new(),
            rootproto: None,
            func_index: 0,
        }
    }

    pub fn load(&mut self,f:File) -> & Self {
        let proto = ParseProto::load(f);
        self.rootproto = Some(Rc::new(RefCell::new(proto)));
        self
    }

    #[allow(unused_variables)]
    pub fn run(&mut self,narg:u8,nres:u8,base:u8) -> i32 {
        let proto = match & self.rootproto {
            Some(proto) => Rc::clone(proto),
            None => panic!("no rootproto"),
        };

        let ref_proto = proto.borrow();
        for code in ref_proto.byte_codes.iter() {
            match *code {
                ByteCode::GetGlobal(dst, name_idx) => {
                    let name: &Value = &ref_proto.constants[name_idx as usize];
                    if let Value::String(key) = name {
                        let v = self.globals.get(key).unwrap_or(&Value::Nil).clone();
                        self.set_stack(dst, v);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                // ByteCode::SetGlobal(name, src) => {
                //     let name = ref_proto.constants[name as usize].clone();
                //     if let Value::String(key) = name {
                //         let value = self.stack[src as usize].clone();
                //         self.globals.insert(key, value);
                //     } else {
                //         panic!("invalid global key: {name:?}");
                //     }
                // }
                // SetGlobal and SetGlobalConst are cureent have basically same logic, 
                // so they temporarily share logic code
                ByteCode::SetGlobal(name, src) | ByteCode::SetGlobalConst(name, src) => {
                    let name = ref_proto.constants[name as usize].clone();
                    if let Value::String(key) = name {
                        let value = ref_proto.constants[src as usize].clone();
                        self.globals.insert(key, value);
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
                }
                ByteCode::SetGlobalGlobal(name_idx, src_idx) => {
                    let name = ref_proto.constants[name_idx as usize].clone();
                    if let Value::String(key) = name {
                        let src = &ref_proto.constants[src_idx as usize];
                        if let Value::String(src_name) = src {
                            let value = self.globals.get(src_name).unwrap_or(&Value::Nil).clone();
                            self.globals.insert(key, value);
                        } else {
                            panic!("invalid global key: {src:?}");
                        }
                    } else {
                        panic!("invalid global key: {name:?}");
                    }
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