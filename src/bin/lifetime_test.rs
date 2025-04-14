use std::fmt::Display;

fn main() {
    let u = user { part: "asdasd" };
    
    let r;
    
    let x:u32=5;
    r=&x;
    
    println!("r: {}", r);


    let s1=String::from("545");
    let s2="231231";

    

    let s3=largernum(s1.as_str(), s2);
    println!("{}",s3);

    let s4=String::from("long string is long");
    let result;
    {
        let s5= "xyz";
        result=largernum(s4.as_str(), s5);
    }
    println!("{}",result);

    fn largernum<'a>(x:&'a str,y:&'a str) ->&'a str{
        if x.len()>y.len() {
            x
        }
        else {
            y
        }

    }

    fn longest<'a>(x:&str,y:&str) ->String{
            // let x=5;
            // &'a x
            let result = String::from("really long string");
            result
    }

}
impl<'a> user<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn annouce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention pls:{}", announcement);
        self.part
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() > x.len() { y } else { x }
}
struct user<'a> {
    part: &'a str,
}

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("announcement!{}", ann);
    if x.len() > y.len() { x } else { y }
}
