# rescripts
> CLI tool for dumping, decompiling and analyzing FlyffU wasm files

## Prerequisites
> The `rescripts` wrapper script will check for prerequisites before running the main scripts.

- `node`: Runtime
- `zx`: Runtime wrapper
- `diff`: For diffing wasm/wat files
- `wabt`: For translating, analyzing and decompiling wasm/wat files

## Usage

**Check current FlyffU client version**

```sh
./rescripts version
```

**Dump and decompile latest FlyffU client**

```sh
./rescripts dump
```