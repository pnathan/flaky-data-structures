// tree.rs.
// Happy hacking.

extern mod std;
use core::cmp::{Eq, Ord};
use core::rand::{random};

/*
Initial exploration: Pure functional data structures
*/
// A binary tree node.
enum MaybeNode<T>
{
    Empty,
    Node(@T, @MaybeNode<T>,@MaybeNode<T>)
}


impl<T: Ord Eq> MaybeNode<T>: Eq {

    pure fn eq(other: &MaybeNode<T>)-> bool {
        match (self, other) {
          ( Empty, &Empty ) => { true }
          ( Empty, _ )  => { false }
          ( Node(selfdata, selfLeft, selfRight), &Empty ) => { false }
          ( Node(selfdata, selfLeft, selfRight),
           &Node(otherdata, otherLeft, otherRight) ) => {
            if ( selfdata == otherdata ) {
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

#[test]
fn test_eq()
{
    let test_tree : @MaybeNode<int> = @Empty;
    let test_tree2 : @MaybeNode<int> = @Empty;
    let test_tree3 = @Node(@-10, @Empty, @Empty);

    assert test_tree == test_tree2;
    assert test_tree3 == test_tree3;

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

    assert test_tree4 == test_tree4;
    assert test_tree4 == test_tree5;
}

#[test]
fn test_ne()
{
    let test_tree : @MaybeNode<int> = @Empty;
    let test_tree3 = @Node(@-10, @Empty, @Empty);

    //commute the !-
    assert test_tree3 != test_tree;
    assert test_tree != test_tree3;

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

    assert test_tree4 != test_tree3;
    assert test_tree4 != test_tree5;
}

// Pure functional delete
fn delete<T: Eq Ord> (data: @T, root: @MaybeNode<T>) -> @MaybeNode<T> {
    match root {
      @Empty => { @Empty }
      @Node(test_data, left, right) => {
        if (data == test_data) {
            // No children
            if (left == @Empty && right == @Empty) {
                @Empty
            }
            //One child
            else if (left == @Empty || right == @Empty) {
                //Return one or the other
                if (left != @Empty) { left }
                else if (right != @Empty) { right }
                else { fail(~"Logic bomb; No one should be here"); }
            }
            //Two children.
            else {
                //stubbed in
                return @Empty
            }
        }
        else if ( test_data < data) { @Node(test_data, delete(data, left), right) }
        else if ( test_data > data) { @Node(test_data, left, delete(data, right)) }
        else { fail(~"A number became neither <, >, or ="); }
      }
    }
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
    assert root == @Node(@10, @Empty, @Empty);

    root = delete(@10, root);
    // check for correct deletion
    assert root == @Empty;
}
#[test]
fn delete_single_leaf () {
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
/*
fn insert_vec<T: Ord> (data: ~[T],  root: @MaybeNode<T>) -> @MaybeNode<T>
{
    let mut newroot : @MaybeNode<T> = root;
    let mut idx = 0;

    while idx < data.len() {
        newroot = insert(@data[idx], newroot);
        idx += 1;
    }

    return newroot

}*/

fn insert<T: Ord> (newdata: @T, root: @MaybeNode<T>) -> @MaybeNode<T> {
    @match root {
      @Empty => {
        Node(newdata, @Empty, @Empty)
      }
      @Node(data, left, right) => {
        if ( data < newdata ) {
            Node(data, insert(newdata, left), right)
        }
        else {
            Node(data, left, insert(newdata, right))
        }
      }
    }
}

fn find<T: Eq Ord> (newdata: @T, root: @MaybeNode<T>) -> bool {
    @match root {
      @Empty => {
        return false;
      }
      @Node(data, left, right) => {
        if (data == newdata)
        {
            return true;
        }
        else if ( data < newdata ) {
            return find(newdata, left);
        }
        else {
            return find(newdata, right);
        }
      }
    }
}

fn give_me_test_tree () -> @MaybeNode<int> {
    let test_tree : @MaybeNode<int> =
              @Node(@8,
                    @Node(@11,
                          @Empty,
                          @Empty),
                    @Node(@2,
                          @Node(@3,
                                @Empty,
                                @Node(@3, @Empty, @Empty)),
                          @Empty));

    return test_tree;
}

#[test]
fn check_finding_top () {
    let test_tree = give_me_test_tree();
    assert find(@8, test_tree) == true;
}
#[test]
fn check_finding_right () {
    let test_tree = give_me_test_tree();
    assert find(@11, test_tree) == true;
}
#[test]
fn check_finding_left () {
    let test_tree = give_me_test_tree();
    assert find(@2, test_tree) == true;
}
#[test]
fn check_finding_lower () {
    let test_tree = give_me_test_tree();
    assert find(@11, test_tree) == true;
}
#[test]
fn check_finding_not_there () {
    let test_tree = give_me_test_tree();
    assert find(@80, test_tree) == false;
}

#[test]
fn check_add_one () {
    let root : @MaybeNode<int> = @Empty;

    let expected : @MaybeNode<int> = @Node(@2, @Empty, @Empty);

    let inserted = insert(@2, root);

    match  expected {
      @Empty => { fail; }
      @Node(expected_data, _, _) => {
        match inserted {
          @Empty => { fail;}
          @Node(inserted_data, _, _) => {
            assert expected_data == inserted_data
          }
        }
      }
    }
}

fn main() {
    let mut root : @MaybeNode<uint> = @Empty;

    let mut counter = 10;
    while counter > 0 {

        let rint = random() % 20;
        root = insert(@rint, root);

        io::println(fmt!("Inserted %? ", rint));
        io::println(fmt!("%?", root));

        counter -= 1;
    }

}