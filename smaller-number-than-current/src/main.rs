use std::collections::HashMap;

pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
    let mut vec = Vec::with_capacity(nums.len());
    let mut map = HashMap::new();

    let mut nums_sorted = nums.clone();
    nums_sorted.sort_unstable();

    for (i, &x) in nums_sorted.iter().enumerate() {
        map.entry(x).or_insert(i as i32);
    }

    for &x in nums.iter() {
        vec.push(map[&x]);
    }

    vec
}

// pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
//     let mut vec = Vec::new();

//     for (i, &x) in nums.iter().enumerate() {
//         let mut counter = 0;

//         for (j, &y) in nums.iter().enumerate() {
//             if i == j {
//                 continue;
//             }

//             if y < x {
//                 counter += 1;
//             }
//         }

//         vec.push(counter);
//     }

//     vec
// }

fn main() {
    println!("{:?}", smaller_numbers_than_current(vec![8, 1, 2, 2, 3]));
}
