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

fn get_pair_frequencies(vocab: &Vec<WordGroup>) -> HashMap<(String, String), usize> {
    let mut pairs = HashMap::new();

    for WordGroup { word, freq } in vocab.into_iter() {
        for i in 0..(word.len() - 1) {
            let entry = pairs.entry((word[i].to_string(), word[i + 1].to_string())).or_insert(0);
            *entry += freq;
        }
    }

    return pairs;
}

#[derive(Debug)]
struct WordGroup {
    pub freq: usize,
    pub word: Vec<String>,
}

fn byte_pair_encode(corpus: &HashMap<String, usize>, num_merges: usize) -> HashMap<(String, String), usize> {
    let mut vocab: Vec<WordGroup> = corpus
        .into_iter()
        .map(|(word, &freq)| WordGroup {
            freq,
            word: word.chars().map(|x| x.to_string()).collect::<Vec<String>>() 
        })
        .collect::<Vec<WordGroup>>();

    for _ in 0..num_merges {
        let pairs = get_pair_frequencies(&vocab);
        
        let (most_freq_pair, _) = match pairs.iter().max_by(|(_, a), (_, b)| a.cmp(b)) {
            None => break,
            Some(pair) => pair,
        };

        println!("MOST FREQUENT PAIR {:?}", most_freq_pair);
        
        let mut merged_vocab = vec![];
        let bigram = [&most_freq_pair.0, &most_freq_pair.1];


        for WordGroup { word, freq } in vocab {
            let mut new_word = vec![];

            let mut i = 0;

            while i < word.len() - 1 {
                let pair = [&word[i], &word[i + 1]];

                if pair[0] == bigram[0] && pair[1] == bigram[1] {
                    new_word.push(format!("{}{}", bigram[0], bigram[1]));
                    i += 1; // skip 2
                } else {
                    new_word.push(word[i].clone());
                }

                i += 1;
            }

            if word.len() > 0 && i < word.len() {
                new_word.push(word.last().unwrap().clone());
            }

            merged_vocab.push(WordGroup { freq, word: new_word });

            /*
            let new_word = word.replace(&bigram, &replacement);
            merged_vocab.insert(new_word, freq);
            */
        }

        println!("{:?}", merged_vocab);
        unimplemented!("stop for now");
        // println!("---------\n{:?}", merged_vocab);
        
        vocab = merged_vocab;
    }

    
    unimplemented!()
}

fn main() {
    let input = fs::read_to_string("./text.txt").expect("Should read text file");

    let text = preprocess_text(&input);

    let vocab = build_corpus(&text);

    println!("Vocab blyat: {:?}", byte_pair_encode(&vocab, 55));

    // println!("{:?}", byte_pair_encode(&vocab));
}
