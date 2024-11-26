#!/bin/bash
mkdir -p src/internet_identity
curl -L -o src/internet_identity/internet_identity.wasm.gz https://github.com/dfinity/internet-identity/releases/download/release-2024-01-26/internet_identity_dev.wasm.gz
gunzip -f src/internet_identity/internet_identity.wasm.gz
curl -L -o src/internet_identity/internet_identity.did https://github.com/dfinity/internet-identity/releases/download/release-2024-01-26/internet_identity.did
