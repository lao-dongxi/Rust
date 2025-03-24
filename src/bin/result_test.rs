use std::{fs::File, io::ErrorKind};

fn main() {
    let v=File::open("hello.txt");
   let e=match v {
        Ok(s)=>s,
        Err(error)=>match error.kind() {
            ErrorKind::NotFound=>match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e)=>panic!("error create file:{:?}",e),
            },
            other=>panic!("error opening file {}",other),
            
        },
    };



    // let q=File::open("china.txt").unwrap();
    let q=File::open("china.txt").expect("找不到指定文件");
    
}