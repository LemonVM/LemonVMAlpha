pub const GC_GRAY_MAX_SIZE: usize = 10240;
pub const GC_MAX_MEMORY_USAGE: usize = 10240;

#[derive(Clone, PartialEq, Debug)]
pub enum Color {
    White = 0x01,
    Gray = 0x02,
    Black = 0x03,
}

#[derive(Debug)]
pub struct GCHeap {
    // for debug
    pub total_white_size: usize,
    pub total_black_size: usize,
    pub total_gray_size: usize,
    // for fast index
    // pub white_indexs : Vec<u32>,
    // pub gray_indexs : Vec<u32>,
    // pub black_indexs : Vec<u32>,
    pub blocks: Vec<Option<GCBlock>>,
}

#[derive(Clone, Debug)]
pub struct GCBlock {
    pub ptr: *mut u8,
    pub size: usize,
    pub color: Color,
    // index
    pub ref_to: Option<Vec<usize>>,
}
use std::alloc::*;
use Color::*;
impl GCBlock {
    // gray block
    pub fn new(size: usize) -> GCBlock {
        let mut mem = Vec::with_capacity(size);
        GCBlock {
            ptr: mem.as_mut_ptr(),
            size,
            color: Gray,
            ref_to: None,
        }
    }
    // white block
    pub fn from_ptr(size: usize, ptr: *mut u8) -> GCBlock {
        GCBlock {
            ptr: ptr,
            size,
            color: Gray,
            ref_to: None,
        }
    }
    pub fn add_ref(&mut self, index: usize) {
        match &mut self.ref_to {
            None => self.ref_to = Some(vec![index]),
            Some(refs) => refs.push(index),
        }
    }
}

impl GCHeap {
    pub fn new() -> GCHeap {
        GCHeap {
            total_white_size: 0,
            total_gray_size: 0,
            total_black_size: 0,

            // white_indexs : vec!(),
            // gray_indexs : vec!(),
            // black_indexs : vec!(),
            blocks: vec![],
        }
    }
    pub fn test_push_white(&mut self, blc: GCBlock) -> usize {
        self.total_white_size += blc.size;
        let mut nblc = blc.clone();
        nblc.color = White;
        self.blocks.push(Some(nblc));
        self.blocks.len() - 1
    }
    pub fn push(&mut self, blc: GCBlock, root: &mut Vec<usize>) -> usize {
        // when gray has enough space
        let size = blc.size;
        if self.total_gray_size + size < GC_GRAY_MAX_SIZE {
            self.blocks.push(Some(blc));
            self.total_gray_size += size;
            self.blocks.len() - 1
        } else {
            self.gc(root, false);
            // check memory after gc
            if self.total_white_size + self.total_gray_size + self.total_black_size + size
                > GC_MAX_MEMORY_USAGE
            {
                panic!("ERROR! Out of Memory");
            }
            return self.push(blc, root);
        }
    }

    pub fn move_single(&mut self, idx: &mut usize) {
        let blc = &mut self.blocks[*idx];
        if let Some(blc) = blc {
            if blc.color == White {
                self.total_white_size -= blc.size;
                self.total_gray_size += blc.size;
                blc.color = Gray;
            }
        }
        if let Some(blc) = blc {
            if blc.color == Gray {
                self.total_gray_size -= blc.size;
                self.total_black_size += blc.size;
                blc.color = Black;
            }
        }
        // recursive for refs
        match blc {
            Some(idxs) => match &mut idxs.ref_to.clone() {
                Some(ve) => {
                    for v in 0..ve.len() {
                        self.move_single(&mut ve[v]);
                    }
                }
                None => (),
            },
            None => (),
        }
    }
    pub fn order_all_refs_for_one_block(
        &mut self,
        pre_last: usize,
        index: &mut usize,
    ) -> Vec<Option<GCBlock>> {
        let mut refs_reorderd = vec![];
        if let Some(blc) = &mut self.blocks[*index] {
            refs_reorderd.push(Some(blc.clone()));
            match &mut blc.ref_to.clone() {
                Some(vec) => {
                    for i in 0..vec.len() {
                        refs_reorderd.push(self.blocks[vec[i]].clone());
                        let cl = refs_reorderd.len();
                        if let Some(reffrom) = &mut refs_reorderd[0] {
                            if let Some(reffrom_to) = &mut reffrom.ref_to {
                                reffrom_to[i] = cl - 1 + pre_last;
                            }
                        }
                        println!("{:?}", refs_reorderd);
                        refs_reorderd.append(
                            &mut self
                                .order_all_refs_for_one_block(refs_reorderd.len(), &mut vec[i]),
                        );
                    }
                }
                None => {}
            }
        }
        return refs_reorderd;
    }
    pub fn order(&mut self, root: &mut Vec<usize>) {
        // let mut new_vec = vec!();
        // for r in root{
        //     new_vec.append(&mut self.order_all_refs_for_one_block(new_vec.len(),r));
        // }
        // self.blocks = new_vec;
        // println!("{:?}",self);
        let mut len = self.blocks.len();
        let mut i = 0;
        while i < len {
            if let None = self.blocks[i] {
                // 清除这个内存，让所有的引用 > i的-1
                self.blocks.remove(i);
                len -= 1;
                self.blocks.iter_mut().for_each(|blc| {
                    if let Some(blc) = blc {
                        if let Some(vec) = &mut blc.ref_to {
                            for v in vec {
                                if *v > i {
                                    *v -= 1;
                                }
                            }
                            for r in 0..root.len() {
                                if root[r] > i {
                                    root[r] -= 1;
                                }
                            }
                        }
                    }
                });
            }
            i += 1;
        }
    }

    pub fn gc(&mut self, root: &mut Vec<usize>, order: bool) {
        // TODO: ザ・ワールド
        for r in 0..root.len() {
            self.move_single(&mut root[r]);
        }

        // move useless gray to white
        for i in 0..self.blocks.len() {
            if let Some(r) = &mut self.blocks[i].clone() {
                match r.color {
                    White => {
                        self.total_white_size -= r.size;
                        self.blocks[i] = None
                    }
                    Gray => {
                        if let Some(b) = &mut self.blocks[i] {
                            self.total_gray_size -= r.size;
                            self.total_white_size += r.size;
                            b.color = White;
                        }
                    }
                    Black => {}
                }
            }
        }
        // TODO: continue
        println!("{:?}", &self);
        // clean whites and move black to white
        for i in 0..self.blocks.len() {
            if let Some(r) = &mut self.blocks[i] {
                match r.color {
                    White => {
                        self.total_white_size -= r.size;
                        unsafe {
                            std::alloc::dealloc(
                                r.ptr,
                                std::alloc::Layout::from_size_align(r.size, 1)
                                    .expect("ERROR! MEMORY IS NOT ALIGNED AS 1"),
                            )
                        };
                        self.blocks[i] = None
                    }
                    Black => {
                        self.total_white_size += r.size;
                        self.total_black_size -= r.size;
                        r.color = White
                    }
                    _ => (),
                }
            }
        }
        println!("{:?}", &self);
        // order mem by copy
        // TODO: ザ・ワールド
        // TODO: 这绝壁不对
        self.order(root);
        println!("{:?}", &self);
        // TODO: continue
    }
}
