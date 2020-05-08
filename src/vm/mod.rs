pub mod executer;
pub mod gc;
pub mod op;

use std::pin::Pin;
use std::sync::Mutex;
pub struct ThreadsRegister {
    pub threads: Vec<*mut executer::state::State>,
    pub channels: Vec<Mutex<Channel>>,
}
unsafe impl Send for ThreadsRegister {}
unsafe impl Sync for ThreadsRegister {}

lazy_static! {
    pub static ref THREAD_REGISTER: Mutex<ThreadsRegister> = Mutex::new(ThreadsRegister {
        threads: vec!(),
        channels: vec!(),
    });
}

pub fn new_thread(stack: executer::stack::Stack) -> async_std::task::JoinHandle<()> {
    let mut c = Box::pin(executer::state::State::new());
    (*c).push_stack(stack);
    let mut tr = THREAD_REGISTER.lock().unwrap();
    let len = tr.threads.len().clone();
    tr.threads.push(&mut (*c));
    let cc = tr.threads[len];
    unsafe { async_std::task::spawn((&mut (*cc)).execute()) }
}

use std::collections::LinkedList;
use async_std::sync::channel;
pub struct Channel {
    pub buffer: LinkedList<executer::Value>,
    pub current_send_index: usize,
    pub current_recv_index: usize,

    pub senders: Vec<*mut executer::state::State>,
    pub recvers: Vec<*mut executer::state::State>,
}