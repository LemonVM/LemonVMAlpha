use super::*;
#[repr(C)]
#[derive(Clone)]
pub struct Label {
    pub label: u16,
    pub instructions: Vec<*const u8>,
}
impl std::fmt::Debug for Label{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is = vec!();
        for i in self.instructions.clone(){
            let u = unsafe{*(i as *const u32)};
            is.push(format!("0x{:08X}",u32::from_be(u)));
        }
        f.write_fmt(format_args!("{:?}",is))
    }
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
        for label in 0..self.instruction_table.len() {
            instructions += format!(
                "  0x{:04X}:  {:?}\n",
                u16::from_be(label as u16),
                self.instruction_table[label]
            )
            .as_str();
        }
        writeln!(
            f,
            "===== Prototype =====
< arg_types: {:?}, ret_types: {:?} >
( params: {}, is_vargs?: {}, rets:{} )
{{ number of labels: {} }}
sub functions: {}
instructions:
{}",
            self.arg_types,self.ret_types,
            self.params,
            self.is_vargs != 0,
            self.rets,
            self.instruction_table.len(),
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