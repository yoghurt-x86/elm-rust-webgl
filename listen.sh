#!/bin/bash
inotifywait -m -r -e modify ./src/ | while read directory event file
do
    wasm-pack build --target web
done 
