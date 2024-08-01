# `wasi-wit-download`

Currently, there's lots of Wasm projects that require manual setup from the user. Part of that manual setup commonly involves downloading WIT files from a specific Wasmtime release. This project aims to automate that process by providing a CLI tool that can download WIT files for a specific Wasmtime release.

## How to install?

```bash
cargo install wasi-wit-download
```

## How to use?

```bash
wasi-wit-download <release> <wit-dep-1> <wit-dep-2> ... <wit-dep-n>
```

## Example

```bash
wasi-wit-download 18 cli clocks
```

Output:

```text
Downloading from Wasmtime version: 18
WIT dependencies to download: ["cli", "clocks"]
Downloaded: "cli"
Downloaded: "clocks"
```

This will download the WIT files for the `cli` and `clocks` dependencies from the Wasmtime release `18` to the current directory.

If you request a WIT dependency that doesn't exist, the tool will print an error message and exit.

```bash
$ wasi-wit-download 18 cli foo clocks
Downloading from Wasmtime version: 18
WIT dependencies to download: ["cli", "foo", "clocks"]
Downloaded: "cli"
Downloaded: "clocks"
Error: The following folders were not found in the archive: ["foo"]
error: process didn't exit successfully: `wasi-wit-download.exe 18 cli clocks foo` (exit code: 1)
```

> Note: The WIT dependencies `cli` and `clocks` will still be downloaded.

## Limitations

- This only works for Wasmtime versions 10 and above because we hardcode WIT deps search from `wasmtime-{version}.0.0/crates/wasi/wit/deps` which only exists in versions 10 onwards.
- There's no CI for testing. Only manual testing.
