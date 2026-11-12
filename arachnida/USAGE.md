# Prerequisites

Cargo is installed automatically in `/goinfre/$USER` by each exercise's
Makefile. This keeps the Rust toolchain and Cargo registry off the main
filesystem.

From either exercise directory, run:

```bash
make init
make
```

`make init` is safe to run repeatedly. If Cargo and `rustc` are already
installed in `/goinfre/$USER/.cargo/bin`, it does nothing. The Makefile
exports `CARGO_HOME`, `RUSTUP_HOME`, and `PATH`, so `make` works immediately
after `make init` without sourcing an environment file or restarting the
shell.

To use a different temporary location:

```bash
make GOINFRE_DIR=/path/to/goinfre init
make GOINFRE_DIR=/path/to/goinfre
```
