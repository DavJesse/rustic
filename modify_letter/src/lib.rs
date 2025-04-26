pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars()
    .filter(|c| *c != letter)
    .collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.chars()
    .filter(|c| {
        let curr = c.to_ascii_lowercase();
        let targ = letter.to_ascii_lowercase();
        curr != targ
    })
    .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut result = String::new();
    let targ = letter.to_ascii_lowercase();

    for c in s.chars() {
        if c.to_ascii_lowercase() == targ {
            if c.is_lowercase() {
                result.push(c.to_ascii_uppercase());
            } else {
                result.push(c.to_ascii_lowercase());
            }
        } else {
            result.push(c);
        }
    }
    result
}