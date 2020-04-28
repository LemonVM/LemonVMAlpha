use super::*;
// ===== CONTROL FLOW ===== 0x20-0x3F
//pub const CONCAT:u8 =0x4d;
pub const JMP: u8 = 0x20;
pub const UFCALL: u8 = 0x21;
pub const CALL: u8 = 0x22;
pub const TAILCALL: u8 = 0x23;
pub const RET: u8 = 0x24;

//----------
pub const JMP_OP: Op = Op::FIX(FixOp{op:JMP,opmode:FixOpMode::AIBX(R,V)});
pub const UFCALL_OP: Op = Op::FIX(FixOp{op:UFCALL,opmode:FixOpMode::ABC(R,R,K)});
pub const CALL_OP: Op = Op::FIX(FixOp{op:CALL_OP,opmode:FixOpMode::ABC(R,V,V)});
pub const TAILCALL_OP: Op = Op::FIX(FixOp{op:TAILCALL,opmode:FixOpMode::ABC(R,V,V)});
pub const RET_OP: Op = Op::FIX(FixOp{op:RET,opmode:FixOpMode::None}); //TODO: discuss multiple return value
