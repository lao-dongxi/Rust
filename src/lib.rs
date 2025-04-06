use std::{
    clone,
    fmt::{Debug, Display},
};
use std::{env, error::Error, fs, iter::Skip, process};

//grep代码单元
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    // println!("with text:\n{}",contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("argments is not enough!");
        }
        let config = Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        };
        Ok(config)
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe,fast,productive.
pick three.
Duck tape";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe,fast,productive.
pick three.
Duck tape
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(query, contents)
        )
    }
}

//trait练习
pub trait Summary {
    // fn summarize(&self)->String;
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    // fn summarize(&self)->String
    // {
    //     format!("{}:{}",self.username,self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
pub fn notify(item1: impl Summary, item2: impl Summary) {
    println!("Breaking news! {}", item1.summarize());
}
pub fn notify2<T: Summary>(item: T, item2: T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify3(item1: impl Summary + Display) {
    println!("Breaking news! {}", item1.summarize());
}
pub fn notify4<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify5<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("{}", a.summarize())
}
pub fn notify6<T, U>(a: T, b: U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    format!("{}", a.summarize())
}
pub fn notify7(s: &str) -> impl Summary {
    NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    }
}

//学生管理
pub trait Everyday {
    fn eat(&self);
    fn drink(&self);
    fn gotoschool() {
        println!("we go to school everyday");
    }
}
pub struct Student {
    name: String,
    adress: String,
    age: i32,
}

impl Student {
    pub fn playftb(&self) {
        println!("We play football when free");
    }
    pub fn new(x: String, y: String, z: i32) -> Self {
        Student {
            name: x,
            adress: y,
            age: z,
        }
    }
}
impl Everyday for Student {
    fn drink(&self) {
        println!("we drink milk when breakfast");
    }
    fn eat(&self) {
        println!("we eat rice food when lunch")
    }
    fn gotoschool() {}
}


// module system

pub use front_of_house::hosting;
 mod front_of_house; 


pub fn eat_at_restaurant() {
    // 绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    // 相对路径
    // front_of_house::hosting::add_to_waitlist();
}
