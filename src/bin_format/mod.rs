pub mod reader;
pub mod writer;
pub mod func_type;
pub mod constant_and_pool;

use func_type::*;
use std::collections::HashMap;

mod test;

pub const SIG: [u8; SIG_LEN] = [0x4c, 0x65, 0x4d, 0x30, 0x26];
pub const SIG_LEN: usize = 5;
pub const VERSION: u8 = 0x01;
pub const INSSIZE: u8 = 4;
pub const VM_CHAR_SIZE: u8 = 2; // u16
pub const VM_INT_SIZE: u8 = 4; // u32
pub const VM_FLOAT_SIZE: u8 = 8; //f64

pub const TAG_NULL: u8 = 0x00;
pub const TAG_BOOL: u8 = 0x01;
pub const TAG_CHAR: u8 = 0x02;
pub const TAG_INT: u8 = 0x03;
pub const TAG_NUM: u8 = 0x04;
pub const TAG_SYM: u8 = 0x05;
pub const TAG_SIMDCHAR: u8 = 0x06;
pub const TAG_SIMDINT: u8 = 0x07;
pub const TAG_SIMDNUM: u8 = 0x08;
// ROW in constant pool ROW will only consist of pure data above
pub const TAG_ROW: u8 = 0x09;
pub const TAG_USERDATA: u8 = 0x10;
pub const TAG_FUNC:u8 = 0x11;
pub type VMBool = bool;
pub type VMChar = u16;
pub type VMInt = u32;
pub type VMNum = f64;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VMSym(pub Vec<VMChar>);
impl std::fmt::Display for VMSym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\"{}\"",
            String::from_utf16(self.0.as_ref()).unwrap()
        ))
    }
}
impl std::fmt::Debug for VMSym {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct BinaryChunk {
    pub header: Header,
    pub up_value_size: u8,
    pub entry: FuncType,
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct Header {
    sig: [u8; SIG_LEN],
    version: u8,
    instruction_size: u8,
    // default U16
    sizeof_vm_char: u8,
    // default U32
    sizeof_vm_int: u8,
    // defualt f64
    sizeof_vm_number: u8,
    //TODO:SIMD support
}
impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let mut chars = vec![];
        for i in self.sig.iter() {
            if i.is_ascii() {
                chars.push(i.clone() as char);
            }
        }
        let sig: String = chars.iter().collect();
        writeln!(f, "[ sig: {} | version: {} | instruction_size: {} | sizeof_vm_char: {} | sizeof_vm_int: {} | sizeof_vm_number: {} ]",sig,self.version,self.instruction_size,self.sizeof_vm_char,self.sizeof_vm_int,self.sizeof_vm_number)
    }
}

impl Header {
    pub fn validate(&self) -> bool {
        let mut status = true;
        for i in 0..SIG_LEN {
            status = self.sig[i] == SIG[i];
        }
        status = self.version == VERSION;
        status = self.instruction_size == INSSIZE;
        status = self.sizeof_vm_char == VM_CHAR_SIZE;
        status = self.sizeof_vm_int == VM_INT_SIZE;
        status = self.sizeof_vm_number == VM_FLOAT_SIZE;
        status
    }
}
use arrayvec::*;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Null, // 0x00
    Mono(u8), // 0x01 0x??
    
    Poly(Box<Type>,Vec<Type>), // 0x02 type types
    Hole(u8), // 0xFF 0x?? 
    Arrow(Vec<Type>,Vec<Type>), // 0x03 types types
    
    Row(Vec<(VMSym,Type)>), // 0x04 len (sym,type)*
}

use std::collections::HashSet;
impl Type{
    pub fn holes_count(&self)->HashSet<Type>{
        use Type::*;
        if let Poly(h,ts) = self.clone(){
            match *h{
                Arrow(args,rets) => {
                    
                    let mut ret = HashSet::new();
                    for t in args{
                        ret.insert(t);
                    }
                    for r in rets{
                        ret.insert(r);
                    }
                    return ret;
                },
                Row(rs) => {
                    
                    let mut ret = HashSet::new();
                    for (_,t) in rs{
                        ret.insert(t);
                    }
                    return ret;
                }
                _ => panic!("ERROR! TEMPLATE ARGUMENTS OCCURS IN OTHER TYPE")
            }
        }else{
            return HashSet::new();
        }
    }
    pub fn holes_fill(&mut self,len:u8){
        use Type::*;
        let mut nt = self.clone();
        if let Poly(h,ts) = &mut nt{
            if ts.len() != len as usize{
                panic!("ERROR! LENGTH OF TEMPLATE DID NOT MATCH");
            }
            match &mut **h{
                Arrow(args,rets) => {
                    for i in 0..args.len(){
                        if let Hole(n) = args[i] {
                            args[i] = ts[n as usize].clone();
                        }
                    }
                    for i in 0..rets.len(){
                        if let Hole(n) = rets[i] {
                            rets[i] = ts[n as usize].clone();
                        }
                    }
                },
                Row(rs) => {
                    for i in 0..rs.len(){
                        if let Hole(n) = rs[i].1 {
                            rs[i].1 = ts[n as usize].clone();
                        }
                    }
                }
                _ => panic!("ERROR! TEMPLATE ARGUMENTS OCCURS IN OTHER TYPE")
            }
            *self = (*(*h)).clone();
        }else{
            eprintln!("I dont think normal types have holes");
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    pub arr: Vec<constant_and_pool::Constant>,
    pub row: HashMap<VMSym, constant_and_pool::Constant>,
    pub is_arr: bool,
}