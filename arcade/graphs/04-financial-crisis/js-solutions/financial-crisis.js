/**
 * This solution seems niave in time complexity as we are mapping
 * over the input matrix multiple times (one for each possible removal)
 * next solution is more efficient
 *
 * Benchmark Results:
 * roadRegisterNaive#test x 242,311 ops/sec ±1.45% (86 runs sampled)
 * roadRegisterEfficient#test x 429,924 ops/sec ±1.04% (84 runs sampled)
 * Fastest is roadRegisterEfficient#test
 */

function financialCrisisNaive(roadRegister) {
  let possibilities = [];
  for (let i = 0; i < roadRegister.length; i++) {
    possibilities.push(
      roadRegister
        .filter((city, index) => index !== i)
        .map(city => [...city.slice(0, i), ...city.slice(i + 1)]),
    );
  }
  return possibilities;
}

/**
 * This solution is better even though it's less readable -
 * we simply map over the input matrix once
 * and if the column or row matches the city to be removed from our
 * output possibilities choice then we simply don't place the value
 * in that possibility matrix.
 * 
 * Benchmarked and this solution performs much faster with large inputs
 */

function financialCrisis(roadRegister) {
  // Pre-populate with arrays to hold all of our possible road
  // registers
  let possibilities = [];
  for (let m = 0; m < roadRegister.length; m++) {
    possibilities.push([]);
  }

  for (let i = 0; i < roadRegister.length; i++) {
    // At this point we will push a new array to each
    // possibility sub-array in our possibilities array
    // for each that does not match i (our city to be thrown out);
    for (let l = 0; l < possibilities.length; l++) {
      if (i !== l) possibilities[l].push([]);
    }

    // array to iterate through input matrix rows (j)
    for (let j = 0; j < roadRegister[0].length; j++) {

      // k is here to move through our possibilities of road registers 
      // and push boolean from input matrix to appropriate possibility matrices
      for (let k = 0; k < possibilities.length; k++) {
        
        //latest represents the most recent array pushed into our possibility matrix
        let latest = possibilities[k].length - 1;
        if (i !== k && j !== k) {
          possibilities[k][latest].push(roadRegister[i][j]);
        }
      }
    }
  }

  return possibilities;
}

module.exports = financialCrisis;
