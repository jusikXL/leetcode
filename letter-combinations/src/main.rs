use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut result = vec![];

    let digits: Vec<u32> = digits
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.is_empty() {
        return Vec::new();
    }

    let map: HashMap<u32, Vec<char>> = HashMap::from_iter(
        vec![
            (2, vec!['a', 'b', 'c']),
            (3, vec!['d', 'e', 'f']),
            (4, vec!['g', 'h', 'i']),
            (5, vec!['j', 'k', 'l']),
            (6, vec!['m', 'n', 'o']),
            (7, vec!['p', 'q', 'r', 's']),
            (8, vec!['t', 'u', 'v']),
            (9, vec!['w', 'x', 'y', 'z'])
        ]
    );

    fn backtrack(
        i: usize,
        combination: &mut Vec<char>,
        digits: &Vec<u32>,
        map: &HashMap<u32, Vec<char>>,
        result: &mut Vec<String>
    ) {
        if i == digits.len() {
            result.push(combination.iter().collect());
            return;
        }

        for &l in map.get(&digits[i]).unwrap() {
            combination.push(l);

            backtrack(i + 1, combination, digits, map, result);

            combination.pop();
        }
    }

    backtrack(0, &mut vec![], &digits, &map, &mut result);

    result
}

pub fn letter_combinations2(digits: String) -> Vec<String> {
    let digits: Vec<u32> = digits
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.is_empty() {
        return Vec::new();
    }

    let map: HashMap<u32, Vec<char>> = HashMap::from_iter(
        vec![
            (2, vec!['a', 'b', 'c']),
            (3, vec!['d', 'e', 'f']),
            (4, vec!['g', 'h', 'i']),
            (5, vec!['j', 'k', 'l']),
            (6, vec!['m', 'n', 'o']),
            (7, vec!['p', 'q', 'r', 's']),
            (8, vec!['t', 'u', 'v']),
            (9, vec!['w', 'x', 'y', 'z'])
        ]
    );

    digits.iter().fold(vec!["".to_string()], |acc, &digit| {
        let letters = map.get(&digit).unwrap();

        acc.iter()
            .flat_map(|prefix| {
                letters.iter().map(move |&letter| format!("{}{}", prefix, letter))
            })
            .collect()
    })
}

fn main() {
    println!("{:?}", letter_combinations("23".to_string()));
}
