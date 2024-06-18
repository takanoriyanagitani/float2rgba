#!/bin/sh

addr=0.0.0.0
port=61680
docd=./

python3 \
	-m http.server \
	--bind "${addr}" \
	--directory "${docd}" \
	${port}
