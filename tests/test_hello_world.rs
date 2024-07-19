use std::fs::File;
use luar::vm::LuaState;

#[test]
fn hello_once() {
    let file = File::open("./tests/data/hello.lua").unwrap();
    let mut l = LuaState::new();
    l.load(file);
    let res = l.run(0, 0, 0);
    assert_eq!(res, 0);
}

#[test]
fn hello_main_times() {
    let file = File::open("./tests/data/hello3.lua").unwrap();
    let mut l = LuaState::new();
    l.load(file);
    let res = l.run(0, 0, 0);
    assert_eq!(res, 0);
}
