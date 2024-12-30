// #[derive(Debug)]
// pub enum ByteCode {
//     /// ## GetGlobal(dst, name_idx)
//     /// 
//     /// GetGlobal will use name_idx to search name in constants table
//     /// then use name to get Value in globals, finally, push the value to 
//     /// stack 
//     ///  
//     /// - dst: target index in stack  
//     /// - name_idx: index of Global's name in constants table
//     GetGlobal(u8, u8),

//     /// ## SetGlobal(key_idx, src)
//     /// 
//     /// SetGlobal will use key_idx to search key name in constants table, then 
//     /// `clone` the Value at stack\[src\], and set to Globals table with given 
//     /// name({key:stack\[src\]})
//     ///  
//     /// - key_idx: index of name in constants table
//     /// - src: index of value which will be add to Globals table 
//     SetGlobal(u8, u8),

//     /// ## SetGlobalConst(key_idx, src)
//     /// 
//     /// SetGlobalConst will use key_idx to search key name in constants table, then 
//     /// `clone` the Value at stack\[src\], and set to Globals table with given 
//     /// name({key:stack\[src\]})
//     ///  
//     /// - key_idx: index of name in constants table
//     /// - src: index of value which will be add to Globals table 
//     SetGlobalConst(u8, u8),

//     /// ## SetGlobalGlobal(name_idx, src_idx)
//     /// 
//     /// SetGlobalGloabl assign a global value to a global value
//     /// 
//     /// - name_idx: index of Value::String(new_global_name) in constants table
//     /// - src_idx: index of Value::String(source_global_name) in constants table
//     SetGlobalGlobal(u8, u8),

//     /// ## LoadConst(dst, const_idx)
//     /// 
//     /// LoadConst push the Value at constants table\[const_idx\] to stack[dst]
//     LoadConst(u8, u16),

//     /// ## LoadNil(dst)
//     /// 
//     /// LoadNil will push a Nil to stack[dst]
//     LoadNil(u8),

//     /// ## LoadBool(dst,b)
//     /// 
//     /// LoadBool will push a boolean value b to stack
//     LoadBool(u8, bool),
//     LoadInt(u8, i16),

//     /// ## Move(dst,src)
//     /// 
//     /// Move will Clone the Value at stack[src] and push to stack[dst]
//     Move(u8, u8),

//     /// ## Call(func_idx,_) //TODO ignore the second params now
//     /// 
//     /// Call the function at stack[func_index]
//     Call(u8, u8),
// }  

#[derive(Debug)]
pub enum ByteCode {
    // local variable
    LoadConst(u8, u16),
    LoadNil(u8, u8),
    LoadBool(u8, bool),
    LoadInt(u8, i16),
    Move(u8, u8),

    // upvalues
    GetUpvalue(u8, u8),
    SetUpvalue(u8, u8),
    SetUpvalueConst(u8, u8),
    Close(u8),

    // table
    NewTable(u8, u8, u8),
    SetTable(u8, u8, u8),
    SetField(u8, u8, u8),
    SetInt(u8, u8, u8),
    SetTableConst(u8, u8, u8),
    SetFieldConst(u8, u8, u8),
    SetIntConst(u8, u8, u8),
    SetList(u8, u8),
    GetTable(u8, u8, u8),
    GetField(u8, u8, u8),
    GetInt(u8, u8, u8),
    GetFieldSelf(u8, u8, u8),

    // upvalue table, covers global variables
    SetUpField(u8, u8, u8),
    SetUpFieldConst(u8, u8, u8),
    GetUpField(u8, u8, u8),

    // condition structures
    Jump(i16),
    TestAndJump(u8, i16),
    TestOrJump(u8, i16),
    TestAndSetJump(u8, u8, u8),
    TestOrSetJump(u8, u8, u8),

    // for-loop
    ForPrepare(u8, u16),
    ForLoop(u8, u16),
    ForCallLoop(u8, u8, u8),

    // function call
    Closure(u8, u16),
    Call(u8, u8, u8),
    CallSet(u8, u8, u8),
    TailCall(u8, u8),
    Return0,
    Return(u8, u8),
    VarArgs(u8, u8),

    // unops
    Neg(u8, u8),
    Not(u8, u8),
    BitNot(u8, u8),
    Len(u8, u8),

    // binops
    Add(u8, u8, u8),
    AddConst(u8, u8, u8),
    AddInt(u8, u8, u8),
    Sub(u8, u8, u8),
    SubInt(u8, u8, u8),
    SubConst(u8, u8, u8),
    Mul(u8, u8, u8),
    MulInt(u8, u8, u8),
    MulConst(u8, u8, u8),
    Mod(u8, u8, u8),
    ModInt(u8, u8, u8),
    ModConst(u8, u8, u8),
    Div(u8, u8, u8),
    DivInt(u8, u8, u8),
    DivConst(u8, u8, u8),
    Idiv(u8, u8, u8),
    IdivInt(u8, u8, u8),
    IdivConst(u8, u8, u8),
    Pow(u8, u8, u8),
    PowInt(u8, u8, u8),
    PowConst(u8, u8, u8),
    BitAnd(u8, u8, u8),
    BitAndInt(u8, u8, u8),
    BitAndConst(u8, u8, u8),
    BitXor(u8, u8, u8),
    BitXorInt(u8, u8, u8),
    BitXorConst(u8, u8, u8),
    BitOr(u8, u8, u8),
    BitOrInt(u8, u8, u8),
    BitOrConst(u8, u8, u8),
    ShiftL(u8, u8, u8),
    ShiftLInt(u8, u8, u8),
    ShiftLConst(u8, u8, u8),
    ShiftR(u8, u8, u8),
    ShiftRInt(u8, u8, u8),
    ShiftRConst(u8, u8, u8),

    Equal(u8, u8, bool),
    EqualInt(u8, u8, bool),
    EqualConst(u8, u8, bool),
    NotEq(u8, u8, bool),
    NotEqInt(u8, u8, bool),
    NotEqConst(u8, u8, bool),
    LesEq(u8, u8, bool),
    LesEqInt(u8, u8, bool),
    LesEqConst(u8, u8, bool),
    GreEq(u8, u8, bool),
    GreEqInt(u8, u8, bool),
    GreEqConst(u8, u8, bool),
    Less(u8, u8, bool),
    LessInt(u8, u8, bool),
    LessConst(u8, u8, bool),
    Greater(u8, u8, bool),
    GreaterInt(u8, u8, bool),
    GreaterConst(u8, u8, bool),

    SetFalseSkip(u8),

    Concat(u8, u8, u8),
}