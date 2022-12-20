#!/bin/bash
inotifywait -m -r -e modify ./src/ ./elm_rust/src | while read directory event file
do
    wasm-pack build --dev --target web
done 
