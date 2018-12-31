use std::collections::HashSet;
use std::collections::HashMap;

fn firstNotRepeatingCharacter(s: String) -> char {
  let mut occurances: HashSet<char> = HashSet::with_capacity(24);
  let mut uniq: HashMap<char, usize> = HashMap::with_capacity(24);
 
  s.chars().enumerate().for_each(|(i, c)| {
    if occurances.insert(c) == true {
      uniq.insert(c, i);
    } else {
      uniq.remove(&c);
    }
  });
  
  if uniq.is_empty() {
    '_'
  } else {
    let mut min_idx = s.len();
    let mut answer = '_';
    for (k, v) in uniq.into_iter() {
        if v < min_idx {
            min_idx = v;
            answer = k;
        }
    };
    answer
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
    fn test_3() {
        assert_eq!(firstNotRepeatingCharacter("z".to_string()), 'z');
    }

    #[test]
    fn test_10() {
        assert_eq!(firstNotRepeatingCharacter("ngrhhqbhnsipkcoqjyviikvxbxyphsnjpdxkhtadltsuxbfbrkof".to_string()), 'g');
    }
}
