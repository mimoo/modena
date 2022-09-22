//! The role of the lexer is to parse a string as a series of tokens.
//! It will also do simple checks like, make sure that all characters make sense (no uppercase, for example).

use crate::errors::{ErrorKind, ModenaError, Result};

/// Tokens
// TODO: replace "word" and "number" and etc. with their translation (once we know them)
#[derive(Clone, Debug)]
pub enum TokenKind {
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

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug, Clone, Copy)]
pub struct Span(pub usize, pub usize);

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

pub fn parse(input: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().enumerate();

    let mut in_word: Option<(usize, String)> = None;
    let mut in_number: Option<(usize, String)> = None;

    while let Some((offset, char)) = chars.next() {
        // if it's a character, we are parsing a word
        if matches!(char, 'a'..='z') && !matches!(char, 'g' | 'h' | 'q' | 'r' | 'x' | 'z') {
            if let Some((_, word)) = in_word.as_mut() {
                // we're either already parsing a word
                word.push_str(&char.to_string());
            } else {
                // or we're starting to parse a word
                in_word = Some((offset, char.to_string()));
            }

            continue;
        } else if matches!(char, '0'..='9') {
            if let Some((_, number)) = in_number.as_mut() {
                // we're either already parsing a word
                number.push_str(&char.to_string());
            } else {
                // or we're starting to parse a word
                in_number = Some((offset, char.to_string()));
            }

            continue;
        } else if let Some((offset, word)) = in_word.take() {
            // we were parsing a word previously
            let span = Span(offset, word.len());
            tokens.push(Token::new(TokenKind::Word(word), span));
        } else if let Some((offset, number)) = in_number.take() {
            // we were parsing a number previously
            let span = Span(offset, number.len());
            tokens.push(Token::new(TokenKind::Number(number), span));
        }

        // we're not parsing an ascii character
        match char {
            '-' => tokens.push(Token::new(TokenKind::Dash, Span(offset, 1))),
            '.' => tokens.push(Token::new(TokenKind::Period, Span(offset, 1))),
            '?' => tokens.push(Token::new(TokenKind::QuestionMark, Span(offset, 1))),
            ' ' => tokens.push(Token::new(TokenKind::Whitespace, Span(offset, 1))),
            x => {
                let span = Span(offset, 1);
                return Err(ModenaError::new(ErrorKind::UnrecognizedChar(x), span));
            }
        };
    }

    // we reached the end, but we might have been parsing a word
    // we could just check for that, but it's incorrect to not have a period or question mark at the end of a sentence in modena
    // so let's just check that
    if let Some(t) = tokens.last() {
        use TokenKind::*;
        if !matches!(t.kind, Period | QuestionMark) {
            return Err(ModenaError::new(
                ErrorKind::SentenceMustEndWithPeriod,
                t.span, // TODO: this is the wrong span
            ));
        }
    }

    Ok(tokens)
}
