# Keep Rust off the main filesystem. The exported variables also make the
# locally installed toolchain available to cargo in every recipe.
GOINFRE_DIR ?= /tmp/ft_otp
CARGO_HOME = $(GOINFRE_DIR)/.cargo
RUSTUP_HOME = $(GOINFRE_DIR)/.rustup
CARGO := $(CARGO_HOME)/bin/cargo
CARGO_TARGET_DIR = $(GOINFRE_DIR)/target
RUSTUP := $(CARGO_HOME)/bin/rustup
RUST_VERSION = 1.86.0

export CARGO_HOME
export RUSTUP_HOME
export CARGO_TARGET_DIR
export PATH := $(CARGO_HOME)/bin:$(PATH)

all: init
	mkdir -p $(CARGO_TARGET_DIR)
	$(CARGO) +$(RUST_VERSION) build --release --locked
	cp $(CARGO_TARGET_DIR)/release/ft_otp ./ft_otp

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
		sh "$$install_script" -y --default-toolchain stable --profile minimal; \
	fi
	@if ! $(RUSTUP) run $(RUST_VERSION) rustc --version >/dev/null 2>&1; then \
		$(RUSTUP) toolchain install $(RUST_VERSION) --profile minimal; \
	fi


clean:
	$(CARGO) clean
	rm -rf ./ft_otp

re: clean all

.PHONY: all init clean re
