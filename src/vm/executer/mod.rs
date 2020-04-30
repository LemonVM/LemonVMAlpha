pub mod stack;
pub mod state;

use super::super::bin_format::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Null,
    Mono(u8),
    Poly(Vec<Type>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value(PrimeValue, Type);

#[derive(Debug,Clone)]
pub struct Closure{
    proto: Box<super::super::bin_format::Prototype>,
    current_label_number: u16 // this is not the label name
}
impl Closure{
    fn new(proto:Box<super::super::bin_format::Prototype>)->Closure{
        Closure{proto,current_label_number:0}
    }
}
impl PartialEq for Closure{
    fn eq(&self, other: &Self) -> bool {
        (*(self.proto)).uuid == (*(other.proto)).uuid
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimeValue {
    Null,
    Bool(u8),
    Char(VMChar),
    Int(VMInt),
    Num(VMNum),
    Sym(VMSym),

    SIMDInt(VMInt, VMInt, VMInt, VMInt),
    SIMDNum(VMNum, VMNum, VMNum, VMNum),
    SIMDChar(VMChar, VMChar, VMChar, VMChar),
    UserData(*mut u8), //TODO:完成这玩意儿
    Row(Row),
    Closure(Closure), //TODO: 完成这玩意儿
    Thread(),  //TODO: 完成这玩意儿
}

use std::collections::HashMap;
#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    arr: Vec<Value>,
    row: HashMap<VMSym, Value>,
    is_arr: bool,
}
impl From<super::super::bin_format::Row> for Row {
    fn from(f: super::super::bin_format::Row) -> Self {
        Self {
            arr: f.arr.iter().map(|i| Value::from(i.clone())).collect(),
            row: f
                .row
                .iter()
                .map(|(k, v)| (k.clone(), Value::from(v.clone())))
                .collect(),
            is_arr: f.is_arr,
        }
    }
}
impl From<super::super::bin_format::Constant> for PrimeValue {
    fn from(f: super::super::bin_format::Constant) -> Self {
        match f {
            super::super::bin_format::Constant::Null => Self::Null,
            super::super::bin_format::Constant::Bool(b) => Self::Bool(b),
            super::super::bin_format::Constant::Int(i) => Self::Int(i),
            super::super::bin_format::Constant::Num(n) => Self::Num(n),
            super::super::bin_format::Constant::Sym(s) => Self::Sym(s),

            super::super::bin_format::Constant::SIMDInt(i1, i2, i3, i4) => {
                Self::SIMDInt(i1, i2, i3, i4)
            }
            super::super::bin_format::Constant::SIMDNum(n1, n2, n3, n4) => {
                Self::SIMDNum(n1, n2, n3, n4)
            }
            super::super::bin_format::Constant::SIMDChar(c1, c2, c3, c4) => {
                Self::SIMDChar(c1, c2, c3, c4)
            }

            super::super::bin_format::Constant::Row(r) => Self::Row(Row::from(r)),
        }
    }
}
impl From<super::super::bin_format::Constant> for Value {
    fn from(f: super::super::bin_format::Constant) -> Self {
        Self::from(PrimeValue::from(f))
    }
}
impl From<PrimeValue> for Type {
    fn from(f: PrimeValue) -> Self {
        use PrimeValue::*;
        match f {
            Null => Self::Null,
            Bool(_) => Self::Mono(TAG_BOOL),
            Int(_) => Self::Mono(TAG_INT),
            Num(_) => Self::Mono(TAG_NUM),
            Sym(_) => Self::Mono(TAG_SYM),

            SIMDInt(_, _, _, _) => Self::Mono(TAG_SIMDINT),
            SIMDNum(_, _, _, _) => Self::Mono(TAG_SIMDNUM),
            SIMDChar(_, _, _, _) => Self::Mono(TAG_SIMDCHAR),
            UserData(_) => Self::Mono(TAG_USERDATA), //TODO:完成这玩意儿
            Row(r) => {
                if r.is_arr {
                    if r.arr.len() == 0 {
                        Self::Null
                    } else {
                        Self::Poly(vec![r.arr[0].clone().1])
                    }
                } else {
                    let mut t = vec![];
                    r.row.iter().for_each(|(k, v)| t.push(v.clone().1));
                    Self::Poly(t)
                }
            }
            // Closure() => {},//TODO: 完成这玩意儿
            //Thread(),//TODO: 完成这玩意儿
            _ => unimplemented!(),
        }
    }
}
impl From<PrimeValue> for Value {
    fn from(f: PrimeValue) -> Self {
        Self(f.clone(), Type::from(f))
    }
}
