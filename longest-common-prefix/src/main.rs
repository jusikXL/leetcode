pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = String::new();
    let mut chars = strs[0].chars();

    while let Some(c) = chars.next() {
        let candidate_prefix = format!("{}{}", prefix, c);

        if strs.iter().all(|s| s.starts_with(&candidate_prefix)) {
            prefix = candidate_prefix;
        } else {
            break;
        }
    }

    prefix
}

// pub fn longest_common_prefix(strs: Vec<String>) -> String {
//     let mut prefix = strs[0].clone();

//     for s in strs {
//         while !s.starts_with(&prefix) {
//             prefix.pop();
//         }
//     }

//     prefix
// }

fn main() {
    println!("Hello, world!");
}
