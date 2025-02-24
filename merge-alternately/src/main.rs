pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut merged = String::with_capacity(word1.len() + word2.len());
    let mut iter1 = word1.chars();
    let mut iter2 = word2.chars();

    while let (Some(c1), Some(c2)) = (iter1.next(), iter2.next()) {
        merged.push(c1);
        merged.push(c2);
    }

    merged.extend(iter1);
    merged.extend(iter2);

    merged
}
fn main() {
    println!("{:?}", merge_alternately("ab".to_string(), "pqrs".to_string()));
}
