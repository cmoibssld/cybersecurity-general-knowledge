# Keep Rust off the main filesystem. The exported variables also make the
# locally installed toolchain available to cargo in every recipe.
GOINFRE_DIR ?= /tmp/arachnida/$(USER)
CARGO_HOME = $(GOINFRE_DIR)/.cargo
RUSTUP_HOME = $(GOINFRE_DIR)/.rustup
CARGO := $(CARGO_HOME)/bin/cargo
CARGO_TARGET_DIR = $(GOINFRE_DIR)/spider/target

export CARGO_HOME
export RUSTUP_HOME
export CARGO_TARGET_DIR
export PATH := $(CARGO_HOME)/bin:$(PATH)

all: init
	mkdir -p $(CARGO_TARGET_DIR)
	$(CARGO) build --release
	cp $(CARGO_TARGET_DIR)/release/spider ./spider

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

clean:
	$(CARGO) clean
	rm -rf ./spider
	rm -rd ./data

re: clean all

.PHONY: all init clean re
