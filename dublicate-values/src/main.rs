use std::collections::HashSet;

pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
    nums.sort();

    for i in 0..nums.len() - 1 {
        if nums[i] == nums[i + 1] {
            return true;
        }
    }

    false
}

fn main() {
    contains_duplicate(vec![1, 2, 3, 4]);
}
