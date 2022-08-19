## Introduction

Folder to learn how to use Massif.

## Compile with -g

The [docs say](<https://valgrind.org/docs/manual/ms-manual.html>) `you should compile with debugging info (the -g option)`.

### Rust

The [debug info option](<https://doc.rust-lang.org/rustc/codegen-options/index.html?highlight=debuginfo#debuginfo>) is activated with `-C debuginfo=2`. The command `cargo build` does this by default, as you can see with:

```bash
cargo build --verbose
   Compiling foo v0.1.0 (/tmp/foo)
     Running `rustc --crate-name foo --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 -C metadata=48ffd6d29e667245 -C extra-filename=-48ffd6d29e667245 --out-dir /tmp/foo/target/debug/deps -C incremental=/tmp/foo/target/debug/incremental -L dependency=/tmp/foo/target/debug/deps`
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
```

The `cargo build --relase` command does not use debuginfo by default, this is specificed in [the docs](<https://doc.rust-lang.org/rustc/codegen-options/index.html#opt-level>) `If not specified, debug assertions are automatically enabled only if the opt-level is 0.`:

```bash
cargo build --release --verbose
   Compiling foo v0.1.0 (/tmp/foo)
     Running `rustc --crate-name foo --edition=2021 src/main.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type bin --emit=dep-info,link -C opt-level=3 -C embed-bitcode=no -C metadata=99b4a5d477b3560b -C extra-filename=-99b4a5d477b3560b --out-dir /tmp/foo/target/release/deps -L dependency=/tmp/foo/target/release/deps`
    Finished release [optimized] target(s) in 0.22s
```

## Configuration

### C and Rust. Compile the program

First, compile the program running the following script that you can find in the folder of each language:

```bash
./compile-the-program
```

### Python

Install the requirements.txt file.

## Run

Run and measure the program:

```bash
# C
./run-and-measure c/prog
# Rust debug
./run-and-measure rust/prog/target/debug/prog
# Rust release
./run-and-measure rust/prog/target/release/prog
```

## Analyze the results

To see the results, you can run:

```bash
ms_print massif.out.97395  | less
```

### Notes

- `massif.out....` result files:
  - Rust debug version, shows only one function of the `main.rs` file.
  - Rust release version does not show any function.
  - The c program has more info about the program functions executed.

## Resources

<https://valgrind.org/docs/manual/ms-manual.html>
