// like bin_format/mod.rs/constant
use super::*;
// use super::PrimeType::*;
use arrayvec::*;
// stack could only save 256 address which means if you write a function with more than 256+48 local variables(+ arguments) then you are fuking dumm or you works in observatory.
// for register allocation, amd64 has 4 x 64bit register
// then 6x8 8 bit register (48)
// so use linear scan algorithm for 16 register allocation
// ========================================================
// well ignore me i wont write asm by my self would I?
// but we only use u8 for index ~ so ~

#[derive(Debug)]
pub struct Stack{
    pub stack:ArrayVec<[Value;256]>,
    pub closure:Box<Closure>,
    pub pc: usize,
    pub ir: *const u8,
    pub fixed_top : usize
}
impl Stack{
    pub fn new_from_closure(closure:Box<Closure>)->Stack{
        Stack{stack:ArrayVec::new(),pc:0,ir:std::ptr::null(),closure,fixed_top:255}
    }
    pub fn new(func:Box<super::super::super::bin_format::func_type::FuncType>)->Stack{
        Stack{stack:ArrayVec::new(),pc:0,ir:std::ptr::null(),closure:Box::new(Closure::new(func.clone(),func.arg_types,func.ret_types)),fixed_top:255} //FIXME:GC this will be allocated in heap
    }

    pub fn top(&self) -> isize {
        self.stack.len() as isize
    }

    pub fn check_ramain_enougth(&mut self, n: usize) -> bool {
        self.stack.remaining_capacity() + n > 255
    }

    pub fn push(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn abs_index(&self, idx: isize) -> isize {
        if idx >= 0 {
            idx
        } else {
            idx + self.top() + 1
        }
    }
    #[inline]
    fn is_valid_abs(&self,idx:isize)->bool{
        return idx >= 0 && idx <= self.top()
    }
    pub fn get(&self, idx: isize) -> Value {
        let abs_idx = self.abs_index(idx);
        if self.is_valid_abs(abs_idx) {
            self.stack[abs_idx as usize].clone() // TODO
        } else {
            panic!("ERROR! INVALID INDEX {}",idx);
        }
    }

    pub fn set(&mut self, idx: isize, val: Value) {
        let abs_idx = self.abs_index(idx);
        if self.is_valid_abs(abs_idx) {
            self.stack[abs_idx as usize] = val;
        } else {
            eprintln!("ERROR! INVALID INDEX {}",idx);
        }
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