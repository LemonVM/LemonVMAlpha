use super::*;
// ===== VM =====
pub const NOP: u8 = 0x00;
// ----------
pub const NOP_OP: Op = Op::FIX(FixOp{op:NOP,opmode:FixOpMode::None});