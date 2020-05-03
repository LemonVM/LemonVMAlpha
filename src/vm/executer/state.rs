use super::stack::*;
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
    pub fn fetch(&mut self) -> Option<*const u8> {
        let pc = self.pc().clone();
        let current_label = self.stack().closure.current_label_number;
        if let Some(instr) = self
            .stack()
            .closure
            .func
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
    pub fn load_func(&mut self, idx: usize) {
        let func = Box::new(self.stack().closure.func.const_func_refs[idx].clone());
        self.stack()
            .push(super::Value::from(super::PrimeValue::from(
                super::super::super::bin_format::constant_and_pool::get_constant(func.0, func.1),
            )));
    }
    pub fn push_function_frame_and_args(
        &mut self,
        closure: Box<super::Closure>,
        args: Vec<super::Value>,
    ) {
        let mut stack = Stack::new_from_closure(closure);
        if stack.check_ramain_enougth(args.len()) {
            use arrayvec::*;
            use std::iter::FromIterator;
            stack.stack = ArrayVec::from_iter(args);
        }
        self.frames.push(stack);
    }
    // args is how deep on stack top will copy as argument
    pub fn call(&mut self, f: Box<super::Closure>, args: u8) {
        let top = self.stack().top();
        let mut v = vec![];
        for i in 0..args {
            v.push(self.stack().pop())
        }
        self.push_function_frame_and_args(f, v);
        // self.stack().pc = 0
    }
    // move return value to current stack top
    pub fn return_(&mut self) {
        let res = self.stack().fixed_tops();
        // pop function call
        self.frames.pop();
        if !self.stack().check_ramain_enougth(res.len()) {
            panic!("ERROR! STACK OVERFLOWED")
        }
        res.iter().for_each(|r| self.stack().push(r.clone()))
    }
    pub fn execute(&mut self) {
        loop {
            if let Some(ins) = self.fetch() {
                let mut iins = unsafe { *ins as u8 };
                println!("IR: 0x{:02x}", iins);
                loop {
                    // vm
                    if iins == 0x00 {
                        // debug
                        println!("NOP");
                        println!("{:?}",self.stack());
                        break;
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
                                use super::super::super::bin_format::constant_and_pool::get_constant;
                                let cons = get_constant(tag, uuid);
                                self.stack()
                                    .push(Value::from(super::PrimeValue::from(cons)));
                            }
                            LOADNULL => {
                                let opmodes = unsafe {
                                    LOADNULL_OP.get_fix().opmode.get_ab(*(ins as *const u32))
                                };
                                let rs1 = opmodes.0;
                                let rs2 = opmodes.1;
                                for i in rs1..rs2 {
                                    let res = self.stack().stack.try_push(Value::from(super::PrimeValue::Null));
                                    if res.is_err(){
                                        eprintln!("well~ this is a bug, trying to fix");
                                    }
                                }
                            }
                            LOADBOOL => {
                                let (b) = unsafe {
                                    LOADBOOL_OP.get_fix().opmode.get_a(*(ins as *const u32))
                                };
                                if b != 0x00 {
                                    self.stack()
                                        .push(Value::from(super::PrimeValue::Bool(true)));
                                } else {
                                    self.stack()
                                        .push(Value::from(super::PrimeValue::Bool(false)));
                                }
                            }
                            _ => unimplemented!(),
                        }
                        break;
                    }
                    // cf
                    else if iins > 0x19 && iins < 0x40 {
                        use super::super::op::cf::*;
                        match iins {
                            JPE => {
                                let (e, loc) = unsafe {
                                    JPE_OP.get_fix().opmode.get_abx(*(ins as *const u32))
                                };
                                if self.stack().get(e as isize).0 == super::PrimeValue::Bool(true) {
                                    let label = self
                                        .stack()
                                        .closure
                                        .func
                                        .instruction_table
                                        .iter()
                                        .position(|r| r.label == loc)
                                        .expect("ERROR! LABEL DOES NOT EXIST");
                                    self.stack().closure.current_label_number = label as u16;
                                    *self.pc() = 0;
                                }
                            }
                            JPN => {
                                let (e, loc) = unsafe {
                                    JPE_OP.get_fix().opmode.get_abx(*(ins as *const u32))
                                };
                                if self.stack().get(e as isize).0 == super::PrimeValue::Bool(false)
                                {
                                    let label = self
                                        .stack()
                                        .closure
                                        .func
                                        .instruction_table
                                        .iter()
                                        .position(|r| r.label == loc)
                                        .expect("ERROR! LABEL DOES NOT EXIST");
                                    self.stack().closure.current_label_number = label as u16;
                                    *self.pc() = 0;
                                }
                            }
                            JMP => {
                                let value =
                                    unsafe { JMP_OP.get_fix().opmode.get_ax(*(ins as *const u32)) };
                                let label = self
                                    .stack()
                                    .closure
                                    .func
                                    .instruction_table
                                    .iter()
                                    .position(|r| r.label == value)
                                    .expect("ERROR! LABEL DOES NOT EXIST");
                                self.stack().closure.current_label_number = label as u16;
                                *self.pc() = 0;
                            }
                            UFCALL => {}
                            CALL => {
                                let (cls, till) = unsafe {
                                    CALL_OP.get_fix().opmode.get_ab(*(ins as *const u32))
                                };
                                if let super::PrimeValue::Closure(ccls) =
                                    self.stack().get(cls as isize).clone().0
                                {
                                    self.call(Box::new(ccls), till)
                                } else {
                                    panic!("ERROR! RS{} IS NOT CLOSURE", cls)
                                }
                            }
                            TAILCALL => {
                                let len = self.stack().stack.len();
                                use arrayvec::*;
                                use std::iter::FromIterator;
                                let mut new_stack = self.stack().fixed_tops();
                                self.stack().stack = ArrayVec::from_iter(new_stack);
                                println!("============= TAIL CALL ===========\nstack: {:?}",self.stack());
                                *self.pc() = 0;
                                self.stack().fixed_top = 255;
                            }
                            RET => {
                                self.frames.pop();
                                if self.frames.len() == 0 {
                                    return;
                                }
                            }
                            RETURN => {
                                self.return_();
                            }
                            _ => unimplemented!(),
                        }
                        break;
                    }
                    // // comp
                    else if iins > 0x89 && iins < 0xB0 {
                        use super::super::op::comp::*;
                        match iins {
                            EQ => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) =
                                    unsafe { EQ_OP.get_fix().opmode.get_abc(*(ins as *const u32)) };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::eq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            LE => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) =
                                    unsafe { LE_OP.get_fix().opmode.get_abc(*(ins as *const u32)) };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::le(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            GT => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) =
                                    unsafe { GT_OP.get_fix().opmode.get_abc(*(ins as *const u32)) };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::gt(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            NEQ => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    NEQ_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::neq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            LEEQ => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    LEEQ_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::leeq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            GTEQ => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    GTEQ_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::gteq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            _ => unimplemented!(),
                        }
                        break;
                    }
                    // num
                    else if iins > 0x59 && iins < 0x90 {
                        use super::super::op::arith::*;
                        match iins {
                            // NEGM => {}
                            // ADDM => {}
                            // SUBM => {}
                            // MULM => {}
                            // MODM => {}
                            // POWM => {}
                            // DIVM => {}
                            NEG => {
                                let opmodes = unsafe {
                                    NEGM_OP.get_fix().opmode.get_ab(*(ins as *const u32))
                                };
                                let rs1 = opmodes.0;
                                let rs2 = opmodes.1;
                                use super::PrimeValue::*;
                                use super::Value;

                                match &mut self.stack().get(rs2 as isize).0 {
                                    Null => {
                                        self.stack().set(rs1 as isize, Value::from(Null));
                                    }
                                    Char(c) => {
                                        self.stack().set(
                                            rs1 as isize,
                                            Value::from(Char((-(*c as i16)) as u16)),
                                        );
                                    }
                                    Int(i) => {
                                        self.stack().set(
                                            rs1 as isize,
                                            Value::from(Int((-(*i as i32)) as u32)),
                                        );
                                    }
                                    Num(n) => {
                                        self.stack().set(rs1 as isize, Value::from(Num(-*n)));
                                    }
                                    _ => unimplemented!(),
                                }

                                break;
                            }
                            ADD => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::add(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            SUB => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::sub(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            MUL => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::mul(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            MOD => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::modu(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            DIV => {
                                use super::super::super::bin_format::*;
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::div(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
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
                        break;
                    }
                    // stack
                    else if iins > 0x39 && iins < 0x60 {
                        use super::super::op::stack::*;
                        match iins {
                            CLOSURE => {
                                let idx = unsafe {
                                    CLOSURE_OP.get_fix().opmode.get_ax(*(ins as *const u32))
                                };
                                self.load_func(idx as usize);
                            }
                            FIXTOP => {
                                let idx = unsafe {
                                    FIXTOP_OP.get_fix().opmode.get_a(*(ins as *const u32))
                                };
                                self.stack().fix_to_top(idx as usize);
                            }
                            _ => unimplemented!(),
                        }
                        break;
                    }
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
