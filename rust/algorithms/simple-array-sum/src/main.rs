use std::io;

fn main() {
  let mut input1 = String::new();
  io::stdin()
    .read_line(&mut input1)
    .expect("failed to read line from stdin");

  let mut input2 = String::new();
  io::stdin()
    .read_line(&mut input2)
    .expect("failed to read line from stdin");
  let iter = input2.split_whitespace().map(|x| x.parse::<i64>().expect("unable to parse"));
  let sum: i64 = iter.sum();

  println!(" {} ", sum);
}
