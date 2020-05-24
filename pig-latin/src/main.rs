fn main() {
    println!("{}, {}", pig_latin("hello"), pig_latin("world"));
    let words_to_try = vec!["food", "social", "distancing", "pikachu", "spot", "squirtle"];
    for word in words_to_try {
        println!("{} → {}", word, pig_latin(word));
    }
}

/// Converts the given word to pig Latin.
/// Requires: The first character in the string is a letter.
fn pig_latin(word: &str) -> String {
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

