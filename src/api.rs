use crate::{lexer, parser, syntax_checker};

pub fn parse(input: &str) -> Result<String, &'static str> {
    let tokens = lexer::parse(input)?;
    syntax_checker::check(&tokens)?;
    parser::parse(&tokens)
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
            let translated = match parse(input) {
                Ok(s) => s,
                Err(e) => panic!("error in the translation of \"{input}\": {e}"),
            };
            if translated != expected {
                panic!("the translation of \"{input}\" was expected to be \"{expected}\" but was \"{translated}\"");
            }
        }
    }
}
