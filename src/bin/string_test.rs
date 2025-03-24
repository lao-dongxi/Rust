use core::fmt;

fn main() {
    //声明字符串
    let mut s1=String::from("21445");
    let s2="65456+4".to_string();
    //更新字符串
    s1.push_str("16354");
    println!("{}",s1);
    s1.push('1');
    println!("{}",s1);
    //字符串拼接
    let s3="131";
    println!("{}",s3);
    let s=format!("{}{}{}",s1,s2,s3);
    println!("{}",s);
    //字符串内部访问
    let s5=String::from("我的家").len();
    let s6=String::from("很爱很爱你");
    println!("{}",s5);
    for i in s1.bytes()  {
        println!("{}",i);
    }
    for i in s6.chars()  {
        println!("{}",i);
    }
    //字符串切割
    let spl=&s6[0..3];
    println!("{},{}",s6.len(),spl);
}