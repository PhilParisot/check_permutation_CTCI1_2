use std::collections::HashMap;

fn main() {
    println!("{}", check_permutation_sort("abc", "cbaa"));
    println!("{}", check_permutation_hash_map("abcc", "cbac"));
}

fn check_permutation_sort(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let mut string1: Vec<_> = string1.chars().collect();
    string1.sort_unstable();

    let mut string2: Vec<_> = string2.chars().collect();
    string2.sort_unstable();

    string1 == string2
}

fn check_permutation_hash_map(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let mut char_count = HashMap::new();

    for c in string1.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    for c in string2.chars() {
        let entry = char_count.entry(c).or_insert(0);
        *entry -= 1;
        if *entry < 0 {
            return false;
        }
    }
    true
}
