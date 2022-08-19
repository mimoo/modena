//! The role of the syntax checker is to check things like "no double whitespace".
//!
//! List of things we want to check:
//! - no double whitespace
//! - space between words and numbers
//! - no space between word and ponctuation?

use crate::{
    errors::{ErrorKind, ModenaError, Result},
    lexer::{Span, Token, TokenKind},
};

pub fn check(tokens: &[Token]) -> Result<()> {
    let mut previous: Option<TokenKind> = None;

    for token in tokens {
        match token.kind {
            TokenKind::QuestionMark => {
                if !matches!(previous, Some(TokenKind::Whitespace)) {
                    return Err(ModenaError::new(
                        ErrorKind::QuestionMarkNotPrecededByWhitespace,
                        token.span,
                    ));
                }
            }
            TokenKind::Period => {
                if !matches!(previous, Some(TokenKind::Whitespace)) {
                    return Err(ModenaError::new(
                        ErrorKind::PeriodNotPrecededByWhitespace,
                        token.span,
                    ));
                }
            }
            TokenKind::Word(_) => {
                if let Some(previous) = &previous {
                    if !matches!(previous, TokenKind::Whitespace | TokenKind::Dash) {
                        return Err(ModenaError::new(
                            ErrorKind::WordNotPrecededByWhitespaceOrDash,
                            token.span,
                        ));
                    }
                }
            }
            TokenKind::Number(_) => {
                if let Some(previous) = &previous {
                    if !matches!(previous, TokenKind::Whitespace) {
                        return Err(ModenaError::new(
                            ErrorKind::NumberNotPrecededByWhitespace,
                            token.span,
                        ));
                    }
                }
            }
            TokenKind::Dash => {
                if !matches!(previous, Some(TokenKind::Word(..))) {
                    return Err(ModenaError::new(
                        ErrorKind::DashNotUsedToSeparateWords,
                        token.span,
                    ));
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
                    return Err(ModenaError::new(
                        ErrorKind::WhitespaceNotPrecededByWordNumberOrPunctuation,
                        token.span,
                    ));
                }
            }
        }

        previous = Some(token.kind.clone());
    }

    // make sure that we end with a period
    if !matches!(previous, Some(TokenKind::Period)) {
        return Err(ModenaError::new(
            ErrorKind::SentenceMustEndWithPeriod,
            Span(usize::MAX, 0),
        ));
    }

    return Ok(());
}
