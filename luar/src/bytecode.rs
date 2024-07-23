#[derive(Debug)]
pub enum ByteCode {
    /// ## GetGlobal(dst, name_idx)
    /// 
    /// GetGlobal will use name_idx to search name in constants table
    /// then use name to get Value in globals, finally, push the value to 
    /// stack 
    ///  
    /// - dst: target index in stack  
    /// - name_idx: index of Global's name in constants table
    GetGlobal(u8, u8),

    /// ## SetGlobal(key_idx, src)
    /// 
    /// SetGlobal will use key_idx to search key name in constants table, then 
    /// `clone` the Value at stack\[src\], and set to Globals table with given 
    /// name({key:stack\[src\]})
    ///  
    /// - key_idx: index of name in constants table
    /// - src: index of value which will be add to Globals table 
    SetGlobal(u8, u8),

    /// ## SetGlobalConst(key_idx, src)
    /// 
    /// SetGlobalConst will use key_idx to search key name in constants table, then 
    /// `clone` the Value at stack\[src\], and set to Globals table with given 
    /// name({key:stack\[src\]})
    ///  
    /// - key_idx: index of name in constants table
    /// - src: index of value which will be add to Globals table 
    SetGlobalConst(u8, u8),

    /// ## SetGlobalGlobal(name_idx, src_idx)
    /// 
    /// SetGlobalGloabl assign a global value to a global value
    /// 
    /// - name_idx: index of Value::String(new_global_name) in constants table
    /// - src_idx: index of Value::String(source_global_name) in constants table
    SetGlobalGlobal(u8, u8),

    /// ## LoadConst(dst, const_idx)
    /// 
    /// LoadConst push the Value at constants table\[const_idx\] to stack[dst]
    LoadConst(u8, u16),

    /// ## LoadNil(dst)
    /// 
    /// LoadNil will push a Nil to stack[dst]
    LoadNil(u8),

    /// ## LoadBool(dst,b)
    /// 
    /// LoadBool will push a boolean value b to stack
    LoadBool(u8, bool),
    LoadInt(u8, i16),

    /// ## Move(dst,src)
    /// 
    /// Move will Clone the Value at stack[src] and push to stack[dst]
    Move(u8, u8),

    /// ## Call(func_idx,_) //TODO ignore the second params now
    /// 
    /// Call the function at stack[func_index]
    Call(u8, u8),
}  