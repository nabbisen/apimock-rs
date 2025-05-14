#[cfg(test)]
mod tests;

pub fn wildcard_match(pattern: &str, text: &str) -> bool {
    let pattern_bytes = pattern.as_bytes();
    let text_bytes = text.as_bytes();

    let mut pattern_index = 0;
    let mut text_index = 0;

    let mut star_idx = None;
    let mut match_idx = 0;

    while text_index < text_bytes.len() {
        if pattern_index < pattern_bytes.len()
            && (pattern_bytes[pattern_index] == b'?'
                || pattern_bytes[pattern_index] == text_bytes[text_index])
        {
            pattern_index += 1;
            text_index += 1;
        } else if pattern_index < pattern_bytes.len() && pattern_bytes[pattern_index] == b'*' {
            star_idx = Some(pattern_index);
            match_idx = text_index;
            pattern_index += 1;
        } else if let Some(star_pos) = star_idx {
            pattern_index = star_pos + 1;
            text_index = match_idx + 1;
            match_idx = text_index;
        } else {
            return false;
        }
    }

    while pattern_index < pattern_bytes.len() && pattern_bytes[pattern_index] == b'*' {
        pattern_index += 1;
    }

    pattern_index == pattern_bytes.len()
}
