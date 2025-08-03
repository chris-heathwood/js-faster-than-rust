// Inspired by https://github.com/trekhleb/javascript-algorithms/blob/master/src/algorithms/sets/combinations/combineWithRepetitions.js
/**
 * @param {*[]} comboOptions
 * @param {number} comboLength
 * @return {*[]}
 */
function combineWithRepetitions(comboOptions, comboLength) {
  // If the length of the combination is 1 then each element of the original array
  // is a combination itself.
  if (comboLength === 1) {
    return comboOptions.map((comboOption) => [comboOption]);
  }

  // Init combinations array.
  const combos = [];

  // Remember characters one by one and concatenate them to combinations of smaller lengths.
  // We don't extract elements here because the repetitions are allowed.
  for (const [optionIndex, currentOption] of comboOptions.entries()) {
    // Generate combinations of smaller size.
    const smallerCombos = combineWithRepetitions(
      comboOptions.slice(optionIndex),
      comboLength - 1,
    );

    // Concatenate currentOption with all combinations of smaller size.
    for (const smallerCombo of smallerCombos) {
      combos.push([currentOption].concat(smallerCombo));
    };
  };

  return combos;
}

function combinationsWithTwoTheSame(combos) {
  const comboTTS = [];

  for (const combo of combos) {
    let found = false;
    const foundState = {};
    
    for (const letter of combo) {
      if (foundState[letter]) {
        found = true;
        break;
      }

      foundState[letter] = true;
    }

    if (found === true) {
      comboTTS.push(combo);
    }
  }

  return comboTTS;
}

function logCombinations(n = 2) {
  const az = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];

  const combos = combineWithRepetitions(az, n);
  const comboWithTwoTheSame = combinationsWithTwoTheSame(combos);

  console.log(`${n} | ${comboWithTwoTheSame.length}/${combos.length}`);
}

logCombinations(2);
logCombinations(3);
logCombinations(14);