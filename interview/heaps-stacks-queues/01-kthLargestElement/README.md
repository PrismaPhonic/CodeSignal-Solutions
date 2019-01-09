# Kth Largest Element

## Problem

Find the kth largest element in an unsorted array. This will be the kth largest
element in sorted order, not the kth distinct element.

Note: This problem hints that we should be using a maxheap.  I benchmarked in
rust to find that using `sort_unstable` is definitely much faster than using a
maxheap, but have included both solutions in the readme.

## Solutions
1. [Rust Solution](rs-solution/kth-largest-element)
