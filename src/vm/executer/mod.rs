pub mod stack;
pub mod state;
mod test;
use super::super::bin_format::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Value(pub PrimeValue, pub Type);

macro_rules! tylet {
    ($t1:ident,$v1:tt,$t2:ident,$v2:tt,$t3:ident,$t4:ty,$s1:tt) => {
        if let PrimeValue::$t1(c1) = $v1{
            if let PrimeValue::$t2(c2) = $v2{
                let ret = PrimeValue::$t3((c1 as $t4) $s1 (c2 as $t4));
                Value::from(ret)
            }else{
                panic!("ERROR! TYPE NOT MATCH WITH VALUE")
            }
        }else{
            panic!("ERROR! TYPE NOT MATCH WITH VALUE")
        }
    };
}
macro_rules! binary_expr {
    ($name:ident,$s1:tt) => {
        pub fn $name(a:Value,b:Value)->Value{
            use super::super::bin_format::*;
            use super::super::bin_format::Type::*;
            let Value(v1,t1) = a.clone();
            let Value(v2,t2) = b.clone();
            use PrimeValue::*;
            match t1{
                Type::Null => {b},
                Mono(TAG_CHAR) => {
                    match t2{
                        Type::Null => {a},
                        Mono(TAG_CHAR) => {
                            tylet!(Char,v1,Char,v2,Char,VMChar,$s1)
                        },
                        Mono(TAG_INT) => {
                            tylet!(Char,v1,Int,v2,Int,VMInt,$s1)
                        },
                        Mono(TAG_NUM) => {
                            tylet!(Char,v1,Num,v2,Num,VMNum,$s1)
                        },
                        _ => unimplemented!()
                    }
                },
                Mono(TAG_INT) => {
                    match t2{
                        Type::Null => {a},
                        Mono(TAG_CHAR) => {
                            tylet!(Int,v1,Char,v2,Int,VMInt,$s1)
                        },
                        Mono(TAG_INT) => {
                            tylet!(Int,v1,Int,v2,Int,VMInt,$s1)
                        },
                        Mono(TAG_NUM) => {
                            tylet!(Int,v1,Num,v2,Num,VMNum,$s1)
                        },
                        _ => unimplemented!()
                    }
                },
                Mono(TAG_NUM) => {
                    match t2{
                        Type::Null => {a},
                        Mono(TAG_CHAR) => {
                            tylet!(Num,v1,Char,v2,Num,VMNum,$s1)
                        },
                        Mono(TAG_INT) => {
                            tylet!(Num,v1,Int,v2,Num,VMNum,$s1)
                        },
                        Mono(TAG_NUM) => {
                            tylet!(Num,v1,Num,v2,Num,VMNum,$s1)
                        },
                        _ => unimplemented!()
                    }
                },
                _ => unimplemented!()
            }
        }
    };
}
macro_rules! binary_comp_expr {
        ($name:ident,$s1:tt) => {
            pub fn $name(a:Value,b:Value)->Value{
                use super::super::bin_format::*;
                use super::super::bin_format::Type::*;
                let Value(v1,t1) = a.clone();
                let Value(v2,t2) = b.clone();
                use PrimeValue::*;
                match t1{
                    Type::Null => {Value(PrimeValue::Bool(false),Type::Mono(TAG_BOOL))},
                    Mono(TAG_CHAR) => {
                        match t2{
                            Type::Null => {Value(PrimeValue::Bool(false),Type::Mono(TAG_BOOL))},
                            Mono(TAG_CHAR) => {
                                tylet!(Char,v1,Char,v2,Bool,VMChar,$s1)
                            },
                            Mono(TAG_INT) => {
                                tylet!(Char,v1,Int,v2,Bool,VMInt,$s1)
                            },
                            Mono(TAG_NUM) => {
                                tylet!(Char,v1,Num,v2,Bool,VMNum,$s1)
                            },
                            _ => unimplemented!()
                        }
                    },
                    Mono(TAG_INT) => {
                        match t2{
                            Type::Null => {Value(PrimeValue::Bool(false),Type::Mono(TAG_BOOL))},
                            Mono(TAG_CHAR) => {
                                tylet!(Int,v1,Char,v2,Bool,VMInt,$s1)
                            },
                            Mono(TAG_INT) => {
                                tylet!(Int,v1,Int,v2,Bool,VMInt,$s1)
                            },
                            Mono(TAG_NUM) => {
                                tylet!(Int,v1,Num,v2,Bool,VMNum,$s1)
                            },
                            _ => unimplemented!()
                        }
                    },
                    Mono(TAG_NUM) => {
                        match t2{
                            Type::Null => {Value(PrimeValue::Bool(false),Type::Mono(TAG_BOOL))},
                            Mono(TAG_CHAR) => {
                                tylet!(Num,v1,Char,v2,Bool,VMNum,$s1)
                            },
                            Mono(TAG_INT) => {
                                tylet!(Num,v1,Int,v2,Bool,VMNum,$s1)
                            },
                            Mono(TAG_NUM) => {
                                tylet!(Num,v1,Num,v2,Bool,VMNum,$s1)
                            },
                            _ => unimplemented!()
                        }
                    },
                    _ => unimplemented!()
                }
            }
        };
}

binary_expr!(add,+);
binary_expr!(sub,-);
binary_expr!(mul,*);
binary_expr!(div,/);
binary_expr!(modu,%);

binary_comp_expr!(eq,==);
binary_comp_expr!(neq,!=);
binary_comp_expr!(leeq,<=);
binary_comp_expr!(gteq,>=);
binary_comp_expr!(le,<);
binary_comp_expr!(gt,>);


#[derive(Debug,Clone)]
pub struct Closure{
    func: Box<super::super::bin_format::func_type::FuncType>,
    arg_types: Vec<Type>,
    ret_types: Vec<Type>,
    current_label_number: u16 // this is not the label name
}
impl Closure{
    fn new(func:Box<super::super::bin_format::func_type::FuncType>,arg_types:Vec<Type>,ret_types:Vec<Type>)->Closure{
        Closure{func,current_label_number:0,arg_types,ret_types}
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
    Bool(VMBool),
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
            super::super::bin_format::constant_and_pool::Constant::Func(p) => Self::Closure(Closure::new(Box::new(p.clone()),p.arg_types,p.ret_types))
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
                    let t = r.row.iter().map(|(s,v)|(s.clone(),v.1.clone())).collect::<Vec<_>>();
                    Self::Row(t)
                }
            }
            Closure(c) => {
                let ret = Self::Arrow(c.arg_types,c.ret_types);
                if Self::holes_count(&ret).len() > 0{
                    Self::Poly(Box::new(ret),vec!())
                }else{
                    ret
                }
            },//TODO: 完成这玩意儿
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
