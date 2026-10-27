use regex::Regex;

#[derive(Debug)]
struct Item {
    title: String,
    completed: bool,
}

impl Item {
    fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            completed: false,
        }
    }
}

fn is_valid_title(title: &str) -> bool {
    Regex::new(r"\S").unwrap().is_match(title)
}

fn main() {
    let task = Item::new("Prepare stacked PR demo");
    println!("{task:?}");
    println!("Valid title: {}", is_valid_title(&task.title));
}
