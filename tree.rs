// tree.rs.
// Happy hacking.

extern mod std;
use core::cmp::{Eq, Ord};
use core::rand::{random};
use core::option;
/*

pcwalton is looking for a doubly-linked tree

parent; first child; last child; next sibling; prev sibling

operations are: add as first child; add as last child; insert before some child; remove from tree
*/
struct Node<T> {
    data: T,
    parent: Option<@Node<T>>,
    first_child: Option<@Node<T>>,
    last_child: Option<@Node<T>>,
    next_sibling: Option<@Node<T>>,
    prev_sibling: Option<@Node<T>>
}

trait DoublyLinkedNode<T> {
    fn push_first_child<T>(@Node<T>) -> @Node<T>;
}

/*
Initial exploration: Pure functional data structures
*/
// A binary tree node.
enum MaybeNode<T>
{
    Empty,
    // Data, parent, left, right
    Node(@T, @MaybeNode<T>,@MaybeNode<T>, @MaybeNode<T>)
}


impl<T: Ord Eq> MaybeNode<T>: Eq {

    pure fn eq(other: &MaybeNode<T>)-> bool {
        match (self, other) {
          ( Empty, &Empty ) => { true }
          ( Empty, _ )  => { false }
          ( Node(_, _, _, _), &Empty ) => { false }
          ( Node(selfdata, _, selfLeft, selfRight),
           &Node(otherdata, _, otherLeft, otherRight) ) => {

            // We don't check parent equality. If we did, we'd have to
            // recurse into ourselves. Ungood.

            if (selfdata == otherdata) {

                // Note that this kicks out the equality question to
                // T's specification. If that happens to be a
                // MaybeNode, we just recurse and have done with
                // it. :-)
                return (true
                        && (selfLeft == otherLeft)
                        && (selfRight == otherRight))
            }
            else { false }
          }
        }
    }

    pure fn ne(other: &MaybeNode<T>)-> bool {
        ! self.eq(other)
    }
}

fn node_data<T> (node: @MaybeNode<T>) -> Option<@T> {
    match node {
      @Empty => {None}
      @Node(data, _, _, _) => { Some(data) }
    }
}

#[test]
fn check_minimum() {
    let mut tree = insert(@20, @Empty);
    match node_data(minimum(tree)) {
      None => { fail(~"Rong") }
      Some(data) => { assert data == @20 }
    }

    let mut tree = insert(@20, @Empty);
    tree = insert(@30, tree);
    tree = insert(@40, tree);
    match node_data(minimum(tree)) {
      None => { fail(~"Rong") }
      Some(data) => { assert data == @20 }
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
      None => { fail(~"Rong") }
      Some(data) => { assert data == @-20 }
    }

    tree = insert(@-21, tree);

    match node_data(minimum(tree)) {
      None => { fail(~"Rong") }
  Some(data) => { assert data == @-21 }
}
    tree = insert(@-50, tree);
    tree = insert(@-40, tree);
    tree = insert(@-42, tree);
    match node_data(minimum(tree)) {
      None => { fail(~"Rong") }
      Some(data) => { assert data == @-50 }
    }
}


// Pure functional delete
fn delete<T: Eq Ord> (data_to_delete: @T, node: @MaybeNode<T>) -> @MaybeNode<T> {
    //io::println(fmt!("%?\n", node));
    match node {
      @Empty => { @Empty }
      @Node(node_data, parent, left, right) => {
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
                    else { fail(~"Error: invalid"); };

                //rewrite the parent slot
                match child_node {
                  //blank is the parent that's been deleted
                  @Node(child_data, _, left, right) => {
                    @Node(child_data, parent, left, right)
                  }
                  @Empty => { @Empty }
                }
            }
            //Two children.
            else {
                //stubbed in
                fail(~"Unimplemented. Please send hate mail to /dev/null");
                //return @Empty
            }
        }
        else if ( node_data > data_to_delete) {
            //io::println(fmt!("left\n"));
            @Node(node_data, parent, delete(data_to_delete, left), right) }
        else if ( node_data < data_to_delete) {
            //io::println(fmt!("right\n"));
            @Node(node_data, parent, left, delete(data_to_delete, right)) }
        else { fail(~"A number became neither <, >, or ="); }
      }
    }
}

fn insert<T: Eq Ord> (newdata: @T,
                      node: @MaybeNode<T>) -> @MaybeNode<T> {
    // workaround to use @empty as the default argument
    insert_under(newdata, node, @Empty)
}

fn insert_under<T: Eq Ord> (newdata: @T,
                      node: @MaybeNode<T>,
                      node_parent: @MaybeNode<T>) -> @MaybeNode<T> {
    let result = @match node {
      @Empty => {
        Node(newdata, node_parent, @Empty, @Empty)
      }
      @Node(data, parent, left, right) => {
        if (data == newdata) {
            // a bst doesn't have duplicates
            Node(data, parent, left, right)
        }
        else if ( data > newdata ) {
            Node(data, parent, insert_under(newdata, left, node), right)
        }
        else {
            Node(data, parent, left, insert_under(newdata, right, node))
        }
      }
    };

    return result
}


