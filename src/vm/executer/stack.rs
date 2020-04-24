// like bin_format/mod.rs/constant
use super::*;
use super::PrimeType::*;
use arrayvec::*;
// stack could only save 256 address which means if you write a function with more than 256+48 local variables(+ arguments) then you are fuking dumm or you works in observatory.
// for register allocation, amd64 has 4 x 64bit register
// then 6x8 8 bit register (48)
// so use linear scan algorithm for 16 register allocation
// ========================================================
// well ignore me i wont write asm by my self would I?
pub struct Stack{
    pub stack:Vec<PrimeType>,
}
impl Stack{
    pub fn new()->Stack{
        Stack{stack:vec!()}
    }

    pub fn top(&self) -> isize {
        self.stack.len() as isize
    }

    pub fn reserve_capacity(&mut self, n: usize)->bool {
        self.vec.reserve(n);
    }

    pub fn push(&mut self, val: PrimeType) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> PrimeType {
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
    pub fn get(&self, idx: isize) -> PrimeType {
        let abs_idx = self.abs_index(idx);
        if self.is_valid_abs(abs_idx) {
            self.stack[abs_idx as usize].clone() // TODO
        } else {
            eprintln!("ERROR! INVALID INDEX {}",idx);
            PrimeType::Null
        }
    }

    pub fn set(&mut self, idx: isize, val: PrimeType) {
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

    fn rotate(&mut self, idx: isize, n: isize) {
        let abs_idx = self.abs_index(idx);
        if abs_idx < 0 || !self.is_valid_abs(abs_idx){
            panic!("ERROR! INVALID INDEX {}",idx);
        }

        let t = self.top() - 1; /* end of stack segment being rotated */
        let p = abs_idx - 1; /* start of segment */
        let m = if n >= 0 { t - n } else { p - n - 1 }; /* end of prefix */
        self.reverse(p as usize, m as usize); /* reverse the prefix with length 'n' */
        self.reverse(m as usize + 1, t as usize); /* reverse the suffix */
        self.reverse(p as usize, t as usize); /* reverse the entire segment */
    }
    fn is_null (&self,idx:isize)->bool{
        if let NULL = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_bool (&self,idx:isize)->bool{
        if let Bool(_) = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_char (&self,idx:isize)->bool{
        if let Char(_) = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_int (&self,idx:isize)->bool{
        if let Int(_) = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_num (&self,idx:isize)->bool{
        if let Num(_) = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_sym (&self,idx:isize)->bool{
        if let Sym(_) = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_user_data (&self,idx:isize)->bool{
        if let UserData(_) = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_row (&self,idx:isize)->bool{
        if let Row() = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_closure (&self,idx:isize)->bool{
        if let Closure() = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
    fn is_thread (&self,idx:isize)->bool{
        if let Thread() = self.get(idx){
            return true;
        }else{
            return false;
        }
    }
}