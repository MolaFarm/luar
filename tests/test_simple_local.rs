use std::fs::File;
use luar::vm::LuaState;
use luar::value::Value;
use luar::lualib::baselib::{lua_print};
use luar::unwarp_option_rc;

#[test]
fn test_simple_assign() {
    let file = File::open("./tests/data/assign.lua").unwrap();
    let mut l: LuaState = LuaState::new();
    l.load(file);
    let res = l.run(0, 0, 0);
    let expect_l_constants: Vec<Value> = vec![Value::String(String::from("print")),Value::String(String::from("g")),Value::Integer(123),Value::String(String::from("g2"))];
    let l_consts: &Vec<Value> = &unwarp_option_rc!(&l.rootproto).constants;
    assert_eq!(l_consts,&expect_l_constants);
    assert_eq!(res, 0);
}

#[test]
fn test_simple_local() {
    let file = File::open("./tests/data/simple_local.lua").unwrap();
    let mut l: LuaState = LuaState::new();
    l.load(file);
    let res = l.run(0, 0, 0);
    assert_eq!(res, 0);
}
