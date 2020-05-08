pub mod ffi;
pub mod stack;
pub mod state;
// mod test;
use super::super::bin_format::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub PrimeValue, pub Type);
unsafe impl Send for Value {}
unsafe impl Sync for Value {}

#[derive(Clone)]
pub enum FuncInClosure {
    Func(Box<super::super::bin_format::func_type::FuncType>),
    NFunc(*const u8),
}

#[derive(Clone)]
pub struct Closure {
    uuid: u32,
    is_native_func: bool,
    func: FuncInClosure,
    arg_types: Vec<Type>,
    ret_types: Vec<Type>,
    current_label_number: u16, // this is not the label name
}
// struct JoinHandlePtr(pub *mut async_std::task::JoinHandle<()>);
// unsafe impl Send for JoinHandlePtr{}
// unsafe impl Sync for JoinHandlePtr{}
use async_std::sync::{Receiver, Sender};
#[derive(Debug, Clone, PartialEq)]
pub enum PrimeValue {
    Null,
    Bool(VMBool),
    Byte(u8),
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
    NType(Type),      // V just for naming issue, so this is only used in reflection!

    Thread(*const async_std::task::JoinHandle<()>), //TODO: 完成这玩意儿
    Channel(CSender, CReceiver),
}
#[derive(Debug, Clone)]
pub struct CSender(u32, pub Sender<Value>);
#[derive(Debug, Clone)]
pub struct CReceiver(u32, pub Receiver<Value>);
impl std::cmp::PartialEq for CSender {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::PartialEq for CReceiver {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
use std::collections::HashMap;
#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    arr: Vec<Value>,
    row: HashMap<VMSym, Value>,
    is_arr: bool,
}

impl std::fmt::Debug for Closure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("<<Closure: {} >>", self.uuid))
    }
}
impl Closure {
    pub fn new(
        uuid: u32,
        func: FuncInClosure,
        arg_types: Vec<Type>,
        ret_types: Vec<Type>,
    ) -> Closure {
        Closure {
            uuid,
            is_native_func: false,
            func,
            current_label_number: 0,
            arg_types,
            ret_types,
        }
    }
    pub fn func(&self) -> Box<super::super::bin_format::func_type::FuncType> {
        if let FuncInClosure::Func(func) = self.func.clone() {
            func
        } else {
            //考虑JIT咋办
            unimplemented!();
        }
    }
}
impl PartialEq for Closure {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
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

            super::super::bin_format::constant_and_pool::Constant::Row(r) => {
                Self::Row(Row::from(r))
            }
            super::super::bin_format::constant_and_pool::Constant::Func(p) => {
                Self::Closure(Closure::new(
                    p.uuid,
                    FuncInClosure::Func(Box::new(p.clone())),
                    p.arg_types,
                    p.ret_types,
                ))
            }
            super::super::bin_format::constant_and_pool::Constant::NType(t) => Self::NType(t),
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
            Null => Self::Kind,
            Bool(_) => Self::Mono(TAG_BOOL),
            Byte(_) => Self::Mono(TAG_BYTE),
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
                        Self::Kind
                    } else {
                        Self::Poly(Box::new(Self::Mono(TAG_ROW)), vec![r.arr[0].clone().1])
                    }
                } else {
                    let t = r
                        .row
                        .iter()
                        .map(|(s, v)| (s.clone(), v.1.clone()))
                        .collect::<Vec<_>>();
                    Self::Row(t)
                }
            }
            Closure(c) => {
                let ret = Self::Arrow(c.arg_types, c.ret_types);
                if Self::holes_count(&ret).len() > 0 {
                    Self::Poly(Box::new(ret), vec![])
                } else {
                    ret
                }
            } //TODO: 完成这玩意儿
            //Thread(),//TODO: 完成这玩意儿 和 Channel
            NType(t) => Self::Kind,
            _ => unimplemented!(),
        }
    }
}
impl From<PrimeValue> for Value {
    fn from(f: PrimeValue) -> Self {
        Self(f.clone(), Type::from(f))
    }
}
