pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();

    let mut closest_sum = i32::MAX;

    for i in 0..nums.len() - 2 {
        let (mut l, mut r) = (i + 1, nums.len() - 1); // init l, r

        while l < r {
            let sum = nums[i] + nums[l] + nums[r];

            if sum < target {
                l += 1;
            } else if sum > target {
                r -= 1;
            } else {
                return target;
            }

            if (target - sum).abs() < (target - closest_sum).abs() {
                closest_sum = sum;
            }
        }
    }

    closest_sum
}

fn main() {
    println!("{:?}", three_sum_closest(vec![1, 1, 1, 0], -100));
}
