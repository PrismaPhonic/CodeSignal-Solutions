# First Duplicate
## JS Solution

**The Problem**: 
Given an array 'a', that contains only numbers in        
the range from 1 to a.length, find the first duplicate number for   
which the second occurance (The duplicate) has the lowest index 
(comes first)  

**Solution**: create a hash set to store numbers that have not been
stored yet.  as soon as we find one that already exists in the
set - we have found our first duplicate - return that num.

Here's my JS solution:

```javascript
function firstDuplicate(a) {
    let occurance = new Set();
    
    for (let num of a) {
        if (!occurance.has(num)) {
            occurance.add(num);
        } else {
            return num;
        }
    }
    
    return -1;
}
```


