use miette::IntoDiagnostic;

use crate::{lexer, parser, syntax_checker};

fn parse_inner(input: &str) -> miette::Result<String> {
    let tokens = lexer::parse(input)?;
    syntax_checker::check(&tokens)?;
    parser::parse(&tokens).into_diagnostic()
}

pub fn parse(input: &str) -> miette::Result<String> {
    parse_inner(input).map_err(|e| e.with_source_code(input.to_string()))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn sentences() -> HashMap<&'static str, &'static str> {
        let mut map = HashMap::new();
        map.insert("jo pupi tu", "I love you.");
        map.insert(
            "jo nu-ama asse potato-fine",
            "I don't have enough french fries.",
        );
        map.insert("jo yu-la mite ban-dessi", "I'm typing on a keyboard.");
        map.insert(
            "tu kana-done ko kana-la ok ma?",
            "Did you find what you were looking for?",
        );
        map
    }

    #[test]
    fn test_sentences() {
        for (input, expected) in sentences() {
            let translated = parse(input).unwrap();
            if translated != expected {
                panic!("the translation of \"{input}\" was expected to be \"{expected}\" but was \"{translated}\"");
            }
        }
    }
}
