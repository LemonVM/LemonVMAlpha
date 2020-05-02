use super::*;
// ===== CONTROL FLOW ===== 0x20-0x3F
//pub const CONCAT:u8 =0x4d;
pub const JMP: u8 = 0x20; // TODO: label of line?
pub const JPE:u8 = 0x26; // TODO: add boolean register
pub const JPN:u8 = 0x27;
pub const UFCALL: u8 = 0x21;
pub const CALL: u8 = 0x22;
pub const TAILCALL: u8 = 0x23;
pub const RET: u8 = 0x24;
pub const RETURN: u8 = 0x25;

//----------
pub const JMP_OP: Op = Op::FIX(FixOp{op:JMP,opmode:FixOpMode::ABX(RS,VI)});
pub const UFCALL_OP: Op = Op::FIX(FixOp{op:UFCALL,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const CALL_OP: Op = Op::FIX(FixOp{op:CALL,opmode:FixOpMode::ABC(RS,RCC,RS)});
pub const TAILCALL_OP: Op = Op::FIX(FixOp{op:TAILCALL,opmode:FixOpMode::None});
pub const RET_OP: Op = Op::FIX(FixOp{op:RET,opmode:FixOpMode::None});
pub const RETURN_OP: Op = Op::FIX(FixOp{op:RET,opmode:FixOpMode::None});
