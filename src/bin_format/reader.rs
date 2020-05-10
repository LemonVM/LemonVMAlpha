pub struct Reader {
    data: *const u8,
    pos: usize,
}
fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Sized + Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

impl Reader {
    pub fn new(data: *const u8) -> Reader {
        Reader { data, pos: 0 }
    }

    pub fn read_byte(&mut self) -> u8 {
        unsafe {
            let b = *self.data.add(self.pos);
            self.pos += 1;
            b
        }
    }
    pub fn read_bytes(&mut self, len: usize) -> *const u8 {
        unsafe {
            let buffer = std::alloc::alloc(std::alloc::Layout::new::<u8>());
            std::ptr::copy(self.data.add(self.pos), buffer, len);
            self.pos += len;
            buffer
        }
    }
    pub fn read_vm_char(&mut self) -> super::VMChar {
        let byte = self.read_bytes(std::mem::size_of::<super::VMChar>());
        unsafe { *(byte as *const super::VMChar) }
    }
    pub fn read_vm_int(&mut self) -> super::VMInt {
        let byte = self.read_bytes(std::mem::size_of::<super::VMInt>());
        unsafe { *(byte as *const super::VMInt) }
    }

    pub fn read_vm_number(&mut self) -> super::VMNum {
        let byte = self.read_bytes(std::mem::size_of::<super::VMNum>());
        unsafe { *(byte as *const super::VMNum) }
    }

    pub fn read_vm_symbol(&mut self) -> super::VMSym {
        super::VMSym(self.read_vec(|f| f.read_vm_char()))
    }

    pub fn read_vec<T, F>(&mut self, f: F) -> Vec<T>
    where
        F: Fn(&mut Reader) -> T,
    {
        let n = self.read_vm_int() as usize;
        let mut vec = Vec::with_capacity(n);
        for _i in 0..n {
            vec.push(f(self));
        }
        vec
    }
    pub fn read_header(&mut self) -> super::Header {
        unsafe {
            let sig =
                std::ptr::slice_from_raw_parts(self.read_bytes(super::SIG_LEN), super::SIG_LEN);
            let version = self.read_byte();
            let instruction_size = self.read_byte();
            let sizeof_vm_char = self.read_byte();
            let sizeof_vm_int = self.read_byte();
            let sizeof_vm_number = self.read_byte();
            super::Header {
                sig: clone_into_array(&*sig),
                version,
                instruction_size,
                sizeof_vm_char,
                sizeof_vm_int,
                sizeof_vm_number,
            }
        }
    }

