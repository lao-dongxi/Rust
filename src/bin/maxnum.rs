fn main() {
    println!("{},{}",first_word(&String::from("123")),largest_num(&[1,2,3]));
}

//获取字符串长度
pub fn first_word(s:&String)->usize {
    let bytes=s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item==b' ' {
            return i;
        }
    }
    s.len()
}

//获取数组最大值
fn largest_num<T:PartialOrd>(list:&[T])->&T{
    let mut max=&list[0];
    for num in list.iter() {
        if num>max {
            max=num;
        }
    }
    max
}
