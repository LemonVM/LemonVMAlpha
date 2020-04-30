use super::*;
// ===== CONTROL FLOW ===== 0x20-0x3F
//pub const CONCAT:u8 =0x4d;
pub const JMP: u8 = 0x20; // TODO: label of line?
pub const UFCALL: u8 = 0x21;
pub const CALL: u8 = 0x22;
pub const TAILCALL: u8 = 0x23;
pub const RET: u8 = 0x24;
pub const RETURN: u8 = 0x25;

//----------
pub const JMP_OP: Op = Op::FIX(FixOp{op:JMP,opmode:FixOpMode::AIBX(RS,VI as u16)});
pub const UFCALL_OP: Op = Op::FIX(FixOp{op:UFCALL,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const CALL_OP: Op = Op::FIX(FixOp{op:CALL,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const TAILCALL_OP: Op = Op::FIX(FixOp{op:TAILCALL,opmode:FixOpMode::ABC(RS,RS,RS)});
pub const RET_OP: Op = Op::FIX(FixOp{op:RET,opmode:FixOpMode::None});
pub const RETURN_OP: Op = Op::FIX(FixOp{op:RET,opmode:FixOpMode::None}); //TODO: MULTIPLE RETURN VALUE
