/*
 Linear.rs
 License: AGPL3

List, stack, queue, deque.

Recall that stack is FILO, queue is FIFO, and dequeue is both stack
and queue.

When terminology is chosen, it is chosen after the notation of the
Common Lisp standard:  http://www.lispworks.com/reference/HyperSpec/

Generally, the style adopted for iteration is tail recursion. This is
largely because I (pnathan) got stuck on a typing error and chose to
make the code work rather then investing the time to fix the error. It
also plays well with the pure functional approach.

*/


//extern mod extra;
use std::rc;
//use std::gc;
//use extra::arc::Arc;

#[deriving(PartialEq,Clone)]
pub enum List<T> {
    Nil,
    Cons(T, rc::Rc<List<T>>)
}

pub fn cons<T: Clone> (data: T, seq : rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    rc::Rc::new(Cons(data, seq))
}

// A concise way ot writing nil
pub fn nil<T>() -> rc::Rc<List<T>> {
    rc::Rc::new(Nil::<T>)
}

// Convert from a vector to a linked list
pub fn from_vec<T: Clone>(v: &[T]) -> rc::Rc<List<T>> {
    let mut accum = nil();
    let len : uint = v.len();
    let mut i  = len - 1;

    while (i < len) {
        accum = cons((v[i].clone()), accum);
        i -= 1;
    }
    return accum
}

pub fn first<T: Clone>(list : &rc::Rc<List<T>>) -> Option<T> {
    let thing = list.deref();
    match (thing) {
        &Nil => {
            None
        }
        &Cons(ref e, _) => {
            Some(e.clone())
        }
    }
}

pub fn rest<T>(list : &rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    match(list.deref()) {
        &Nil => {
            nil()
        }
        &Cons(_, ref rest) => {
            rest.clone()
        }
    }
}

pub fn append<T: Clone>(list : &rc::Rc<List<T>>,
             other: &rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    match (list.deref()) {
        &Nil => {
            other.clone()
        }
        &Cons(ref e, ref rest) => {
            cons(e.clone(), append(rest, other))
        }
    }
}


pub fn elt<T : Clone>(list : &rc::Rc<List<T>>, idx: u64) -> Option<T> {
    let mut counter : u64 = 0;

    // typical temp node in a list walk
    let mut temp : &rc::Rc<List<T>> = list;

    let retval = None;
    while counter <= idx {
        match(temp.deref()) {
            &Nil => {
                break;
            }
            &Cons(ref e, ref rest) => {
                if (counter == idx) {
                    return Some(e.clone())
                }
                else {
                    temp = rest;
                }
            }
        }
        counter += 1;
    }
    return retval
}

pub fn length<T>(list :&rc::Rc<List<T>>) -> u64 {
    match (list.deref()) {
        &Nil => {
            0
        }
        &Cons(_, ref rest) => {
            length(rest) + 1
        }
    }
}

pub fn peek<T : Clone>(list : &rc::Rc<List<T>>) -> Option<T> {
    first(list)
}

// Returns the first element, along with a list that doesn't have
// the first element.
pub fn pop<T: Clone >(list : &rc::Rc<List<T>>) -> (Option<T>, rc::Rc<List<T>>) {
    let start = first(list);
    let others = rest(list);
    return (start, others)
}

// Returns the new list with the new element
pub fn push<T: Clone>(value : T, list : &rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    cons(value, list.clone())
}

// Push onto the end of the list
pub fn push_back<T:Clone>(value : T, list : &rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    match(list.deref()) {
        &Nil => {
            return cons(value, nil())
        }
        &Cons(ref e, ref rest) => {
            return cons(e.clone(), push_back(value, rest))
        }
    }
}

pub fn enqueue<T:Clone>(value : T, list : &rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    push_back(value, list)
}

pub fn delete<T: Eq + Clone> (data : T,
                                   list : &rc::Rc<List<T>>) -> rc::Rc<List<T>> {
    match(list.deref()) {
        &Nil => {
            nil()
        }
        &Cons(ref head, ref rest) => {
            if (*head == data) {
                rest.clone()
            }
            else {
                cons(head.clone(),
                     delete(data, rest))
            }
        }
    }
}

pub fn delete_at<T: Eq + Clone>(list: &rc::Rc<List<T>>, idx: u64) -> rc::Rc<List<T>> {
    delete_at_under(list,idx,0)
}

