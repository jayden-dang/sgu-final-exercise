#!/bin/sh

echo ">> Building contract"

yarn near-sdk-js build src/contract.ts build/contract.wasm
