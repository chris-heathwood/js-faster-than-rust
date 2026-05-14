function findFirstFourteen(findString, window) {
  for (let i = window, n = findString.length; i < n; ++i) {
    if (new Set(findString.slice(i - window, i)).size === window) {
      return i;
    }
  }
}

module.exports = { findFirstFourteen };
