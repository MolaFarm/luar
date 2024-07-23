
#[deprecated(
    since = "0.1.0",
    note = "Macro design for LuaState::rootproto which is removed"
)]
#[macro_export]
/// ## unwarp_option_rc
/// It will try to convert `Option<Rc<RefCell<T>>>` to `&T`
macro_rules! unwarp_option_rc {
    ($option:expr) => {
        match $option {
            Some(ref rc_refcell) => rc_refcell.borrow(),
            None => panic!("Called `unwarp_rc!` on a `None` value"),
        }
    };
}

#[macro_export]
macro_rules! vm_exec_input_debug {
    ($i:expr) => {
        {
            let ls = vm::LuaState::new();
            let proto = parse::ParseProto::load($i);
            match panic::catch_unwind(|| {
                ls.run(&proto,0,0,0);
            }) {
                Ok(_) => 0,
                Err(err) => {
                    dbg!(&ls);
                    dbg!(&proto);
                    println!("Caught a panic: {:?}", err);
                    -1
                }
            }
        }
    };
}

#[macro_export]
macro_rules! vm_exec_input {
    ($i:expr) => {
        {
            let mut ls = vm::LuaState::new();
            let proto = parse::ParseProto::load($i);
            ls.run(&proto,0,0,0)
        }
    };
}

#[macro_export]
macro_rules! value_vec {
    ($($value:expr),* $(,)?) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($value.into());
            )*
            vec
        }
    };
}
#[macro_export]
macro_rules! panicdbg {
    ($struct:expr, $msg:expr) => {
        {
            dbg!(&$struct);
            panic!($msg);
        }
    };
}