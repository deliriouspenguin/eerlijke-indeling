#!/bin/bash
# Copyright (C) 2020 Delirious Penguin
# 
# This file is part of Eerlijke Indeling.
# 
# Eerlijke Indeling is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
# 
# Eerlijke Indeling is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
# 
# You should have received a copy of the GNU General Public License
# along with Eerlijke Indeling.  If not, see <http://www.gnu.org/licenses/>.


set -e

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    PLATFORM="unknown-linux-musl"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="apple-darwin"
else
    echo "OS $OSTYPE is not supported"
    exit 1
fi
URL="https://github.com/rustwasm/wasm-pack/releases/download/v0.9.1/wasm-pack-v0.9.1-x86_64-$PLATFORM.tar.gz"
VERSION="v0.9.1-x86_64-$PLATFORM"
curl $URL -Lo "wasm-pack-"$VERSION".tgz"
tar xzf "wasm-pack-"$VERSION".tgz"
mkdir -p bin
cp "wasm-pack-"$VERSION"/wasm-pack" ./bin/wasm-pack
rm -rf "wasm-pack-"$VERSION".tgz" "wasm-pack-"$VERSION