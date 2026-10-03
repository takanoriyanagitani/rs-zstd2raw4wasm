#!/bin/sh

iwasm=./rs_zstd2raw4wasm.wasm

wasm-opt \
	-Oz \
	-o opt.wasm \
	--enable-simd \
	--enable-relaxed-simd \
	--enable-bulk-memory \
	--enable-nontrapping-float-to-int \
	--enable-multivalue \
	--enable-tail-call \
	"${iwasm}"
