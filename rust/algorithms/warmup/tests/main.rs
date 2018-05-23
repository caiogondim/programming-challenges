extern crate assert_cli;

#[test]
fn main() {
  assert_cli::Assert::command(&["./target/debug/warmup"])
    .stdin("2\n3")
    .stdout()
    .is("5")
    .unwrap();
}
