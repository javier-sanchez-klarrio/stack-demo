use regex::Regex;

#[derive(Debug)]
struct TodoItem {
    title: String,
    completed: bool,
}

impl TodoItem {
    fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            completed: false,
        }
    }
    
    fn complete(&mut self) {
        self.completed = true;
    }
}

fn is_valid_title(title: &str) -> bool {
    Regex::new(r"\S").unwrap().is_match(title)
}

fn main() {
    let task = TodoItem::new("Prepare stacked PR demo");
    println!("{task:?}");
    println!("Valid title: {}", is_valid_title(&task.title));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completing_a_task_marks_it_completed() {
        let mut task = TodoItem::new("Write demo");

        assert!(!task.completed);

        task.complete();

        assert!(task.completed);
    }
}
