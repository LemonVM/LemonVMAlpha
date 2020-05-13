// ===== STACK ===== 0x40-0x5F
use super::*;
// CC stands for closre capture
pub const GETCC: u8 = 0x40;
pub const SETCC: u8 = 0x41;

pub const GETROW: u8 = 0x42;
pub const SETROW: u8 = 0x43;
pub const NEWROW: u8 = 0x44; //heap

pub const CLOSURE: u8 = 0x45;
pub const VARG: u8 = 0x46;

pub const MOVE: u8 = 0x48;
pub const SWAP: u8 = 0x4a;

pub const TYPEOF: u8 = 0x49;

pub const DUP1: u8 = 0x4b;
pub const DUP2: u8 = 0x4c;
pub const FIXTOP: u8 = 0x4d;

pub const NEWTHREAD: u8 = 0x4e;
pub const GETYIELD: u8 = 0x4f;
pub const GETTRET: u8 = 0x50;

pub const REF:u8 = 0x51;
pub const UNREF:u8 = 0x52; // copy
pub const MODREF:u8 = 0x55;

pub const POP:u8 = 0x53;
pub const REMV:u8 = 0x54;
pub const GETERR:u8 = 0x55;

pub const CLOSURE_OP: Op = Op::FIX(FixOp{op:CLOSURE,opmode:FixOpMode::AX(RP)});
// only used before return
// example
    // fixtop 0x00
    // return
// it will return address 0x00
pub const FIXTOP_OP: Op = Op::FIX(FixOp{op:FIXTOP,opmode:FixOpMode::A(RS)});
// A new thread needs a Closure and push a Thread type to stack top
pub const NEWTHREAD_OP: Op = Op::FIX(FixOp {op:NEWTHREAD,opmode:FixOpMode::A(RS)});
pub const GETYIELD_OP:Op = Op::FIX(FixOp {op:GETYIELD,opmode:FixOpMode::A(RS)});
pub const GETTRET_OP:Op = Op::FIX(FixOp {op:GETTRET,opmode:FixOpMode::A(RS)});
pub const GETERR_OP:Op = Op::FIX(FixOp {op:GETTRET,opmode:FixOpMode::A(RS)});