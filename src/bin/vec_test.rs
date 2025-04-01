fn main() {
    // let v:Vec<i32>=Vec::new();
    //声明且赋值
    let mut v = vec![1, 2, 3];
    //添加元素
    v.push(1);
    v.push(4);
    v.push(5);
    //访问元素
    let third = &v[2];
    println!("{}", third);
    match v.get(101) {
        Some(third) => println!("{}", third),
        None => println!("No num!"),
    }
    let first = &v[0];
    v.push(6);
    println!("123");
    // print!("{}",first);
    // print!("{}",&v[2]);
    //遍历元素
    for i in &mut v {
        *i = *i + 50;
    }
    for num in &mut v {
        println!("{}", num);
    }

    #[derive(Debug)]
    enum User {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let friend = vec![
        User::Int(3),
        User::Float(10.12),
        User::Text(String::from("23456")),
    ];

    for temp in &friend {
        match temp {
            User::Int(c) => println!("{}", c),
            User::Float(value) => println!("{}", value),
            User::Text(s) => println!("{}", s),
        }
    }
    for temp in &friend {
        println!("{:?}", temp);
    }
}
