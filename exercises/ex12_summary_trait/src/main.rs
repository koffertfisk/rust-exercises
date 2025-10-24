mod newsarticle;
mod tweet;

use std::string;

use chrono::{DateTime, Utc};
use custom_trait_implementation::{summarize_and_print, Summary};

use crate::{newsarticle::NewsArticle, tweet::Tweet};

fn main() {
    println!("Text Summaries using Traits\n");

    let post = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
        timestamp: Utc::now(),
        retweets: 3,
        likes: 5,
        hashtags: vec![String::from("#horse")]
    };

    println!("1 new post: {}", post.summarize());
    summarize_and_print(&post);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
    summarize_and_print(&post);
}
