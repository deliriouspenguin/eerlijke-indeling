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

.PHONY: all rustup build

all: rustup bin/wasm-pack static/assets build

build:
	. $$HOME/.cargo/env && ./bin/wasm-pack build --target web --out-dir ./static --out-name wasm

static/assets:
	mkdir -p static/assets; \
		cd static/assets; \
		curl -L https://github.com/deliriouspenguin/eerlijke-indeling-assets/releases/download/0.1.0/eerlijke-indeling-assets-0.1.0.tgz -o eerlijke-indeling-assets-0.1.0.tgz; \
		tar xzf eerlijke-indeling-assets-0.1.0.tgz; \
		rm eerlijke-indeling-assets-0.1.0.tgz; \
		cd ../..

bin/wasm-pack:
	./contrib/download-wasm-pack.sh

rustup:
	curl https://sh.rustup.rs -sSf | sh -s -- -y