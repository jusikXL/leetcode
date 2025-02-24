use std::cmp::{ max, min };

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area = 0;

    let mut iter = height.iter().enumerate();

    let (mut l, mut r) = (iter.next(), iter.next_back());

    while let (Some((i, h1)), Some((j, h2))) = (l, r) {
        let area = ((j - i) as i32) * min(h1, h2);
        max_area = max(max_area, area);

        if h1 < h2 {
            l = iter.next(); // move left + 1
        } else {
            r = iter.next_back(); // move right - 1
        }
    }

    max_area
}

// pub fn max_area(height: Vec<i32>) -> i32 {
//     let mut max_area = 0;

//     let (mut l, mut r) = (0, height.len() - 1);

//     while l < r {
//         let area = ((r - l) as i32) * min(height[l], height[r]);
//         max_area = max(max_area, area);

//         println!("h[{}] * h[{}] = {} * {} -> {}", l, r, height[l], height[r], area);

//         if height[l] < height[r] {
//             l += 1;
//         } else {
//             r -= 1;
//         }
//     }

//     max_area
// }

fn main() {
    println!("{}", max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
}
