use super::summary::Summary;

pub struct NewsArticle {
    pub user: String,
    pub headline: String,
    pub contents: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{0} by {1}", self.headline, self.user)
    }
}
