extern crate crossbeam_queue;
extern crate threadpool;

use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use crossbeam_queue::ArrayQueue;
use threadpool::ThreadPool;
use threadpool::Builder;

//    registry
//     ├── code_0
//     │    ├──  handler_0 (multi)
//     │    └──  handler_1
//     │── code_1
//     │    ├──  handler_0 (multi)
//     │    ├──  handler_2
//     │    └──  handler_3
//     │── code_2
//     ...

const cores: usize = 8;
const queue_size: usize = 20;

#[derive(Clone)]
pub struct Msg {
    pub code: usize,
    pub data: Vec<u8>
}

pub struct Hub{
    registry: Arc<RwLock<HashMap<usize, Vec<fn(Msg)>>>>,
    workers: ThreadPool,
    receivers: Vec<Receiver<Msg>>,
}

impl Hub {

    pub fn new() -> Hub {
        Hub {
            registry: Arc::new(RwLock::new(HashMap::new())),
            workers: Builder::new().thread_name(String::from("hub-workers")).num_threads(cores).build(),
            receivers: Vec::new()
        }
    }

    pub fn register(&mut self, code: usize, handler: fn(Msg)) -> Sender<Msg>{
        let (s, r) = channel::<Msg>();
        //self.receivers.insert(0, r);
        let mut new_code: bool = false;

        match self.registry.write() {
            Ok(mut write) => {
                match write.get_mut(&code) {
                    Some(mut handlers) => {
                        handlers.insert(0, handler);
                    },
                    None => {
                        new_code = true;
                    }
                }
            },
            Err(err) => {}
        }

        if new_code {
            let mut handlers: Vec<fn(Msg)> = vec!();
            handlers.insert(0, handler);
            self.registry.write().unwrap().insert(code, handlers);
        }

        let registry = self.registry.clone();
        &self.workers.execute(move|| {
            loop{
                match r.recv() {
                    Ok(msg) => {
                        //println!("receive {0}",  msg.code);
                        match registry.read().unwrap().get(&msg.code) {

                            Some(listeners) => {
                                for l in listeners {
                                    let l0 = l.clone();
                                    let msg0 = msg.clone();
                                    thread::spawn(move||{
                                        l0(msg0);
                                    });
                                }
                                //println!("listeners {0}", listeners.len());
                            },
                            None => {
                                println!("hub: unknown code {0}", msg.code);
                            }
                        }
                    },
                    Err(err) => {}
                };
                //thread::sleep(Duration::from_millis(1));
            }
        });

        s
    }
    pub fn echo(&self){
        println!("hub");
        match self.registry.read() {
            Ok(read) => {
                for (k,v) in read.iter() {
                    println!("├── code({0}) calls({1})", k, v.len());
                }
            },
            Err(err) => {

            }
        }
    }
}