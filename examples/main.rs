extern crate lib_rs_hub;
extern crate threadpool;
extern crate rand;

mod modules;

use std::time::Duration;
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
    let pool = Builder::new().thread_name(String::from("dump-workers")).num_threads(8).build();

    let module1 = Module1::new();
    let module2 = Module2::new();
    let module3 = Module3::new();
    let module4 = Module4::new();

    let mut hub = Hub::new();
    
    //! mapping code
    //! module 1 -> 1
    //! module 2 -> 2
    //! module 3 -> 3
    //! module 4 -> 1

    println!("register Module1 to code 1");
    let s1: Sender<Msg> = hub.register(1, |msg: Msg|{
        println!("module1 receive code {0}", &msg.code);
        Module1::handler(msg.data);
    });
    println!("register Module1 to code 2");
    let s2: Sender<Msg> = hub.register(2, |msg: Msg|{
        println!("module2 receive code {0}", &msg.code);
        Module2::handler(msg.data);
    });
    println!("register Module1 to code 3");
    let s3: Sender<Msg> = hub.register(3, |msg: Msg|{
        println!("module3 receive code {0}", &msg.code);
        Module3::handler(msg.data);
    });
    println!("register Module1 to code 4");
    let s4: Sender<Msg> = hub.register(1, |msg: Msg|{
        println!("module4 receive code {0}", &msg.code);
        Module4::handler(msg.data);
    });
    hub.echo();

    thread::sleep(Duration::from_secs(5));
    let t_0 = pool.execute(move ||{
        loop {
            let mut rng = thread_rng();
            let code = rng.gen_range(1, 4);
            s1.send(Msg{
                code,
                data:[0].to_vec()
            });
            thread::sleep(Duration::from_millis(code as u64));
        }
    });

    pool.join();
}