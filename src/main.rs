// #![feature(new_uninit)]
// #![feature(vec_into_raw_parts)]
// #![feature(alloc_layout_extra)]
// #![feature(slice_from_raw_parts)]

#[macro_use]
extern crate lazy_static;
extern crate libc;

pub mod bin_format;
pub mod vm;

use bin_format::*;
// fn main() {
//     let constant_pool = [
//         0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
//         0x01, 0x00, 0x00, 0x00,
//         0x01, 0x01
//     ];
//     let bytes = [
//         0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
//         0x00, 0x00, 0x00, 0x00,
//         0x00, 0x00, 0x00, 0x00,
//         0x00,
//         0x00,
//         0x00,
//         0x01,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x00,
//         0x00,0x00,0x00,0x00,
//     ];
//     reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
//     let mut reader = reader::Reader::new(bytes.as_ptr());
//     let proto = reader.read_proto();
//     println!("{}",proto);
// }

fn main(){
        // use vm::gc::tri_color::*;
        // // 一个链表
        // use std::alloc::*;
        // let mut _1 =    unsafe{libc::malloc(4) as *mut u8};
        // _1 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(1 as u32))};
        // let mut _2 =    unsafe{libc::malloc(4) as *mut u8};
        // _2 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(2 as u32))};
        // let mut _3 =    unsafe{libc::malloc(4) as *mut u8};
        // _3 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(3 as u32))};
        // let mut _4 =    unsafe{libc::malloc(4) as *mut u8};
        // _4 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(4 as u32))};

        // let mut heap = GCHeap::new();
        // println!("{:?}",heap);

        // let mut g_1 = GCBlock::from_ptr(4, _1);
        // let mut g_2 = GCBlock::from_ptr(4, _2);
        
        // let mut g_3 = GCBlock::from_ptr(4, _3);
        // let mut g_4 = GCBlock::from_ptr(4, _4);
        
        // let mut root = vec!();
        // let i_2 = heap.push(g_2,&mut root);
        // g_1.add_ref(i_2);
        // let i_4 = heap.push(g_4,&mut root);
        // g_3.add_ref(i_4);
        // let i_3 = heap.push(g_3,&mut root);
        // g_1.add_ref(i_3);
        // // this will be a none after gc
        // // and when is ordered will remove completly
        // let mut _5 =    unsafe{libc::malloc(4) as *mut u8};
        // _5 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(5 as u32))};
        // let g_5 = GCBlock::from_ptr(4, _5);
        // let i_5 = heap.push(g_5,&mut root);

        // let i_1 = heap.push(g_1,&mut root);
        // root.push(i_1.clone());

        // let mut _6 =    unsafe{libc::malloc(4) as *mut u8};
        // _6 = unsafe{std::mem::transmute_copy::<u32,*mut u8>(&(6 as u32))};
        // let g_6 = GCBlock::from_ptr(4, _6);
        // let i_6 = heap.push(g_6,&mut root);
        
        // if let Some(h1) = &mut heap.blocks[i_1]{
        //         if let Some(vec) = &mut h1.ref_to{
        //                 vec.push(i_6);
        //         }
        // }
        // println!("{:?}",heap);
        // heap.gc(&mut root,true);
        // assert_eq!(heap.total_white_size,20);

        // let v_1 = unsafe{std::mem::transmute_copy::<*mut u8,u32>(&_1)};
        // let v_2 = unsafe{std::mem::transmute_copy::<*mut u8,u32>(&_2)};
        // let v_3 = unsafe{std::mem::transmute_copy::<*mut u8,u32>(&_3)};
        // let v_4 = unsafe{std::mem::transmute_copy::<*mut u8,u32>(&_4)};
        // println!("{}{}{}{}",v_1,v_2,v_3,v_4);

}