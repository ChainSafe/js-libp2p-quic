#!/bin/bash


# Typescript dislikes the generated js code from napi-rs
# so we need to disable type checking for that file
# Also, we're hacking the generated output to convert cjs to esm

# this script should be run once after running yarn build:rs

OUTPUT="src/napi.js"

{ cat scripts/JS_GEN_PREFIX; cat $OUTPUT; } > ${OUTPUT}.tmp
mv ${OUTPUT}.tmp $OUTPUT

sed -i '' "s/\.\/libp2p/..\/..\/libp2p/" $OUTPUT
sed -i '' "s/'libp2p/'..\/..\/libp2p/" $OUTPUT
sed -i '' "s/module\.exports\.\(.*\) = \(.*\)/export { \2 }/g" $OUTPUT