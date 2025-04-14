#! /bin/bash

if [ $(git status --porcelain | wc -c) -ne 0 ]; then
    echo "Must deploy from porcelain git state!"
    exit 1
fi

rm -rf pkg/*
wasm-pack build --target web --no-typescript
git add -f pkg/
git commit -m "deploy"
git push -f origin @:www
