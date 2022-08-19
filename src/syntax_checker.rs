//! The role of the syntax checker is to check things like "no double whitespace".
//!
//! List of things we want to check:
//! - no double whitespace
//! - space between words and numbers
//! - no space between word and ponctuation?

use crate::lexer::{Token, TokenKind};

pub fn check(tokens: &[Token]) -> Result<(), &'static str> {
    let mut previous: Option<TokenKind> = None;

    for token in tokens {
        match token.kind {
            TokenKind::QuestionMark => {
                if !matches!(previous, Some(TokenKind::Whitespace)) {
                    return Err("question marks must be preceded by whitespace");
                }
            }
            TokenKind::Period => {
                if !matches!(previous, Some(TokenKind::Whitespace)) {
                    return Err("periods must be preceded by whitespace");
                }
            }
            TokenKind::Word(_) => {
                if let Some(previous) = &previous {
                    if !matches!(previous, TokenKind::Whitespace | TokenKind::Dash) {
                        return Err("words must be preceded by whitespace or dashes");
                    }
                }
            }
            TokenKind::Number(_) => {
                if let Some(previous) = &previous {
                    if !matches!(previous, TokenKind::Whitespace) {
                        return Err("numbers must be preceded by whitespace");
                    }
                }
            }
            TokenKind::Dash => {
                if !matches!(previous, Some(TokenKind::Word(..))) {
                    return Err("dashes separate words");
                }
            }
            TokenKind::Whitespace => {
                if !matches!(
                    previous,
                    Some(
                        TokenKind::Word(..)
                            | TokenKind::Number(..)
                            | TokenKind::QuestionMark
                            | TokenKind::Period
                    )
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
    if !matches!(previous, Some(TokenKind::Period)) {
        return Err("sentence must end with a period");
    }

    return Ok(());
}
