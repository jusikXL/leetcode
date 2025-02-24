pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut prev = 0;

    for i in 1..nums.len() {
        if nums[prev] != nums[i] {
            prev += 1;
            nums[prev] = nums[i];
        }
    }

    (prev + 1) as i32
}

fn main() {
    let mut v = vec![1, 1, 1, 4, 5];

    println!("{:?}", remove_duplicates(&mut v));
    println!("{:?}", v)
}
