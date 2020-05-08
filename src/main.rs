// #![feature(new_uninit)]
// #![feature(vec_into_raw_parts)]
// #![feature(alloc_layout_extra)]
// #![feature(slice_from_raw_parts)]

#[macro_use]
extern crate lazy_static;
extern crate libc;

pub mod bin_format;
pub mod vm;
use std::env;
use bin_format::*;

#[repr(C)]
struct A{
    a:i32,
}
extern "C" fn push_user_data(state:&mut vm::executer::state::State){
    use std::alloc::{alloc, dealloc, Layout};
    unsafe{
        let layout = Layout::new::<A>();
        let _1 = alloc(layout) as *mut A;
    // let mut _1 = System.alloc();    Vec::with_capacity(std::mem::size_of::<A>()).as_mut_ptr();
        *_1 = A{a:1};

    let value = vm::executer::PrimeValue::UserData(_1 as *mut u8);
    state.stack().push(vm::executer::Value::from(value));
    }
}
extern "C" fn mod_user_data(state:&mut vm::executer::state::State){
    let vm::executer::Value(ud,_) = state.stack().stack.last_mut().unwrap();
    if let vm::executer::PrimeValue::UserData(ud) = ud{
        let _ud:*mut A = *ud as *mut A;
        unsafe{(*_ud).a = 2;}
        // let value = vm::executer::PrimeValue::UserData(_ud as *mut u8);
        // state.stack().push(vm::executer::Value::from(value));
    }else{
        panic!("is not user data");
    }
}
use async_std::prelude::*;
#[async_std::main]
async fn main() {
    let constant_pool = [
        0x02,
        0x13,
        0x01,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00, 0x01,0x03,

        0x05,
        0x02,0x00,0x00,0x00,
        
        0x01,0x00,0x00,0x00, 0x05,0x00,0x00,0x00,0x61,0x00, 0x2E,0x00,  0x64,0x00,  0x6C,0x00,  0x6C,0x00, 
        0x02,0x00,0x00,0x00, 0x04,0x00,0x00,0x00,0x73,0x00, 0x68,0x00 ,0x69,0x00, 0x74,0x00,
    ];
    let bytes = [
        0x03, 0x00, 0x00, 0x00, 
        0x6a, 0x00, 0x6d,0x00,0x70,0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x01,0x00,
    // start
    // label : 0
        0x00,0x00,
        0x01,0x00,0x00,0x00,
        0x00,0x02,0x00,0x01,0x00,
        // // 0xFF,0x01,0x05,0x00,0x05,0x01,0x00,0x00,0x00,
        // // 0xFF,0x01,0x05,0x00,0x05,0x02,0x00,0x00,0x00,
        // // 0xFF,0x01,0x05,0x00,0x13,0x01,0x00,0x00,0x00,
        // // 0x00,0x28,0x00,0x01,0x00,
        // 0x00,0x00,0x00,0x00,0x00,
    // end
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    //println!("{:?}",constant_and_pool::CONSTANT_POOL.read().unwrap());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let func = reader.read_func();
    println!("{}",func);
    let mut reg = vm::ThreadsRegister{threads:vec!(),channels:vec!()};
    let mut stack= vm::executer::stack::Stack::new(Box::new(func));
    use vm::*;
    let h = new_thread(stack);
    println!("===== testing Async =====");
    let mut tr = THREAD_REGISTER.lock().unwrap();
    //let st = unsafe{&mut(*(tr.threads[0]))};
    // println!("before execute {:?}",st.stack().stack);
    // push_user_data(st);
    // mod_user_data(st);
    use async_std::task::*;
    h.await;
    //println!("after execute {:?}\n", st.stack().stack);
}