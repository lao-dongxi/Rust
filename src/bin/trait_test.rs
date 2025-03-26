use Playground::Summary;
use Playground::Tweet;
use Playground::NewsArticle;
fn main() {
    let tweet=Tweet{
        username:String::from("horse_ebooks"),
        content:String::from("of course, as you probably aleary know,poeple"),
        reply:false,
        retweet:false,
    };
    print!("1 new tweet:{}",tweet.summarize());
}