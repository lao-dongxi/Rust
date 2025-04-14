fn main() {
    let s=String::from("call me Davion. Some years ago...");
    let first_sentence=s.split('.')
                            .next()
                            .expect("could not find a '.'");
    let i=ImportantExcerpt{
        part:first_sentence,
    };
}

struct ImportantExcerpt <'a>{
    part:&'a str,
}

