.PHONY: all run clean step0_test

TARGET = step0_repl
SHELL = zsh

all: $(TARGET) run

$(TARGET): src/main.rs
	cargo build

run: $(TARGET)
	@echo '#!/bin/sh' > run
	@echo 'exec $(PWD)/$(TARGET) "$$@"' >> run
	@chmod +x run

clean:
	cargo clean
	rm -f run

step0_test:
	cd ../../ && make "test^rust^step0"
