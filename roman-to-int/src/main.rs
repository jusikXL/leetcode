use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let mut num = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let val = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        let val = match chars.peek() {
            Some('V') | Some('X') if c == 'I' => -val,
            Some('L') | Some('C') if c == 'X' => -val,
            Some('D') | Some('M') if c == 'C' => -val,
            _ => val,
        };

        num += val;
    }

    num
}

// pub fn roman_to_int(s: String) -> i32 {
//     let chars = s.chars().peekable();

//     let map: HashMap<char, i32> = HashMap::from([
//         ('I', 1),
//         ('V', 5),
//         ('X', 10),
//         ('L', 50),
//         ('C', 100),
//         ('D', 500),
//         ('M', 1000),
//     ]);

//     let s = s.chars();

//     let mut num = 0;

//     let mut prev: char = 'J';

//     for c in s {
//         num += map.get(&c).unwrap();

//         if prev == 'I' {
//             if c == 'V' || c == 'X' {
//                 num -= 2;
//             }
//         } else if prev == 'X' {
//             if c == 'L' || c == 'C' {
//                 num -= 20;
//             }
//         } else if prev == 'C' {
//             if c == 'D' || c == 'M' {
//                 num -= 200;
//             }
//         }

//         prev = c;
//     }

//     num
// }

fn main() {
    println!("{}", roman_to_int(String::from("MCMXCIV")));
}
