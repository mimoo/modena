use std::collections::HashMap;

use once_cell::sync::Lazy;

const DICTIONARY_STR: &str = r#"
# pronouns
jo: me/I
tu: you (sg.)
sa: he/she/it
jo-mene: we
tu-mene: you (pl.)
sa-mene: they

# verb / nouns
yu: to type
kana: to look
kana-done: to find

# adverb
done: success
nu: negation
"#;

pub static DICTIONARY: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut dictionary = HashMap::new();

    for line in DICTIONARY_STR.lines() {
        let line = line.trim();

        // ignore comments or empty lines
        if line.starts_with('#') || line.is_empty() {
            continue;
        }

        // parse the line
        let parsed: Vec<_> = line.split(':').collect();
        let modena = parsed[0].trim();
        let english = parsed[1].trim();

        dictionary.insert(modena, english);
    }

    // pronouns
    /*
    dictionary.insert("jo", "me/I");
    dictionary.insert("tu", "you (sg.)");
    dictionary.insert("sa", "he/she/it");
    dictionary.insert("jo-mene", "we");
    dictionary.insert("tu-mene", "you (pl.)");
    dictionary.insert("sa-mene", "they");
    */

    dictionary
});
