use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_map: HashMap<char, usize> = HashMap::new();

    let mut l = 0;
    let mut max_len = 0;

    let chars: Vec<char> = s.chars().collect();

    for r in 0..chars.len() {
        let count = char_map.entry(chars[r]).or_insert(0);
        *count += 1;

        while *char_map.get(&chars[r]).unwrap() > 1 {
            *char_map.get_mut(&chars[l]).unwrap() -= 1;
            l += 1;
        }

        max_len = max_len.max(r - l + 1);
    }

    // for (key, value) in &char_map {
    //     println!("{key}: {value}");
    // }

    max_len as i32
}

fn main() {
    let s = String::from("abcabcbb");

    let n = length_of_longest_substring(s);

    println!("{n}");
}
