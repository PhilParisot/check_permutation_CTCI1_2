fn main() {
    println!("{}", check_permutation_sort("abc", "cbaa"));
}

fn check_permutation_sort(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }

    let mut string1: Vec<_> = string1.chars().collect();
    string1.sort();

    let mut string2: Vec<_> = string2.chars().collect();
    string2.sort();

    string1 == string2
}
