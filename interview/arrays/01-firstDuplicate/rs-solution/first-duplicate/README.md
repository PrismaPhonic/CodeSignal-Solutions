# First Duplicate
## JS Solution

**The Problem**: 
Given an array 'a', that contains only numbers in        
the range from 1 to a.length, find the first duplicate number for   
which the second occurance (The duplicate) has the lowest index 
(comes first). Return -1 if no duplicates are found.

**Solution**: create a hash set to store numbers that have not been
stored yet.  as soon as we find one that already exists in the
set - we have found our first duplicate - return that num.

**Note**: Rust's HashSet API is really nice in that inserting into a HashSet
will return true if insert succeeded or falsee if the value already exists in
the HashSet so we can skip an explicit check into the HashSet. I also
benchmarked this using the hashbrown external crate which uses Googles new
SwissTable hashing algorithm which is supposed to be very fast. Didn't see
a discernable difference in speed with my benchmarking.

Here's my RS solution:

```Rust
use std::collections::HashSet;

fn firstDuplicate(a: Vec<i32>) -> i32 {
    let mut occurances: HashSet<i32> = HashSet::new();
    
    for num in a {
        if occurances.insert(num) == false {
            return num;
        }
    }
    
    -1
}
```
