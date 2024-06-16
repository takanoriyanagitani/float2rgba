#!/bin/sh

features(){
    echo simple

    echo simple_red
    echo simple_red_wasm

    echo simple_gray
    echo simple_gray_wasm
}

cargo \
    test \
    --features $( features | tr '\n' , | sed 's/,$//' )
