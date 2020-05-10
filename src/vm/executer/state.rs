use super::stack::*;
#[derive(PartialEq)]
pub enum Status {
    STOP = 0x00,
    RUNNING = 0x01,
    YIELD = 0x02,
    ERROR = 0xFF,
}
use super::super::VMMessage;
use super::stack::IR;
use async_std::sync::*;
pub struct State {
    pub debug_mode:bool,
    pub uuid: u32,
    pub status: Status,
    pub frames: Vec<Stack>,
    pub sr: (Sender<String>, Receiver<VMMessage>),
}
unsafe impl Send for State {}
unsafe impl Sync for State {}

impl State {
    pub fn stack(&mut self) -> &mut Stack {
        self.frames
            .last_mut()
            .expect("ERROR! FAILED TO GET CURRENT CALL STACK")
    }
    
    pub fn stack_with_name(&self)->Vec<(super::super::super::VMSym, super::Value)>{
        if !self.debug_mode{
            return vec!();
        }
        let stack = self.frames.last().unwrap();
        let pc = stack.pc.clone() as u32;
        let vars = super::super::super::func_type::LOCAL_VARS.read().unwrap();
        let lvar = vars.iter().filter(|v| v.func_uuid != stack.closure.get_func_uuid());
        let mut cvar = lvar.filter(|l| l.start_pc > pc as u32 || l.end_pc < pc as u32).collect::<Vec<_>>();
        cvar.sort_by(|a,b|a.stack_pos.partial_cmp(&b.stack_pos).unwrap());
        let mut ps = vec!();
        for v in cvar{
            ps.push((v.name.clone(),stack.stack[v.stack_pos as usize].clone()));
        }
        ps
    }

    pub fn push_stack(&mut self, stack: Stack) {
        self.frames.push(stack);
    }
    pub fn pop_stack(&mut self) {
        self.frames.pop();
    }
    pub fn new(debug_mode:bool,uuid: u32, sender: Sender<String>, receiver: Receiver<VMMessage>) -> Self {
        State {
            debug_mode,
            uuid,
            frames: vec![],
            status: Status::RUNNING,
            sr: (sender, receiver),
        }
    }
    pub fn ir(&mut self) -> &mut *const u8 {
        &mut self.stack().ir.0
    }
    pub fn pc(&mut self) -> &mut usize {
        &mut self.stack().pc
    }
    pub fn fetch(&mut self) -> Option<IR> {
        if self.frames.len() == 0 {
            return None;
        }
        let pc = self.pc().clone();
        let current_label = self.stack().closure.current_label_number;
        if let Some(instr) = self
            .stack()
            .closure
            .func()
            .instruction_table
            .get(current_label as usize)?
            .instructions
            .clone()
            .get(pc)
        {
            *self.pc() += 1;
            *self.ir() = *instr;
            return Some(IR(*instr));
        } else {
            self.stack().closure.current_label_number += 1;
            *self.pc() = 0;
            self.fetch()
        }
    }
    // args is how deep on stack top will copy as argument
    // pub fn call(&mut self, f: Box<super::Closure>, args: u8) {
    //     let top = self.stack().top();
    //     let mut v = vec![];
    //     for i in 0..args {
    //         v.push(self.stack().pop())
    //     }
    //     let mut stack = Stack::new_from_closure(f);
    //     stack.stack = v;
    //     self.frames.push(stack);
    //     // self.stack().pc = 0
    // }
    // move return value to current stack top
    // pub fn return_(&mut self)->Vec<super::Value> {

