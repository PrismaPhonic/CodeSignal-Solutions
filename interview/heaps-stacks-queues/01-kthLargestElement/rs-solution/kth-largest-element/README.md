# Kth Largest Element

## Rust Solution

I did extensive benchmarking and found that using a simple sort was always more
efficient than using the max heap solution that this problem tries to hint that
you should use.  

Max Heap solution:

```rust
use std::collections::BinaryHeap;

fn kthLargestElement(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);

    for _ in 0..k-1 { heap.pop(); }

    heap.pop().unwrap()
}
```

Sort unstable solution:

```rust
fn kthLargestElement(mut nums: Vec<i32>, k: i32) -> i32 {
    nums[..].sort_unstable();
    
    nums[nums.len()-k as usize]
}
```
