use crossterm::tty::IsTty;
use std::{
    env,
    io::{self, Error, Read},
    path::Path,
};

use brewmaster::{run, Action, ChangeSet};

fn main() -> io::Result<()> {
    let mut arguments = env::args();

    for (i, arg) in env::args().enumerate() {
        println!("arg #{i} = {arg}");
    }

    // Parse the incoming changeset into a format we know ie. the Changeset type
    if let Some(changeset_raw) = arguments.nth(1) {
        match parse_args(changeset_raw.as_str()) {
            Some(changeset) => run(changeset)?,
            None => {
                bad_input();
                let error = Error::new(io::ErrorKind::Other, "Bad input");
                return Err(error);
            }
        }
    } else {
        let is_tty = io::stdin().is_tty();
        if is_tty {
            println!("Should print usage");
        } else {
            let from_stdin = parse_stdin()?;
            match parse_args(from_stdin.as_str()) {
                Some(changeset) => run(changeset)?,
                None => todo!(),
            }
        }
    }

    Ok(())
}

fn bad_input() {
    println!("Bad input was given to brewmaster!\nExpected input format for this command is:\n<action> <filename>")
}

fn parse_args<'a>(changeset_raw: &'a str) -> Option<Vec<ChangeSet<'a>>> {
    let res: Vec<ChangeSet> = changeset_raw
        .lines()
        .filter(|line| {
            if line.split(" ").count() != 2 {
                return false;
            }
            let action = line.split(" ").nth(0);
            let file_name = line.split(" ").nth(1);

            if action.is_none() || file_name.is_none() {
                return false;
            }

            true
        })
        .map(|line| {
            let action = line.split(" ").nth(0).unwrap();
            let path = line.split(" ").nth(1).unwrap();

            let action = match action {
                "??" => Action::Untrack(),
                "A" => Action::Add(),
                "D" => Action::Delete(),
                "R" => Action::Rename(),
                "M" => Action::Modify(),
                _ => panic!("Unknown action!"),
            };

            ChangeSet {
                file_name: Path::new(path),
                action,
            }
        })
        .collect();

    Some(res)
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
