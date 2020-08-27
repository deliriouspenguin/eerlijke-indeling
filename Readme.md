# Eerlijke indeling

This is the repository for the app [Eerlijke indeling](https://eerlijke-indeling.nl). `Eerlijke Indeling` is an application to safely assign students to activities or workshops. To fairly assign student, we use the algoritm that is used in Amsterdam to assign students to schools. This algorithm is implememented in the [matchmaker](https://github.com/deliriouspenguin/matchmaker) package. See that repository for more information.

## Assets

The assets for the application are in a separate repository, because not all of them play nice with the GPL. So you need to download them separately.

If you have `make` installed you can run:

```bash
% make static/assets
```

Otherwise you need to download the assets from [GitHub](https://github.com/deliriouspenguin/eerlijke-indeling-assets/releases/download/0.1.0/eerlijke-indeling-assets-0.1.0.tgz) and unpack them in `static/assets`.

## Compiling

In order to compile the project, you need to have `Rust` installed. You also need to install `wasm-pack` using `Cargo`.

We can install `Rust` using `rustup`:

```bash
% curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then we can install `wasm-pack` using `Cargo`:

```bash
% cargo install wasm-pack
```

When this is done, the project can be build:

```bash
% wasm-pack build --target web --out-dir ./static --out-name wasm
```

Now upload the content of `static` to any webserver and the application is ready for use.

## Deployment

`Eerlijke indeling` is a web-app, so has very little hosting requirements. You basically need a webserver which is capable of serving static files. The project needs to be compiled, though, to get these static files. See above for instructions.

## Development

The application is base on [Yew](https://yew.rs/), a Rust framework to build frond-end apps.

To start developing you need to install `Rust` and `wasm-pack`, see [Compiling](#compiling) for instructions. You also need `cargo-make` and `simple-http-server`:

```bash
% cargo install cargo-make
% cargo install simple-http-server
```

Then open a terminal window, `cd` to the project's working directory and type:

```bash
% cargo make build
```

Open an other window, `cd` to the project's working directory and type:

```
% cargo make serve
```

The webapplication is now available at [http://localhost:3000/](http://localhost:3000/). Any changes in the code will trigger a recompilation of the project.

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