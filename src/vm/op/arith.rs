// ===== ARITH ===== 0x60-0x8F
use super::*;
// SIMD
pub const NEGM: u8 = 0x60;
pub const ADDM: u8 = 0x61;
pub const SUBM: u8 = 0x62;
pub const MULM: u8 = 0x63;
pub const MODM: u8 = 0x64;
pub const DIVM: u8 = 0x66;

// NUM // auto cast upper: bool -> char -> int -> num
pub const NEG: u8 = 0x68;
pub const ADD: u8 = 0x69;
pub const SUB: u8 = 0x6a;
pub const MUL: u8 = 0x6b;
pub const MOD: u8 = 0x6c;
pub const DIV: u8 = 0x6e;
pub const IDIV: u8 = 0x6f;
// Bin
pub const BNOT: u8 = 0x70;
pub const BAND: u8 = 0x71;
pub const BOR: u8 = 0x72;
pub const BXOR: u8 = 0x73;
pub const SHL: u8 = 0x74;
pub const SHR: u8 = 0x75;
pub const LEN: u8 = 0x76;

pub const NEGM_OP: Op = Op::FIX(FixOp{op:NEGM,opmode:FixOpMode::AB(RS,RS)});
pub const ADDM_OP: Op = Op::FIX(FixOp{op:ADDM,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const SUBM_OP: Op = Op::FIX(FixOp{op:SUBM,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const MULM_OP: Op = Op::FIX(FixOp{op:MULM,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const MODM_OP: Op = Op::FIX(FixOp{op:MODM,opmode:FixOpMode::ABC(RS,RS,RS)});

pub const DIVM_OP: Op = Op::FIX(FixOp{op:DIVM,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const NEG_OP: Op = Op::FIX(FixOp{op:NEG,opmode:FixOpMode::AB(RS,RS)});
//                                                              dst src src
pub const ADD_OP: Op = Op::FIX(FixOp{op:ADD,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const SUB_OP: Op = Op::FIX(FixOp{op:SUB,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const MUL_OP: Op = Op::FIX(FixOp{op:MUL,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const MOD_OP: Op = Op::FIX(FixOp{op:MOD,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const DIV_OP: Op = Op::FIX(FixOp{op:DIV,opmode:FixOpMode::ABC(RS,RS,RS)});

pub const BAND_OP: Op = Op::FIX(FixOp{op:BAND,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const BOR_OP: Op = Op::FIX(FixOp{op:BOR,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const BXOR_OP: Op = Op::FIX(FixOp{op:BXOR,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const SHL_OP: Op = Op::FIX(FixOp{op:SHL,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const SHR_OP: Op = Op::FIX(FixOp{op:SHR,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const BNOT_OP: Op = Op::FIX(FixOp{op:BNOT,opmode:FixOpMode::AB(RS,RS)});
pub const LEN_OP: Op = Op::FIX(FixOp{op:LEN,opmode:FixOpMode::AB(RS,RS)});

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
macro_rules! binary_expr {
    ($name:ident,$s1:tt) => {
        #[inline]
        pub fn $name(a:Value,b:Value)->Value{
            let Value(v1,t1) = a.clone();
            let Value(v2,t2) = b.clone();
            use PrimeValue::*;
            match t1{
                Type::Kind => {b},
                Mono(TAG_CHAR) => {
                    match t2{
                        Type::Kind => {a},
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
                        Type::Kind => {a},
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
                        Type::Kind => {a},
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


binary_expr!(add,+);
binary_expr!(sub,-);
binary_expr!(mul,*);
binary_expr!(div,/);
binary_expr!(modu,%);