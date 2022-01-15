use crate::io::read_input;

pub fn execute() {
    let words: Vec<String> = read_input::read_str_array();

    let mut translation: Vec<String> = Vec::new();

    for word in words {
        let mut latin_word = String::new();

        let first_letter = word.chars().next();
        match first_letter {
            Some(character) => {
                // println!("First character: {}", character);
                let rest_of_word = word.strip_prefix(character);

                if let Some(letters) = rest_of_word {
                    latin_word.push_str(letters);
                }

                match character {
                    'a' | 'e' | 'i' | 'o' | 'u' => latin_word.push_str("hay"),
                    _ => {
                        latin_word.push(character);
                        latin_word.push_str("ay");
                    }
                }
            }
            None => println!("No first character for string!"),
        }
        translation.push(latin_word);
    }

    let mut translated_sentence = String::new();
    for word in translation {
        translated_sentence.push_str(&word);
        translated_sentence.push(' ');
    }

    println!("{}", translated_sentence.trim_end());
}
