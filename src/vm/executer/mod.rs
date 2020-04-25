pub mod stack;
pub mod state;

use super::super::bin_format::*;
#[derive(Debug,Clone,PartialEq)]
pub enum PrimeType{
    Null,
    Bool(bool),
    Char(VMChar),
    Int(VMInt),
    Num(VMNum),
    Sym(VMSym),

    SIMDInt(VMInt,VMInt,VMInt,VMInt),
    SIMDNum(VMNum,VMNum,VMNum,VMNum),
    SIMDChar(VMChar,VMChar,VMChar,VMChar),
    
    UserData(*mut u8), //TODO:完成这玩意儿
    Row(),//TODO: 完成这玩意儿
    Closure(),//TODO: 完成这玩意儿
    Thread(),//TODO: 完成这玩意儿
}