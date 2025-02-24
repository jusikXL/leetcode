use itertools::Itertools;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut results = Vec::new();
    let mut permutations = Vec::new();

    for permutation in words.iter().permutations(words.len()).unique() {
        let concatenated: String = permutation.into_iter().map(|s| s.as_str()).collect();
        permutations.push(concatenated);
    }

    for pattern in permutations {
        for (index, _) in s.match_indices(&pattern) {
            results.push(index as i32)
        }
    }

    results
}

fn main() {
    println!("{:?}", find_substring("barfoofoobarthefoobarman".to_string(), vec!["bar".to_string(), "foo".to_string(), "the".to_string()]));
}
