use std::{collections::HashMap, fs};

fn preprocess_text(input: &str) -> String {

    let text = input.to_lowercase();

    // Remove punctuation
    let text = text.replace(['#', '$', '%', '&', '\'', '(', ')', '*', '+', '-', '.', ',', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'], "");

    // Remove digits
    let text: String = text.chars().filter(|x| ! x.is_digit(10)).collect();

    // Squash whitespace
    let text = text
        .trim()
        .split([' ', '\n', '\r', '\t'])
        .filter(|x| ! x.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    return text;
}

fn build_corpus(text: &str) -> HashMap<&str, usize> {
    let mut map: HashMap<&str, usize> = HashMap::new();
    
    let words = text.split(' ').collect::<Vec<&str>>();

    for word in words {
        let entry = map.entry(word).or_insert(0);

        *entry += 1;
    }

    return map;
}

fn main() {
    let input = fs::read_to_string("./text.txt").expect("Should read text file");

    let text = preprocess_text(&input);

    let freq_map = build_corpus(&text);

    println!("{:?}", freq_map);
}
