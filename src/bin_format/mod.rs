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
pub const TAG_SIMDCHAR: u8 = 0x06;
pub const TAG_SIMDINT: u8 = 0x07;
pub const TAG_SIMDNUM: u8 = 0x08;
// ROW in constant pool ROW will only consist of pure data above
pub const TAG_ROW: u8 = 0x09;
pub const TAG_USERDATA: u8 = 0x10;
pub const TAG_PROTO:u8 = 0x11;

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

use std::collections::HashMap;
use std::sync::RwLock;
lazy_static! {
    pub static ref CONSTANT_POOL: RwLock<ConstantPool> = RwLock::new(ConstantPool {
        pool_of_int: (TAG_INT, HashMap::new()),
        pool_of_num: (TAG_NUM, HashMap::new()),
        pool_of_sym: (TAG_SYM, HashMap::new()),
        pool_of_simdchar: (TAG_SIMDCHAR, HashMap::new()),
        pool_of_simdint: (TAG_SIMDINT, HashMap::new()),
        pool_of_simdnum: (TAG_SIMDNUM, HashMap::new()),
        pool_of_row: (TAG_ROW, HashMap::new()),
        pool_of_proto: (TAG_PROTO, HashMap::new())
    });
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

#[derive(Clone, Debug, PartialEq)]
pub enum Constant {
    Null,
    Bool(u8),
    // up no need pool just need to implement in row
    Int(VMInt),
    Num(VMNum),
    Sym(VMSym),

    SIMDInt(VMInt, VMInt, VMInt, VMInt),
    SIMDNum(VMNum, VMNum, VMNum, VMNum),
    SIMDChar(VMChar, VMChar, VMChar, VMChar),
    Proto(Prototype),
    Row(Row), // TODO：完成这厮
}

#[derive(Clone, PartialEq, Debug)]
pub struct Row {
    pub arr: Vec<Constant>,
    pub row: HashMap<VMSym, Constant>,
    pub is_arr: bool,
}

// layout: types_len (tag len (uuid:data)*)*
//          u8         u8  u32 u8*
// types_len is how many types in this constant pool
#[repr(C)]
#[derive(Debug)]
pub struct ConstantPool {
    pub pool_of_int: (u8, HashMap<u32, Constant>),
    pub pool_of_num: (u8, HashMap<u32, Constant>),
    pub pool_of_sym: (u8, HashMap<u32, Constant>),
    pub pool_of_simdchar: (u8, HashMap<u32, Constant>),
    pub pool_of_simdint: (u8, HashMap<u32, Constant>),
    pub pool_of_simdnum: (u8, HashMap<u32, Constant>),
    pub pool_of_row: (u8, HashMap<u32, Constant>),
    pub pool_of_proto:(u8, HashMap<u32, Constant>)
}
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Label {
    pub label: u16,
    pub instructions: Vec<*const u8>,
}
#[repr(C)]
#[derive(Clone, Debug)]
// now directly read from global constant pool
pub struct Prototype {
    // use uuid for finding constant in common constant pool
    // could reduce the size of single binary file
    name: VMSym,
    pub uuid: u32,
    line_start: VMInt,
    line_end: VMInt,
    params: u8,
    is_vargs: u8,
    stack_size: u8,
    pub instruction_table: Vec<Label>,
    // closure captured outer variable
    closure_caps: Vec<ClosureCap>,
    pub const_proto_refs: Vec<(u8,u32)>,
    // -- debug
    debug_line_info: Vec<VMInt>,
    debug_local_variables: Vec<LocalVar>,
    debug_closure_cap_names: Vec<VMSym>,
}
unsafe impl Send for Prototype{}
unsafe impl Sync for Prototype{}

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
                "\t{}\t[{}]\t{:?}\n",
                pc + 1,
                line,
                self.instruction_table[pc]
            )
            .as_str();
        }
        writeln!(
            f,
            "===== Prototype =====
< from line: {} ,to line: {} > ( params: {} ,is_vargs?: {} )
{{ stack size: {} ,number of labels: {} }}
varialbles: {} closure_caps: {} functions: {}
instructions:
{}",
            self.line_start,
            self.line_end,
            self.params,
            self.is_vargs != 0,
            self.stack_size,
            self.instruction_table.len(),
            self.debug_local_variables.len(),
            self.closure_caps.len(),
            self.const_proto_refs.len(),
            instructions
        )
    }
}
pub fn get_constant(tag: u8, uuid: u32) -> Constant {
    match tag {
        TAG_INT => CONSTANT_POOL.read().unwrap().pool_of_int.1[&uuid].clone(),
        TAG_NUM => CONSTANT_POOL.read().unwrap().pool_of_num.1[&uuid].clone(),
        TAG_SYM => CONSTANT_POOL.read().unwrap().pool_of_sym.1[&uuid].clone(),
        TAG_SIMDCHAR => CONSTANT_POOL.read().unwrap().pool_of_simdchar.1[&uuid].clone(),
        TAG_SIMDINT => CONSTANT_POOL.read().unwrap().pool_of_simdint.1[&uuid].clone(),
        TAG_SIMDNUM => CONSTANT_POOL.read().unwrap().pool_of_simdnum.1[&uuid].clone(),
        TAG_ROW => CONSTANT_POOL.read().unwrap().pool_of_row.1[&uuid].clone(),
        TAG_PROTO => CONSTANT_POOL.read().unwrap().pool_of_proto.1[&uuid].clone(),
        _ => unimplemented!(),
    }
}
