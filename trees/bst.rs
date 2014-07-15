// Happy hacking.
/*
Initial exploration: Pure functional data structures
*/

// std::rc has Eq derivable. But std::gc doesn't.
use std::rc;

// A binary tree node.
#[deriving(PartialEq)]
pub enum MaybeNode<T>
{
    Empty,
    // Data, left, right
    Node(rc::Rc<T>, rc::Rc<MaybeNode<T>>, rc::Rc<MaybeNode<T>>)
}


pub fn node_data<T> (node: rc::Rc<MaybeNode<T>>) -> Option<rc::Rc<T>> {
    match node.deref() {
      &Empty => {None}
      &Node(ref data, _, _) => { Some(data.clone()) }
    }
}

/*
// Pure functional delete
pub fn delete<T: Eq + Ord> (data_to_delete: @T, node: @MaybeNode<T>) -> @MaybeNode<T> {
    //io::println(fmt!("%?\n", node));
    match node {
      @Empty => { @Empty }
      @Node(node_data, left, right) => {
        if (data_to_delete == node_data)
        {
            //io::println(fmt!("eq\n"));
            // No children
            if (left == @Empty && right == @Empty) {
                @Empty
            }
            //One child
            else if (left == @Empty || right == @Empty) {
                //Return one or the other to take the place of the current node
                let child_node =
                    if       (left != @Empty) { left  }
                    else if (right != @Empty) { right }
                    else { fail!(~"Error: invalid"); };


                match child_node {
                  @Node(child_data, left, right) => {
                    @Node(child_data, left, right)
                  }
                  @Empty => { @Empty }
                }
            }
            //Two children.
            else {
                // If we had no right child, successor is less
                // straightforward. Since we (at this branch) by
                // definition HAVE a right child, we can assume away that case of walking up to root checking for left branches.

                //construct this node as:
                //
                //Grab successor data and use it, use left, use left
                //tree that has had successor removed.
                let successor = minimum(right);
                match successor {
                  @Node(s_data, _, _) => {
                    @Node(s_data, left,
                          delete(s_data, right))
                  }
                  @Empty => { fail!(~"Should be impossible"); }
                }
            }
        }
        else if ( node_data > data_to_delete) {
            //io::println(fmt!("left\n"));
            @Node(node_data, delete(data_to_delete, left), right) }
        else if ( node_data < data_to_delete) {
            //io::println(fmt!("right\n"));
            @Node(node_data, left, delete(data_to_delete, right)) }
        else { fail!(~"A number became neither <, >, or ="); }
      }
    }
}


pub fn insert<T: Eq + Ord> (newdata: @T,
                          node: @MaybeNode<T>) -> @MaybeNode<T> {
    let result = @match node {
      @Empty => {
        Node(newdata,@Empty, @Empty)
      }
      @Node(data, left, right) => {
        if (data == newdata) {
            // a bst doesn't have duplicates
            Node(data, left, right)
        }
        else if ( data > newdata ) {
            Node(data, insert(newdata, left), right)
        }
        else {
            Node(data, left, insert(newdata, right))
        }
      }
    };

    return result
}


pub fn find<T: Eq + Ord> (newdata: @T, root: @MaybeNode<T>) -> @MaybeNode<T> {
    @match root {
      @Empty => {
        return @Empty;
      }
      @Node(data, left, right) => {
        if (data == newdata)       { return root }
        else if ( data > newdata ) { return find(newdata, left) }
        else                       { return find(newdata, right) }
      }
    }
}

pub fn minimum<T>(node: @MaybeNode<T>) -> @MaybeNode<T> {
    match node {
      @Node(_, left, _) => {
        // walk down one level
        match left {
          @Empty => { node }
          _ => { minimum(left) }
        }
      }
      @Empty => { @Empty }
    }
}


pub fn maximum<T>(node: @MaybeNode<T>) -> @MaybeNode<T> {
    match node {
      @Node(_, _, right) => {
        maximum(right)
      }
      @Empty => { node }
    }
}


//////////////////////////////////////////////////////////////////////
// Tests
//////////////////////////////////////////////////////////////////////


// helper function
fn give_me_test_tree () -> @MaybeNode<int> {
    let mut test_tree : @MaybeNode<int> = @Empty;
    test_tree = insert(@8, test_tree);
    test_tree = insert(@11, test_tree);
    test_tree = insert(@2, test_tree);
    test_tree = insert(@-10, test_tree);

    return test_tree;
}


#[test]
fn check_add_one () {
    let root : @MaybeNode<int> = @Empty;

    let expected : @MaybeNode<int> = @Node(@2, @Empty, @Empty);

    let inserted = insert(@2, root);

    match  (expected, inserted) {
      (@Empty, _)  => { fail!("Illogic"); }
      (_, @Empty)  => { fail!("Illogic"); }
      (@Node(expected_data, _, _),
       @Node(inserted_data, _, _)) => {
            assert!( expected_data == inserted_data);
      }
    }
}

#[test]
fn check_proper_inserts () {
    //Test that the structure is properly created
    let mut test_tree : @MaybeNode<int> = @Empty;

    // root
    test_tree = insert(@8, test_tree);

    assert!( test_tree == @Node(@8, @Empty, @Empty));

    // Right
    test_tree = insert(@11, test_tree);

    assert!( test_tree == @Node(@8,
                              @Empty,
                              @Node(@11, @Empty, @Empty)));

    // Left
    test_tree = insert(@2, test_tree);

    assert!( test_tree == @Node(@8,
                              @Node(@2, @Empty, @Empty),
                              @Node(@11, @Empty, @Empty)));
    //And a dupe test
    test_tree = insert(@2, test_tree);

    assert!( test_tree == @Node(@8,
                              @Node(@2, @Empty, @Empty),
                              @Node(@11, @Empty, @Empty)));

    // And recurse a bit on the left
    test_tree = insert(@3, test_tree);

    assert!( test_tree == @Node(@8,
                              @Node(@2,
                                    @Empty,
                                    @Node(@3, @Empty, @Empty)),
                              @Node(@11, @Empty, @Empty)));

}

#[test]
fn check_finding_top () {
    let test_tree = give_me_test_tree();
    assert!( find(@8, test_tree) == test_tree);
}
#[test]
fn check_finding_right () {
    let test_tree : @MaybeNode<int> = give_me_test_tree();
    match test_tree
    {
      @Node(_, _, right) =>
      {
        assert!( find(@11, test_tree) == right);
      }
      _ => {fail!(~"Error") }
    }
}

#[test]
fn check_finding_left () {
    let test_tree : @MaybeNode<int> = give_me_test_tree();
    match test_tree
    {
      @Node(_, left, _) =>
      {
        assert!( find(@2, test_tree) == left);
      }
      _ => {fail!(~"Error") }
    }
}
#[test]
fn check_finding_lower () {
    let test_tree = give_me_test_tree();
    match test_tree {
      @Node(_, left, _) => {
        match left {
          @Node(_, deeper_left, _) => { assert!( find(@-10, test_tree) == deeper_left) }
          _ => { fail!(~"Error") }
        }
      }
      _ => {fail!(~"Error") }
    }
}

#[test]
fn check_finding_not_there () {
    let test_tree = give_me_test_tree();
    assert!( find(@80, test_tree) == @Empty);
}



#[test]
fn delete_empty () {
    let test_tree : @MaybeNode<int> = @Empty;
    assert!( delete(@-1, test_tree) == @Empty);
}
#[test]
fn delete_root () {
    let mut root = @Empty;
    root = insert(@10, root);
    // sanity check
    assert!( root == @Node(@10, @Empty, @Empty));

    root = delete(@10, root);
    // check for correct deletion
    assert!( root == @Empty);
}
#[test]
fn delete_single_leaf_short () {

    let mut root = insert(@10, @Empty);
    root = insert(@30, root);

    let mut other = insert(@10, @Empty);
    other = insert(@30, other);
    other = insert(@40, other);

    other = delete(@40, other);

    // check for correct deletion
    assert!( root == other);
}
#[test]
fn delete_single_leaf_longer () {
    // Now check the ability to walk deeper
    let mut root = insert(@10, @Empty);
    root = insert(@20, root);
    root = insert(@25, root);
    root = insert(@30, root);

    let mut other = insert(@10, @Empty);
    other = insert(@10, other);
    other = insert(@20, other);
    other = insert(@25, other);

    root = delete(@30, root);

    // check for correct deletion
    assert!( root == other);

}
#[test]
fn delete_single_middle () {
    let mut root = @Empty;
    root = insert(@10, root);
    root = insert(@20, root);
    root = insert(@25, root);
    root = insert(@30, root);

    let mut other = insert(@10, @Empty);
    other = insert(@25, other);
    other = insert(@30, other);

    root = delete(@20, root);

    // check for correct deletion
    assert!( root == other);

    // now for a left insert check
    root = insert(@-10, @Empty);
    root = insert(@-20, root);
    root = insert(@-30, root);

    other = insert(@-10, @Empty);
    other = insert(@-30, other);

    root = delete(@-20, root);
    assert!( root == other);


    //check sides
    root = insert(@10, @Empty);
    root = insert(@20, root);
    root = insert(@-30, root);

    other = insert(@10, @Empty);
    other = insert(@-30, other);

    root = delete(@20, root);
    assert!( root == other);

}


#[test]
fn check_eq() {
    let test_tree : @MaybeNode<int> = @Empty;
    let test_tree2 : @MaybeNode<int> = @Empty;
    let test_tree3 = @Node(@-10, @Empty, @Empty);

    assert!( test_tree == test_tree2);
    assert!( test_tree3 == test_tree3);

    let mut test_tree4 = @Node(@-1, @Empty, @Empty);
    test_tree4 = insert(@10, test_tree4);
    test_tree4 = insert(@-10, test_tree4);
    test_tree4 = insert(@20, test_tree4);
    test_tree4 = insert(@40, test_tree4);
    test_tree4 = insert(@39, test_tree4);
    test_tree4 = insert(@42, test_tree4);

    let mut test_tree5 = @Node(@-1, @Empty, @Empty);
    test_tree5 = insert(@10, test_tree5);
    test_tree5 = insert(@-10, test_tree5);
    test_tree5 = insert(@20, test_tree5);
    test_tree5 = insert(@40, test_tree5);
    test_tree5 = insert(@39, test_tree5);
    test_tree5 = insert(@42, test_tree5);

    assert!( test_tree4 == test_tree4);
    assert!( test_tree4 == test_tree5);
}

#[test]
fn check_ne() {
    let test_tree : @MaybeNode<int> = @Empty;
    let test_tree3 = @Node(@-10, @Empty, @Empty);

    //commute the !-
    assert!( test_tree3 != test_tree);
    assert!( test_tree != test_tree3);

    let mut test_tree4 = @Node(@-1, @Empty, @Empty);
    test_tree4 = insert(@10, test_tree4);
    test_tree4 = insert(@-10, test_tree4);
    test_tree4 = insert(@20, test_tree4);
    test_tree4 = insert(@40, test_tree4);
    test_tree4 = insert(@39, test_tree4);
    test_tree4 = insert(@42, test_tree4);

    let mut test_tree5 = @Node(@-1, @Empty, @Empty);
    test_tree5 = insert(@10, test_tree5);
    test_tree5 = insert(@-10, test_tree5);
    test_tree5 = insert(@20, test_tree5);
    test_tree5 = insert(@40, test_tree5);
    test_tree5 = insert(@39, test_tree5);
    test_tree5 = insert(@42, test_tree5);
    test_tree5 = insert(@51, test_tree5);

    assert!( test_tree4 != test_tree3);
    assert!( test_tree4 != test_tree5);
}

#[test]
fn check_parental_ignoring () {
    assert! ( @Node(@8,
                 @Empty,
                 @Node(@11, @Empty, @Empty)) ==
            @Node(@8,
                  @Empty,
                  @Node(@11,
                        @Empty,
                        @Empty)));

}



#[test]
fn check_minimum() {
    let tree = insert(@20, @Empty);
    match node_data(minimum(tree)) {
      None => { fail!(~"Rong") }
      Some(data) => { assert!( data == @20) }
    }

    let mut tree = insert(@20, @Empty);
    tree = insert(@30, tree);
    tree = insert(@40, tree);
    match node_data(minimum(tree)) {
      None => { fail!(~"Rong") }
      Some(data) => { assert!( data == @20) }
    }
    // this setup segfaults!
    let mut tree = insert(@0, @Empty);
    tree = insert(@-10, tree);
    tree = insert(@10, tree);
    tree = insert(@11, tree);
    tree = insert(@5, tree);
    tree = insert(@1, tree);
    tree = insert(@-20, tree);
    tree = insert(@-15, tree);

    match node_data(minimum(tree)) {
      None => { fail!(~"Rong") }
      Some(data) => { assert!( data == @-20) }
    }

    tree = insert(@-21, tree);

    match node_data(minimum(tree)) {
      None => { fail!(~"Rong") }
  Some(data) => { assert!( data == @-21) }
}
    tree = insert(@-50, tree);
    tree = insert(@-40, tree);
    tree = insert(@-42, tree);
    match node_data(minimum(tree)) {
      None => { fail!(~"Rong") }
      Some(data) => { assert!( data == @-50) }
    }
}


#[test]
fn check_delete_empty () {
    // empty case
    let tree : @MaybeNode<int> = @Empty;
    assert!( delete(@20, tree) == @Empty);
}
#[test]
fn check_delete_root () {
    // just a root
    let tree : @MaybeNode<int> = insert(@20, @Empty);
    assert!( delete(@20, tree) == @Empty);
}

#[test]
fn check_delete_root_2_kids () {
    // root and 2 children, delete root
    let mut tree : @MaybeNode<int> = insert(@20, @Empty);
    tree = insert(@10, tree);
    tree = insert(@30, tree);
    assert!( node_data(delete(@20, tree)) == Some(@30));

    // construct a tree that should have the same shape
    let mut compare_tree = insert(@30, @Empty);
    compare_tree = insert(@10, compare_tree);
    assert!( delete(@20, tree) == compare_tree);
}
#[test]
fn check_delete_right_one () {
    // root and 2 children, delete right child
    let mut tree : @MaybeNode<int> = insert(@0, @Empty);
    tree = insert(@-10, tree);
    tree = insert(@10, tree);
    // pop off right child.
    tree = delete(@10, tree);

    let mut compare_tree = insert(@0, @Empty);
    compare_tree = insert(@-10, compare_tree);
    assert!( compare_tree == tree);
}

#[test]
fn check_delete_left_one () {
    // root and 2 children, delete left child
    let mut tree : @MaybeNode<int> = insert(@0, @Empty);
    tree = insert(@-10, tree);
    tree = insert(@10, tree);
    // pop off left child.
    tree = delete(@-10, tree);

    let mut compare_tree = insert(@0, @Empty);
    compare_tree = insert(@10, compare_tree);
    assert!( compare_tree == tree);
}
#[test]
fn check_delete_left_chain () {
    //root and a left child chain
    let mut tree : @MaybeNode<int> = insert(@0, @Empty);
    tree = insert(@-10, tree);
    tree = insert(@-20, tree);
    tree = insert(@-30, tree);

    tree = delete(@-30, tree);

    let mut compare_tree = insert(@0, @Empty);
    compare_tree = insert(@-10, compare_tree);
    compare_tree = insert(@-20, compare_tree);

    assert!( compare_tree == tree);
}

#[test]
fn check_delete_right_chain () {
    // root and a right child chain
    let mut tree : @MaybeNode<int> = insert(@0, @Empty);
    tree = insert(@10, tree);
    tree = insert(@20, tree);
    tree = insert(@30, tree);

    tree = delete(@30, tree);

    let mut compare_tree = insert(@0, @Empty);
    compare_tree = insert(@10, compare_tree);
    compare_tree = insert(@20, compare_tree);

    assert!( compare_tree == tree);
}

#[test]
fn check_delete_double () {
    //check deleting a node IN THE MIDDLE
    let mut tree : @MaybeNode<int> = insert(@0, @Empty);
    tree = insert(@10, tree);
    tree = insert(@20, tree);
    tree = insert(@15, tree);


    tree = delete(@10, tree);

    let mut compare_tree = insert(@0, @Empty);
    compare_tree = insert(@20, compare_tree);
    compare_tree = insert(@15, compare_tree);

    assert!( compare_tree == tree);
}
*/
