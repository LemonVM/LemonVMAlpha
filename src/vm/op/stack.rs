// ===== STACK ===== 0x40-0x5F
use super::*;
// CC stands for closre capture
pub const GETCC: u8 = 0x40;
pub const SETCC: u8 = 0x41;
pub const GETCCSFT:u8 =0x06;
pub const SETCCSFT:u8 =0x08;
pub const GETROW: u8 = 0x42;
pub const SETROW: u8 = 0x43;
pub const NEWROW: u8 = 0x44;

pub const CLOSURE: u8 = 0x45;
pub const VARG: u8 = 0x46;

pub const MOVE: u8 = 0x48;
pub const SWAPREG: u8 = 0x49;
pub const SWAPSTACK: u8 = 0x4a;

pub const DUP1: u8 = 0x4b;
pub const DUP2: u8 = 0x4c;
pub const FIXTOP: u8 = 0x4d;

pub const CLOSURE_OP: Op = Op::FIX(FixOp{op:CLOSURE,opmode:FixOpMode::AX(RP)});
// only used before return
// example
    // fixtop 0x00
    // return
// it will return address 0x00
pub const FIXTOP_OP: Op = Op::FIX(FixOp{op:FIXTOP,opmode:FixOpMode::A(RS)});