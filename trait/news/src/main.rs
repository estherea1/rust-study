use news::{self, NewsArticle, Summary, Tweet};

fn main() {
    let tweet =Tweet {
        username: String::from("esther"),
        content: String::from(
            "tweeeeeeet",
        ),
        reply: false,
        retweet: false,
    };

    println!("{}",tweet.summarize());

    let article = NewsArticle {
        headline: String::from("暴雨来临"),
        location: String::from("Mainland, China"),
        author: String::from("Mice"),
        content: String::from(
            "请公民们注意安全，及时收取衣物，暴雨即将来袭。",
        ),
    };

    println!("{}", article.summarize());

}
