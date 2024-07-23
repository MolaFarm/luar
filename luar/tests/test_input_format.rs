use std::{fs::File, io::BufReader};
use luar::{vm,parse};
use luar::vm_exec_input;

#[test]
fn test_string_input() {
    let input = std::io::Cursor::new("print \"i am from string!\""); 
    let res = vm_exec_input!(input);
    assert_eq!(res,0)
}

#[test]
fn test_buffer_input() {
    let file = File::open("./tests/data/strings.lua").unwrap();
    let input = BufReader::new(file); 

    let res = vm_exec_input!(input);
    assert_eq!(res,0);
}