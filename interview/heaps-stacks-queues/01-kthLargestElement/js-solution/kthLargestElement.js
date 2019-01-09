/**
 * Only including this solution - even though it's supposed to 
 * be a binary heap problem, the binary heap solution proved to
 * be much less efficient after extensive benchmarking
 */

function kthLargestElement(nums, k) {
  nums.sort((a,b) => b - a);
  return nums[k-1];
}
