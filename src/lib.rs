use std::{fs, io, path::Path, time::Instant};

const NODE_FILE_CONSTANTS: [&str; 3] = ["README.md", "FIGURE.", "DATA."];

#[derive(Debug)]
struct Entry {
    slug: String,
    entry_type: Type,
}

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

pub fn run(changes: Vec<ChangeSet>) -> io::Result<()> {
    let changes: Vec<&ChangeSet> = changes
        .iter()
        .filter(|ch| match ch.action {
            Action::Untrack() => false,
            _ => true,
        })
        .collect();

    // detect the node type by inspect the slug/file_name
    let _entries = detect_node_types(changes)?;

    // create isosec for each entry
    let t = Instant::now();
    println!("{:#?}", t);

    Ok(())
}

// isosec is
// exec date -u +%Y%m%d%H%M%S "$@"
fn create_isosec() {}

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
fn detect_node_types(changes: Vec<&ChangeSet>) -> io::Result<Vec<Entry>> {
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

    let entries: Vec<Entry> = dirs
        .iter()
        .map(|dir| {
            let (dir_name, node_type) = get_type(dir).unwrap();

            println!("type for dir = {} is = {:?}", dir_name, node_type);

            Entry {
                slug: String::from(dir_name),
                entry_type: node_type,
            }
        })
        .collect();

    Ok(entries)
}

fn get_type<'a>(dirs_files: &'a (String, Vec<String>)) -> Option<(&'a str, Type)> {
    let (dir_name, files) = dirs_files;
    match &files[..] {
        [x] => Some((dir_name, Type::Single(infer_type(x).unwrap()))),
        [x, y] => Some((
            dir_name,
            Type::Binary(infer_type(x).unwrap(), infer_type(y).unwrap()),
        )),
        [x, y, z] => Some((
            dir_name,
            Type::Complex(
                infer_type(x).unwrap(),
                infer_type(y).unwrap(),
                infer_type(z).unwrap(),
            ),
        )),
        _ => None,
    }
}

fn infer_type(s: &String) -> Option<NodeType> {
    match s {
        s if s.matches(NODE_FILE_CONSTANTS.get(0).unwrap()).count() > 0 => Some(NodeType::Text),
        s if s.matches(NODE_FILE_CONSTANTS.get(1).unwrap()).count() > 0 => Some(NodeType::Figure),
        s if s.matches(NODE_FILE_CONSTANTS.get(2).unwrap()).count() > 0 => Some(NodeType::Data),
        _ => None,
    }
}
