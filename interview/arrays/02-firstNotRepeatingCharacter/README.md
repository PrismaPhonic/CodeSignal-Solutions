# First Not Repeating Character

**The Problem**: 
Given a string 's' return the first occurance of a unique (non-repeating)
character in the string. Do it in O(n) time and O(1) space

**Algorithmic Efficiency**:
My algorithm is in O(n) time as it only iterates through the string once. 
It's in O(1) space because my sets are at most 27 chars long even if the 
string was millions of characters long - meaning that it is O(1) after
amortization.

# Solutions
1. [JS Solution](/js-solution)
2. [Rust Solution](/rs-solution/first-not-repeating-character)
