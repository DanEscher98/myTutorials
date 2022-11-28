#!/usr/bin/bash

wasm-pack build
rm -rd www/node_modules/rsbook_wasmconway
npm --prefix www install
npm --prefix www run start
