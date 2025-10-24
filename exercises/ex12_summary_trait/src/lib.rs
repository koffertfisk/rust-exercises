pub trait Summary {
    fn summarize(&self) -> String;
}

pub fn summarize_and_print<T: Summary>(item: &T) { 
    println!("Summary: {}", item.summarize());
}

pub trait Engagement {
    fn likes(&self) -> u32;
    fn retweets(&self) -> u32 {
        0
    }
    
    fn engagement_score(&self) -> u32 {
        self.likes() + self.retweets()
    }
}