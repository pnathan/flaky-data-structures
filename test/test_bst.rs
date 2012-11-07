/*
Tests importing correctly.
*/

extern mod structures;
use structures::trees::bst::*;

fn main() {
    let tree : @MaybeNode<int> = insert(@-1,
                                       @Empty);
}