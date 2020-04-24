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

fn execute(ins:u8,state:usize){
    // vm
    if ins == 0{}
    // load
    else if ins < 0 && ins > 0{}
    // cf
    else if ins < 0 && ins > 0{}
    // comp
    else if ins < 0 && ins > 0{}
    // num
    else if ins < 0 && ins > 0{}
    // stack
    else if ins < 0 && ins > 0{}
    // user def
    else if ins < 0 && ins > 0{}
    // debug
    else if ins < 0 && ins > 0{}
    else{
        panic!("ERROR INSTRUCTION '0x{:02X}' NOT SUPPORTED", ins);
    }
}