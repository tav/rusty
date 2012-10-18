# Public Domain (-) 2012 The Rusty Authors.
# See the Rusty UNLICENSE file for details.

.PHONY: clean install

CRATE_FILE = rusty.rc
RUST_FILES = $(wildcard *.rs)

build/rusty: $(RUST_FILES) $(CRATE_FILE)
	@mkdir -p build
	@echo "=> Compiling rusty ..."
	@rustc rusty.rc -o build/rusty

clean:
	@rm -rf build
	@echo "=> Cleaned build"

install: build/rusty
ifeq ($(strip $(RUSTBIN)),)
	@echo "!! ERROR: The RUSTBIN environment variable hasn't been set"
else
	@cp build/rusty ${RUSTBIN}/
	@echo "=> Installed ${RUSTBIN}/rusty"
endif
