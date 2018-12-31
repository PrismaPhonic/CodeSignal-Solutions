# First Not Repeating Character

## Rust Solution

**The Problem**:
Given a string `s` return the first occurance of a unique (non-repeating) character in the string. Do it in O(n) time and O(1) space

**Solution**:
We'll create two hash sets:
1. To store occurances
2. To store unique characters (HashMap in Rust Solution)

The standard library HashSet in Rust is unordered so I couldn't accomplish this using the same method I used in my JS solution. I was able to do it that way with an external create linked_hash_set but CodeSignal doesn't let you use external crates. Instead I stored unique in a HashMap w/ an index it was found out.  then after we've done that we can iterate through our HashMap and return the key at the lowest value (lowest index):

```Rust
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
```

My linked-hash-set solution is more similar to my js solution and I personally prefer it:

```Rust
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
```

