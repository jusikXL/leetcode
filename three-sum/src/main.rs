use std::collections::HashSet;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut triplets = HashSet::new();

    for i in 0..nums.len() {
        let (mut l, mut r) = (i + 1, nums.len() - 1); // init l, r

        while l < r {
            let (x, y, z) = (nums[i], nums[l], nums[r]);
            let sum = x + y + z;
            if sum < 0 {
                l += 1;
            } else if sum > 0 {
                r -= 1;
            } else {
                triplets.insert(vec![x, y, z]);

                l += 1;
            }
        }
    }

    triplets.into_iter().collect::<Vec<Vec<i32>>>()
}

fn main() {
    println!("{:?}", three_sum(vec![-1, 0, 1, 2, -1, -4]));
}
