//! The role of the lexer is to parse a string as a series of tokens.
//! It will also do simple checks like, make sure that all characters make sense (no uppercase, for example).

/// Tokens
// TODO: replace "word" and "number" and etc. with their translation (once we know them)
pub enum Token {
    /// Question marks are good because you clearly see that the question is a sentence.
    QuestionMark, // ?
    /// We need something to separate sentences. Period works well
    Period, // .
    /// A word
    Word(String),
    /// A number is a series of 0..9 digits
    Number(String),
    /// A connecting dash
    Dash,
    /// We save whitespace because we want to stricly check whitespaces.
    Whitespace,
    // TODO: what about + ,
}

pub fn parse(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    let mut chars = input.chars();

    let mut in_word: Option<String> = None;
    let mut in_number: Option<String> = None;

    while let Some(char) = chars.next() {
        // if it's a character, we are parsing a word
        if matches!(char, 'a'..='z') && !matches!(char, 'f' | 'g' | 'h' | 'q' | 'r' | 'x' | 'z') {
            if let Some(mut word) = in_word.as_mut() {
                // we're either already parsing a word
                word.push_str(&char.to_string());
            } else {
                // or we're starting to parse a word
                in_word = Some(char.to_string());
            }

            continue;
        } else if matches!(char, '0'..='9') {
            if let Some(mut number) = in_number.as_mut() {
                // we're either already parsing a word
                number.push_str(&char.to_string());
            } else {
                // or we're starting to parse a word
                in_number = Some(char.to_string());
            }

            continue;
        } else if let Some(word) = in_word.take() {
            // we were parsing a word previously
            tokens.push(Token::Word(word));
        } else if let Some(number) = in_number.take() {
            // we were parsing a number previously
            tokens.push(Token::Number(number));
        }

        // we're not parsing an ascii character
        match char {
            '-' => tokens.push(Token::Dash),
            '.' => tokens.push(Token::Period),
            '?' => tokens.push(Token::QuestionMark),
            ' ' => tokens.push(Token::Whitespace),
            _ => return Err("character not recognized"),
        };
    }
    Ok(tokens)
}
