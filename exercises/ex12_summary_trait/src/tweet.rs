use core::fmt;

use chrono::{Date, DateTime, Utc};
use custom_trait_implementation::{Engagement, Summary};

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
    pub timestamp: DateTime<Utc>,
    pub retweets: u32,
    pub likes: u32,
    pub hashtags: Vec<String>
}

impl Tweet {
    pub fn new(
        username: String, 
        content: String, 
        reply: bool, 
        repost: bool, 
        timestamp: DateTime<Utc>, 
        retweets: u32, 
        likes: u32, 
        hashtags: Vec<String>) -> Tweet {
            Self {username, content, reply, repost, timestamp, retweets, likes, hashtags}
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "@{}: {} ({} retweets, {} likes, engagement score: {})",
            self.username,
            self.content,
            self.retweets,
            self.likes,
            self.engagement_score()
        )
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{} tweeted: \"{}\"", self.username, self.content)
    }
}

impl Engagement for Tweet {
    fn likes(&self) -> u32 {
        self.likes    
    }

    fn retweets(&self) -> u32 {
        self.retweets
    }
}