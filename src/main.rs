fn main() {
    // Challenge: Write a function that takes a &str sentence and returns a
    // HashMap<String, usize> of word frequencies (case-insensitive). In Python
    // this is Counter(s.lower().split()). Translate it to Rust.

    // My Solution:
    use std::collections::HashMap;

    let strings_sentence = "Hello world! Hello Rust. Hello everyone.";

    fn word_freq(sentence: &str) -> HashMap<String, usize> {
        let mut hash: HashMap<String, usize> = HashMap::new();

        for word in sentence.split_whitespace() {
            let lower = word.to_lowercase();
            if !hash.contains_key(&lower) {
                hash.insert(lower, 1);
            } else {
                *hash.get_mut(&lower).unwrap() += 1;
            }
        }
        hash
    }

    // Option 2:
    // use std::collections::HashMap;

    // fn word_frequencies(s: &str) -> HashMap<String, usize> {
    //     let mut map = HashMap::new();

    //     for word in s.split_whitespace() {
    //         let word = word.to_lowercase();

    //         map.entry(word).and_modify(|count| *count += 1).or_insert(1);
    //     }

    //     map
    // }

    // Option 3:
    // let text = "the quick brown fox jumps over the lazy fox";
    // fn word_frequencies(text: &str) -> HashMap<String, usize> {
    //     let mut counts = HashMap::new();
    //     for word in text.split_whitespace() {
    //         let key = word.to_lowercase();
    //         *counts.entry(key).or_insert(0) += 1;
    //     }
    //     counts
    // }
    // let freq = word_frequencies(strings_sentence);
    // for (word, count) in &freq {
    //     println!("{}: {}", word, count);
    // }

    println!("{:?}", word_freq(strings_sentence));
}
