use std::cmp::min;
use std::collections::HashMap;

// given two strings, how many characters do we need to remove (from either) to make them anagrams?
// example: hello, billion -> ANSWER: 6 ("he" from "hello" and "biin" from billion)

fn get_dif(s1: &str, s2: &str) -> i32 {
    let char_hash1 = get_char_count(&s1);
    let char_hash2 = get_char_count(&s2);

    get_delta_count(char_hash1, char_hash2)
}

fn get_char_count(s: &str) -> HashMap<char, i32> {
    let mut char_hash: HashMap<char, i32> = HashMap::new();
    for char in s.chars() {
        if let Some(entry) = char_hash.get_mut(&char) {
            *entry += 1
        } else {
            char_hash.insert(char, 1);
        }
    }

    char_hash
}

fn get_delta_count(char_hash1: HashMap<char, i32>, char_hash2: HashMap<char, i32>) -> i32 {
    let mut delta: i32 = 0;

    for (key, hash1_val) in &char_hash1 {
        if let Some(hash2_val) = char_hash2.get(&key) {
            delta += hash1_val - hash2_val;
        } else {
            delta += hash1_val;
        }
    }

    for (key, hash2_val) in char_hash2 {
        if let None = char_hash1.get(&key) {
            delta += hash2_val
        }
    }
    delta
}

fn main() {
    let s1 = "abc";
    let s2 = "ajk";

    println!("{}", get_dif(s1, s2));
}
