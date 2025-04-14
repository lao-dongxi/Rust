fn main() {
    let s="12312312   31  54654  545";
    println!("{}",first_word(s));
}

fn first_word(s:&str)->&str {
    let s1=s.as_bytes();
    for (a,&item) in s1.iter().enumerate()  {
        if b' '==item {
            return &s[..a]
        }
    }
    s
}