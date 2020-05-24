use itertools::join;

fn main() {
    println!("{}", translate_line("hello world"));
    let sentences = vec![
        "hello world",
        "social distancing",
        "stop the spread",
        "pikachu",
        "x marks the spot",
        "squirtle",
        "truck"
    ];
    for sentence in sentences {
        println!("{} → {}", sentence, translate_line(sentence));
    }
}

/// Translates a line of whitespace-separated words into Pig Latin.
fn translate_line(line: &str) -> String {
    join(line.split_whitespace().map(translate_word), " ")
}

/// Converts the given word to Pig Latin.
/// Requires: The first character in the string is a letter.
fn translate_word(word: &str) -> String {
    let mut word_chars = word.chars();
    if let Some(first_letter) = word_chars.next() {
        if is_vowel(first_letter) {
            format!("{}-hay", word)
        } else {
            let affix = word_chars.as_str();
            format!("{}-{}ay", affix, first_letter)
        }
    } else {
        String::new()
    }
}

fn is_vowel(letter: char) -> bool {
    let letter = letter.to_ascii_lowercase();
    letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
}

