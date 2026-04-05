fn summarize_string(s: &str) -> &str {
    &s[..20.min(s.len())]
}

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    title: String,
    body: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, summarize_string(&self.body))
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.username, summarize_string(&self.content))
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let article = Article {
        title: String::from("The Rust Programming Language"),
        body: String::from("This is a book about Rust programming language."),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is a systems programming language."),
    };

    notify(&article);
    notify(&tweet);
}

// Solution
// trait Summary {
//     fn summarize(&self) -> String;
// }

// struct Article { title: String, body: String }
// struct Tweet { username: String, content: String }

// impl Summary for Article {
//     fn summarize(&self) -> String {
//         format!("{} — {}...", self.title, &self.body[..20.min(self.body.len())])
//     }
// }

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("@{}: {}", self.username, self.content)
//     }
// }

// fn notify(item: &impl Summary) {
//     println!("📢 {}", item.summarize());
// }

// fn main() {
//     let article = Article {
//         title: "Rust is great".into(),
//         body: "Here is why Rust beats Python for systems...".into(),
//     };
//     let tweet = Tweet {
//         username: "rustacean".into(),
//         content: "Just shipped my first crate!".into(),
//     };
//     notify(&article);
//     notify(&tweet);
// }
