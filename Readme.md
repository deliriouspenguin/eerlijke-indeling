# Eerlijke indeling

This is the repository for the app [Eerlijke indeling](https://eerlijke-indeling.nl). `Eerlijke Indeling` is an application to safely assign students to activities or workshops. To fairly assign student, we use the algoritm that is used in Amsterdam to assign students to schools. This algorithm is implememented in the [matchmaker](https://github.com/deliriouspenguin/matchmaker) package. See that repository for more information.

## Deployment

`Eerlijke indeling` is a web-app, so has very little hosting requirements. You basically need a webserver which is capable of serving static files. You can get the static files from one of the [releases](https://github.com/deliriouspenguin/eerlijke-indeling/releases). Download the file `eerlijke-indeling-<version>-static.zip` and upload its contents to your webserver.

## Building

You can of course also build the static files yourself. The quickest way to do this, is with the `make all` command:

```bash
% make all
```

However, this is meant for use in a container or a virtual machine and not a development machine.

## Development

The application is based on [Yew](https://yew.rs/), a Rust framework to build frond-end apps.

To start developing you need to get the assets from a seperate repository. You also need to install `Rust`, `wasm-pack`, `cargo-make` and `simple-http-server`.

### Assets

The assets for the application are in a separate repository, because not all of them play nice with the GPL. So you need to download them separately.

If you have `make` installed you can run:

```bash
% make static/assets
```

Otherwise you need to download the assets from [GitHub](https://github.com/deliriouspenguin/eerlijke-indeling-assets/releases/download/0.1.0/eerlijke-indeling-assets-0.1.0.tgz) and unpack them in `static/assets`.

### Installing Rust, wasm-pack, cargo-make and simple-http-server

We can install `Rust` using `rustup`:

```bash
% curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then we can install `wasm-pack`, `cargo-make` and `simple-http-server` using `Cargo`:

```bash
% cargo install wasm-pack
% cargo install cargo-make
% cargo install simple-http-server
```

### Running development environment

Then open a terminal window, `cd` to the project's working directory and type:

```bash
% cargo make build
```

Open an other window, `cd` to the project's working directory and type:

```
% cargo make serve
```

The webapplication is now available at [http://localhost:3000/](http://localhost:3000/). Any changes in the code will trigger a recompilation of the project.

### Building production code on development machine

You can build a production wasm target by using the following command:

```bash
% wasm-pack build --target web --out-dir ./static --out-name wasm
```

---

Copyright (C) 2020 Delirious Penguin

Eerlijke Indeling is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Eerlijke Indeling is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.