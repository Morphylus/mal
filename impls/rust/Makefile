EXEC_DIR := target/release

STEP0 := $(EXEC_DIR)/step0_repl
STEPS := $(STEP0)
STEP1 :=$(EXEC_DIR)/step1_read_print

STEPS := $(STEP0) $(STEP1)

all: $(STEPS)

$(STEPS): $(EXEC_DIR)/%: %.rs
	cargo build --release --bin $*

lint:
	rustfmt *.rs
	cargo clippy

.PHONY: clean

clean:
	cargo clean
