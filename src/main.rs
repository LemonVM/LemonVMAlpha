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
fn main() {
    let constant_pool = [
        0x01,
        0x04,0x01,0x00,0x00,0x00,0x01,0x00,0x00,0x00,0x4a,0xd8,0x12,0x4d,0xfb,0x21,0x09,0x40
    ];
    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00,0x00,0x00,0x00,
        0x00, 0x00, 0x00, 0x00,
        0x03, 0x00, 0x00, 0x00,
        0x00,
        0x00,
        0x00,

        0x04,0x00,0x00,0x00,
// start
        0x00,0x02,0x00,0x01,0x00,
        0xFF,0x01,0x00,0x05,0x04,0x01,0x00,0x00,0x00,
        0x00,0x68,0x00,0x01,0x00,
        0x00,0x24,0x00,0x00,0x00,
// end
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    if let bin_format::Constant::Num(n) = bin_format::get_constant(0x04, 1){
        println!("{}",n);
    }
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let proto = reader.read_proto();
    println!("{}",proto);

    let mut state = vm::executer::state::State::new(proto);
    println!("===== testing neg =====");
    println!("before execute {:?}",state.stack);
    state.execute();
    println!("after execute {:?}\n",state.stack);
}