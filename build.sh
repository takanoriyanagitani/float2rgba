#!/bin/sh

features() {
	echo simple

	echo simple_red
	echo simple_red_wasm

	echo simple_gray
	echo simple_gray_wasm

	echo ext_colorgrad
	echo ext_colorgrad_wasm

	echo ext_colorgrad_turbo_wasm

	echo canvas
}

export RUSTFLAGS='-C target_feature=+simd128'
cargo \
	build \
	--target wasm32-unknown-unknown \
	--features $(features | tr '\n' , | sed 's/,$//') \
	--profile release-wasm
