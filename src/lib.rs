pub struct KMPPattern<'s, C: 's> {
    pattern: &'s[C],
    borders: Option<Vec<usize>>,
}

impl<'s, C> KMPPattern<'s, C>
    where C: PartialEq {
    
    pub fn new(pattern: &'s[C]) -> KMPPattern<'s, C> {
        return KMPPattern{
            pattern: pattern,
            borders: None,
        };
    }
    
    pub fn linear(&self, text: &[C]) -> Option<usize> {
        linear_search(self.pattern, text)
    }
    
    pub fn kmp(&mut self, text: &[C]) -> Option<usize>
        where C: PartialEq {
        
        // Generate the prefix table using the pattern.
        let borders: &[usize] = match self.borders {
            None => {
                self.borders = Some(border_table(self.pattern));
                &self.borders.as_ref().unwrap()[..]
            },
            Some(ref b) => &b[..]
        };
      
        // Search the text using the pattern and prefix table.
        kmp_search(self.pattern, text, borders)
    }
}

pub struct BMHPattern<'s> {
    u8_kmp: KMPPattern<'s, u8>,
    
    pattern: &'s str,
    bad_char_table: Option<Vec<usize>>,
}

impl<'s> BMHPattern<'s> {
    pub fn new<'p: 's>(pattern: &'p str) -> BMHPattern<'s> {
        BMHPattern{
            u8_kmp: KMPPattern{
                pattern: pattern.as_bytes(),
                borders: None,
            },
            
            pattern: pattern,
            bad_char_table: None,
        }
    }
    
    pub fn linear(&self, text: &str) -> Option<usize> {
        self.u8_kmp.linear(text.as_bytes())
    }
    
    pub fn kmp(&mut self, text: &str) -> Option<usize> {
        self.u8_kmp.kmp(text.as_bytes())
    }
    
    pub fn bmh(&mut self, text: &str) -> Option<usize> {
        // Generate the bad character table using the pattern.
        let bad_char_table: &[usize] = match self.bad_char_table {
            None => {
                self.bad_char_table = Some(bad_character_table(self.pattern));
                &self.bad_char_table.as_ref().unwrap()[..]
            },
            Some(ref b) => &b[..]
        };
      
        // Search the text using the pattern and bad character table.
        bmh_search(self.pattern, text, bad_char_table)
    }
}

pub fn linear_search<C>(pattern: &[C], text: &[C]) -> Option<usize>
    where C: PartialEq {
    
    // For each starting point in the text:
    'text:
    for i_text in 0..text.len() {
        // For each character in the pattern:
        for i_pattern in 0..pattern.len() {
            // If there is a mismatch, try the next position.
            if text[i_text + i_pattern] != pattern[i_pattern] {
                continue 'text;
            }
            // If we reached the end of the text, the pattern isn't
            // in the text, so don't try any more positions.
            if i_text + i_pattern == text.len() {
                break 'text;
            }
        }
        // If we went through the whole pattern with no mismatch,
        // we found the first instance of the pattern.
        return Some(i_text);
    }
    return None
}

pub fn border_table<C>(pattern: &[C]) -> Vec<usize>
    where C: PartialEq {
        
    let mut borders = Vec::with_capacity(pattern.len());
  
    // TODO: why do we always start with these?
    borders.push(0);
    borders.push(0);
    
    // For each prefix `p` of length `i` in the pattern,
    // followed by the character `c`,
    // starting with the prefix of length 2:
    for (i, c) in pattern.iter().skip(1).enumerate().skip(1) {
        // Starting with longest border of p,
        // keep checking the current border's longest border until
        // (`the prefix of length b`+`c`) is a border of (`p`+`c`),
        // or there are no more borders to check.
        let mut b = borders[i];
        while pattern[b] != *c && b != 0 {
            b = borders[b]
        }
        // If an extensible border was found, extend it,
        // otherwise this prefix has no border.
        if pattern[b] == *c {
            borders.push(b + 1);
        } else {
            borders.push(0);
        }
    }
    assert_eq!(pattern.len(), borders.len());
    
    borders
}

pub fn kmp_search<C>(pattern: &[C], text: &[C], borders: &[usize]) -> Option<usize>
    where C: PartialEq {
    let mut t = 0;
    let mut p = 0;
    // While we haven't reached the last possible starting point
    // for the pattern in the text.
    while t+p < text.len() {
        // If there is a match, move forward in the pattern.
        if text[t+p] == pattern[p] {
            p += 1;
            // If we reached the end of the pattern, return 
            // the substring's starting position in the text.
            if p == pattern.len() {
                return Some(t)    
            }
            continue;
        }
       
        // There was a mismatch.
        // If we're at the beginning of the pattern, shift by one.
        // Otherwise, shift so that the previous characters form
        // the longest possible prefix of the pattern, then recheck
        // this character.
        if p == 0 {
            t += 1;
        } else {
            t += p - borders[p];
            p = borders[p];
        }
    }
    return None
}

pub fn bad_character_table(pattern: &str) -> Vec<usize> {
    // If the character doesn't appear in the pattern,
    // then we can skip ahead by the length of the whole
    // pattern if the character appears in the text.
    let mut bad_char_table =
        (0..std::u8::MAX).map(|_| -> usize { pattern.len() })
                         .collect::<Vec<_>>();
   
    // Otherwise we should skip ahead by the distance between the
    // end of the pattern and the last occurence of that character.
    for (i, c) in pattern.bytes().enumerate() {
        bad_char_table[c as usize] = pattern.len() - 1 - i;
    }
    
    bad_char_table
}

pub fn bmh_search(pattern: &str, text: &str, bad_char_table: &[usize]) -> Option<usize> {
    if pattern.len() == 0 {
        return None;
    }
    let text = text.as_bytes();
    let pattern = pattern.as_bytes();
    
    let mut t = 0;
    // While there's enough room in the text for the pattern:
    while t + pattern.len() <= text.len() {
        // Starting at the end of the pattern,
        // while the pattern matches the text,
        // move back.
        let mut p = pattern.len() - 1;
        while text[t+p] == pattern[p] {
            // If we reached the start of the pattern, return 
            // the pattern's start position in the text.
            if p == 0 {
                return Some(t)    
            }
            p -= 1;
        }
        // There was a mismatch.
        // Shift forwards in the text so that the character
        // in the text lines up with the last occurence
        // of that character in the in the pattern.
        t += bad_char_table[text[t+p] as usize];
    }
    None
}

#[cfg(test)]
mod correct_return {
    use super::KMPPattern;
    pub use super::BMHPattern;
    
