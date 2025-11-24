trait Summary {
    fn summarize(&self) -> String {
        // default summary
        String::from("Read more...")
    }
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!(
            "@{}\n{}\nReply: {}\nRepost: {}",
            self.username, self.content, self.reply, self.repost
        )
    }
}

/// print the summary of any item that implements the Summary trait
fn notify(item: &impl Summary) {
    println!("NEW!\n{}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    }
}

fn main() {
    let post = returns_summarizable();

    notify(&post);
}
