#!/bin/bash

set -u

input1(){
  printf helo |
    zstd -1
}

input2(){
  find . |
    sort |
    zstd -1
}

echo using zstdcat
input2 | zstdcat | shasum -a 256
echo

echo using wasm
input2 | node main.mjs | shasum -a 256
echo

echo using wasm
input2 | node main.mjs | tail
