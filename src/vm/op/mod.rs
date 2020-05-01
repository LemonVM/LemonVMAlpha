#[derive(Debug)]
pub enum FixOpMode {
    None,
    A(u8),      // 0x00 0x00 0x00
    AX(u16),    // 0x00
    AB(u8, u8), // 0x00
    ABX(u8, u16),
    ABC(u8, u8, u8),
}
impl FixOpMode{
    pub fn get_a(&self,ins:u32)->u8{
        use FixOpMode::*;
        if let A(a) = &self {
            let a  = (ins >> 8) as u8;
            a
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE A");
        }
    }
    pub fn get_ab(&self,ins:u32)->(u8,u8){
        use FixOpMode::*;
        if let AB(a,b) = &self {
            let a  = (ins >> 8) as u8;
            let b =  (ins >> 16) as u8;
            (a,b)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE AB");
        }
    }
    pub fn get_abx(&self,ins:u32)->(u8,u16){
        use FixOpMode::*;
        if let ABX(a,b) = &self {
            let a  = (ins >> 8) as u8;
            let b =  (ins >> 16) as u16;
            (a,b)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE ABX");
        }
    }
    // pub fn get_aibx(&self,ins:u32)->(u8,u16){
    //     use FixOpMode::*;
    //     if let AIBX(a,b) = &self {
    //         let a  = (ins >> 8) as u8;
    //         let b =  (ins >> 16) as u16;
    //         (a,b)
    //     }else{
    //         panic!("ERROR! INSTRUCTION IS NOT AT MODE AIBX");
    //     }
    // }
    pub fn get_abc(&self,ins:u32)->(u8,u8,u8){
        use FixOpMode::*;
        if let ABC(a,b,c) = &self {
            let a  = (ins >> 8) as u8;
            let b =  (ins >> 16) as u8;
            let c = (ins >> 24) as u8;
            (a,b,c)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE ABC");
        }
    }
}
// tag 0x00 + fix op
#[derive(Debug)]
pub struct FixOp {
    pub op: u8,
    pub opmode: FixOpMode,
}
// tag 0xFF + var op
#[derive(Debug)]
pub struct VarOp {
    pub op: u8,
    pub len: u8,
    pub offset: u8, // len % 4 != 0 fill n byte as offset in front
}
#[derive(Debug)]
pub enum Op {
    FIX(FixOp),
    VAR(VarOp),
}
impl Op {
    pub fn get_fix(self) -> FixOp {
        use Op::*;
        match self {
            FIX(f) => f,
            VAR(_) => panic!("FATAL ERROR! THIS OP SHOULD HAS FIXED LENGTH"),
        }
    }
    pub fn get_var(self) -> VarOp {
        use Op::*;
        match self {
            VAR(v) => v,
            FIX(_) => panic!("FATAL ERROR! THIS OP SHOULD HAS VARIABLE LENGTH"),
        }
    }
}

pub const RS:u8 = 0x01; // register ref stack
pub const RC:u8 = 0x02; // register ref constant table
pub const RP:u16 = 0x0300; // register ref sub protos
pub const RCC:u8 = 0x04; // register ref closures

pub const VI:u16 = 0xFF00; // only use for jump

pub mod vm;
pub mod load;
pub mod cf;
pub mod arith;
pub mod stack;
pub mod comp;
pub mod debug;