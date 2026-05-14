// @ts-nocheck
const fs = require('fs');
const { performance } = require('perf_hooks');
const { findFirstFourteen } = require('./mike-find-first-fourteen');

const input = fs.readFileSync('../input.txt',
  { encoding: 'utf8', flag: 'r' });

function callFindFirstFourteen() {
  return findFirstFourteen(input, 14);
}

%PrepareFunctionForOptimization(callFindFirstFourteen);

let start = 0;

// Call function once to fill type information
start = callFindFirstFourteen();

// Call function again to go from uninitialized -> pre-monomorphic -> monomorphic
start = callFindFirstFourteen();

%OptimizeFunctionOnNextCall(callFindFirstFourteen);

start = callFindFirstFourteen();

let t = 100;
const times = [];

while (t > 0) {
  const before = performance.now();
  start = callFindFirstFourteen();
  const after = performance.now();

  times.push((after - before) * 1000000);

  t--;
}

const average = times.reduce((a, b) => a + b) / times.length;

console.log(`findFirstFourteen found ${start} and took ${average} nanoseconds`);
