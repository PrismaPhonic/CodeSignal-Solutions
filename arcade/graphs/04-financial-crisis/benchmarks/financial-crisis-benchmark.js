const Benchmark = require('benchmark');
const suite = new Benchmark.Suite;

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
// add listeners
.on('cycle', function(event) {
  console.log(String(event.target));
})
.on('complete', function() {
  console.log('Fastest is ' + this.filter('fastest').map('name'));
})
// run async
.run({ 'async': true });