    //     let mut res = self.stack().fixed_tops();
    //     // pop function call
    //     let f = self.frames.pop().unwrap();
    //     if self.frames.len() == 0{
    //         return res;
    //     }
    //     if !self.stack().check_ramain_enougth(res.len()) {
    //         panic!("ERROR! STACK OVERFLOWED")
    //     }
    //     self.stack().stack.append(&mut res);
    //     res
    // }
    pub async fn execute(mut self) -> (Vec<super::Value>, Option<Stack>) {
        use super::{CReceiver,CSender};
        let mut bk = false;
        let mut step_into = false;
        let mut step_over = false;
        while self.status == Status::RUNNING {
            if let Some(ins) = self.fetch() {
                let iins = unsafe { *ins.0 as u8 };
                
                if self.debug_mode{
                    println!("IR: 0x{:02x}\nStack: {:?}", iins,self.stack());
                }else{
                    println!("IR: 0x{:02x}",iins);
                }
                loop {
                    loop {
                        if self.sr.1.is_empty() {
                            if bk || self.debug_mode {
                                continue;
                            } else {
                                break;
                            }
                        }
                        let m = self.sr.1.recv().await;
                        if let Some(m) = m {
                            use super::super::VMMessage::*;
                            match m {
                                PrintFrame => {
                                    self.sr.0.send(format!("{:?}", self.frames)).await;
                                }
                                PrintStack => {
                                    self.sr.0.send(format!("stack: {:?}\nnamed: {:?}", self.frames.last(),self.stack_with_name())).await;
                                }
                                Break => {
                                    bk = true;
                                }
                                Continue => {
                                    bk = false;
                                }
                                StepInto => {
                                    step_into = true;
                                    break;
                                }
                                StepOver => {
                                    step_over = true;
                                    break;
                                }
                            }
                        }
                        if bk || self.debug_mode {
                            continue;
                        } else {
                            break;
                        }
                    }

                    // vm
                    if iins == 0x00 {
                        // debug
                        println!("NOP");
                        break;
                    }
                    // load
                    else if iins > 0x00 && iins < 0x20 {
                        use super::super::op::load::*;
                        use super::Value;
                        match iins {
                            LOADK => {
                                let tag = unsafe { *(ins.0.add(3)) };
                                let uuid = unsafe { *(ins.0.add(3 + 1)) as u32 };
                                use super::super::super::bin_format::constant_and_pool::get_constant;
                                let cons = get_constant(tag, uuid);
                                self.stack()
                                    .push(Value::from(super::PrimeValue::from(cons)));
                            }
                            LOADNULL => {
                                let opmodes = unsafe {
                                    LOADNULL_OP.get_fix().opmode.get_ab(*(ins.0 as *const u32))
                                };
                                let rs1 = opmodes.0;
                                let rs2 = opmodes.1;
                                for i in rs1..rs2 {
                                    self.stack()
                                        .stack
                                        .insert(i as usize, Value::from(super::PrimeValue::Null));
                                }
                            }
                            LOADBOOL => {
                                let b = unsafe {
                                    LOADBOOL_OP.get_fix().opmode.get_a(*(ins.0 as *const u32))
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
                                    JPE_OP.get_fix().opmode.get_abx(*(ins.0 as *const u32))
                                };
                                if self.stack().get(e as isize).0 == super::PrimeValue::Bool(true) {
                                    let label = self
                                        .stack()
                                        .closure
                                        .func()
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
                                    JPE_OP.get_fix().opmode.get_abx(*(ins.0 as *const u32))
                                };
                                if self.stack().get(e as isize).0 == super::PrimeValue::Bool(false)
                                {
                                    let label = self
                                        .stack()
                                        .closure
                                        .func()
                                        .instruction_table
                                        .iter()
                                        .position(|r| r.label == loc)
                                        .expect("ERROR! LABEL DOES NOT EXIST");
                                    self.stack().closure.current_label_number = label as u16;
                                    *self.pc() = 0;
                                }
                            }
                            JMP => {
                                let value = unsafe {
                                    JMP_OP.get_fix().opmode.get_ax(*(ins.0 as *const u32))
                                };
                                let label = self
                                    .stack()
                                    .closure
                                    .func()
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
                                    CALL_OP.get_fix().opmode.get_ab(*(ins.0 as *const u32))
                                };
                                if let super::PrimeValue::Closure(ccls) =
                                    self.stack().get(cls as isize).clone().0
                                {
                                    // let top = self.stack().top();
                                    let mut v = vec![];
                                    for _ in 0..till {
                                        v.push(self.stack().pop())
                                    }
                                    let mut stack = Stack::new_from_closure(Box::new(ccls));
                                    stack.stack = v;
                                    self.frames.push(stack);
                                } else {
                                    panic!("ERROR! RS{} IS NOT CLOSURE", cls)
                                }
                            }
                            TAILCALL => {
                                self.stack().stack = self.stack().fixed_tops();
                                println!(
                                    "============= TAIL CALL ===========\nstack: {:?}",
                                    self.stack()
                                );
                                *self.pc() = 0;
                                self.stack().fixed_top = 255;
                            }
                            RET => {
                                self.frames.pop();
                                if self.frames.len() == 0 {
                                    break;
                                }
                            }
                            RETURN => {
                                // pop function call
                                let mut res = self.frames.pop().unwrap().fixed_tops();
                                if self.frames.len() == 0 {
                                    return (res, None);
                                } else {
                                    self.stack().stack.append(&mut res);
                                }
                            }
                            CALLC => {
                                let (a, b, till) = unsafe {
                                    CALLC_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                if let super::Value(super::PrimeValue::NType(ty), _) =
                                    self.stack().pop()
                                {
                                    if let super::Value(super::PrimeValue::Sym(path), _) =
                                        self.stack().get(a as isize)
                                    {
                                        if let super::Value(super::PrimeValue::Sym(name), _) =
                                            self.stack().get(b as isize)
                                        {
                                            let r = super::ffi::dynamic_lib::pass_args_to_NFunc_and_call(
                                                path,name,ty,
                                                self.stack(),
                                                till,
                                            );
                                            self.stack().push(r);
                                        } else {
                                            panic!("ERROR! IS NOT SYM")
                                        }
                                    } else {
                                        panic!("ERROR! IS NOT SYM")
                                    }
                                } else {
                                    panic!("ERROR! IS NOT TYPE")
                                }
                            }
                            YIELD => {
                                self.status = Status::YIELD;
                            }
                            RESUME => {
                                let idx = unsafe {
                                    RESUME_OP.get_fix().opmode.get_a(*(ins.0 as *const u32))
                                };
                                let super::Value(c, _) = &mut self.stack().get(idx as isize);
                                if let super::PrimeValue::Thread(_, stack) = &c {
                                    if let Some(stack) = stack {
                                        println!("resumed with stack:\n  {:?}", stack);
                                        let h = super::super::new_sub_thread(step_into,stack.clone(),self.sr.0.clone(),self.sr.1.clone());
                                        self.stack().set(
                                            idx as isize,
                                            super::Value::from(super::PrimeValue::Thread(
                                                h,
                                                Some(stack.clone()),
                                            )),
                                        );
                                    }
                                }
                                // get state of that thread,pc+1,create a new state and execute
                            }
                            CALLC => {
                                let (a, b, till) = unsafe {
                                    CALLC_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                if let super::Value(super::PrimeValue::NType(ty), _) =
                                    self.stack().pop()
                                {
                                    if let super::Value(super::PrimeValue::Sym(path), _) =
                                        self.stack().get(a as isize)
                                    {
                                        if let super::Value(super::PrimeValue::Sym(name), _) =
                                            self.stack().get(b as isize)
                                        {
                                            let r = super::ffi::dynamic_lib::pass_args_to_NFunc_and_call(
                                                path,name,ty,
                                                self.stack(),
                                                till,
                                            );
                                            self.stack().push(r);
                                        } else {
                                            panic!("ERROR! IS NOT SYM")
                                        }
                                    } else {
                                        panic!("ERROR! IS NOT SYM")
                                    }
                                } else {
                                    panic!("ERROR! IS NOT TYPE")
                                }
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
                                let (dst, src1, src2) = unsafe {
                                    EQ_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::comp::eq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            LE => {
                                let (dst, src1, src2) = unsafe {
                                    LE_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::comp::le(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            GT => {
                                let (dst, src1, src2) = unsafe {
                                    GT_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::comp::gt(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            NEQ => {
                                let (dst, src1, src2) = unsafe {
                                    NEQ_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::comp::neq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            LEEQ => {
                                let (dst, src1, src2) = unsafe {
                                    LEEQ_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::comp::leeq(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            GTEQ => {
                                let (dst, src1, src2) = unsafe {
                                    GTEQ_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::comp::gteq(vsrc1, vsrc2);
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
                                    NEGM_OP.get_fix().opmode.get_ab(*(ins.0 as *const u32))
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
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::arith::add(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            SUB => {
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::arith::sub(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            MUL => {
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::arith::mul(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            MOD => {
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::arith::modu(vsrc1, vsrc2);
                                self.stack().set(dst as isize, res);
                            }
                            DIV => {
                                let (dst, src1, src2) = unsafe {
                                    ADD_OP.get_fix().opmode.get_abc(*(ins.0 as *const u32))
                                };
                                let vsrc1 = self.stack().get(src1 as isize);
                                let vsrc2 = self.stack().get(src2 as isize);
                                let res = super::super::op::arith::div(vsrc1, vsrc2);
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
                                    CLOSURE_OP.get_fix().opmode.get_ax(*(ins.0 as *const u32))
                                };
                                let func = Box::new(
                                    self.stack().closure.func().const_func_refs[idx as usize]
                                        .clone(),
                                );
                                self.stack()
                                    .push(super::Value::from(super::PrimeValue::from(
                                        super::super::super::bin_format::constant_and_pool::get_constant(func.0, func.1),
                                    )));
                            }
                            FIXTOP => {
                                let idx = unsafe {
                                    FIXTOP_OP.get_fix().opmode.get_a(*(ins.0 as *const u32))
                                };
                                self.stack().fix_to_top(idx as usize);
                            }
                            NEWTHREAD => {
                                let idx = unsafe {
                                    NEWTHREAD_OP.get_fix().opmode.get_a(*(ins.0 as *const u32))
                                };
                                let super::Value(c, _) = self.stack().get(idx as isize);
                                if let super::PrimeValue::Closure(c) = c {
                                    use super::super::*;
                                    let h = new_sub_thread(step_into,Stack::new_from_closure(Box::new(c)),self.sr.0.clone(), self.sr.1.clone());
                                    let v = super::Value::from(super::PrimeValue::Thread(h, None));
                                    self.stack().push(v);
                                } else {
                                    panic!("ERROR CURRENT STACK ADDRESS IS NOT CLOSURE")
                                }
                            }
                            GETTRET => {
                                let idx = unsafe {
                                    GETTRET_OP.get_fix().opmode.get_a(*(ins.0 as *const u32))
                                };
                                let super::Value(c, _) = self.stack().get(idx as isize);
                                if let super::PrimeValue::Thread(t, _) = &c {
                                    let mut res = super::super::get_join_handle(*t).await;
                                    println!("thread returned:\n  {:?}",res);
                                    self.stack().stack.append(&mut res.0);
                                }
                            }

                            GETYIELD => {
                                let idx = unsafe {
                                    GETTRET_OP.get_fix().opmode.get_a(*(ins.0 as *const u32))
                                };
                                let super::Value(c, _) = self.stack().get(idx as isize);
                                if let super::PrimeValue::Thread(t,_) = &c {
                                    let (mut res, stack) = super::super::get_join_handle(*t).await;
                                    println!("yielded thread with stack:\n  {:?}", stack);
                                    self.stack().stack.append(&mut res);
                                    self.stack().set(
                                        idx as isize,
                                        super::Value::from(super::PrimeValue::Thread(
                                            *t,
                                            stack.clone(),
                                        )),
                                    );
                                }
                            }
                            _ => unimplemented!(),
                        }
                        break;
                    }
                    // // debug
                    else if iins > 0xDF {
                        use super::super::op::debug::*;
                        match iins {
                            BREAK => {
                                println!("== BREAK AT LINE {} ==", self.pc());
                                bk = true;
                            }
                            _ => unimplemented!(),
                        }
                        break;
                    } else {
                        panic!("ERROR INSTRUCTION '0x{:02X}' NOT SUPPORTED", iins);
                    }
                }
            } else {
                return (self.stack().fixed_tops(), None);
            }
        }

        match self.status {
            Status::RUNNING => panic!("ERROR NO WAY! IS NOT RUNNING"),
            Status::ERROR => {
                println!("STOPED WITH ERROR OCCURS");
                let stack: Stack = self.frames.last().unwrap().clone();
                return (self.stack().fixed_tops(), Some(stack));
            }
            Status::YIELD => {
                println!("YIELDED");
                self.stack().fixed_top = 255;
                let stack: Stack = self.frames.last().unwrap().clone();
                return (self.stack().fixed_tops(), Some(stack));
            }
            Status::STOP => {
                println!("STOPED");
                let stack: Stack = self.frames.last().unwrap().clone();
                return (self.stack().fixed_tops(), Some(stack));
            }
        }
    }
}
