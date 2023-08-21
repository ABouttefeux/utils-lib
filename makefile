
include utility.mk 

lib_name = utils-lib.rlib

source_sufix := rs
source_files := $(foreach sufix, $(source_sufix), $(wildcard *.$(sufix) */*.$(sufix) */*/*.$(sufix) */*/*/*.$(sufix) */*/*/*/*.$(sufix) */*/*/*/*/*.$(sufix)))

cargo := cargo
cargo_build := b
cargo_test := test
cargo_release_flag := --release
cargo_doc := doc
cargo_doc_flag := --document-private-items --no-deps
cargo_all_flag := --all

rust_stable := +stable
rust_nightly := +nightly

.PHONY: all
all: target/release/$(lib_name)


.PHONY: build
build: $(source_files)
	$(cargo) $(rust_nightly) $(cargo_build)
	$(cargo) $(rust_stable) $(cargo_build)


.PHONY: test
test: $(source_files)
	$(cargo) $(rust_nightly) $(cargo_test) $(cargo_all_flag)
	$(cargo) $(rust_stable) $(cargo_test) $(cargo_all_flag)


.PHONY: doc
doc: $(source_files)
	$(cargo) $(cargo_doc) $(cargo_all_flag) $(cargo_doc_flag)


.PHONY: clean
clean: .FORCE
	$(cargo) clean


.PHONY: setup
setup: .git/hooks/pre-commit


.git/hooks/pre-commit: tools/git_hook/pre-commit
	- $(copy) $(call canonicalize_path,$<) $(call canonicalize_path,$@)


target/release/$(lib_name): $(source_files)
	$(cargo) $(cargo_build) $(cargo_release_flag)


target/debug/$(lib_name): $(source_files)
	$(cargo) $(cargo_build)
