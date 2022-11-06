use globset::{Glob, GlobSetBuilder};
use std::{collections::hash_map::Entry, fs, io, path::Path};

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

#[derive(Debug)]
pub enum NodeType {
    Text,
    Data,
    Figure,
}

#[derive(Debug)]
pub enum Type {
    Single(NodeType),
    Binary(NodeType, NodeType),
    Complex(NodeType, NodeType, NodeType),
}

pub fn run(changes: Vec<ChangeSet>) {
    let changes: Vec<&ChangeSet> = changes
        .iter()
        .filter(|ch| match ch.action {
            Action::Untrack() => false,
            _ => true,
        })
        .collect();

    // detect the node type by inspect the slug/file_name
    detect_node_types(changes);
}

const NODE_FILE_CONSTANTS: [&str; 3] = ["README.md", "FIGURE.", "DATA."];
// A directory can contain the following cases
// README.md only -> NodeType::Text
// DATA.* -> NodeType::Data
// FIGURE.* -> NodeType::Figure
// README.md + DATA.* -> NodeType::DataText
// README.md + FIGURE.* -> NodeType::TextFigure
// DATA.* + FIGURE.* -> NodeType::DataFigure
// FIGURE.* + DATA.* -> ?
// FIGURE.* + README.md -> ? how to specify priority between files (text + figure, figure + text)
// README.md + DATA.* + FIGURE.* -> NodeType::Rich
fn detect_node_types(changes: Vec<&ChangeSet>) -> io::Result<()> {
    let dirs: Vec<(String, Vec<String>)> = changes
        .iter()
        .map(|ch| ch.file_name.parent())
        .filter(|dir| dir.is_some())
        .map(|dir| {
            let dir = dir.expect("Not able to get slug");

            let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
            // println!("{dir_name}");
            let entries = fs::read_dir(dir).expect("Failed to read dir");

            let entries: Vec<String> = entries
                .filter(|e| e.is_ok())
                .map(|e| {
                    let e = e.unwrap();

                    let file_name = e.file_name().to_str().unwrap().to_owned();

                    file_name
                })
                .filter(|file_name| {
                    let is_match = NODE_FILE_CONSTANTS
                        .iter()
                        .any(|file| file_name.matches(file).count() > 0);

                    is_match
                })
                .collect();

            (dir_name, entries)
        })
        .collect();
    println!("Entries: {:?}", dirs);

    dirs.iter().for_each(|d| {
        let (dir_name, t) = get_type(d.to_owned());

        println!("type for dir = {} is = {:?}", dir_name, t);
    });

    Ok(())
}

fn get_type(dirs_files: (String, Vec<String>)) -> (String, Type) {
    let (dir_name, files) = dirs_files;
    let t = match &files[..] {
        [x] => {
            let t = match x {
                s if s.matches(NODE_FILE_CONSTANTS.get(0).unwrap()).count() > 0 => NodeType::Text,
                s if s.matches(NODE_FILE_CONSTANTS.get(1).unwrap()).count() > 0 => NodeType::Figure,
                s if s.matches(NODE_FILE_CONSTANTS.get(2).unwrap()).count() > 0 => NodeType::Data,
                _ => panic!("Non possible pattern"),
            };
            Type::Single(t)
        }
        [_x, _y] => Type::Binary(NodeType::Text, NodeType::Text),
        _ => Type::Complex(NodeType::Text, NodeType::Data, NodeType::Figure),
    };
    (dir_name, t)
}
