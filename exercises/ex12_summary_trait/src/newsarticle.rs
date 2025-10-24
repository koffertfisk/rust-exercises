use core::fmt;

use custom_trait_implementation::Summary;

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl NewsArticle {
    pub fn new(headline: String, location: String, author: String, content: String) -> NewsArticle {
        Self {headline, location, author, content}
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "@{} wrote: \"{}\"", self.author, self.headline)
    }
}