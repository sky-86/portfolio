#!/opt/homebrew/bin/fish

tailwindcss -i tailwind/tailwind.css -o tailwind/out/tailwind.css --minify
trunk build --release

rm -rf release/*
mv dist/* release/
cd release

set -l wasm (find . -name *_bg.wasm)

wasm-opt -Oz -o $wasm.bak $wasm
rm $wasm
mv $wasm.bak $wasm
