use std::{
    env,
    io::{self, Read},
};

use brewmaster::{run, ChangeSet};

fn main() {
    let mut arguments = env::args();

    for (i, arg) in env::args().enumerate() {
        println!("arg #{i} = {arg}");
    }

    // Parse the incoming changeset into a format we know ie. the Changeset type

    if let Some(changeset_raw) = arguments.nth(1) {
        match parse_args(changeset_raw) {
            Some(changeset) => run(changeset),
            None => todo!(),
        }
    } else {
        parse_stdin();
    }
}

fn parse_args(changeset_raw: String) -> Option<Vec<ChangeSet>> {
    Some(vec![])
}

fn parse_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("From stdin: ");
    for line in buffer.lines() {
        println!("\t{line}");
    }

    Ok(buffer)
}
