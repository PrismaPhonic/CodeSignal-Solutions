extern crate linked_hash_set;
use std::collections::HashSet;
use linked_hash_set::LinkedHashSet;

fn firstNotRepeatingCharacter(s: String) -> char {
  let mut occurances: HashSet<char> = HashSet::with_capacity(24);
  let mut uniq = LinkedHashSet::new();
 
  s.chars().for_each(|c| {
    if occurances.insert(c) == true {
      uniq.insert(c);
    } else {
      uniq.remove(&c);
    }
  });
  
  if uniq.is_empty() {
    '_'
  } else {
    uniq.pop_front().unwrap()
  }
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_1() {
        assert_eq!(firstNotRepeatingCharacter("abacabad".to_string()), 'c');
    }

    #[test]
    fn test_10() {
        assert_eq!(firstNotRepeatingCharacter("ngrhhqbhnsipkcoqjyviikvxbxyphsnjpdxkhtadltsuxbfbrkof".to_string()), 'g');
    }
}
