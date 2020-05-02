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
fn main() {
    let args: Vec<String> = env::args().collect();
    reader::Reader::read_constant_pool_from_file(String::from("./"), args[1].clone());
    let bc = reader::Reader::read_binary_chunk_from_file(String::from("./"), args[1].clone());
    let mut state = vm::executer::state::State::new();
    let mut stack= vm::executer::stack::Stack::new(Box::new(bc.entry));
    state.push_stack(stack);
    println!("===== testing =====");
    println!("before execute {:?}",state.stack().stack);
    state.execute();
    println!("after execute {:?}\n",state.stack().stack);
    assert_eq!(*state.stack().stack.last().unwrap(),vm::executer::Value(vm::executer::PrimeValue::Null,bin_format::Type::Null));
}