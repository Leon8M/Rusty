
pub struct NewsArticle {
    pub author :String,
    pub headline: String,
    pub content: String
}

impl Summary for NewsArticle{
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet{
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary{
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub fn notify<T: Summary>(item: &T){
    println!("Breaking News! {}", item.summarize());
}

pub fn some_fn <T, U> (t: &T, u: &U)
    where T: Summary,
          U: Summary
{
    //
}

fn main() {
    let tweet = Tweet{
        username: String::from("@john"),
        content: String::from("Hello"),
        reply: false,
        retweet: false
    };

    let article = NewsArticle{
        author: String::from("John Wess"),
        headline: String::from("The sky"),
        content: String::from("No it is not")
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
}
