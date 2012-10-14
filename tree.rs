// tree.rs.
// Happy hacking.
//use std::test;
extern mod std;
use core::cmp::{Eq, Ord};
use core::rand::{random};

// A binary tree node
enum MaybeNode<T>
{
    Empty,
    Node(@T, @MaybeNode<T>,@MaybeNode<T>)
}


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