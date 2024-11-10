mod news_article;
mod summary;

use news_article::NewsArticle;
use summary::Summary;

struct Tweet {
    user: String,
    contents: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{0} by {1}", self.contents, self.user)
    }
}

// We can now define a function which accepts any type,
// that implements the trait.
fn notify(item: &impl Summary) {
    println!("New: {0}", item.summarize());
}

pub fn traits() {
    let article = NewsArticle {
        user: String::from("Che"),
        headline: String::from("I got rework"),
        contents: String::from("I got new tactical and ultimate abilities"),
    };

    // Note that when we're using a method,
    // that is defined as part of implementation
    // of trait, we must bring the trait in scope
    // to be able to use it.

    // When implementing traits in our code,
    // we must ensure, either the trait or the type
    // is local to our crate.

    // We can implement these in "our" crate:
    // 1. 'Display' (from std lib) in 'NewsArticle' (from our crate).
    // 2. 'Summary' (from out crate) in 'Vec' (from std lib).

    // We can't implement 'Display' on 'Vec' on our crate.
    // As both (the type and the trait) are external to our crate.

    println!("{0}", article.summarize());

    let tweet = Tweet {
        user: String::from("musk"),
        contents: String::from("I can catch rockets now"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}
