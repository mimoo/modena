use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::lexer::Token;

static DICTIONARY: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut dictionary = HashMap::new();
    // pronouns
    dictionary.insert("jo", "me/I");
    dictionary.insert("tu", "you (sg.)");
    dictionary.insert("sa", "he/she/it");
    dictionary.insert("jo-mene", "we");
    dictionary.insert("tu-mene", "you (pl.)");
    dictionary.insert("sa-mene", "they");
    dictionary
});

pub fn parse(tokens: &[Token]) -> Result<String, &'static str> {
    let mut sentence = String::new();
    let mut sentences: Vec<String> = vec![];
    let mut tokens = tokens.iter();

    while let Some(token) = tokens.next() {
        match token {
            Token::QuestionMark => sentence.push_str("? "),
            Token::Period => {
                sentences.push(sentence.clone());
                sentence.truncate(0);
            }
            Token::Word(w) => sentence.push_str(DICTIONARY[w.as_str()]),
            Token::Number(n) => sentence.push_str(n),
            Token::Dash => todo!(),
            Token::Whitespace => sentence.push_str(" "),
        }
    }

    Ok(sentences.join(". "))
}
