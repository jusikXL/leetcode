use std::collections::HashSet;

pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let range: HashSet<i32> = (1..=n).map(|x| x as i32).collect();
    let nums: HashSet<i32> = nums.into_iter().collect();

    let diff = range.difference(&nums);

    return diff.cloned().collect();
}

fn main() {
    println!("Hello, world!");
}
