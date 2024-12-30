use std::{cell::RefCell, rc::Rc};

use crate::{value::Value, vm::ExeState};

pub fn test_new_counter(state: &mut ExeState) -> i32 {
    let mut i = 0_i32;
    let c = move |_: &mut ExeState| {
        i += 1;
        println!("counter: {i}");
        0
    };
    state.push(Value::RustClosure(Rc::new(RefCell::new(Box::new(c)))));
    1
}

pub fn ipairs_aux(state: &mut ExeState) -> i32 {
    let table = match state.get::<&Value>(1) {
        Value::Table(t) => t.borrow(),
        _ => panic!("ipairs non-table"),
    };

    let i: i64 = state.get(2);
    if i < 0 || i as usize >= table.array.len() {
        return 0;
    }

    let v = table.array[i as usize].clone();
    drop(table);

    state.push(i + 1);
    state.push(v);
    2
}

pub fn ipairs(state: &mut ExeState) -> i32 {
    state.push(Value::RustFunction(ipairs_aux));
    state.push(state.get::<&Value>(1).clone());
    state.push(0);
    3
}
