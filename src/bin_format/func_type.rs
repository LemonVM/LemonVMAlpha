use super::*;
#[repr(C)]
#[derive(Clone, Debug)]
pub struct Label {
    pub label: u16,
    pub instructions: Vec<*const u8>,
}
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct ClosureCap {
    instack: u8,
    idx: u8,
}

#[repr(C)]
#[derive(Clone, Debug)]
// now directly read from global constant pool
pub struct FuncType {
    // use uuid for finding constant in common constant pool
    // could reduce the size of single binary file
    pub name: VMSym,
    pub uuid: u32,
    pub params: u8,
    pub is_vargs: u8,
    pub rets: u8,
    pub arg_types : Vec<Type>,
    pub ret_types : Vec<Type>,
    pub instruction_table: Vec<Label>,
    pub const_func_refs: Vec<(u8,u32)>,
    // -- debug
    pub debug_local_variables: Vec<LocalVar>,
}
unsafe impl Send for FuncType {}
unsafe impl Sync for FuncType {}

impl PartialEq for FuncType {
    fn eq(&self, other: &FuncType) -> bool {
        return self.uuid == other.uuid;
    }
}
impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut instructions = String::new();
        for pc in 0..self.instruction_table.len() {
            instructions += format!(
                "\t{}\t{:?}\n",
                pc + 1,
                self.instruction_table[pc]
            )
            .as_str();
        }
        writeln!(
            f,
            "===== Prototype =====
( params: {}, is_vargs?: {}, rets:{} )
{{ number of labels: {} }}
varialbles: {} functions: {}
instructions:
{}",
            self.params,
            self.is_vargs != 0,
            self.rets,
            self.instruction_table.len(),
            self.debug_local_variables.len(),
            self.const_func_refs.len(),
            instructions
        )
    }
}

// FOR DEBUG
#[repr(C)]
#[derive(Clone, PartialEq, Debug)]
pub struct LocalVar {
    pub name: VMSym,
    pub start_pc: VMInt,
    pub end_pc: VMInt,
}