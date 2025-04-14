use Playground::NewsArticle;
use Playground::Summary;
use Playground::Tweet;
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably aleary know,poeple"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet:{}", tweet.summarize());
    println!("1 new tweet:{}", tweet.summarize_data());

   

}

