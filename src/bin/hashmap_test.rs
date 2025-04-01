use std::collections::HashMap;
fn main() {
    //声明和赋值hashmap
    let mut scores = HashMap::new();
    scores.insert(String::from("45"), 2);
    scores.insert(String::from("45465"), 12);

    let teams = vec![String::from("blue"), String::from("Yellow")];
    let initalscores = vec![10, 20];
    let scor: HashMap<_, _> = teams.iter().zip(initalscores.iter()).collect();

    //访问hashmap的键值
    let name = String::from("45");
    let sc = scores.get(&name);
    match sc {
        Some(s) => println!("{}", s),
        None => println!("No"),
    };

    for (k, v) in &scores {
        println!("{},{}", k, v);
    }

    //更新hashmap
    scores.insert(String::from("45465"), 10);
    let m1 = scores.entry(String::from("45"));
    println!("{:#?}", m1);
    let r1 = m1.or_insert(5);
    print!("{}", r1);
    println!("===========");
    let m2 = scores.entry(String::from("blue"));
    println!("{:#?}", m2);
    let r2 = m2.or_insert(5);
    print!("{}", r2);
    println!("===========");

    for (k, v) in &scores {
        println!("{},{}", k, v);
    }

    //entry的or_insert方法应用
    let str7 = "we are from China and China is a great country";
    let mut hs = HashMap::new();
    for word in str7.split_whitespace() {
        let temp = hs.entry(word).or_insert(0);
        *temp += 1;
    }
    println!("{:?}", hs);
}
