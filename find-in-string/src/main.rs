// pub fn str_str(haystack: String, needle: String) -> i32 {
//     match haystack.find(&needle) {
//         Some(index) => index as i32,
//         None => -1,
//     }
// }

pub fn str_str(haystack: String, needle: String) -> i32 {
    let (mut i, mut j) = (0, needle.len());

    while j <= haystack.len() {
        let chars: Vec<char> = haystack.chars().collect();

        if chars[i..j].iter().collect::<String>() == needle {
            return i as i32;
        }

        i += 1;
        j += 1;
    }

    -1
}

fn main() {
    println!("Hello, world!");
}
