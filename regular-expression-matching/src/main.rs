// . - any single char
// * zero or more of preceding char

pub fn is_match(s: String, p: String) -> bool {
    fn helper(s: &[char], p: &[char]) -> bool {
        // Base case: if the pattern is empty, the text must also be empty
        if p.is_empty() {
            return s.is_empty();
        }

        // Check if the first character matches
        let first_match = !s.is_empty() && (s[0] == p[0] || p[0] == '.');

        // Handle '*' in the second position of the pattern
        if p.len() >= 2 && p[1] == '*' {
            // Two options: ignore '*' or use '*' to match
            return helper(s, &p[2..]) || (first_match && helper(&s[1..], p));
        } else {
            // If no '*', check first character and recurse on the rest
            return first_match && helper(&s[1..], &p[1..]);
        }
    }

    let s: Vec<char> = s.chars().collect();
    let p: Vec<char> = p.chars().collect();
    helper(&s, &p)
}

fn main() {
    let string = String::from("aa");
    let pattern = String::from("a*");

    println!("{}", is_match(string, pattern));
}
