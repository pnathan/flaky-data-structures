all: build run_test

# this builds the library
build:
	rustc --lib structures.rc

# these tests are unit tests
run_test: trees/test_bst trees/test_llrb linear/test_linear

linear/test_linear: linear/linear.rs
	rustc --test linear/linear.rs -o linear/test_linear
	./linear/test_linear

trees/test_llrb: trees/llrb.rs
	rustc --test trees/llrb.rs -o trees/test_llrb
	./trees/test_llrb

trees/test_bst: trees/bst.rs
	rustc --test trees/bst.rs -o trees/test_bst
	./trees/test_bst

# these tests verify that it can be linked appropriately
run_link_test: build
	rustc -L . test/test_bst.rs
