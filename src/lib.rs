pub fn linear_search(pattern: &str, text: &str) -> Option<usize> {
    // For each starting point where the pattern could fit:
    let t_chars: Vec<char> = text.chars().collect();
    let p_chars: Vec<char> = pattern.chars().collect();
    let mut i_text = 0;
    // For each starting point in the text:
    'text: while i_text < t_chars.len() {
        // For each character in the pattern:
        let mut i_pattern = 0;
        while i_pattern < p_chars.len() {
            // If there is a mismatch, try the next starting point.
            if t_chars[i_text + i_pattern] != p_chars[i_pattern] {
                i_text += 1;
                continue 'text;
            }
            // If we reached the end of the text, the pattern isn't
            // in the text, so fail now.
            if i_text + i_pattern == t_chars.len() {
                break 'text;
            }
            i_pattern += 1;
        }
        // If we went through the whole pattern with no mismatch,
        // we found the first instance of the pattern.
        return Some(i_text);
    }
    return None
}

pub fn kmp_search(pattern: &str, text: &str) -> Option<usize> {
    return None
}

pub fn bmh_search(pattern: &str, text: &str) -> Option<usize> {
    return None
}

#[cfg(test)]
mod correct_index_test {
    use super::linear_search;
    use super::kmp_search;
    use super::bmh_search;
    
    const CASES: [(Option<usize>, &'static str); 7] = [
        (Some(0), "the"),
        (Some(0), "the dog is"),
        (Some(1), "he "),
        (Some(4), "dog"),
        (Some(16), "dead"),
        (Some(21), "then"),
        (None,    "frank"),
    ];
    const TEXT: &'static str = "the dog is very dead then";
    
    #[test]
    fn linear_correct_index() {
        for &(want, pattern) in (&CASES).iter() {
            assert_eq!(linear_search(pattern, TEXT), want)
        }
    }

    #[test]
    fn kmp_correct_index() {
        for &(want, pattern) in CASES.iter() {
            assert_eq!(kmp_search(pattern, TEXT), want)
        }
    }

    #[test]
    fn bmh_correct_index() {
        for &(want, pattern) in CASES.iter() {
            assert_eq!(bmh_search(pattern, TEXT), want)
        }
    }
}
