all: run_test
run: run_usual run_test

run_test: test
	./tree_test

run_usual: usual
	./tree

usual: tree.rs
	rustc tree.rs -o tree

test: tree.rs
	rustc --test tree.rs -o tree_test
