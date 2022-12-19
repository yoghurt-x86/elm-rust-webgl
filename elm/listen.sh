#!/bin/bash
inotifywait -m -r -e modify ./src/ ./bind | while read directory event file
do
    elm make src/Main.elm --output ../assets/js/elm.js
done 
