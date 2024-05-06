use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_main_output() {
    let mut cmd = Command::cargo_bin("Cole_Ragans_Proj").unwrap(); // Use the name from your Cargo.toml
    // Adjust the file path with a raw string literal or double backslashes
    cmd.arg(r"C:\Users\aasmi\Downloads\testproj1.csv"); // Using raw string literal
    // or cmd.arg("C:\\Users\\aasmi\\Downloads\\testproj1.csv"); // Using escaped backslashes
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Graph node and edge count: 12 nodes, 4 edges"));

    println!("Test verifies that the output contains the expected node and edge count.");
}

