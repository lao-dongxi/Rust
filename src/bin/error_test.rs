use core::error;
use std::{error::Error, fs::{self, File}, io::{self, ErrorKind, Read}};
fn main() {

    //match嵌套
    // let fl=File::open("hello_world.txt");
    // let fl= match fl {
    //     Ok(file)=>file,
    //     Err(error)=>match error.kind() {
    //         ErrorKind::NotFound=>match File::create("hello_world.txt") {
    //             Ok(fs)=>fs,
    //             Err(error)=>  {panic!("problem creating the file:{:?}",error)},
    //         },
    //         other_err=>panic!("Problem opening the file:{:?}",error),
    //     }
    // };

    // let fs=File::open("hello.txt").unwrap_or_else(|error|{
    //     if error.kind()==ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error|{
    //             panic!("problem creating file:{:?}",error);
    //         })
    //     }else {
    //         panic!("problem opening file:{:?}",error);
    //     }
    // });

    // let f = File::open("hel.txt").unwrap();
    // let f = File::open("hel.txt").expect("failed to open hel.txt");

    fn read_username_from_file()->Result<String,io::Error> {
        // let  f=File::open("hello.txt");
        // let mut f=match f {
        //     Ok(file)=>file,
        //     Err(e)=>return Err(e),
        // };
        // let mut s=String::new();
        // match f.read_to_string(&mut s) {
        //     Ok(_)=>Ok(s),
        //     Err(e)=>return Err(e),
        // }

        // let mut f=File::open("hello.txt")?;
        
        // let mut s=String::new();
        // f.read_to_string(&mut s)?;
        // Ok(s)

        // let mut s=String::new();
        // File::open("hello.txt")?.read_to_string(&mut s)?;
        // Ok(s)
        fs::read_to_string("hell.txt")
    }
}