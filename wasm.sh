#!/bin/bash

export RUSTFLAGS='-C target_feature=+simd128'

cargo \
	build \
	--target wasm32-unknown-unknown \
	--profile release-wasm
