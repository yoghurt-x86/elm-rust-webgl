#!/bin/bash
#
wasm-pack build --release --target web --out-dir ./pages/pkg/ 

cd ./elm
elm make src/Main.elm --output ../pages/assets/js/elm.js
cd ..

cp ./assets/js/rustcanvas.js ./pages/assets/js/.
cp ./assets/playerstart.txt ./pages/assets/.
cp ./index.html ./pages/.
cp -r ./assets/images ./pages/assets/.
