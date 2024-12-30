use std::fs::File;
use luar::{value_vec, vm};
use luar::value::Value;
use luar::{parse, vm_exec_input};

#[test]
fn test_simple_assign() {
    let file = File::open("./tests/luas/assign.lua").unwrap();
    let proto = parse::load(file);
    let expect_l_constants: Vec<Value> = value_vec!["print","g",123,"g2"];
    let l_consts: &Vec<Value> = &proto.constants;
    assert_eq!(l_consts,&expect_l_constants);

    let file = File::open("./tests/luas/assign.lua").unwrap();
    vm_exec_input!(file);
}

#[test]
fn test_simple_local() {
    let file = File::open("./tests/luas/simple_local.lua").unwrap();
    vm_exec_input!(file);
}
