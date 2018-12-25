const Benchmark = require('benchmark');
const suite = new Benchmark.Suite;
const ffi = require('ffi');

// Import JS and RS solutions for benchmarking
// NOTE: CAN'T FIGURE OUT HOW TO REPRESENT AN ARRAY OF ARRAYS OF BOOLEANS
// USING REF TYPES
// const rust = ffi.Library('../rs-solutions/financial-crisis/target/release/libembed.so', {
//   'financialCrisis': [['Object'], ['Object']]
// });
const nodeFinancialCrisis = require('../js-solutions/financial-crisis.js');

const testRegister = 
[[false,false,false,false,true,false], 
 [false,false,true,false,true,false], 
 [false,true,false,true,true,true], 
 [false,false,true,false,false,false], 
 [true,true,true,false,false,true], 
 [false,false,true,false,true,false]];

// add tests
suite.add('roadRegisterNode#test', function() {
  nodeFinancialCrisis(testRegister);
})
// .add('roadRegisterRust#test', function() {
// 	rust.FinancialCrisis(testRegister);
// })
// add listeners
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
// run async
.run({ 'async': true });
