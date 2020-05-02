pub mod stack;
pub mod state;

use super::super::bin_format::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Value(PrimeValue, Type);

#[derive(Debug,Clone)]
pub struct Closure{
    func: Box<super::super::bin_format::func_type::FuncType>,
    args_types: Vec<Type>,
    return_types: Vec<Type>,
    current_label_number: u16 // this is not the label name
}
impl Closure{
    fn new(func:Box<super::super::bin_format::func_type::FuncType>,args_types:Vec<Type>,return_types:Vec<Type>)->Closure{
        Closure{func,current_label_number:0,args_types,return_types}
    }
}
impl PartialEq for Closure{
    fn eq(&self, other: &Self) -> bool {
        (*(self.func)).uuid == (*(other.func)).uuid
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
impl From<super::super::bin_format::constant_and_pool::Constant> for PrimeValue {
    fn from(f: super::super::bin_format::constant_and_pool::Constant) -> Self {
        match f {
            super::super::bin_format::constant_and_pool::Constant::Null => Self::Null,
            super::super::bin_format::constant_and_pool::Constant::Bool(b) => Self::Bool(b),
            super::super::bin_format::constant_and_pool::Constant::Int(i) => Self::Int(i),
            super::super::bin_format::constant_and_pool::Constant::Num(n) => Self::Num(n),
            super::super::bin_format::constant_and_pool::Constant::Sym(s) => Self::Sym(s),

            super::super::bin_format::constant_and_pool::Constant::SIMDInt(i1, i2, i3, i4) => {
                Self::SIMDInt(i1, i2, i3, i4)
            }
            super::super::bin_format::constant_and_pool::Constant::SIMDNum(n1, n2, n3, n4) => {
                Self::SIMDNum(n1, n2, n3, n4)
            }
            super::super::bin_format::constant_and_pool::Constant::SIMDChar(c1, c2, c3, c4) => {
                Self::SIMDChar(c1, c2, c3, c4)
            }

            super::super::bin_format::constant_and_pool::Constant::Row(r) => Self::Row(Row::from(r)),
            super::super::bin_format::constant_and_pool::Constant::Proto(p) => Self::Closure(Closure::new(Box::new(p.clone()),p.arg_types,p.ret_types))
        }
    }
}
impl From<super::super::bin_format::constant_and_pool::Constant> for Value {
    fn from(f: super::super::bin_format::constant_and_pool::Constant) -> Self {
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
                        Self::Poly(Box::new(Self::Mono(TAG_ROW)),vec![r.arr[0].clone().1])
                    }
                } else {
                    let mut t = vec![];
                    r.row.iter().for_each(|(k, v)| t.push(v.clone().1));
                    Self::Poly(Box::new(Self::Mono(TAG_ROW)),t)
                }
            }
            Closure(c) => Self::Mono(TAG_FUNC),//TODO: 完成这玩意儿
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
