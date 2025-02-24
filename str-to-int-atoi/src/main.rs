use std::num::IntErrorKind;

pub fn my_atoi(s: String) -> i32 {
    let mut chars = s.trim().chars(); // trim and convert to chars

    let (is_sign_present, is_neg) = match chars.clone().next() {
        Some(c) if c == '-' => (true, true),
        Some(c) if c == '+' => (true, false),
        _ => (false, false),
    };

    if is_sign_present {
        chars.next(); // skip the sign
    }

    let str: String = chars.take_while(|c| c.is_digit(10)).collect();

    let num = str.parse::<i32>();

    match num {
        Ok(n) => {
            if is_neg { -n } else { n }
        }
        Err(e) => {
            match e.kind() {
                IntErrorKind::PosOverflow => if is_neg { i32::MIN } else { i32::MAX }
                _ => 0,
            }
        }
    }
}

fn main() {
    println!("{}", my_atoi(String::from("-91283472332")));
}
