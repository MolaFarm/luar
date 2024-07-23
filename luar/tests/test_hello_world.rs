use std::fs::File;
use luar::vm;
use luar::vm_exec_input;
use luar::parse;
use std::panic;
#[test]
fn hello_once() {
    let file = File::open("./tests/data/hello.lua").unwrap();
    // let res = vm_exec_input!(file);
    let proto = parse::ParseProto::load(file);
    let res = vm::LuaState::new().run(&proto,0,0,0);
    assert_eq!(res, 0);
}

#[test]
fn hello_main_times() {
    let file = File::open("./tests/data/hello3.lua").unwrap();
    let res = vm_exec_input!(file);
    assert_eq!(res, 0);
}
