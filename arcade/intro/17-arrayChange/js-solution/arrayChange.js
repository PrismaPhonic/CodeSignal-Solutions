/* The Problem: You are given an input array, and allowed to increment one of the elements by +1 each 
 * round.  Your function should return how many rounds it takes until you achieve a strictly increasing
 * sequence from the input.
 * 
 * Input: Array of integers, which may be negative
 * Output: an integer of how many rounds it took to achieve the end result
*/

function arrayChange(inputArray) {
  let max = -Infinity;
  let round = 0;

  for (let num of inputArray) {
    if (num > max) {
      max = num;
    } else {
      round += (max - num) + 1;
      max++;
    }
  }

  return round;
}
