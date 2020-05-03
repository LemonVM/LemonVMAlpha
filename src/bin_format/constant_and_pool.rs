use std::collections::HashMap;
use std::sync::RwLock;
use super::*;
lazy_static! {
    pub static ref CONSTANT_POOL: RwLock<ConstantPool> = RwLock::new(ConstantPool {
        pool_of_int: (TAG_INT, HashMap::new()),
        pool_of_num: (TAG_NUM, HashMap::new()),
        pool_of_sym: (TAG_SYM, HashMap::new()),
        pool_of_simdchar: (TAG_SIMDCHAR, HashMap::new()),
        pool_of_simdint: (TAG_SIMDINT, HashMap::new()),
        pool_of_simdnum: (TAG_SIMDNUM, HashMap::new()),
        pool_of_row: (TAG_ROW, HashMap::new()),
        pool_of_func: (TAG_FUNC, HashMap::new())
    });
}


#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Null,
    Bool(VMBool),
    // up no need pool just need to implement in row
    Int(VMInt),
    Num(VMNum),
    Sym(VMSym),

    SIMDInt(VMInt, VMInt, VMInt, VMInt),
    SIMDNum(VMNum, VMNum, VMNum, VMNum),
    SIMDChar(VMChar, VMChar, VMChar, VMChar),
    Func(FuncType),
    Row(Row), // TODO：完成这厮
}


// layout: types_len (tag len (uuid:data)*)*
//          u8         u8  u32 u8*
// types_len is how many types in this constant pool
#[repr(C)]
#[derive(Debug)]
pub struct ConstantPool {
    pub pool_of_int: (u8, HashMap<u32, Constant>),
    pub pool_of_num: (u8, HashMap<u32, Constant>),
    pub pool_of_sym: (u8, HashMap<u32, Constant>),
    pub pool_of_simdchar: (u8, HashMap<u32, Constant>),
    pub pool_of_simdint: (u8, HashMap<u32, Constant>),
    pub pool_of_simdnum: (u8, HashMap<u32, Constant>),
    pub pool_of_row: (u8, HashMap<u32, Constant>),
    pub pool_of_func:(u8, HashMap<u32, Constant>)
}

pub fn get_constant(tag: u8, uuid: u32) -> Constant {
    match tag {
        TAG_INT => CONSTANT_POOL.read().unwrap().pool_of_int.1[&uuid].clone(),
        TAG_NUM => CONSTANT_POOL.read().unwrap().pool_of_num.1[&uuid].clone(),
        TAG_SYM => CONSTANT_POOL.read().unwrap().pool_of_sym.1[&uuid].clone(),
        TAG_SIMDCHAR => CONSTANT_POOL.read().unwrap().pool_of_simdchar.1[&uuid].clone(),
        TAG_SIMDINT => CONSTANT_POOL.read().unwrap().pool_of_simdint.1[&uuid].clone(),
        TAG_SIMDNUM => CONSTANT_POOL.read().unwrap().pool_of_simdnum.1[&uuid].clone(),
        TAG_ROW => CONSTANT_POOL.read().unwrap().pool_of_row.1[&uuid].clone(),
        TAG_FUNC => CONSTANT_POOL.read().unwrap().pool_of_func.1[&uuid].clone(),
        _ => unimplemented!(),
    }
}