#! /bin/bash

rm -rf pkg/*
wasm-pack build --target web --no-typescript
git add -f pkg/
git commit -m "deploy"
git push -f origin @:www
