# Kth Largest Element

## Javascript Solution

I did extensive benchmarking and found that using a simple sort was always more
efficient than using the max heap solution that this problem tries to hint that
you should use.  

Sort solution:

```javascript
function kthLargestElement(nums, k) {
  nums.sort((a,b) => b - a);
  return nums[k-1];
}
```
