
include utility.mk 

lib_name := utils-lib.rlib

source_sufix := rs
source_files := $(foreach sufix, $(source_sufix), $(wildcard *.$(sufix) */*.$(sufix) */*/*.$(sufix) */*/*/*.$(sufix) */*/*/*/*.$(sufix) */*/*/*/*/*.$(sufix)))

cargo := cargo
cargo_build := build
cargo_test := test
cargo_doc := doc
cargo_all_flag := --all
cargo_crate_derive := -p utils-lib-derive

rust_release_flag := --release
rust_doc_flag_private_item := --document-private-items
rust_doc_flag_no_dep := --no-deps
rust_example_flag := --examples
rust_tests = --tests

rust_coverage_feature := features="coverage"

rust_stable := +stable
rust_nightly := +nightly

ifeq ($(OS),$(windows_os))
    powershell := powershell
    null := $$null
    call_with_var = $(powershell) $$env:$(1)=$(2); $(3); $$env:$(1)=$(null)
else
    call_with_var = $(1)=$(2) $(3)
endif


.PHONY: all
all: target/release/$(lib_name)


.PHONY: build
build: $(source_files)
	$(cargo) $(rust_nightly) $(cargo_build)
	$(cargo) $(rust_stable) $(cargo_build)


.PHONY: test
test: $(source_files)
	$(cargo) $(rust_nightly) $(cargo_test) $(cargo_all_flag)
	$(cargo) $(rust_nightly) $(cargo_test) $(cargo_all_flag) $(rust_example_flag)
	$(cargo) $(rust_nightly) $(cargo_test) $(release) $(cargo_all_flag)
	$(cargo) $(rust_nightly) $(cargo_test) $(release) $(cargo_all_flag) $(rust_example_flag)
	$(cargo) $(rust_stable) $(cargo_test) $(cargo_all_flag)
	$(cargo) $(rust_stable) $(cargo_test) $(cargo_all_flag) $(rust_example_flag)


.PHONY: bless_ui
bless_ui: $(source_files)
	$(call call_with_var,TRYBUILD,\"overwrite\",$(cargo) $(rust_nightly) $(cargo_test) $(cargo_crate_derive) $(rust_tests))


.PHONY: doc_check
doc_check: $(source_files)
	$(cargo) $(cargo_doc) $(cargo_all_flag) $(rust_doc_flag_private_item) $(rust_doc_flag_no_dep)


.PHONY: doc
doc: $(source_files)
	$(cargo) $(cargo_doc) $(cargo_all_flag) $(rust_doc_flag_no_dep)


.PHONY: clean
clean: .FORCE
	$(cargo) clean


.PHONY: setup
setup: .git/hooks/pre-commit


.git/hooks/pre-commit: tools/git_hook/pre-commit
	- $(copy) $(call canonicalize_path,$<) $(call canonicalize_path,$@)


target/release/$(lib_name): $(source_files)
	$(cargo) $(cargo_build) $(rust_release_flag)


target/debug/$(lib_name): $(source_files)
	$(cargo) $(cargo_build)

