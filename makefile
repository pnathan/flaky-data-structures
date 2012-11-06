all: run_test

build:
	rustc --lib structures.rc


run_test: test_bst test_llrb

test_llrb: trees/llrb.rs
	rustc --test trees/llrb.rs -o trees/test_llrb
	./trees/test_llrb

test_bst: trees/bst.rs
	rustc --test trees/bst.rs -o trees/test_bst
	./trees/test_bst
