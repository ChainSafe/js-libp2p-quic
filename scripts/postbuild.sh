#!/bin/bash

# typescript dislikes the generated js code from napi-rs
# so we need to disable type checking for that file

# this script should be run once after running yarn build:rs

OUTPUT="src/napi.cjs"

{ cat scripts/JS_GEN_PREFIX; cat $OUTPUT; } > ${OUTPUT}.tmp
mv ${OUTPUT}.tmp $OUTPUT
sed -i "s/\.\/libp2p/..\/libp2p/" $OUTPUT
sed -i "s/'libp2p/'..\/libp2p/" $OUTPUT
