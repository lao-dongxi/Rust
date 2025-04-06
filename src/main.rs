
use Playground::eat_at_restaurant;
use Playground::hosting;
use Playground::Everyday;
use Playground::Student;
fn main() {
    let stu1 = Student::new(String::from("xiaoli"), String::from("xinyang"), 12);
    stu1.playftb();
    stu1.drink();
    stu1.eat();
    eat_at_restaurant();
}
