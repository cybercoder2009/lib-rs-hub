extern crate lib_rs_hub;
extern crate threadpool;
extern crate rand;

mod modules;

use std::time::Duration;
use std::time::Instant;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::mpsc::Sender;
use std::thread;

use threadpool::Builder;
use rand::Rng;
use rand::thread_rng;
use lib_rs_hub::Hub;
use lib_rs_hub::Msg;
use modules::Module1;
use modules::Module2;
use modules::Module3;
use modules::Module4;


fn main(){
    let pool = Builder::new().thread_name(String::from("dump-workers")).num_threads(4).build();

    let module1 = Module1::new();
    let module2 = Module2::new();
    let module3 = Module3::new();
    let module4 = Module4::new();

    let mut hub = Hub::new();

    // mapping code
    // module 1 -> 1
    // module 2 -> 2
    // module 3 -> 3
    // module 4 -> 1

    println!("register Module1 to code 1");
    let s1: Sender<Msg> = hub.register(1, |msg: Msg|{
        println!("module1 receive code {0} data len {1}", &msg.code, &msg.data.len());
        Module1::handler(msg.data);
    }).unwrap();

    println!("register Module1 to code 2");
    let s2: Sender<Msg> = hub.register(2, |msg: Msg|{
        println!("module2 receive code {0} data len {1}", &msg.code, &msg.data.len());
        Module2::handler(msg.data);
    }).unwrap();

    println!("register Module1 to code 3");
    let s3: Sender<Msg> = hub.register(3, |msg: Msg|{
        println!("module3 receive code {0} data len {1}", &msg.code, &msg.data.len());
        Module3::handler(msg.data);
    }).unwrap();

    println!("register Module1 to code 4");
    let s4: Sender<Msg> = hub.register(1, |msg: Msg|{
        println!("module4 receive code {0} data len {1}", &msg.code, &msg.data.len());
        Module4::handler(msg.data);
    }).unwrap();
    hub.echo();

    let senders = [s1, s2, s3, s4];
    const dump_data_len: usize = 1024 * 1024; // MB
    let dump_data: [u8; dump_data_len]  = [0; dump_data_len];
    let dump_workers_num = 4;
    let dump_worker_dump_num = 10;// total_data_len / each_dump / dump_threads_num;

    thread::sleep(Duration::from_secs(5));

    let now = Instant::now();
    for x in 1..5 {
        let d0 = dump_data.clone();
        let s = senders[x - 1].clone();
        let _ = pool.execute(move ||{
            let mut i = 0;
            loop {
                let mut rng = thread_rng();
                let code = rng.gen_range(1, 4);
                println!("send code {0}", &code);
                s.send(Msg{
                    code,
                    data: d0.to_vec()
                });
                i += 1;

                if i > dump_worker_dump_num {
                    s.send(Msg{
                        code: 0,
                        data: vec!()
                    });
                    return;
                }
                //thread::sleep(Duration::from_millis(1));
            }
        });
    }
    pool.join();
    hub.run();
    let duration = now.elapsed();
    println!("time elapsed {:?}", duration);
}