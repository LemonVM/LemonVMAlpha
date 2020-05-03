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
