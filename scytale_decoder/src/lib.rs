pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let mut result = Vec::new();
    let mut turn = Vec::new();
    let mut decoded = String::new();
    let size = letters_per_turn as usize;

    for c in s.chars() {
        turn.push(c);
        if turn.len() == size {
            result.push(turn.clone());
            turn.clear();
        }
    }

    if turn.len() > 0 {
        result.push(turn);
    }

    for i in 0..size {
        for turn in result.iter() {
            if i < turn.len() {
                decoded.push(turn[i]);
            }
        }
    }
    Some(decoded)
}