fn delete_at_under<T: Eq + Clone>(list: &rc::Rc<List<T>>, idx: u64, counter: u64) -> rc::Rc<List<T>> {
    match(list.deref()) {
      &Nil => {
            nil()
      }
      &Cons(ref head, ref rest) => {
        if (counter == idx) {
            rest.clone()
        }
        else {
            cons(head.clone(),
                 delete_at_under(rest, idx, counter + 1))
        }
      }
    }
}



//////////////////////////////////////////////////////////////////////
/// Tests

#[test]
fn check_cons() {
    let nils : rc::Rc<List<int>> = rc::Rc::new(Nil::<int>);

    let list : rc::Rc<List<int>> = rc::Rc::new(Cons(10, rc::Rc::new(Nil::<int>)));

    let otherlist : rc::Rc<List<int>> = cons(10, nils);

    assert!( list == otherlist);
}

#[test]
fn check_from_vec_empty() {
    let empty_list : rc::Rc<List<int>> = rc::Rc::new(Nil::<int>);
    let empty_vector = [];


    assert!(empty_list == from_vec(empty_vector));
}

#[test]
fn check_from_vec_one() {
    let one_list = cons(10, rc::Rc::new(Nil::<int>));
    let one_vector = [10];


    assert!(one_list == from_vec(one_vector));
}

#[test]
fn check_from_vec_some() {
    let some_list = cons(10,
                         cons(20,
                              cons(30,
                                   rc::Rc::new(Nil::<int>))));
    let some_vector = [10i,20,30];


    assert!(some_list == from_vec(some_vector));
}

#[test]
fn check_first() {
    let three_long : rc::Rc<List<int>> = from_vec([10i, 20, 30]);
    let first_thing = first(&three_long);
    assert!( first_thing == Some(10));

}

#[test]
fn check_rest() {
    let three_long : rc::Rc<List<int>> = from_vec([10i, 20, 30]);
    let two_long : rc::Rc<List<int>> = from_vec([20i, 30]);

    let others = rest(&three_long);

    assert!( others == two_long);

}

#[test]
fn check_append() {
    let three_long : rc::Rc<List<int>> = from_vec([10i, 20, 30]);
    let three_long2 : rc::Rc<List<int>> = from_vec([40i, 50, 60]);

    assert!( append(&three_long, &three_long2) == from_vec([10i, 20,30,40,50,60]));
}

#[test]
fn check_length() {
    let three_long = from_vec([10i, 20, 30]);
    let mut len : u64 = length(&three_long);

    assert!( len == 3);

    let list : rc::Rc<List<int>>  = nil();
    len = length(&list);

    assert!( len == 0);
}

#[test]
fn check_elt() {
    let three_long : rc::Rc<List<int>> = from_vec([10, 20, 30]);

    assert!( elt(&three_long, 0) == Some(10));
    assert!( elt(&three_long, 1) == Some(20));
    assert!( elt(&three_long, 2) == Some(30));

    assert!( elt(&three_long, -1) == None);
    assert!( elt(&three_long, 4) == None);
}

#[test]
fn check_delete() {
    let three_long : rc::Rc<List<int>> = from_vec([10, 20, 30]);
    let two_long : rc::Rc<List<int>> = from_vec([20, 30]);

    assert!( delete(10, &three_long) == two_long);
}

#[test]
fn check_peek() {
    let q : rc::Rc<List<&str>> = nil();
    assert!(peek(&q) == None);

    let q : rc::Rc<List<&str>> = from_vec(["a"]);
    assert!(peek(&q) == Some("a"));

    let q : rc::Rc<List<&str>> = from_vec(["a", "b"]);
    assert!(peek(&q) == Some("a"));

}

#[test]
fn check_pop() {
    let q : rc::Rc<List<&str>> = nil();
    assert!(pop(&q) == (None, nil()));

    let q : rc::Rc<List<&str>> = from_vec(["a"]);
    assert!(pop(&q) == (Some("a"), nil()));

    let q : rc::Rc<List<&str>> = from_vec(["a", "b"]);
    assert!(pop(&q) == (Some("a"), cons("b", nil())));

}

#[test]
fn check_push_back() {
    let q : rc::Rc<List<&str>> = nil();

    let q = push_back("a", &q);
    assert!(peek(&q) == Some("a"));

    let q = push_back("b", &q);
    assert!(peek(&q) == Some("a"));

    let (_, q) = pop(&q);
    assert!(peek(&q) == Some("b"));
}

#[test]
fn check_push() {
    let q : rc::Rc<List<&str>> = nil();

    let q = push("a", &q);
    assert!(peek(&q) == Some("a"));

    let q = push("b", &q);
    assert!(peek(&q) == Some("b"));

    let (_, q) = pop(&q);
    assert!(peek(&q) == Some("a"));
}
