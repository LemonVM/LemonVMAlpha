pub mod reader;
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

pub type VMChar = u16;
pub type VMInt = u32;
pub type VMNum = f64;
pub type VMSym = Vec<VMChar>;

use std::collections::HashMap;
use std::sync::RwLock;
lazy_static!{
    pub static ref CONSTANT_POOL: RwLock<ConstantPool> = RwLock::new(ConstantPool{pool:HashMap::new()});
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct BinaryChunk {
    header: Header,
    up_value_size: u8,
    entry: Prototype,
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

fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
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

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct ClosureCap {
    instack: u8,
    idx: u8,
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct LocalVar {
    name: VMSym,
    start_pc: VMInt,
    end_pc: VMInt,
}

#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub enum Constant {
    Null,
    Bool(bool),
    Char(VMChar),
    Int(VMInt),
    Num(VMNum),
    Sym(VMSym),
}

#[repr(C)]
#[derive(Debug)]
// layout: symbol len constants
pub struct ConstantPool {
    pub pool: HashMap<VMSym, Vec<Constant>>,
}

#[repr(C)]
#[derive(Clone, Debug)]
// now directly read from global constant pool
pub struct Prototype {
    // use uuid for finding constant in common constant pool
    // could reduce the size of single binary file
    uuid: VMSym,
    line_start: VMInt,
    line_end: VMInt,
    params: u8,
    is_vargs: u8,
    stack_size: u8,
    pub instruction_table: Vec<VMInt>,
    // closure captured outer variable
    closure_caps: Vec<ClosureCap>,
    protos: Vec<Prototype>,
    // -- debug
    debug_line_info: Vec<VMInt>,
    debug_local_variables: Vec<LocalVar>,
    debug_closure_cap_names: Vec<VMSym>,
}
impl PartialEq for Prototype {
    fn eq(&self, other: &Prototype) -> bool {
        return self.uuid == other.uuid;
    }
}
impl std::fmt::Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut instructions = String::new();
        for pc in 0..self.instruction_table.len() {
            let line = self
                .debug_line_info
                .get(pc)
                .map(|n| n.to_string())
                .unwrap_or(String::new());
            instructions += format!(
                "\t{}\t[{}]\t0x{:08X}\n",
                pc + 1,
                line,
                self.instruction_table[pc]
            )
            .as_str();
        }
        writeln!(f,"===== Prototype =====\n  < from line: {} ,to line: {} > ( params: {} ,is_vargs?: {} )\n  {{ stack size: {} ,number of instructions: {} }}\n   number of constants: {} varialbles: {} closure_caps: {} functions: {}\n\tinstructions: \n{}",self.line_start,self.line_end,self.params,self.is_vargs!=0,self.stack_size,self.instruction_table.len(),CONSTANT_POOL.read().unwrap().pool.len(),self.debug_local_variables.len(),self.closure_caps.len(),self.protos.len(),instructions)
    }
}
impl Prototype {
    // two clone is kind of inefficiant
    pub fn get_constants(&self) -> Vec<Constant> {
        let uuid = self.uuid.clone();
        CONSTANT_POOL.read().unwrap().pool[&uuid].clone()
    }
}
