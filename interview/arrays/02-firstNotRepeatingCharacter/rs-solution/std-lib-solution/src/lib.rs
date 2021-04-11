use std::collections::HashMap;

fn first_non_repeating_character(s: String) -> char {
    let mut occurances = HashMap::new();

    for c in s.chars() {
        let count = occurances.entry(c).or_insert(0);
        *count += 1;
    }

    for c in s.chars() {
        if *occurances.get(&c).unwrap() == 1 {
            return c;
        }
    }

    '_'
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_1() {
        assert_eq!(first_non_repeating_character("abacabad".to_string()), 'c');
    }

    #[test]
    fn test_3() {
        assert_eq!(first_non_repeating_character("z".to_string()), 'z');
    }

    #[test]
    fn test_10() {
        assert_eq!(first_non_repeating_character("ngrhhqbhnsipkcoqjyviikvxbxyphsnjpdxkhtadltsuxbfbrkof".to_string()), 'g');
    }
}
