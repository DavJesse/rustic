use std::collections::HashMap;

fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    
    for word in words.split_whitespace() {
        // remove non-alphanumeric characters from start and end of word
        let valid = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !valid.is_empty() {
            *result.entry(valid.to_lowercase()).or_insert(0)+=1;
        }
    }

    result
}