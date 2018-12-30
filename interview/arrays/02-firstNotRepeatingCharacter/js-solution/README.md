# First Not Repeating Character
## JS Solution

**The Problem**: 
Given a string `s` return the first occurance of a unique (non-repeating)
character in the string. Do it in O(n) time and O(1) space

**Solution**: 
We'll create two hash sets: 
1. To store occurances
2. To store unique characters

We iterater over the string looking at each character and if it's not in the
occurances hash set we add it to it, and also add it to the unique characters
hash set.  Otherwise (it is already in the occurances hash set) we explicitely
rmeove it from the unique hash set.  

Because hash sets are **ordered** in js we can then iterate over our unique hash
set at the end to create an iterator object (using a simple for..of) and
instantly return the first item! 

Here's my JS solution:

```javascript
function firstNotRepeatingCharacter(s) {
    let occurances = new Set();
    let uniq = new Set();
    
    for (const char of s) {
        if (!occurances.has(char)) {
            occurances.add(char);
            uniq.add(char);
        } else {
            uniq.delete(char);
        }
    }
    
    if (uniq.size === 0) return '_';
    
    // we have at least one uniq so create
    // an iterator and immediately return first
    // char
    for (let char of uniq) {
        return char;
    }
}
```


