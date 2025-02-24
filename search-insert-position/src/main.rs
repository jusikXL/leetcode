pub fn insert_in_order(nums: Vec<i32>, target: i32) -> i32 {
    for (i, &x) in nums.iter().enumerate() {
        if target > x {
            continue;
        }

        // if we get into else here it means that
        // target <= x - the place we were looking for
        return i as i32;
    }

    // if we get out of for loop here, then we always had been continuing
    nums.len() as i32
}

fn main() {
    println!("{:?}", search_insert(vec![1, 3, 5, 6], 2));
}
