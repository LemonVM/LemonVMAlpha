// like bin_format/mod.rs/constant
use super::*;

#[derive(Debug,Clone,PartialEq)]
pub struct Stack{
    pub on_err:bool,
    pub stack:Vec<Value>,
    pub closure:Box<Closure>,
    pub pc: usize,
    pub ir: IR,
    pub fixed_top : usize
}
#[derive(Debug,Clone,PartialEq)]
pub struct IR(pub *const u8);
unsafe impl Send for IR{}
unsafe impl Sync for IR{}

unsafe impl Send for Stack{}
unsafe impl Sync for Stack{}

impl Stack{
    pub fn new_from_closure(closure:Box<Closure>)->Stack{
        Stack{on_err:false,stack:vec!(),pc:0,ir:IR(std::ptr::null()),closure,fixed_top:255}
    }
    pub fn new(func:Box<super::super::super::bin_format::func_type::FuncType>)->Stack{
        Stack{on_err:false,stack:vec!(),pc:0,ir:IR(std::ptr::null()),closure:Box::new(Closure::new(FuncInClosure::Func(func.clone()),func.arg_types,func.ret_types)),fixed_top:255} //FIXME:GC this will be allocated in heap
    }

    pub fn top(&self) -> usize {
        self.stack.len()
    }

    pub fn check_ramain_enougth(&mut self, n: usize) -> bool {
        self.stack.len() < 256
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn get(&self, idx: usize) -> Value {
            self.stack[idx].clone()
    }

    pub fn set(&mut self, idx: usize, val: Value) {
            self.stack[idx as usize] = val;
    }

    pub fn reverse(&mut self, mut from: usize, mut to: usize) {
        while from < to {
            self.stack.swap(from, to);
            from += 1;
            to -= 1;
        }
    }
    // 255 means no fixed top
    // so only can have at most 254 return values
    pub fn fix_to_top(&mut self,id:usize){
        if self.fixed_top == 255{
            self.fixed_top = self.stack.len().clone() -1;
            self.stack.swap(self.fixed_top , id);
        }else{
            self.fixed_top -= 1;
            self.stack.swap(self.fixed_top , id);
        }
    }
    // because of a,b,c,d,e = f() and f fixed tops will be e,d,c,b,a
    pub fn fixed_tops(&mut self)->Vec<Value>{
        if self.fixed_top == 255{
            vec!()
        }else{
            let len = self.stack.len();
            let mut ret = vec!();
            for v in self.stack[self.fixed_top..len].iter().rev(){
                ret.push(v.clone());
            }
            ret
        }
    }
}