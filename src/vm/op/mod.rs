#[derive(Debug)]
pub enum FixOpMode {
    None,
    A(u8),      // 0x00 0x00 0x00
    AB(u8, u8), // 0x00
    ABX(u8, u16),
    AIBX(u8, u16),
    ABC(u8, u8, u8),
}
impl FixOpMode{
    fn get_a(&self)->u8{
        use FixOpMode::*;
        if let A(a) = &self {
            *a
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE A");
        }
    }
    fn get_ab(&self)->(u8,u8){
        use FixOpMode::*;
        if let AB(a,b) = &self {
            (*a,*b)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE AB");
        }
    }
    fn get_abx(&self)->(u8,u16){
        use FixOpMode::*;
        if let ABX(a,b) = &self {
            (*a,*b)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE ABX");
        }
    }
    fn get_aibx(&self)->(u8,u16){
        use FixOpMode::*;
        if let AIBX(a,b) = &self {
            (*a,*b)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE AIBX");
        }
    }
    fn get_abc(&self)->(u8,u8,u8){
        use FixOpMode::*;
        if let ABC(a,b,c) = &self {
            (*a,*b,*c)
        }else{
            panic!("ERROR! INSTRUCTION IS NOT AT MODE ABC");
        }
    }
}
#[derive(Debug)]
pub struct FixOp {
    pub op: u8,
    pub opmode: FixOpMode,
}
#[derive(Debug)]
pub struct VarOp {
    pub op: u8,
    pub len: u8,
    pub offset: u8, // len % 4 != 0 fill n byte as offset
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

// in address reg
pub const R:u8 = 0x01; // register stack ref
pub const C:u8 = 0x02; // constant table ref

// in value and simd reg
pub const V:u8 = 0x03; // value
pub const M:u8 = 0x04; // simd value

pub mod vm;
pub mod load;
pub mod arith;
pub mod stack;
pub mod comp;
pub mod debug;