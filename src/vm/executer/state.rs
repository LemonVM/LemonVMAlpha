use super::stack::*;
pub struct State {
    pub stack: Stack,
    current_proto: super::super::super::bin_format::Prototype,
    pc: usize,
    ir: *const u8,
}
impl State {
    pub fn new(proto:super::super::super::bin_format::Prototype)->State{
        State{
            stack:Stack::new(),
            current_proto:proto,
            pc:0,
            ir:0 as *const u8,
        }
    }
    pub fn pc(&self) -> usize {
        self.pc
    }
    pub fn sub_pc(&mut self, n: usize) {
        self.pc -= n;
    }
    pub fn add_pc(&mut self, n: usize) {
        self.pc += n;
    }

    pub fn fetch(&mut self) -> *const u8 {
        let instr = self.current_proto.instruction_table[self.pc as usize];
        self.pc += 1;
        self.ir = instr;
        return instr;
    }
    pub fn execute(&mut self) {
        loop {
            let mut ins = self.fetch();
            let iins = unsafe{*ins as u8};
            println!("IR: 0x{:02x}",iins);
            loop {
                // vm
                if iins == 0x24{
                    return;
                }
                if iins == 0x00 {
                    // debug
                    println!("NOP");
                }
                // load
                else if iins > 0x00 && iins < 0x20 {
                    use super::super::op::load::*;
                    use super::Value;
                    match iins {
                        LOADK => {
                            let offset = LOADK_OP.get_var().offset;
                            let len = LOADK_OP.get_var().len;
                            let total_len = offset+len;
                            let tag = unsafe{*(ins.add(3))};
                            let uuid = unsafe{*(ins.add(3+1)) as u32};
                            use super::super::super::bin_format::get_constant;
                            let cons = get_constant(tag, uuid);
                            self.stack.push(Value::from(super::PrimeValue::from(cons)));
                            break;
                        },
                        LOADNULL => {
                            let opmodes = unsafe{LOADNULL_OP.get_fix().opmode.get_ab(*(ins as *const u32))};
                            let rs1 = opmodes.0;
                            let rs2 = opmodes.1;
                            for i in rs1..rs2{
                                self.stack.push(Value::from(super::PrimeValue::Null));
                            }
                            break;
                        },
                        LOADBOOL => {unimplemented!()},
                        _ => {unimplemented!()}
                    }
                }
                // // cf
                // else if ins < 0 && ins > 0 {
                // }
                // // comp
                // else if ins < 0 && ins > 0 {
                // }
                // num
                else if iins > 0x59 && iins < 0x90 {
                    use super::super::op::arith::*;
                    match iins {
                        NEGM => {}
                        ADDM => {}
                        SUBM => {}
                        MULM => {}
                        MODM => {}
                        POWM => {}
                        DIVM => {}
                        NEG => {
                            let opmodes = unsafe{NEGM_OP.get_fix().opmode.get_ab(*(ins as *const u32))};
                            let rs1 = opmodes.0;
                            let rs2 = opmodes.1;
                            use super::PrimeValue::*;
                            use super::Value;
                            match &mut self.stack.stack[rs2 as usize].0 {
                                Null => {
                                    self.stack.stack[rs1 as usize] = Value::from(Null);
                                }
                                Char(c) => {
                                    self.stack.stack[rs1 as usize] = Value::from(Char((-(*c as i16)) as u16))
                                }
                                Int(i) => {
                                    self.stack.stack[rs1 as usize] = Value::from(Int((-(*i as i32)) as u32))
                                }
                                Num(n) => self.stack.stack[rs1 as usize] = Value::from(Num(-*n)),
                                _ => unimplemented!(),
                            }
                            break;
                        }
                        ADD => {}
                        SUB => {}
                        MUL => {}
                        MOD => {}
                        POW => {}
                        DIV => {}
                        IDIV => {}
                        BNOT => {}
                        BAND => {}
                        BOR => {}
                        BXOR => {}
                        SHL => {}
                        SHR => {}
                        LEN => {}
                        _ => panic!("ERROR! INSTRUCTION NOT SUPPORTED"),
                    }
                }
                // // stack
                // else if ins < 0 && ins > 0 {
                // }
                // // user def
                // else if ins < 0 && ins > 0 {
                // }
                // // debug
                // else if ins < 0 && ins > 0 {
                // } else {
                //     panic!("ERROR INSTRUCTION '0x{:02X}' NOT SUPPORTED", ins);
                // }
            }
        }
    }
}
