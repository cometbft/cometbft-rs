#!/bin/bash
set -euo pipefail

# The version of CometBFT to build into the images.
CMTVERSION=${CMTVERSION:-0.34.21}

echo "Building for CometBFT v${CMTVERSION}..."

# Build the CometBFT development image
docker build \
    -f cometbft/Dockerfile \
    --build-arg CMTVERSION=${CMTVERSION} \
    --tag informaldev/cometbft:${CMTVERSION} \
    ./cometbft/

