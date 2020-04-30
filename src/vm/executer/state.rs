use super::stack::*;
use std::rc::Rc;
pub struct State {
    pub frames: Vec<Stack>,
}
impl State {
    pub fn stack(&mut self) -> &mut Stack {
        self.frames
            .last_mut()
            .expect("ERROR! FAILED TO GET CURRENT CALL STACK")
    }
    pub fn push_stack(&mut self, stack: Stack) {
        self.frames.push(stack);
    }
    pub fn pop_stack(&mut self) {
        self.frames.pop();
    }
    pub fn new() -> State {
        State { frames: vec![] }
    }
    pub fn ir(&mut self) -> &mut *const u8 {
        &mut self.stack().ir
    }
    pub fn pc(&mut self) -> &mut usize {
        &mut self.stack().pc
    }
    pub fn sub_pc(&mut self, n: usize) {
        *self.pc() -= n;
    }
    pub fn add_pc(&mut self, n: usize) {
        *self.pc() += n;
    }

    pub fn fetch(&mut self) -> Option<*const u8> {
        let pc = self.pc().clone();
        let current_label = self.stack().closure.current_label_number;
        if let Some(instr) = self
            .stack()
            .closure
            .proto
            .instruction_table
            .get(current_label as usize)?
            .instructions
            .clone()
            .get(pc)
        {
            *self.pc() += 1;
            *self.ir() = *instr;
            return Some(*instr);
        } else {
            self.stack().closure.current_label_number += 1;
            *self.pc() = 0;
            self.fetch()
        }
    }

    pub fn load_proto(&mut self, idx: usize) {
        let proto = Box::new(self.stack().closure.proto.const_proto_refs[idx].clone());
        self.stack()
            .push(super::Value::from(super::PrimeValue::from(
                super::super::super::bin_format::get_constant(proto.0, proto.1),
            )));
    }
    pub fn execute(&mut self) {
        loop {
            if let Some(ins) = self.fetch() {
                let iins = unsafe { *ins as u8 };
                println!("IR: 0x{:02x}", iins);
                loop {
                    // vm
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
                                let total_len = offset + len;
                                let tag = unsafe { *(ins.add(3)) };
                                let uuid = unsafe { *(ins.add(3 + 1)) as u32 };
                                use super::super::super::bin_format::get_constant;
                                let cons = get_constant(tag, uuid);
                                self.stack()
                                    .push(Value::from(super::PrimeValue::from(cons)));
                                break;
                            }
                            LOADNULL => {
                                let opmodes = unsafe {
                                    LOADNULL_OP.get_fix().opmode.get_ab(*(ins as *const u32))
                                };
                                let rs1 = opmodes.0;
                                let rs2 = opmodes.1;
                                for i in rs1..rs2 {
                                    self.stack().push(Value::from(super::PrimeValue::Null));
                                }
                                break;
                            }
                            LOADBOOL => unimplemented!(),
                            _ => unimplemented!(),
                        }
                    }
                    // cf
                    else if iins > 0x19 && iins < 0x40 {
                        use super::super::op::cf::*;
                        match iins {
                            JMP => {
                                let value = unsafe { *(ins.add(1) as *const u16) };
                                let label = self
                                    .stack()
                                    .closure
                                    .proto
                                    .instruction_table
                                    .iter()
                                    .position(|r| r.label == value)
                                    .expect("ERROR! LABEL DOES NOT EXIST");
                                self.stack().closure.current_label_number = label as u16;
                                *self.pc() = 0;
                            }
                            UFCALL => {}
                            CALL => {}
                            TAILCALL => {}
                            RET => {
                                return;
                            }
                            RETURN => {}
                            _ => unimplemented!(),
                        }
                        break;
                    }
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
                                let opmodes = unsafe {
                                    NEGM_OP.get_fix().opmode.get_ab(*(ins as *const u32))
                                };
                                let rs1 = opmodes.0;
                                let rs2 = opmodes.1;
                                use super::PrimeValue::*;
                                use super::Value;

                                match &mut self.stack().stack[rs2 as usize].0 {
                                    Null => {
                                        self.stack().stack[rs1 as usize] = Value::from(Null);
                                    }
                                    Char(c) => {
                                        self.stack().stack[rs1 as usize] =
                                            Value::from(Char((-(*c as i16)) as u16))
                                    }
                                    Int(i) => {
                                        self.stack().stack[rs1 as usize] =
                                            Value::from(Int((-(*i as i32)) as u32))
                                    }
                                    Num(n) => {
                                        self.stack().stack[rs1 as usize] = Value::from(Num(-*n))
                                    }
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
            } else {
                return;
            }
        }
    }
}
