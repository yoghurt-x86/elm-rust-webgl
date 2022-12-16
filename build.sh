#!/bin/bash
#
wasm-pack build --release --target web --out-dir ./docs/pkg/ 

cd ./elm
elm make src/Main.elm --optimize --output ../assets/js/elm.js
esbuild ../assets/js/elm.js --minify --target=es5 --outfile=../docs/assets/js/elm.js
cd ..

cp ./assets/js/rustcanvas.js ./docs/assets/js/.
cp ./assets/playerstart.txt ./docs/assets/.
cp ./index.html ./docs/.
cp -r ./assets/images ./docs/assets/.
