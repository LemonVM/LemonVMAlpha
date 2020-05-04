use super::*;

// ===== CONTROL FLOW ===== 0x20-0x3F
pub const CALLCC: u8 = 0x26;

// ===== LOAD ===== 0x01-0x1F
pub const LOADDYN: u8 = 0x04;

//----------
pub const CALLCC_OP: Op = Op::FIX(FixOp { op: CALLCC, opmode: FixOpMode::ABC(RS, RS, RS) });
pub const LOADDYN_OP: Op = Op::FIX(FixOp { op: LOADDYN, opmode: FixOpMode::AB(RS, RS) });