// #![feature(new_uninit)]
// #![feature(vec_into_raw_parts)]
// #![feature(alloc_layout_extra)]
// #![feature(slice_from_raw_parts)]

#[macro_use]
extern crate lazy_static;
extern crate libc;
use colored::*;
pub mod bin_format;
pub mod vm;
use std::env;
use bin_format::*;
use std::io;
use rustyline::Editor;

use async_std::prelude::*;
#[async_std::main]
async fn main() {
    let constant_pool = [
        0x01,
        0x11,
        0x01,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00,
    
        0x08, 0x00, 0x00, 0x00, 
        0x72, 0x00,  0x65, 0x00,  0x74, 0x00,  0x5F, 0x00,  0x6E, 0x00,  0x75, 0x00,  0x6C, 0x00,  0x6C, 0x00, 
        0x01,0x00,0x00,0x00,
        0x00,
        0x00,
        0x01,
        0x00,0x00,0x00,0x00,
        0x01,0x00,0x00,0x00,
        0x00,
        0x01,0x00,
        // start
        // label : 0
        0x00,0x00,
        0x02,0x00,0x00,0x00,
        0x00,0x02,0x00,0x01,0x00,
        0x00,0x04,0x00,0x00,0x00,
        // end
    
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
    ];

    let bytes = [
        0x02, 0x00, 0x00, 0x00, 0x4c, 0x00, 0x65, 0x00,
        0x00,0x00,0x00,0x00,
        0x00,
        0x00,
        0x00,
        0x00,0x00,0x00,0x00,
        0x00,0x00,0x00,0x00,
        // start
        // label : 0
        0x02,0x00,
        // start
        // label : 0
        0x00,0x00,
        0x05,0x00,0x00,0x00,

        0x00,0x45,0x00,0x00,0x00,
        0x00,0x4e,0x00,0x00,0x00,
        0x00,0x50,0x01,0x00,0x00,
        0x00,0x31,0x00,0x01,0x00,
        0x00,0x4d,0x01,0x00,0x00,

        0x00,0x01,
        0x03,0x00,0x00,0x00,
        0x00,0x55,0x02,0x00,0x00,
        0x00,0x4d,0x02,0x00,0x00,
        0x00,0x24,0x00,0x00,0x00,
        // end
        0x01,0x00,0x00,0x00,
        0x11,0x01,0x00,0x00,0x00,

        0x00,0x00,0x00,0x00,
    ];
    reader::Reader::read_constant_pool(constant_pool.as_ptr(), constant_pool.len());
    let mut reader = reader::Reader::new(bytes.as_ptr());
    let func = reader.read_func();
    //println!("{}",func);
    let stack= vm::executer::stack::Stack::new(Box::new(func));
    use vm::*;
    let debug = false;
    let h = new_thread(debug,stack);
    let (s,r) = get_sender_receiver(h.clone());
    println!("===== testing Async =====");
    println!("UUID: {}", h);
    if debug {
        let mut _rl = Editor::<()>::new();
        async_std::task::spawn(async move {
            println!("{:?}", get_join_handle(h).await);
            std::process::exit(0);
        });
        while debug {
            match _rl.readline("1 - step into\n2 - step over\n") {
                Ok(line) => {
                    match line.trim().as_ref() {
                        "1" => {
                            println!("{}", "    > step into\n".blue().bold());
                            s.send(VMMessage::StepInto).await;
                        }
                        "2" => {
                            println!("{}", "    > step over".blue().bold());
                            s.send(VMMessage::StepOver).await;
                        }
                        _ => {
                            println!("{}", "COMMAND NOT FOUND!".red().bold());
                        }
                    }
                },
                Err(_) => {
                    println!("{}", "Unknown error".red().bold());
                    break;
                }
            }
        }
    } else {
        println!("{:?}", get_join_handle(h).await);
    }
}
