use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

use crate::lexer::Span;

pub type Result<T> = std::result::Result<T, ModenaError>;

impl Into<SourceSpan> for &Span {
    fn into(self) -> SourceSpan {
        SourceSpan::new(self.0.into(), self.1.into())
    }
}

impl Into<SourceSpan> for Span {
    fn into(self) -> SourceSpan {
        (&self).into()
    }
}

#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(
    code(oops::my::bad),
    url(docsrs),
    help("try doing it better next time?")
)]
pub struct ModenaError {
    kind: ErrorKind,
    #[label("This bit here")]
    span: Span,
}

impl ModenaError {
    pub fn new(kind: ErrorKind, span: Span) -> Self {
        ModenaError { kind, span }
    }
}

#[derive(Error, Debug, Diagnostic)]
pub enum ErrorKind {
    #[error("character {0} not recognized")]
    UnrecognizedChar(char),
    #[error("question marks must be preceded by whitespace")]
    QuestionMarkNotPrecededByWhitespace,
    #[error("periods must be preceded by whitespace")]
    PeriodNotPrecededByWhitespace,
    #[error("words must be preceded by whitespace or dashes")]
    WordNotPrecededByWhitespaceOrDash,
    #[error("numbers must be preceded by whitespace")]
    NumberNotPrecededByWhitespace,
    #[error("dashes must be used to separate words")]
    DashNotUsedToSeparateWords,
    #[error("whitespace must be preceded by a word, a number, or some punctuation")]
    WhitespaceNotPrecededByWordNumberOrPunctuation,
    #[error("sentence must end with a period")]
    SentenceMustEndWithPeriod,
}
