//! Parses a series of [Token] and translates them into english.

use crate::{
    dictionary::DICTIONARY,
    errors::{ErrorKind, ModenaError, Result},
    lexer::{Token, TokenKind},
};

pub fn parse(tokens: &[Token]) -> Result<String> {
    let mut sentence: Vec<&str> = vec![];
    let mut sentences: Vec<String> = vec![];
    let mut tokens = tokens.iter();

    let mut combined_word: Vec<&str> = vec![];

    while let Some(token) = tokens.next() {
        // are we done constructing a combined word?
        if !combined_word.is_empty() {
            if !matches!(token.kind, TokenKind::Word(..) | TokenKind::Dash) {
                // pop the sentence which contains the last part of the combined word
                let last = sentence.pop().unwrap();
                combined_word.push(last);

                // figure out what the translation of the combined word is
                let w = translate_combined_word(&combined_word)?;
                sentence.push(w);

                combined_word.clear();
            }
        }

        match &token.kind {
            TokenKind::QuestionMark => sentence.push("? "),
            TokenKind::Period => {
                sentences.push(sentence.join(" "));
                sentence.truncate(0);
            }
            TokenKind::Word(w) => {
                let w = DICTIONARY
                    .get(w.as_str())
                    .ok_or(ModenaError::new(ErrorKind::UnknownWord, token.span))?;
                sentence.push(w)
            }
            TokenKind::Number(n) => sentence.push(n),
            TokenKind::Dash => {
                if combined_word.is_empty() {
                    // note: we already checked that a sentence doesn't start with a dash
                    let first = sentence.pop().unwrap();
                    combined_word.push(first);
                }
            }
            TokenKind::Whitespace => (),
        }
    }

    Ok(sentences.join(". ").trim().to_string())
}

/// Algorithm to translate a combined word
pub fn translate_combined_word(combined_word: &[&str]) -> Result<&'static str> {
    // first, let's see if the combined word already exists in the dictionary
    let joined = combined_word.join("-");
    if let Some(w) = DICTIONARY.get(k) {
        return Ok(w);
    }

    // second, let's try to see if it's a 2-combination
    if combined_word.len() == 2 {
        // negation adverb

        // success adverb
        todo!()
    }

    // if it's more than 2, then it's just a word with a bunch of adjective (WAIT, IS THIS A RULE WE WANT TO ENFORCE YET?)

    todo!()
}
