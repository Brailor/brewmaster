use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use brewmaster;

struct KegNode {
    isosec: String,
    slug: String,
    node_type: brewmaster::NodeType,
}

/// Test whether adding a file will be correctly added to an empty KEGNODES file
#[test]
fn should_work_correctly_with_add() {
    // Arrange
    TestEnv::new(vec![KegNode {
        isosec: "202211212039".to_string(),
        slug: "test-keg/slug-1".to_string(),
        node_type: brewmaster::NodeType::Data,
    }]);
    let ch = brewmaster::ChangeSet {
        file_name: Path::new("tests/test-keg/README.md"),
        action: brewmaster::Action::Add(),
    };
    let changeset = vec![ch];

    // Act
    let res = brewmaster::run(changeset);

    // Assert
    assert!(res.is_ok());
}

struct TestEnv {}

impl TestEnv {
    fn new(keg_nodes: Vec<KegNode>) -> Self {
        println!("[setup] trying to create KEGNODES file");
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open("tests/KEGNODES")
            .expect("Not able to create KEGNODES file");

        for node in &keg_nodes {
            file.write(format!("{} {} {}", node.isosec, node.slug, node.node_type).as_bytes())
                .expect("Not able to write to the KEGNODES file");
        }

        file.flush().expect("Not able to flush the file");
        Self {}
    }

    fn teardown(&self) {
        println!("[teardown] trying to delete KEGNODES file");
        fs::remove_file("tests/KEGNODES").expect("[teardown] Unable to delete KEGNODES file");
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        self.teardown()
    }
}
