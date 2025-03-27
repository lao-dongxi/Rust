use std::{clone, fmt::{Debug, Display}};

pub trait Summary{
    // fn summarize(&self)->String;
    fn summarize_author(&self)->String;
    fn summarize(&self)->String{
        format!("(Read more from {}...)",self.summarize_author())
    }
    
}
pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}
impl Summary for NewsArticle{
    fn summarize(&self)->String{
        format!("{},by {} ({})",self.headline,self.author,self.location)
    }
    fn summarize_author(&self)->String{
        format!("{}",self.author)
    }
} 

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}
impl Summary for Tweet{
    // fn summarize(&self)->String
    // {
    //     format!("{}:{}",self.username,self.content)
    // }
    fn summarize_author(&self)->String {
        format!("@{}",self.username)
    }
}
pub fn notify(item1:impl Summary,item2:impl Summary){ 
    println!("Breaking news! {}",item1.summarize());
} 
pub fn notify2<T:Summary>(item:T,item2:T){
    println!("Breaking news! {}",item.summarize());
}
pub fn notify3(item1:impl Summary+Display){ 
    println!("Breaking news! {}",item1.summarize());
} 
pub fn notify4<T:Summary+Display>(item:T){ 
    println!("Breaking news! {}",item.summarize());
}
pub fn notify5<T:Summary+Display,U:Clone+Debug>(a:T,b:U)->String{
    format!("{}",a.summarize())
} 
pub fn notify6<T,U>(a:T,b:U)->String
where T:Summary+Display,
      U:Clone+Debug,
{
    format!("{}",a.summarize())
    
}
pub fn notify7(s:&str)->impl Summary{
    NewsArticle{
        headline:String::from("Penguins win the Stanley Cup Championship!"),
        location:String::from("Pittsburgh, PA, USA"),
        author:String::from("Iceburgh"),
        content:String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
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
pub struct Student{
    name:String,
    adress:String,
    age:i32,
}

 impl Student {
    pub fn playftb(&self) {
        println!("We play football when free");
    }
    pub fn new(x:String,y:String,z:i32) ->Self{
        Student { 
            name: x, 
            adress: y, 
            age: z 
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
    fn gotoschool() {
        
    }
}
