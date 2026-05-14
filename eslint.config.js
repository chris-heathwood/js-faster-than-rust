const xn = require('eslint-plugin-extreme-node');

module.exports = [
  { ignores: ['js/chris.js', 'js/mike.js', 'godbolt/**'] },
  xn.configs.recommended,
];
