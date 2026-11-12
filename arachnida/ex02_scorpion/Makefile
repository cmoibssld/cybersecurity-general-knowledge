# Keep Rust off the main filesystem. The exported variables also make the
# locally installed toolchain available to cargo in every recipe.
GOINFRE_DIR ?= /goinfre/$(USER)
CARGO_HOME = $(GOINFRE_DIR)/.cargo
RUSTUP_HOME = $(GOINFRE_DIR)/.rustup
CARGO := $(CARGO_HOME)/bin/cargo
RUSTUP := $(CARGO_HOME)/bin/rustup
# I must change from rustc 1.97 to 1.86 because there has been a change in the std that leaves a leak (on the stack overflow checker)
RUST_VERSION ?= 1.86.0

export CARGO_HOME
export RUSTUP_HOME
export PATH := $(CARGO_HOME)/bin:$(PATH)

all: init
	$(CARGO) +$(RUST_VERSION) build --release
	ln -f ./target/release/scorpion ./scorpion

init:
	@if [ -x "$(CARGO)" ] && [ -x "$(CARGO_HOME)/bin/rustc" ]; then \
		echo "Rust is already installed in $(GOINFRE_DIR)."; \
	else \
		set -eu; \
		mkdir -p "$(CARGO_HOME)" "$(RUSTUP_HOME)"; \
		install_script=$$(mktemp); \
		trap 'rm -f "$$install_script"' EXIT; \
		echo "Installing Rust in $(GOINFRE_DIR)..."; \
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$$install_script"; \
		sh "$$install_script" -y --default-toolchain none --profile minimal; \
	fi
	$(RUSTUP) toolchain install $(RUST_VERSION) --profile minimal

clean:
	$(CARGO) clean
	rm -rf ./scorpion

re: clean all

.PHONY: all init clean re
