use std::path::Path;

#[derive(Debug)]
pub struct ChangeSet<'a> {
    pub file_name: &'a Path,
    pub action: Action,
}

#[derive(Debug)]
pub enum Action {
    Add(),
    Modify(),
    Delete(),
    Untrack(),
    Rename(),
}

pub fn run(changes: Vec<ChangeSet>) {
    println!("The changes are: {:?}", changes);
}
