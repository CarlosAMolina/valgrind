## Introduction

Folder to learn how to use Massif.

## Compile with -g

The [docs say](<https://valgrind.org/docs/manual/ms-manual.html>) `you should compile with debugging info (the -g option)`.

### Rust

We must use the [debug info option](<https://doc.rust-lang.org/rustc/codegen-options/index.html?highlight=debuginfo#debuginfo>) with `-C debuginfo=2`. The command `cargo build` does this by default, as you can see with:

```bash
cargo build --verbose
   Compiling ....
     Running `rustc --crate-name ... -C debuginfo=2 ...
```

## Run

First, compile the program:

```bash
./compile-the-program
```

Run and measure the program:

```bash
./run-and-measure
```

See te results. For example:

```bash
ms_print massif.out.97395  | less
```

## Resources

<https://valgrind.org/docs/manual/ms-manual.html>
