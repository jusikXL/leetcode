// CONSTRAINT: 1 <= n <= 8

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut combination = String::new();

    fn helper(result: &mut Vec<String>, combination: &mut String, n: i32, open: i32, close: i32) {
        if open == n && close == n {
            result.push(combination.clone());
            return;
        }

        if open < n {
            combination.push('(');
            helper(result, combination, n, open + 1, close);
            combination.pop();
        }

        if close < open {
            combination.push(')');
            helper(result, combination, n, open, close + 1);
            combination.pop();
        }
    }

    helper(&mut result, &mut combination, n, 0, 0);

    result
}

fn main() {
    println!("{:?}", generate_parenthesis(2));
}
