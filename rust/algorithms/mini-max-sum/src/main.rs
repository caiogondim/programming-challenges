use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut numbers_asc_order = numbers.clone();
    numbers_asc_order.sort();
    let max_sum: i64 = numbers_asc_order.iter().take(4).sum();

    let mut numbers_desc_order = numbers.clone();
    numbers_desc_order.sort_by(|a, b| b.cmp(a));
    let mini_sum: i64 = numbers_desc_order.iter().take(4).sum();

    print!("{} {}", max_sum, mini_sum);
}
