fn main() {
    let news = news_feed::NewsFeed::new(
        String::from("The News"),
        String::from("news.com"),
        Some(String::from("Gossip")),
    );
    println!("NewsFeed: {}", news.url());
}

mod news_feed {
    use proc_getters::Getters;

    #[derive(Getters)]
    pub struct NewsFeed {
        name: String,
        url: String,
        category: Option<String>,
    }

    #[derive(Getters)]
    struct NewsFeedRef<'a> {
        name: &'a str,
        url: &'a str,
        category: Option<&'a str>,
    }

    impl NewsFeed {
        pub fn new(name: String, url: String, category: Option<String>) -> Self {
            NewsFeed {
                name,
                url,
                category,
            }
        }
    }
}
