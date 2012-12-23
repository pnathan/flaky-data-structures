all: test_linear

# this builds the library
build:
	rustc --lib structures.rc

# these tests are unit tests
run_test: test_bst test_llrb test_linear

test_linear: linear/linear.rs
	rustc --test linear/linear.rs -o linear/test_linear
	./linear/test_linear

test_llrb: trees/llrb.rs
	rustc --test trees/llrb.rs -o trees/test_llrb
	./trees/test_llrb

test_bst: trees/bst.rs
	rustc --test trees/bst.rs -o trees/test_bst
	./trees/test_bst

# these tests verify that it can be linked appropriately
run_link_test: build
	rustc -L . test/test_bst.rs
