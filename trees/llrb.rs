extern mod std;
use core::cmp::{Eq, Ord};
use core::option;

enum Color { Red, Black }
/*
Left Leaning Red Black Trees
*/
//after http://www.mew.org/~kazu/proj/red-black-tree/
#[deriving_eq]
enum LLRB_Node<T>
{
    Leaf,
    Fork(@T, @Color, @LLRB_Node<T>, @LLRB_Node<T>)

}

fn insert<T: Eq Ord> (newdata: @T, node: @LLRB_Node<T>) -> @LLRB_Node<T>
{
    return @Leaf
}


#[test]
fn test_root_creation () {
    let tree : @LLRB_Node<int> = insert(@10, @Leaf);

    assert tree == @Leaf
}