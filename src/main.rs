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
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x01, 0x00, 0x00, 0x00,
        0x01, 0x01
    ];
    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00,
        0x00,
        0x00,

        0x02,0x00,0x00,0x00,
// start
        0x68,0x00,0x01,0x00,
        0x24,0x00,0x00,0x00,
// end
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let proto = reader.read_proto();
    println!("{}",proto);

    let mut state = vm::executer::state::State::new(proto);
    use vm::executer::PrimeType::*;
    state.stack.stack.push(Null);
    state.stack.stack.push(Num(0.5));
    println!("===== testing neg =====");
    println!("before execute {:?}",state.stack);
    state.execute();
    println!("after execute {:?}\n",state.stack);

}