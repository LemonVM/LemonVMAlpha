use super::*;
// ===== COMP ===== 0x90-0xAF
pub const EQ: u8 = 0x90;
pub const LE: u8 = 0x91;
pub const GT: u8 = 0x92;
pub const NEQ: u8 = 0x93;
pub const LEEQ: u8 = 0x94;
pub const GTEQ: u8 = 0x95;

//                                                             dst src src
pub const EQ_OP: Op = Op::FIX(FixOp{op:EQ,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const LE_OP: Op = Op::FIX(FixOp{op:LE,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const GT_OP: Op = Op::FIX(FixOp{op:GT,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const NEQ_OP: Op = Op::FIX(FixOp{op:NEQ,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const LEEQ_OP: Op = Op::FIX(FixOp{op:LEEQ,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const GTEQ_OP: Op = Op::FIX(FixOp{op:GTEQ,opmode:FixOpMode::ABC(RS,RS,RS)});

use super::super::super::bin_format::*;
use super::super::super::bin_format::Type::*;
use super::super::executer::*;
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
macro_rules! binary_comp_expr {
    ($name:ident,$s1:tt) => {
        pub fn $name(a:Value,b:Value)->Value{
            let Value(v1,t1) = a.clone();
            let Value(v2,t2) = b.clone();
            use PrimeValue::*;
            match t1{
                Type::Kind => {Value(PrimeValue::Bool(false),self::Type::Mono(TAG_BOOL))},
                Mono(TAG_CHAR) => {
                    match t2{
                        Type::Kind => {Value(PrimeValue::Bool(false),self::Type::Mono(TAG_BOOL))},
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
                        Type::Kind => {Value(PrimeValue::Bool(false),self::Type::Mono(TAG_BOOL))},
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
                        Type::Kind => {Value(PrimeValue::Bool(false),self::Type::Mono(TAG_BOOL))},
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

binary_comp_expr!(eq,==);
binary_comp_expr!(neq,!=);
binary_comp_expr!(leeq,<=);
binary_comp_expr!(gteq,>=);
binary_comp_expr!(le,<);
binary_comp_expr!(gt,>);