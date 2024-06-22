pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    //trait 体中可以有多个方法：一行一个方法签名且都以分号结尾，均无实现。
    fn summarize_author(&self) -> String;
}

//struct NewsArticle
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//对NewsArticle的输出
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("这是一篇新闻文章，标题是{}, 作家是{} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

//struct Tweet
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    /*fn summarize(&self) -> String {
        //不跟分号
        format!("这是一个Tweet：{}，来自用户{}", self.content, self.username)
    }*/
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}