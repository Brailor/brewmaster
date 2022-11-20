use std::{fs, io, path::Path, time::Instant};

const NODE_FILE_CONSTANTS: [&str; 3] = ["README.md", "FIGURE.", "DATA."];

#[derive(Debug)]
struct Entry {
    slug: String,
    entry_type: Type,
    action: Action,
}

#[derive(Debug)]
pub struct ChangeSet<'a> {
    pub file_name: &'a Path,
    pub action: Action,
}

#[derive(Debug, Clone)]
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
    let entries = detect_node_types(changes)?;
    println!("Entries: {:#?}", entries);

    // create isosec for each entry
    let t = Instant::now();
    println!("{:#?}", t);

    Ok(())
}

// isosec is
// exec date -u +%Y%m%d%H%M%S "$@"
fn create_isosec() {}

fn detect_node_types(changes: Vec<&ChangeSet>) -> io::Result<Vec<Entry>> {
    let entries: Vec<Entry> = changes
        .iter()
        .map(|ch| (ch.file_name.parent(), ch))
        .filter(|(dir, _)| dir.is_some())
        .map(|(dir, ch)| {
            let dir = dir.expect("Not able to get slug");
            let dir_name = dir.file_name().unwrap().to_str().unwrap().to_owned();
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

            let dir_entries = (dir_name, entries);
            let (dir_name, node_type) = get_type(&dir_entries).unwrap();

            Entry {
                slug: String::from(dir_name),
                entry_type: node_type,
                action: ch.action.to_owned(),
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
