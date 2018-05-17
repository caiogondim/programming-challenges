use std::io;

fn main() {
  let mut input1 = String::new();
  io::stdin()
    .read_line(&mut input1)
    .expect("failed to read from stdin");

  let mut input2 = String::new();
  io::stdin()
    .read_line(&mut input2)
    .expect("failed to read from stdin");

  let num1 : i32 = input1.trim().parse().ok().expect("parse error");
  let num2 : i32 = input2.trim().parse().ok().expect("parse error");

  println!("{}", num1 + num2);
}
