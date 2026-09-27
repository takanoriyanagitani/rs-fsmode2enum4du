#!/bin/bash

set -u

wsm="./target/wasm32-wasip1/release-wasi/fsmode2enum4du.wasm"

input1(){
  printf $(( 8#040755))
}

input2(){
  printf $(( 8#0100644 ))
}

input3(){
  printf $(( 
    8#$( stat -f '%p' ./Cargo.toml)
  ))
}

der2jer(){
  cat /dev/stdin |
    xxd -ps |
    python3 -m asn1tools \
      convert \
      -i der \
      -o jer \
      ./mode4du.asn \
      Mode4du \
      -
}

input1 | wasmtime run "${wsm}" | der2jer
input2 | wasmtime run "${wsm}" | der2jer
input3 | wasmtime run "${wsm}" | der2jer
