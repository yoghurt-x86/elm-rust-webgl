#!/bin/bash
inotifywait -m -r -e modify ./src/ | while read directory event file
do
    cargo run
done 
