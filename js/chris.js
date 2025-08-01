const fs = require('fs');
const { performance } = require('perf_hooks');

const input = fs.readFileSync('../input.txt',
  { encoding: 'utf8', flag: 'r' });

const alphaLookup = [];

for (let a = 97; a < (97 + 26); a++) {
  alphaLookup[a] = 1 << (a & 31);
}

function findFirstFourteen(findString, window) {
  let check = 0;
  let count = 0;

  let i = window - 1;
  while (i < findString.length) {
    const end = (i + 1) - window;

    let w = i;
    while (w >= end) {
      const marker = alphaLookup[findString.charCodeAt(w)];

      if ((check & marker) === 0) {
        check = check | marker;
        count++;
      } else {
        i = w + window;
        check = 0;
        count = 0;
        break;
      }

      if (count === window) {
        return i + 1;
      }

      w--;
    }
  }

  return 0;
}

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

while (t >= 0) {
  const before = performance.now();
  start = callFindFirstFourteen();
  const after = performance.now();

  times.push((after - before) * 1000000);

  t--;
}

const average = times.reduce((a, b) => a + b) / times.length;

console.log(`findFirstFourteen found ${start} and took ${average} nanoseconds`);
