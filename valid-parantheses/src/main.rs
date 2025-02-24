use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let map: HashMap<char, char> = HashMap::from_iter(vec![(')', '('), (']', '['), ('}', '{')]);
    let mut stack = Vec::new();

    for c in s.chars() {
        if let Some(&opening) = map.get(&c) {
            if stack.pop() != Some(opening) {
                return false;
            }
        } else {
            stack.push(c);
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("]".to_string()));
}
