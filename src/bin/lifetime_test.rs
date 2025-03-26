use std::fmt::Display;

fn main() {
    let u=user{
        part:"asdasd",
    };
}
impl <'a> user <'a>{
    fn level(&self)->i32 {
        3
    }
    fn annouce_and_return_part(&self,announcement:&str)->&str {
        println!("attention pls:{}",announcement);
        self.part
    }
}
fn longest<'a>(x:&'a str,y:&'a str)->&'a str {
    if y.len()>x.len() {
        y
    }
    else {
        x
    }
}
struct user<'a>{
    part:&'a str,
}

fn longest_with_an_annoucement<'a,T>(x:&'a str,y:&'a str,ann:T)->&'a str 
where T:Display
{
    println!("announcement!{}",ann);
    if x.len()>y.len() {
        x
    }    
    else {
        y
    }
}