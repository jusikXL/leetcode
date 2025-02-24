use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            if let Some(&complement_index) = map.get(&(target - num)) {
                return vec![complement_index as i32, i as i32];
            }

            map.insert(num, i);
        }

        vec![]
    }
}

// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut map = HashMap::new();

//     for (i, num) in nums.iter().enumerate() {
//         if let Some(&complement_index) = map.get(&(target - num)) {
//             return vec![complement_index as i32, i as i32];
//         }

//         map.insert(num, i);
//     }

//     vec![]
// }

pub fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result)
}
