struct Searcher<'s, C: 's> {
    pattern: &'s[C],
    borders: Option<Vec<usize>>,
    bmhTable: Option<Vec<usize>>,
}

impl<'s, C> Searcher<'s, C>
    where C: PartialEq {
        
    pub fn new<'t: 's>(pattern: &'t[C]) -> Searcher<'s, C> {
        return Searcher{
            pattern: pattern,
            borders: None,
            bmhTable: None,
        };
    }
    
    pub fn linear(&self, text: &[C]) -> Option<usize> {
        linear_search(self.pattern, text)
    }
    
    pub fn kmp<'t: 's>(&'s mut self, text: &'t[C]) -> Option<usize> {
        // Generate the prefix table using the pattern.
        let borders: &[usize] = match self.borders {
            None => {
                self.borders = Some(border_table(self.pattern));
                self.borders.as_ref().unwrap().as_slice()
            },
            Some(ref b) => b.as_slice()
        };
      
        // Search the text using the pattern and prefix table.
        kmp_search(self.pattern, text, borders)
    }
    
    pub fn bmh(&mut self, text: &[C]) -> Option<usize> {
        // Generate the shift table using the pattern.
        
        // Search the text using the pattern and shift table.
        None
    }
}

pub fn linear_search<C>(pattern: &[C], text: &[C]) -> Option<usize>
    where C: PartialEq {
    
    // For each starting point in the text:
    'text:
    for i_text in 0..text.len() {
        // For each character in the pattern:
        let mut i_pattern = 0;
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
        
    // For each starting point in the text:
    let mut t = 0;
    let mut p = 0;
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

#[cfg(test)]
mod correct_index_test {
    use super::Searcher;
    
    const CASES: [(Option<usize>, &'static str); 7] = [
        (Some(0),  "the"),
        (Some(0),  "the dog is"),
        (Some(1),  "he "),
        (Some(4),  "dog"),
        (Some(16), "dead"),
        (Some(21), "then"),
        (None,     "frank"),
    ];
    const TEXT: &'static str = "the dog is very dead then";
    
    #[test]
    fn linear_correct_index() {
        let text = TEXT.chars().collect::<Vec<_>>();
        for &(want, pattern) in (&CASES).iter() {
            let chars = pattern.chars().collect::<Vec<_>>();
            let searcher = Searcher::new(chars.as_slice());
            assert_eq!(searcher.linear(text.as_slice()), want);
        }
    }

    #[test]
    fn kmp_correct_index() {
        let text = TEXT.chars().collect::<Vec<_>>();
        for &(want, pattern) in CASES.iter() {
            let chars = pattern.chars().collect::<Vec<_>>();
            let mut searcher = Searcher::new(chars.as_slice());
            assert_eq!(searcher.kmp(text.as_slice()), want);
        }
    }

    #[test]
    fn bmh_correct_index() {
        let text = TEXT.chars().collect::<Vec<_>>();
        for &(want, pattern) in CASES.iter() {
            let chars = pattern.chars().collect::<Vec<_>>();
            let mut searcher = Searcher::new(chars.as_slice());
            assert_eq!(searcher.bmh(text.as_slice()), want);
        }
    }
}