    pub const CASES: [(Option<usize>, &'static str); 7] = [
        (Some(0),  "the"),
        (Some(0),  "the dog is"),
        (Some(1),  "he "),
        (Some(4),  "dog"),
        (Some(16), "dead"),
        (Some(21), "then"),
        (None,     "frank"),
    ];
    pub const TEXT: &'static str = "the dog is very dead then";
    
    #[test]
    fn linear() {
        let text = TEXT.chars().collect::<Vec<_>>();
        for &(want, pattern) in (&CASES).iter() {
            let chars = pattern.chars().collect::<Vec<_>>();
            let searcher = KMPPattern::new(&chars[..]);
            assert_eq!(searcher.linear(&text[..]), want);
        }
    }

    #[test]
    fn kmp() {
        let text = TEXT.chars().collect::<Vec<_>>();
        for &(want, pattern) in CASES.iter() {
            let chars = pattern.chars().collect::<Vec<_>>();
            let mut searcher = KMPPattern::new(&chars[..]);
            assert_eq!(searcher.kmp(&text[..]), want);
        }
    }
   
    #[cfg(test)]
    mod bmh_pattern {
        use super::BMHPattern;
        use super::CASES;
        use super::TEXT;
        
        #[test]
        fn linear() {
            for &(want, pattern) in (&CASES).iter() {
                let searcher = BMHPattern::new(pattern);
                assert_eq!(searcher.linear(TEXT), want);
            }
        }

        #[test]
        fn kmp() {
            for &(want, pattern) in CASES.iter() {
                let mut searcher = BMHPattern::new(pattern);
                assert_eq!(searcher.kmp(TEXT), want);
            }
        }

        #[test]
        fn bmh() {
            for &(want, pattern) in CASES.iter() {
                let mut searcher = BMHPattern::new(pattern);
                assert_eq!(searcher.bmh(TEXT), want);
            }
        }
    }
}
