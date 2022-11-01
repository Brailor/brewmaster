use std::path::Path;

pub struct ChangeSet {
    file_name: Box<Path>,
    action: Action,
}

pub enum Action {
    Add(),
    Modify(),
    Delete(),
    Untrack(),
    Rename(),
}

pub fn run(_changes: Vec<ChangeSet>) {
    println!("Hello from Brewmaster!")
}
