use crate::{value::Value, vm::ExeState};

pub fn lua_print(state: &mut ExeState) -> i32 {
    for i in 1 ..= state.get_top() {
        if i != 1 {
            print!("\t");
        }
        print!("{}", state.get::<&Value>(i).to_string());
    }
    println!("");
    0
}
pub fn lua_type(state: &mut ExeState) -> i32 {
    let ty = state.get::<&Value>(1).ty();
    state.push(ty);
    1
}