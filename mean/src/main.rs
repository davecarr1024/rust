use std::io;

fn main() {
    println!("input nums");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .map(|n| n.parse().expect("invalid input"))
        .collect();
    nums.sort();
    let mean = (nums.iter().sum::<i32>() as f64) / (nums.len() as f64);
    let median = nums.get(nums.len() / 2).unwrap();
    println!("nums {nums:?} mean {mean:?} median {median:?}");
}
