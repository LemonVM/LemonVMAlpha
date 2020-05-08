pub mod executer;
pub mod gc;
pub mod op;

use std::pin::Pin;
use std::sync::Mutex;
use async_std::sync::*;

pub struct ThreadsRegister {
    pub threads: Vec<(u32,Sender<VMMessage>,Receiver<String>)>,
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

pub enum VMMessage{
    PrintStack = 0x00,
    PrintFrame = 0x01,
    Continue = 0x02,
    Break = 0x03,
}

pub fn new_thread(stack: executer::stack::Stack,uuid:u32) -> (async_std::task::JoinHandle<()>,Sender<VMMessage>,Receiver<String>) {
    // TODO: free
    use async_std::sync::channel;
    let (message_chan_send,message_recv) = channel(1);
    let (string_chan_send,string_chan_recv) = channel(1);
    let mut state = executer::state::State::new(uuid,string_chan_send,message_recv);
    state.push_stack(stack);
    let mut pc = std::pin::Pin::new(&mut state);
    let mut tr = THREAD_REGISTER.lock().unwrap();
    tr.threads.push((uuid,message_chan_send.clone(),string_chan_recv.clone()));
    (async_std::task::spawn(state.execute()),message_chan_send,string_chan_recv)
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