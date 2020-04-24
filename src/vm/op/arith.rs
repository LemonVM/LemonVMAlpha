// ===== ARITH ===== 0x60-0x8F
use super::*;
// SIMD
pub const NEGM: u8 = 0x60;
pub const ADDM: u8 = 0x61;
pub const SUBM: u8 = 0x62;
pub const MULM: u8 = 0x63;
pub const MODM: u8 = 0x64;
pub const POWM: u8 = 0x65;
pub const DIVM: u8 = 0x66;

// NUM // auto cast upper: bool -> char -> int -> num
pub const NEG: u8 = 0x68;
pub const ADD: u8 = 0x69;
pub const SUB: u8 = 0x6a;
pub const MUL: u8 = 0x6b;
pub const MOD: u8 = 0x6c;
pub const POW: u8 = 0x6d;
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

pub const NEGM_OP: Op = Op::FIX(FixOp{op:NEGM,opmode:FixOpMode::AB(R,M)});
pub const ADDM_OP: Op = Op::FIX(FixOp{op:ADDM,opmode:FixOpMode::ABC(R,M,M)});
pub const SUBM_OP: Op = Op::FIX(FixOp{op:SUBM,opmode:FixOpMode::ABC(R,M,M)});
pub const MULM_OP: Op = Op::FIX(FixOp{op:MULM,opmode:FixOpMode::ABC(R,M,M)});
pub const MODM_OP: Op = Op::FIX(FixOp{op:MODM,opmode:FixOpMode::ABC(R,M,M)});
pub const POWM_OP: Op = Op::FIX(FixOp{op:POWM,opmode:FixOpMode::ABC(R,M,M)});
pub const DIVM_OP: Op = Op::FIX(FixOp{op:DIVF,opmode:FixOpMode::ABC(R,M,M)});

pub const NEG_OP: Op = Op::FIX(FixOp{op:NEG,opmode:FixOpMode::AB(R,V)});
pub const ADD_OP: Op = Op::FIX(FixOp{op:ADD,opmode:FixOpMode::ABC(R,V,V)});
pub const SUB_OP: Op = Op::FIX(FixOp{op:SUB,opmode:FixOpMode::ABC(R,V,V)});
pub const MUL_OP: Op = Op::FIX(FixOp{op:MUL,opmode:FixOpMode::ABC(R,V,V)});
pub const MOD_OP: Op = Op::FIX(FixOp{op:MOD,opmode:FixOpMode::ABC(R,V,V)});
pub const POW_OP: Op = Op::FIX(FixOp{op:POW,opmode:FixOpMode::ABC(R,V,V)});
pub const DIV_OP: Op = Op::FIX(FixOp{op:DIV,opmode:FixOpMode::ABC(R,V,V)});

pub const NOT_OP: Op = Op::FIX(FixOp{op:NOT,opmode:FixOpMode::AB(R,R)});
pub const BAND_OP: Op = Op::FIX(FixOp{op:BAND,opmode:FixOpMode::ABC(R,V,V)});
pub const BOR_OP: Op = Op::FIX(FixOp{op:BOR,opmode:FixOpMode::ABC(R,V,V)});
pub const BXOR_OP: Op = Op::FIX(FixOp{op:BXOR,opmode:FixOpMode::ABC(R,V,V)});
pub const SHL_OP: Op = Op::FIX(FixOp{op:SHL,opmode:FixOpMode::ABC(R,V,V)});
pub const SHR_OP: Op = Op::FIX(FixOp{op:SHR,opmode:FixOpMode::ABC(R,V,V)});
pub const BNOT_OP: Op = Op::FIX(FixOp{op:BNOT,opmode:FixOpMode::AB(R,R)});
pub const LEN_OP: Op = Op::FIX(FixOp{op:LEN,opmode:FixOpMode::AB(R,R)});
