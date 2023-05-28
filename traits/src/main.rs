pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary{
    fn summarize(&self) -> String;
}

fn main() {
    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("hello World"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Look down"),
        content: String:: from("Look above the sky"),
        location: String::from("kathmandu"),
    };

    println!("Tweet summary:{}", tweet.summarize());
    println!("Article summary:{}", article.summarize());
}