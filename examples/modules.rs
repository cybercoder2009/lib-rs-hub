use std::sync::Arc;

pub struct Module1 {
    code: usize,
    data: Vec<u8>
}

impl Module1 {
    pub fn new() -> Module1{
        Module1 {
            code: 1,
            data: vec!()
        }
    }
    pub fn handler(data: Vec<u8>){
        //println!("module1 receive {0}", data.len());
    }
}

pub struct Module2 {
    code: usize,
    data: Vec<u8>
}

impl Module2 {
    pub fn new() -> Module2{
        Module2 {
            code: 2,
            data: vec!()
        }
    }
    pub fn handler(data: Vec<u8>){
        //println!("module 2 receive {0}", data.len());
    }
}

pub struct Module3 {
    code: usize,
    data: Vec<u8>
}

impl Module3 {
    pub fn new() -> Module3{
        Module3 {
            code: 3,
            data: vec!()
        }
    }
    pub fn handler(data: Vec<u8>){
        //println!("module 3 receive {0}", data.len());
    }
}

pub struct Module4 {
    code: usize,
    data: Vec<u8>
}

impl Module4 {
    pub fn new() -> Module4{
        Module4 {
            code: 4,
            data: vec!()
        }
    }
    pub fn handler(data: Vec<u8>){
        //println!("module 4 receive {0}", data.len());
    }
}