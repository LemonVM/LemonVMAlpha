use super::*;
// ===== LOAD ===== 0x01-0x1F
pub const LOADK: u8 = 0x01;

pub const LOADNULL: u8 = 0x02;
pub const LOADBOOL: u8 = 0x03;
// we open a hole here for dynlib loading (LOADDYN since 0x04)
pub const LOADDYN: u8 = 0x04;
// pub const LOADCHAR: u8 = 0x04;
// pub const LOADINT: u8 = 0x05;
// pub const LOADFLOAT: u8 = 0xB6;
// please do not use unless your sure about what are you doing
// pub const LOADPTR",0x00;
// ----------

// 5:type tag 1 + uuid 4
pub const LOADK_OP: Op = Op::VAR(VarOp{op:LOADK,len:5,offset:0});

pub const LOADNULL_OP: Op = Op::FIX(FixOp{op:LOADNULL,opmode:FixOpMode::AB(RS,RS)});
pub const LOADBOOL_OP: Op = Op::FIX(FixOp{op:LOADBOOL,opmode:FixOpMode::A(RS)});
// pub const LOADCHAR_OP: Op = Op::FIX(FixOp{op:LOADCHAR,opmode:FixOpMode::None});
// pub const LOADINT_OP: Op = Op::FIX(FixOp{op:LOADINT,opmode:FixOpMode::None});
// pub const LOADFLOAT_OP: Op = Op::VAR(VarOp{op:LOADFLOAT,len:64,offset:0});

pub const LOADDYN_OP: Op = Op::FIX(FixOp { op: LOADDYN, opmode: FixOpMode::AB(RS, RS) });
