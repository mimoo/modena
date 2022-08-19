//! The role of the syntax checker is to check things like "no double whitespace".
//!
//! List of things we want to check:
//! - no double whitespace
//! - space between words and numbers
//! - no space between word and ponctuation?

use crate::lexer::Token;

pub fn check(tokens: &[Token]) -> Result<(), &'static str> {
    let mut previous: Option<Token> = None;

    for token in tokens {
        match token {
            Token::QuestionMark => {
                if !matches!(previous, Some(Token::Whitespace)) {
                    return Err("question marks must be preceded by whitespace");
                }
            }
            Token::Period => {
                if !matches!(previous, Some(Token::Whitespace)) {
                    return Err("periods must be preceded by whitespace");
                }
            }
            Token::Word(_) => {
                if let Some(previous) = &previous {
                    if !matches!(previous, Token::Whitespace | Token::Dash) {
                        return Err("words must be preceded by whitespace or dashes");
                    }
                }
            }
            Token::Number(_) => {
                if let Some(previous) = &previous {
                    if !matches!(previous, Token::Whitespace) {
                        return Err("numbers must be preceded by whitespace");
                    }
                }
            }
            Token::Dash => {
                if !matches!(previous, Some(Token::Word(..))) {
                    return Err("dashes separate words");
                }
            }
            Token::Whitespace => {
                if !matches!(
                    previous,
                    Some(Token::Word(..) | Token::Number(..) | Token::QuestionMark | Token::Period)
                ) {
                    return Err(
                        "whitespace must be preceded by a word, a number, or some punctuation",
                    );
                }
            }
        }

        let previous = Some(token);
    }

    // make sure that we end with a period
    if !matches!(previous, Some(Token::Period)) {
        return Err("sentence must end with a period");
    }

    return Ok(());
}
