use std::collections::HashMap;

fn areFollowingPatterns(strings: Vec<String>, patterns: Vec<String>) -> bool {
    let mut str_to_patt = HashMap::new();
    let mut patt_to_str = HashMap::new();
    
    let iter = strings.iter().zip(patterns.iter());
    
    for (s, p) in iter {
        if str_to_patt.contains_key(s) {
            if str_to_patt.get(s).unwrap() != &p { return false }
        } else {
            if patt_to_str.contains_key(p) { return false }
        }
        str_to_patt.insert(s, p);
        patt_to_str.insert(p, s);
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let strings = vec!["cat".to_string(), "dog".to_string(), "dog".to_string()];
        let patterns = vec!["a".to_string(), "b".to_string(), "b".to_string()];
        assert_eq!(areFollowingPatterns(strings, patterns), true);
    }

    #[test]
    fn test_2() {
        let strings = vec!["cat".to_string(), "dog".to_string(), "doggy".to_string()];
        let patterns = vec!["a".to_string(), "b".to_string(), "b".to_string()];
        assert_eq!(areFollowingPatterns(strings, patterns), false);
    }

    #[test]
    fn test_3() {
        let strings = vec!["cat".to_string(), "dog".to_string(), "dog".to_string()];
        let patterns = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert_eq!(areFollowingPatterns(strings, patterns), false);
    }
}
