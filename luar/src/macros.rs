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
            use std::io::BufReader;
            let proto = parse::load(BufReader::new($i));
            vm::ExeState::new().execute(&proto, &Vec::new());
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