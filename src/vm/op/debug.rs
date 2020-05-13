// ===== FOR DEBUG USE ===== 0xE0-0xFF
use super::*;
pub const ASSERTEQ: u8 = 0xE0;
pub const ASSERTNE: u8 = 0xE1;
pub const ASSERTNULL: u8 = 0xE4;
pub const ASSERTERR: u8 = 0xE5;
pub const BREAK: u8 = 0xE6;

pub const BREAK_OP:Op = Op::FIX(FixOp {op:BREAK,opmode:FixOpMode::None});