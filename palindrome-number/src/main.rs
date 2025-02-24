pub fn is_palindrome(x: i32) -> bool {
    let x: Vec<char> = x.to_string().chars().collect();

    let mut l = 0;
    let mut r = x.len() - 1;

    while l < r {
        if x[l] != x[r] {
            return false;
        }

        l += 1;
        r -= 1;
    }

    true
}

fn main() {
    println!("{}", is_palindrome(1111881111));
}
