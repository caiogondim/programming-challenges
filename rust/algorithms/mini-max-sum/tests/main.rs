use std::fs::File;
use std::io::prelude::*;
extern crate assert_cli;

#[test]
fn main() {
    let mut input_file = File::open("./tests/input.txt").expect("file not found");
    let mut input = String::new();

    input_file
        .read_to_string(&mut input)
        .expect("error reading file");

    let mut output_file = File::open("./tests/output.txt").expect("file not found");
    let mut output = String::new();
    output_file
        .read_to_string(&mut output)
        .expect("file not found");
    let output_ref: &str = output.as_ref();

    assert_cli::Assert::command(&["./target/debug/mini-max-sum"])
        .stdin(input)
        .stdout()
        .is(output_ref)
        .unwrap();
}
