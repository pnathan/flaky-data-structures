use std::rc;

#[deriving(PartialEq)]
enum Color { Red, Black }
/*
Left Leaning Red Black Trees
*/
//after http://www.mew.org/~kazu/proj/red-black-tree/
#[deriving(PartialEq)]
enum LLRB_Node<T>
{
    Leaf,
    Fork(rc::Rc<T>, rc::Rc<Color>, rc::Rc<LLRB_Node<T>>, rc::Rc<LLRB_Node<T>>)

}

fn insert<T: Eq + Ord> (newdata: rc::Rc<T>, node: rc::Rc<LLRB_Node<T>>)
                        -> rc::Rc<LLRB_Node<T>>
{
    return rc::Rc::new(Leaf)
}


#[test]
fn test_root_creation () {
    let tree : rc::Rc<LLRB_Node<int>> = insert(rc::Rc::new(10), rc::Rc::new(Leaf));

    assert!( tree.deref() == &Leaf);
}
