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

fn build_corpus(text: &str) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();
    
    let words = text.split(' ').collect::<Vec<&str>>();

    for word in words {
        let entry = map.entry(format!("{}_", word)).or_insert(0);

        *entry += 1;
    }

    return map;
}

fn get_pairs(vocab: &HashMap<String, usize>) -> HashMap<(char, char), usize> {
    let mut pairs: HashMap<(char, char), usize> = HashMap::new();

    for (word, freq) in vocab.into_iter() {
        let chars = word.chars().collect::<Vec<char>>();
        for i in 0..(word.len() - 1) {
            let entry = pairs.entry((chars[i], chars[i + 1])).or_insert(0);
            *entry += freq;
        }
    }

    return pairs;
}

// fn get

fn byte_pair_encode(vocab: &HashMap<String, usize>) -> HashMap<(char, char), usize> {
}

fn main() {
    let input = fs::read_to_string("./text.txt").expect("Should read text file");

    let text = preprocess_text(&input);

    let vocab = build_corpus(&text);

    println!("{:?}", byte_pair_encode(&vocab));
}
