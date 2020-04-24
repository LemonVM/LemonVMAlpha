use super::*;
// ===== LOAD ===== 0x01-0x1F
pub const LOADK: u8 = 0x01;
pub const LOADKX: u8 = 0xB0;

pub const LOADNULL: u8 = 0x02;
pub const LOADBOOL: u8 = 0x03;
// pub const LOADCHAR: u8 = 0x04;
// pub const LOADINT: u8 = 0x05;
// pub const LOADFLOAT: u8 = 0xB6;
// please do not use unless your sure about what are you doing
// pub const LOADPTR",0x00;
// ----------
pub const LOADK_OP: Op = Op::FIX(FixOp{op:LOADK,opmode:FixOpMode::ABX(0,0)});
pub const LOADKX_OP: Op = Op::VAR(VarOp{op:LOADKX,len:32,offset:0});

pub const LOADNULL_OP: Op = Op::FIX(FixOp{op:LOADNULL,opmode:FixOpMode::None});
pub const LOADBOOL_OP: Op = Op::FIX(FixOp{op:LOADBOOL,opmode:FixOpMode::None});
// pub const LOADCHAR_OP: Op = Op::FIX(FixOp{op:LOADCHAR,opmode:FixOpMode::None});
// pub const LOADINT_OP: Op = Op::FIX(FixOp{op:LOADINT,opmode:FixOpMode::None});
// pub const LOADFLOAT_OP: Op = Op::VAR(VarOp{op:LOADFLOAT,len:64,offset:0});