fn find<T: Eq Ord> (newdata: @T, root: @MaybeNode<T>) -> @MaybeNode<T> {
    @match root {
      @Empty => {
        return @Empty;
      }
      @Node(data, _, left, right) => {
        if (data == newdata)       { return root }
        else if ( data > newdata ) { return find(newdata, left) }
        else                       { return find(newdata, right) }
      }
    }
}

fn minimum<T>(node: @MaybeNode<T>) -> @MaybeNode<T> {
    match node {
      @Node(_, _, left, _) => {
        // walk down one level
        match left {
          @Empty => { node }
          _ => { minimum(left) }
        }
      }
      @Empty => { @Empty }
    }
}


fn maximum<T>(node: @MaybeNode<T>) -> @MaybeNode<T> {
    match node {
      @Node(_, _, _, right) => {
        maximum(right)
      }
      @Empty => { node }
    }
}
fn find_successor<T: Eq> (node: @MaybeNode<T>) -> @MaybeNode<T> {
    //a node's in-order successor is the left-most child of its right subtree,
    let _ = node;               //shut up the errors.
    match node {
      @Node(_, _, left, _) => {
        minimum(left)
      }
      @Empty => { @Empty }
    }
}


fn main() {

    //let mut root = insert(@10, @Empty);
    //root = insert(@30, root);

    let mut other = insert(@10, @Empty);
    other = insert(@30, other);
    other = insert(@40, other);

    delete(@40, other);

//    io::println(fmt!("%?", other));
//    io::println(fmt!("%?", root));

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
fn test_successor_empty () {
    let tree : @MaybeNode<int> = @Empty;

    let newtree = find_successor(tree);

    assert newtree == @Empty
}

#[test]
fn test_successor_one () {
    let mut tree : @MaybeNode<int> = @Empty;

    tree = insert(@20, tree);
    tree = insert(@30, tree);

    let newtree = find_successor(tree);

    assert newtree == @Node(@30, @Empty, @Empty, @Empty)
}

#[test]
fn test_successor_double () {
    let mut tree : @MaybeNode<int> = @Empty;

    tree = insert(@20, tree);
    tree = insert(@30, tree);
    tree = insert(@-10, tree);

    let newtree = find_successor(tree);

    assert newtree == @Node(@30, @Empty, @Empty, @Empty)
}

#[test]
fn test_successor_longer () {
    let mut tree : @MaybeNode<int> = @Empty;

    tree = insert(@20, tree);
    tree = insert(@30, tree);
    tree = insert(@25, tree);

    let newtree = find_successor(tree);

    assert newtree == @Node(@25, @Empty, @Empty, @Empty)
}


#[test]
fn check_add_one () {
    let root : @MaybeNode<int> = @Empty;

    let expected : @MaybeNode<int> = @Node(@2, @Empty, @Empty, @Empty);

    let inserted = insert(@2, root);

    match  (expected, inserted) {
      (@Empty, _)  => { fail; }
      (_, @Empty)  => { fail; }
      (@Node(expected_data, _, _, _),
       @Node(inserted_data, _, _, _)) => {
            assert expected_data == inserted_data
      }
    }
}

#[test]
fn check_proper_inserts () {
    //Test that the structure is properly created
    let mut test_tree : @MaybeNode<int> = @Empty;

    // root
    test_tree = insert(@8, test_tree);

    assert test_tree == @Node(@8, @Empty, @Empty, @Empty);

    // Right
    test_tree = insert(@11, test_tree);

    assert test_tree == @Node(@8,
                              @Node(@8, @Empty, @Empty, @Empty),
                              @Empty,
                              @Node(@11, @Empty, @Empty, @Empty));

    // Left
    test_tree = insert(@2, test_tree);

    assert test_tree == @Node(@8,
                              @Empty,
                              @Node(@2, @Empty, @Empty, @Empty),
                              @Node(@11, @Empty, @Empty, @Empty));
    //And a dupe test
    test_tree = insert(@2, test_tree);

    assert test_tree == @Node(@8,
                              @Empty,
                              @Node(@2, @Empty, @Empty, @Empty),
                              @Node(@11, @Empty, @Empty, @Empty));

    // And recurse a bit on the left
    test_tree = insert(@3, test_tree);

    assert test_tree == @Node(@8,
                              @Empty,
                              @Node(@2, @Empty, @Empty,
                                    @Node(@3, @Empty, @Empty, @Empty)),
                              @Node(@11, @Empty, @Empty, @Empty));

}

#[test]
fn check_finding_top () {
    let test_tree = give_me_test_tree();
    assert find(@8, test_tree) == test_tree;
}
#[test]
fn check_finding_right () {
    let test_tree : @MaybeNode<int> = give_me_test_tree();
    match test_tree
    {
      @Node(_, _, _, right) =>
      {
        assert find(@11, test_tree) == right
      }
      _ => {fail(~"Error") }
    }
}

#[test]
fn check_finding_left () {
    let test_tree : @MaybeNode<int> = give_me_test_tree();
    match test_tree
    {
      @Node(_, _, left, _) =>
      {
        assert find(@2, test_tree) == left
      }
      _ => {fail(~"Error") }
    }
}
#[test]
fn check_finding_lower () {
    let test_tree = give_me_test_tree();
    match test_tree {
      @Node(_, _, left, _) => {
        match left {
          @Node(_, _, deeper_left, _) => { assert find(@-10, test_tree) == deeper_left }
          _ => { fail(~"Error") }
        }
      }
      _ => {fail(~"Error") }
    }
}

#[test]
fn check_finding_not_there () {
    let test_tree = give_me_test_tree();
    assert find(@80, test_tree) == @Empty;
}



#[test]
fn delete_empty () {
    let test_tree : @MaybeNode<int> = @Empty;
    assert delete(@-1, test_tree) == @Empty;
}
#[test]
fn delete_root () {
    let mut root = @Empty;
    root = insert(@10, root);
    // sanity check
    assert root == @Node(@10, @Empty, @Empty, @Empty);

    root = delete(@10, root);
    // check for correct deletion
    assert root == @Empty;
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
    assert root == other;
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
    assert root == other;

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
    assert root == other;

    // now for a left insert check
    root = insert(@-10, @Empty);
    root = insert(@-20, root);
    root = insert(@-30, root);

    other = insert(@-10, @Empty);
    other = insert(@-30, other);

    root = delete(@-20, root);
    assert root == other;


    //check sides
    root = insert(@10, @Empty);
    root = insert(@20, root);
    root = insert(@-30, root);

    other = insert(@10, @Empty);
    other = insert(@-30, other);

    root = delete(@20, root);
    assert root == other;

}


#[test]
fn check_eq() {
    let test_tree : @MaybeNode<int> = @Empty;
    let test_tree2 : @MaybeNode<int> = @Empty;
    let test_tree3 = @Node(@-10, @Empty, @Empty, @Empty);

    assert test_tree == test_tree2;
    assert test_tree3 == test_tree3;

    let mut test_tree4 = @Node(@-1, @Empty, @Empty, @Empty);
    test_tree4 = insert(@10, test_tree4);
    test_tree4 = insert(@-10, test_tree4);
    test_tree4 = insert(@20, test_tree4);
    test_tree4 = insert(@40, test_tree4);
    test_tree4 = insert(@39, test_tree4);
    test_tree4 = insert(@42, test_tree4);

    let mut test_tree5 = @Node(@-1, @Empty, @Empty, @Empty);
    test_tree5 = insert(@10, test_tree5);
    test_tree5 = insert(@-10, test_tree5);
    test_tree5 = insert(@20, test_tree5);
    test_tree5 = insert(@40, test_tree5);
    test_tree5 = insert(@39, test_tree5);
    test_tree5 = insert(@42, test_tree5);

    assert test_tree4 == test_tree4;
    assert test_tree4 == test_tree5;
}

#[test]
fn check_ne() {
    let test_tree : @MaybeNode<int> = @Empty;
    let test_tree3 = @Node(@-10, @Empty, @Empty, @Empty);

    //commute the !-
    assert test_tree3 != test_tree;
    assert test_tree != test_tree3;

    let mut test_tree4 = @Node(@-1, @Empty, @Empty, @Empty);
    test_tree4 = insert(@10, test_tree4);
    test_tree4 = insert(@-10, test_tree4);
    test_tree4 = insert(@20, test_tree4);
    test_tree4 = insert(@40, test_tree4);
    test_tree4 = insert(@39, test_tree4);
    test_tree4 = insert(@42, test_tree4);

    let mut test_tree5 = @Node(@-1, @Empty, @Empty, @Empty);
    test_tree5 = insert(@10, test_tree5);
    test_tree5 = insert(@-10, test_tree5);
    test_tree5 = insert(@20, test_tree5);
    test_tree5 = insert(@40, test_tree5);
    test_tree5 = insert(@39, test_tree5);
    test_tree5 = insert(@42, test_tree5);
    test_tree5 = insert(@51, test_tree5);

    assert test_tree4 != test_tree3;
    assert test_tree4 != test_tree5;
}

#[test]
fn check_parental_ignoring () {
    assert ( @Node(@8,
                 @Node(@8, @Empty, @Empty, @Empty),
                 @Empty,
                 @Node(@11, @Empty, @Empty, @Empty)) ==
            @Node(@8,
                  @Empty,
                  @Empty,
                  @Node(@11,
                        @Node(@8, @Empty, @Empty, @Empty),
                        @Empty,
                        @Empty)))

}
