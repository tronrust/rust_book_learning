use chap_10::{notify, NewsArticle, Summary, Tweet};
fn main() {
    // let tweet = Tweet {
    //     username: "Tron Nguyen".to_string(),
    //     content: "This is my first tweet".to_string(),
    //     reply: false,
    //     retweet: false,
    // };
    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best \
    //          hockey team in the NHL.",
    //     ),
    // };
    // println!("1 new tweet: {}", tweet.summarize());
    // println!("1 new tweet: {}", tweet.demo_default_impl());
    // println!("New article available! {}", article.demo_default_impl());
    //
    // self::notify(&article);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
            .split('.')
            .next()
            .expect("Could not find a '.'"),
    };
    i.announce_and_return_part(first_sentence);
}
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
