use Playground::{Config, run};
use std::{env, error::Error, fs, iter::Skip, process};

fn main() {
    // let args=env::args().skip(1)
    // for i in env::args()  {
    //     println!("{}",i);
    // }
    let args: Vec<String> = env::args().collect();
    // for i in args  {
    //     println!("{}",i);
    // }
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument:{}", err);
        process::exit(1)
    });
    if let Err(e) = run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}
