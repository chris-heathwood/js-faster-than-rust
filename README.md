# js-faster-than-rust

This repo is to support an article on the posidevely website.

We take the input (shown in [input.txt](./input.txt)) and find the first 14 unique characters in the string.

We write some optimised code in different languages to see how they compare.

## Combinations

```sh
node --max-old-space-size=24576 combinations.js
```

Run this on a lambda!

## js

You will need to install these dependencies

    - `cd js`
    - `node --allow-natives-syntax chris.js`

## rust

    - `cd rust`
    - `rustc benny.rs`
    - `./benny`
    - `rustc chris.rs`
    - `./chris`
    - `rustc david.rs`
    - `./david`

## zig
    - `cd zig`
    - ``

