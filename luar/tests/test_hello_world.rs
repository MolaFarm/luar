use std::fs::File;
use std::io::BufReader;
use luar::vm;
use luar::vm_exec_input;
use luar::parse;

#[test]
fn hello_once() {
    let file = File::open("./tests/luas/hello.lua").unwrap();
    // let res = vm_exec_input!(file);
    let proto = parse::load(BufReader::new(file));
    vm::ExeState::new().execute(&proto, &Vec::new());
}

#[test]
fn hello_main_times() {
    let file = File::open("./tests/luas/hello3.lua").unwrap();
    vm_exec_input!(file);
}
