
function factor(num) {
  if (num < 0) 
        return -1;
  else if (num == 0) 
      return 1;
  else {
      return (num * factor(num - 1));
  }
}

function totalNumberOfCombinations(n, r) {
  // (n + r - 1)! / (r! * (n - 1)!)
  return Math.ceil(factor(n + r - 1) / (factor(r) * factor(n - 1)));
}

function totalNumberOfCombinationsWithoutRepeats(n, r) {
  // n! / r!(n-r)!
  return Math.ceil(factor(n) / (factor(r) * factor(n-r)));
}

function logCombinations(n, r) {
  const combinations = totalNumberOfCombinations(n, r);
  const withoutRepeats = totalNumberOfCombinationsWithoutRepeats(n, r);

  console.log(`${r} | ${(combinations - withoutRepeats)}/${combinations}`);
}

logCombinations(26, 2);
logCombinations(26, 3);
logCombinations(26, 14);
