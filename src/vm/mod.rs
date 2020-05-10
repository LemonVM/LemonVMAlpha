pub mod executer;
pub mod gc;
pub mod op;

use std::pin::Pin;
use std::sync::Mutex;
use async_std::sync::*;

pub struct ThreadsRegister {
    pub threads: Vec<(u32,async_std::task::JoinHandle<(Vec<executer::Value>,Option<executer::stack::Stack>)>,Sender<VMMessage>,Receiver<String>)>,
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
#[derive(PartialEq)]
pub enum VMMessage{
    PrintStack = 0x00,
    PrintFrame = 0x01,
    Continue = 0x02,
    Break = 0x03,
    StepOver = 0x04,
    StepInto = 0x05,
}

pub fn new_thread(debug_mode:bool,stack: executer::stack::Stack) -> u32 {
    // TODO: free
    use async_std::sync::channel;
    let uuid = executer::gen_uuid();
    let (message_chan_send,message_recv) = channel(1);
    let (string_chan_send,string_chan_recv) = channel(1);
    let mut state = executer::state::State::new(debug_mode,uuid,string_chan_send,message_recv);
    state.push_stack(stack);
    let mut pc = std::pin::Pin::new(&mut state);
    let mut tr = THREAD_REGISTER.lock().unwrap();
    tr.threads.push((uuid,async_std::task::spawn(state.execute()),message_chan_send.clone(),string_chan_recv.clone()));
    uuid
}
pub fn new_sub_thread(debug_mode:bool,stack: executer::stack::Stack,sender:Sender<String>,receiver:Receiver<VMMessage>) -> u32 {
    // TODO: free
    use async_std::sync::channel;
    let uuid = executer::gen_uuid();
    let mut state = executer::state::State::new(debug_mode,uuid,sender.clone(),receiver.clone());
    state.push_stack(stack);
    let mut pc = std::pin::Pin::new(&mut state);
    let mut tr = THREAD_REGISTER.lock().unwrap();
    let mut dummy0 = channel(1);
    let mut dummy1 = channel(1);
    tr.threads.push((uuid,async_std::task::spawn(state.execute()),dummy0.0,dummy1.1));
    uuid
}

pub fn get_join_handle(uuid:u32)->async_std::task::JoinHandle<(Vec<executer::Value>,Option<executer::stack::Stack>)>{
    let mut a = THREAD_REGISTER.lock().unwrap();
    for t in 0..a.threads.len(){
        if a.threads[t].0 == uuid{
            let r = a.threads.remove(t);
            return r.1;
        }
    }
    panic!("NO THREAD WITH UUID {}",uuid)
}

pub fn get_sender_receiver(uuid:u32)->(Sender<VMMessage>, Receiver<String>){
    let mut a = THREAD_REGISTER.lock().unwrap();
    for t in 0..a.threads.len(){
        if a.threads[t].0 == uuid{
            return (a.threads[t].2.clone(),a.threads[t].3.clone());
        }
    }
    panic!("NO THREAD WITH UUID {}",uuid)
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