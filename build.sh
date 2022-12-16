#!/bin/bash
#
wasm-pack build --release --target web --out-dir ./pages/pkg/ 

cd ./elm
elm make src/Main.elm --optimize --output ../assets/js/elm.js
esbuild ../assets/js/elm.js --minify --target=es5 --outfile=../pages/assets/js/elm.js
cd ..

cp ./assets/js/rustcanvas.js ./pages/assets/js/.
cp ./assets/playerstart.txt ./pages/assets/.
cp ./index.html ./pages/.
cp -r ./assets/images ./pages/assets/.
