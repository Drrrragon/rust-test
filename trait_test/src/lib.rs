pub mod test {

    pub trait Summarizable {
        fn summary(&self) -> String;
    }
    
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String
    }
    
    impl Summarizable for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }
    
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: String,
        pub retweet: String
    }
    
    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn notify<T: Summarizable>(item: T) {
        println!("Breaking news! {}", item.summary());
    }
}