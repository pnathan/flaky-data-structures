extern mod std;
use core::cmp::{Eq, Ord};
use core::option;

enum Color { Red, Black }
/*
Left Leaning Red Black Trees
*/
//after http://www.mew.org/~kazu/proj/red-black-tree/
enum LLRB_Node<T>
{
    Leaf,
    Fork(@T, @Color, @LLRB_Node<T>, @LLRB_Node<T>)

}

fn insert_llrb<T: Eq Ord> (newdata: @T, node: @LLRB_Node<T>) -> @LLRB_Node<T>
{
    return @Leaf
}
