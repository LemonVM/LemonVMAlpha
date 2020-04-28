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
impl From<super::super::bin_format::Constant> for PrimeType{
    fn from(f:super::super::bin_format::Constant) -> Self {
        use super::super::bin_format::Constant::*;
        match f{
            Int(i) => Self::Int(i),
            Num(n) => Self::Num(n),
            Sym(s) => Self::Sym(s),
        
            SIMDInt(i1,i2,i3,i4) => Self::SIMDInt(i1,i2,i3,i4),
            SIMDNum(n1,n2,n3,n4) => Self::SIMDNum(n1,n2,n3,n4),
            SIMDChar(c1,c2,c3,c4) => Self::SIMDChar(c1,c2,c3,c4),
        
            Row() => Self::Row() // TODO：完成这厮
        }
    }
}