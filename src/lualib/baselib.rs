use crate::vm::LuaState;

pub fn lua_print(state: &mut LuaState) -> i32 {
    println!("{:?}", state.stack[1]);
    0
}