    pub fn read_constant_pool_from_file(path: String, filename: String) {
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        let f = File::open(format!("{}{}.lmvmcp", path, filename)).unwrap();
        let mut bfr = BufReader::new(f);
        let mut buff = vec![];
        bfr.read_to_end(&mut buff);
        Reader::read_constant_pool(buff.as_ptr(), buff.len());
    }
    // assign value to global constant pool
    pub fn read_constant_pool(data: *const u8, len: usize) {
        let mut reader = Reader::new(data);
        let mut types = reader.read_byte();
        for i in 0..types {
            let tag = reader.read_byte();
            let len = reader.read_vm_int();
            for j in 0..len {
                use super::constant_and_pool::Constant::*;
                use super::*;
                match tag {
                    TAG_INT => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_int
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_NUM => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_num
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_SYM => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_sym
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_SIMDCHAR => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_simdchar
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_SIMDINT => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_simdint
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_SIMDNUM => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_simdnum
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_ROW => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_row
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_FUNC => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_func
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    TAG_TYPE => {
                        super::constant_and_pool::CONSTANT_POOL
                            .write()
                            .unwrap()
                            .pool_of_type
                            .1
                            .insert(reader.read_vm_int(), reader.read_constant(tag));
                    }
                    _ => unimplemented!(),
                }
            }
        }
    }
    pub fn read_constant(&mut self, tag: u8) -> super::constant_and_pool::Constant {
        use super::constant_and_pool::Constant::*;
        use super::*;
        if tag == TAG_ROW {
            let is_arr = self.read_byte() == 0x00;
            // 0x00 -> array, 0xff -> row
            // 0x00         len         (flag  data)
            // 0xFF         len         ( vmsym    flag      data     )*
            // row start    row size      key     value type  value
            if is_arr {
                let arr = self.read_vec(|f| {
                    let flag = f.read_byte();
                    f.read_constant(flag)
                });
                return Row(super::Row {
                    arr,
                    row: std::collections::HashMap::new(),
                    is_arr,
                });
            } else {
                let len = self.read_vm_int();
                let mut row = std::collections::HashMap::new();
                for i in 0..len {
                    let sym = self.read_vm_symbol();
                    let flag = self.read_byte();
                    row.insert(sym, self.read_constant(flag));
                }
                return Row(super::Row {
                    arr: vec![],
                    row,
                    is_arr,
                });
            }
        } else if tag == TAG_NULL {
            return Null;
        } else if tag == TAG_BOOL {
            return Bool(self.read_byte() != 0x00);
        } else if tag == TAG_INT {
            return Int(self.read_vm_int());
        } else if tag == TAG_NUM {
            return Num(self.read_vm_number());
        } else if tag == TAG_SYM {
            return Sym(self.read_vm_symbol());
        } else if tag == TAG_SIMDCHAR {
            return SIMDChar(
                self.read_vm_char(),
                self.read_vm_char(),
                self.read_vm_char(),
                self.read_vm_char(),
            );
        } else if tag == TAG_SIMDINT {
            return SIMDInt(
                self.read_vm_int(),
                self.read_vm_int(),
                self.read_vm_int(),
                self.read_vm_int(),
            );
        } else if tag == TAG_SIMDNUM {
            return SIMDNum(
                self.read_vm_number(),
                self.read_vm_number(),
                self.read_vm_number(),
                self.read_vm_number(),
            );
        } else if tag == TAG_FUNC {
            return Func(self.read_func());
        } else if tag == TAG_TYPE {
            return NType(self.read_type());
        } else {
            unimplemented!()
        }
    }
    pub fn read_instructions(&mut self) -> Vec<*const u8> {
        self.read_vec(|r| {
            let tag = r.read_byte();
            match tag {
                0x00 => r.read_bytes(4),
                0xFF => {
                    r.pos += 1; //skip op
                    let offset = r.read_byte();
                    let len = r.read_byte();
                    let total_len = offset + len;
                    r.pos -= 3;
                    // op len off data
                    r.read_bytes((total_len + 3) as usize)
                }
                _ => unimplemented!(),
            }
        })
    }
    pub fn read_labels(&mut self) -> Vec<super::Label> {
        let label_num = self.read_vm_char();
        let mut labels = vec![];
        for i in 0..label_num {
            let label = super::Label {
                label: self.read_vm_char(),
                instructions: self.read_instructions(),
            };
            labels.push(label);
        }
        labels
    }
    pub fn read_type(&mut self) -> super::Type {
        use super::Type::*;
        let flag = self.read_byte();
        match flag {
            0x00 => Kind,
            0x01 => Mono(self.read_byte()),
            0x02 => Poly(Box::new(self.read_type()), self.read_vec(|r| r.read_type())),
            0x04 => Row(self.read_vec(|r| (r.read_vm_symbol(), r.read_type()))),
            0xFF => Hole(self.read_byte()),
            _ => panic!("ERROR! TYPE FLAG NOT RECOGNISED"),
        }
    }
    pub fn read_func(&mut self) -> super::FuncType {
        let name = self.read_vm_symbol();
        let uuid = self.read_vm_int();
        let params = self.read_byte();
        let is_vargs = self.read_byte();
        let rets = self.read_byte();
        let arg_types = self.read_vec(|r| r.read_type());
        let ret_types = self.read_vec(|r| r.read_type());
        let instruction_table = self.read_labels();
        let const_func_refs = self.read_vec(|r| (r.read_byte(), r.read_vm_int()));

        let debug_local_variables = self.read_vec(|r| r.read_loc_var());
        if is_vargs == 0x00 {
            assert_eq!(params, arg_types.len() as u8);
            assert_eq!(rets, ret_types.len() as u8);
        }
        super::func_type::FuncType {
            name,
            uuid,
            params,
            is_vargs,
            rets,
            arg_types,
            ret_types,
            instruction_table,
            const_func_refs,
            debug_local_variables,
        }
    }
    // pub fn read_closure_cap(&mut self) -> super::ClosureCap {
    //     super::ClosureCap {
    //         instack: self.read_byte(),
    //         idx: self.read_byte(),
    //     }
    // }
    pub fn read_loc_vars(&mut self){
        *super::func_type::LOCAL_VARS.write().unwrap() = self.read_vec(|r|r.read_loc_var());
    }
    pub fn read_loc_var(&mut self) -> super::LocalVar {
        super::LocalVar {
            name: self.read_vm_symbol(),
            func_uuid: self.read_vm_int(),
            stack_pos: self.read_byte(),
            start_pc: self.read_vm_int(),
            end_pc: self.read_vm_int(),
        }
    }
    pub fn read_binary_chunk_from_file(path:String,filename:String)->super::BinaryChunk{
        use std::fs::File;
        use std::io::prelude::*;
        use std::io::BufReader;
        let f = File::open(format!("{}{}.lmvmb", path, filename)).unwrap();
        let mut bfr = BufReader::new(f);
        let mut buff = vec![];
        bfr.read_to_end(&mut buff);
        let mut reader = Reader::new(buff.as_ptr());
        reader.read_binary_chunk()
    }
    pub fn read_binary_chunk(&mut self) -> super::BinaryChunk {
        super::BinaryChunk {
            header: self.read_header(),
            up_value_size: self.read_byte(),
            entry: self.read_func(),
        }
    }
}
