fn is_palindrome(s: &String) -> bool {
    let chars: Vec<char> = s.chars().collect();

    let mut l = 0;
    let mut r = chars.len() - 1;

    while l < r {
        if chars[l] != chars[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

// fn substrings(s: &str) -> Vec<String> {
//     let chars: Vec<char> = s.chars().collect();
//     let n = chars.len();

//     let mut substrings: Vec<String> = Vec::new();

//     for i in 0..n {
//         for j in i + 1..=n {
//             substrings.push(chars[i..j].iter().collect());
//         }
//     }

//     substrings
// }

pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();

    let mut len = s.len();

    while len > 0 {
        let mut start = 0;

        while start <= s.len() - len {
            let substr = chars[start..start + len].iter().collect();

            if is_palindrome(&substr) {
                return substr;
            }

            start += 1;
        }

        len -= 1;
    }

    String::new()
}

fn main() {
    println!("{}", longest_palindrome(String::from("babad")));
}
