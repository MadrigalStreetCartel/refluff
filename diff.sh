#!/usr/bin/env bash

VERSION1="$1"
VERSION2="$2"

FILE="main-wasm32.wat"
git diff "v$VERSION1/$FILE" "v$VERSION2/$FILE" >> "v${VERSION1}_v${VERSION2}.wat.diff"

FILE="main-wasm32.pseudo.c"
git diff "v$VERSION1/$FILE" "v$VERSION2/$FILE" >> "v${VERSION1}_v${VERSION2}.pseudo.c.diff"

FILE="main-wasm32.c"
git diff "v$VERSION1/$FILE" "v$VERSION2/$FILE" >> "v${VERSION1}_v${VERSION2}.c.diff"

FILE="main-wasm32.h"
git diff "v$VERSION1/$FILE" "v$VERSION2/$FILE" >> "v${VERSION1}_v${VERSION2}.h.diff"
