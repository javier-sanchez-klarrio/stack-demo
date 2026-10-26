
# Stacked PR Demo

(This readme has been created by AI)

This repository demonstrates stacked pull requests using GitHub CLI and `gh-stack`. Each PR builds on the previous one.

## Setup

```bash
cargo new stack-demo
cd stack-demo

gh extension install github/gh-stack
gh auth login

gh repo create stack-demo --public --source=. --remote=origin --push
```

Implement the initial task tracker in `src/main.rs`, then commit it to `main`:

```bash
git add .
git commit -m "chore: initialize task tracker"
git push -u origin main
```

## PR 1 — Update a dependency

**Goal:** Update `regex` from `1.10.6` to `1.11.1`.

Initialize the stack:

```bash
gh stack init update-dependency
```

Add the dependency to `Cargo.toml`:

```toml
[dependencies]
regex = "=1.10.6"
```

Use `regex` to validate task titles in `src/main.rs`, then update the dependency:

```bash
cargo update -p regex --precise 1.11.1
cargo test

git add Cargo.toml Cargo.lock src/main.rs
git commit -m "chore: update regex dependency"
```

## PR 2 — Rename `Task` to `TodoItem`

**Goal:** Rename the struct throughout the project.

Create the next branch on top of PR 1:

```bash
gh stack add rename-task
```

In `src/main.rs`, rename `Task` to `TodoItem`, including its implementation and all references.

```rust
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
}
```

Keep the existing title validation and update `main()` to use `TodoItem`.

```bash
cargo test
git add src/main.rs
git commit -m "refactor: rename Task to TodoItem"
```

## PR 3 — Add task completion

**Goal:** Add functionality to mark a task as completed.

Create the third branch:

```bash
gh stack add complete-task
```

Add this method to `impl TodoItem`:

```rust
fn complete(&mut self) {
    self.completed = true;
}
```

Update `main()` to create a mutable task and call `task.complete()`.

Add a unit test:

```rust
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
```

Run the tests and commit:

```bash
cargo test
git add src/main.rs
git commit -m "feat: add task completion"
```

## Publish the stack

Submit all three branches and create the stacked PRs:

```bash
gh stack submit --auto
```

Inspect the result:

```bash
gh stack view --json
gh stack up
gh stack down
```

## Expected stack

```text
main
 └── PR 1: Update regex dependency
      └── PR 2: Rename Task to TodoItem
           └── PR 3: Add task completion
```

Each PR targets the preceding branch, so reviewers can focus on one logical change at a time.
