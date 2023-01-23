fn main() {
    println!("{}", capitalize("hello world!"));
}

fn capitalize(source: &str) -> String {
    source
        .split(char::is_whitespace)
        .map(|word| {
            if let Some(first_char_boundary) = word.char_indices().nth(1) {
                let (first, rest) = word.split_at(first_char_boundary.0);
                first.to_uppercase() + rest
            } else {
                word.to_uppercase()
            }
        })
        .fold(String::new(), |mut string, word| {
            string.push_str(" ");
            string.push_str(&word);
            string
        })
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::capitalize;

    /// Macro for creating multiple tests for capitalization of different phrases
    macro_rules! test_capitalize_words {
        ($($name:ident: $phrase:expr, $expected:expr;)*) => {
            $(
                #[test]
                fn $name() {
                    // when the phrase is capitalized
                    let capitalized = capitalize($phrase);

                    // Then expect it to equal the expected phrase
                    assert_eq !(capitalized, $expected);
                }
            )*
        }
    }

    test_capitalize_words! {
        empty_string: "", "";
        single_character: "a", "A";
        single_ascii_word: "hello", "Hello";
        single_unicode_word: "èlo", "Èlo";
        already_capitalized_word: "Hello", "Hello";
        ascii_phrase: "hello world!", "Hello World!";
        unicode_phrase: "world èlo", "World Èlo";
    }
}
