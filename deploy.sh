#! /bin/bash

(cd crates/app && trunk build --release)

DST=../flightsim-2d-web

cp -v Cargo.toml Cargo.lock $DST

mkdir -p $DST/crates
cp -r -v crates $DST

mkdir -p $DST/assets/textures
cp -v assets/textures/atlas.png $DST/assets/textures/atlas.png

echo target > $DST/.gitignore
echo .DS_Store >> $DST/.gitignore

cp -r -v .github $DST

cp -v -r crates/app/dist/ $DST/dist
cp -v -r crates/app/assets/ $DST/assets
cp -v crates/app/index.html $DST
