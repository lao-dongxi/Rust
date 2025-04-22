use std::time::Duration;
use std::thread;

fn main() {
    generate_workout(30, 3);
    generate_workout(10, 7);
}
fn generate_workout(intensity:u32,random_number:u32) {
    let mut closure_exp=Cacher::new(|n|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(5));
        n
    });

    if intensity<25 {
        println!("today do {} pushups",closure_exp.value(intensity));
        println!("today do {} situps",closure_exp.value(intensity));
        }        
    else {
        if random_number==3 {
            println!("Take a break day");
        }
        else {
            println!("
            Run for {} today",closure_exp.value(intensity));
        }
    }

}
struct Cacher<T>
where T:Fn(u32)->u32
{
    caculation:T,
    value:Option<u32>,
}
impl <T> Cacher<T>
where T:Fn(u32)->u32
{
    fn new(caculation:T)->Cacher<T> {
        Cacher { 
            caculation,
            value: None,
        }
    }
    fn value(&mut self,arg:u32)->u32 {
        match self.value {
            Some(v)=>v,
            None=>{
                let v=(self.caculation)(arg);
                self.value=Some(v);
                v
            }
        }
    }
}