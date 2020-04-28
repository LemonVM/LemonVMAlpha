// ===== STACK ===== 0x40-0x5F
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
pub const DUP3: u8 = 0x4d;
