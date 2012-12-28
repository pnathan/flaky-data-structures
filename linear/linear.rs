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

extern mod std;
use core::option;
use core::cmp::{Eq, Ord};

// General container trait.
trait Container<T> {
    // size of the container
    fn size(cont: @self) -> u64;
    // delete the data from cont and return a "new" self. Old self
    // unaffected.
    fn delete(&self, data: @T) -> @self;
}

// A List; classic abstract datatype for a List
trait List<T> {
    // first element
    fn first(&self) -> Option<@T>;
    // non-first elements
    fn rest(&self) -> @self;
    // length of the list
    fn length(&self) -> u64;
    // index into the list
    fn elt(&self, idx: u64) -> Option<@T>;
    //append other to self and return the new list, original lists
    //unmodified
    fn append(&self, @self) -> @self;
}

// A queue, which also is a kind of list
trait Queue<T> : List<T>{
    // check the first element
    fn peek(seq: @self) -> Option<@T>;
    // get the first element and return the new list without the first element
    fn pop(seq: @self) -> (Option<@T>, @self);
}


/*
My implementation of a List, concretized into an enum.
*/
#[deriving_eq]
pub enum List_Data<T> {
    Nil,
    Cons(@T, @List_Data<T>)
}

// cons up an element to a list; this provides a functional (non foo.cons() )
// interface.
pub fn cons<T> (data: @T, seq : @List_Data<T>) -> @List_Data<T> {
    @Cons(data, seq)
}

// Convert from a vector to a linked list
pub fn from_vec<T: Copy>(v: &[T]) -> @List_Data<T> {
    let mut accum = @Nil::<T>;
    let len = v.len();
    let mut i : uint = len - 1;

    while (i < len)  {
        accum = @Cons(@v[i], accum);
        i -= 1;
    }
    return accum
}

impl<T> List_Data<T>: List<T> {

    fn first(&self) -> Option<@T> {
        match (self)  {
          &Nil => {
            None
          }
          &Cons(e, _) => {
            Some(e)
          }
        }
    }

    fn rest(&self) -> @List_Data<T> {
        match(self) {
          &Nil => {
            @Nil
          }
          &Cons(_, rest) => {
            rest
          }
        }
    }

    fn append(&self, other: @List_Data<T>) -> @List_Data<T> {
        match (self) {
          &Nil => {
            other
          }
          &Cons(e, rest) => {
            cons(e, rest.append(other))
          }
        }
    }

    fn elt(&self, idx: u64) -> Option<@T> {
        let mut counter : u64 = 0;
        let mut temp : @List_Data<T> = @*self;

        let mut retval = None;
        while counter <= idx {
            match(temp) {
              @Nil => {
                break;
              }
              @Cons(e, rest) => {
                if (counter == idx) {
                    return Some(e)
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

    fn length(&self) -> u64 {
        match (self) {
          &Nil => {
            0
          }
          &Cons(_, rest) => {
            rest.length() + 1
          }
        }
    }
}

// A list implements the container traits
impl<T : Eq> List_Data<T>: Container<T> {
    fn size (seq: @List_Data<T>) -> u64 {
        seq.length()
    }
    fn delete (&self, data: @T) -> @List_Data<T> {
        match(self) {
          &Nil => {
            @Nil
          }
          &Cons(e, rest) => {
            if (e == data) {
                rest
            }
            else {
                cons(e, rest.delete(data))
            }

          }
        }
    }
}

// And it also implements the queue traits
impl<T> List_Data<T>: Queue<T> {

    fn peek(list : @List_Data<T>) -> Option<@T> {
        list.first()
    }

    // Returns the first element, along with a list that doesn't have
    // the first element.
    fn pop(list : @List_Data<T>) -> (Option<@T>, @List_Data<T>) {
        let start = list.first();
        let others = list.rest();
        return (start, others)
    }
}
/*

pub fn delete<T>(list: @List<T>, idx: u64) -> @List<T> {
    delete_direct(list, idx, 0)
}
fn delete_direct<T>(list: @List<T>, idx: u64, counter: u64) -> @List<T> {
    match(list) {
      @Nil => {@Nil}
      @Cons(e, rest) => {
        if (counter == idx) {
            rest
        }
        else {
            cons(e, delete_direct(rest, idx, counter + 1))
        }
      }
    }
}

//////////////////////////////////////////////////////////////////////
// Tests for the normal list
*/
#[test]
fn check_cons() {
    let nilly : @List_Data<u64> = @Nil;
    let list : @List_Data<u64> = @Cons(@10, @Nil::<u64>);
    let otherlist : @List_Data<u64> = cons(@10, nilly);
    assert list == otherlist;
}

#[test]
fn check_append() {
    let three_long : @List_Data<u64> = from_vec([10, 20, 30]);
    let three_long2 : @List_Data<u64> = from_vec([40, 50, 60]);

    assert three_long.append(three_long2) == from_vec([10, 20,30,40,50,60])
}

#[test]
fn check_delete() {
    let three_long = cons(@10, cons(@20, cons(@30, @Nil)));

    assert three_long.delete(@10) == cons(@20, cons(@30, @Nil));

    // verify non-mutating
    assert three_long == cons(@10, cons(@20, cons(@30, @Nil)));
}

#[test]
fn check_first() {
    let three_long : @List_Data<u64> = from_vec([10, 20, 30]);
    assert three_long.first() == Some(@10)

}
#[test]
fn check_len() {
    let three_long : @List_Data<u64> = from_vec([10, 20, 30]);
    let mut len : u64 = three_long.length();

    assert len == 3;

    let list : @List_Data<u64> = @Nil::<u64>;
    len = list.length();

    assert len == 0;
}
#[test]
fn check_elt() {
    let three_long : @List_Data<u64> = from_vec([10, 20, 30]);

    assert three_long.elt(0) == Some(@10);
    assert three_long.elt(1) == Some(@20);
    assert three_long.elt(2) == Some(@30);

    assert three_long.elt(-1) == None;
    assert three_long.elt(4) == None;
}
