use std::rc::Rc;
pub struct Heap{
    pub blocks : Vec<Rc<Block>>
}
#[derive(Clone)]
pub struct Block{
    pub data: Vec<u8>,
    pub ref_to: Vec<Rc<Block>>,
}
impl Block{
    pub fn new(data : Vec<u8>)->Block{
        Block{
            data,
            ref_to:vec!()
        }
    }
    pub fn add_ref(&mut self,ref_:&Rc<Block>){
        self.ref_to.push(ref_.clone());
    }
    pub fn from_ptr(size:usize,data:*mut u8)->Block{
        let data = unsafe{Vec::from_raw_parts(data, size, 0)};
        Block{
            data,
            ref_to:vec!()
        }
    }
}

impl Heap{
    pub fn new()->Heap{
        Heap{
            blocks:vec!()
        }
    }
    pub fn get(&self,index:usize)->&Rc<Block>{
        &self.blocks[index]
    }
    pub fn push(&mut self,block:Block) -> usize{
        self.blocks.push(Rc::new(block));
        return self.blocks.len()-1;
    }
    // mem you push will be copyed to block you need to clean your self
    pub fn push_ref(&mut self,block:&Block)-> usize{
        self.blocks.push(Rc::new(block.clone()));
        return self.blocks.len()-1;
    }
    pub fn clean(&mut self,root:Vec<usize>){
        let mut _add_ref = vec!();

        for r in root{
            _add_ref.push(self.blocks[r].clone());
        }

        for i in 0..self.blocks.len(){
            if Rc::strong_count(&self.blocks[i]) == 1 && self.blocks[i].ref_to.len() == 0{
                self.blocks.remove(i);
            }
        }

        drop(_add_ref);
    }
}