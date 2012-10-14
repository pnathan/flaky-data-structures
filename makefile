all: usual test

usual: tree.rs
	rustc tree.rs -o tree

test: tree.rs
	rustc --test tree.rs -o tree_test
