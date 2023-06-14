# vuewasm

This template should help get you started developing with Vue 3 in Vite to create WASM and a rust based backend using TypeScript

## Setup development env.
It workers with Ubuntu or Debian( with a user that has sudo capabilities).
Need to install rust, wasm-pack,  greenpgx/vite-plugin-rsw, rsw.
From a user account run 
$ mkdevenv


To create a new project, run from home directory, NAME is the name of your new project
$ mkvuewasm NAME

* see https://github.com/greenpdx/vite-plugin-rsw


## Recommended IDE Setup

[VSCodium](https://vscodium.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) +
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) +
[crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) +
[Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) +
[WebAssembly Toolkit for VSCode](https://marketplace.visualstudio.com/items?itemName=dtsvet.vscode-wasm)

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
    1) Run `Extensions: Show Built-in Extensions` from VSCode's command palette
    2) Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

front-end
```sh
npm install
```

### Compile and Hot-Reload for Development
Need two terminals open, one for vue/WASM front-end and one for rust back-end
The front-end listens on all addresses so it can be used inside a container

front-end
```sh
npm run dev
```

back-end
```sh
cd rustbe
cargo run
```

### Type-Check, Compile and Minify for Production

front-end
```sh
npm run build
```

back-end
```sh
cargo build
```
I like to use a nginx proxy server for the backend for production

### Lint with [ESLint](https://eslint.org/)

```sh
npm run lint
```
