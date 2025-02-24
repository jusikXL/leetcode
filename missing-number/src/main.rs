use std::collections::HashSet;

pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len();

    let range: HashSet<i32> = (0..=n).map(|x| x as i32).collect();
    let nums: HashSet<i32> = nums.into_iter().collect();

    for d in range.difference(&nums) {
        return *d;
    }

    i32::MAX
}

fn main() {
    println!("Hello, world!");
}
