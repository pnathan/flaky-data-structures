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

// General container trait.
trait Container<T> {
    fn size<T> (cont: Container<T>) -> u64;
}

// A queue trait
trait Queue<T> {
    fn length<T> (seq: @Container<T>) -> u64;
    fn peek<T> (seq: @Container<T>) -> Option<@T>;
    fn pop<T> (seq: @Container<T>) -> (Option<@T>, @Container<T>);
}


#[deriving_eq]
pub enum List<T> {
    Nil,
    Cons(@T, @List<T>)
}


// A list implements the container traits
impl<T> List<T>: Container<T> {
    fn size<T> (cont: @List<T>) -> u64 {
        return length(cont)
    }
}
// And it also implements the queue traits
impl<T> List<T>: Queue<T> {
    fn length<T>(list: @List<T>) -> u64 {
        0
    }

    fn peek<T>(list : @List<T>) -> Option<@T> {
        head(list)
    }

    fn pop<T>(list : @List<T>) -> (Option<@T>, @List<@T>) {
        (None, @Nil)
    }
}


pub fn head<T>(list: @List<T>) -> Option<@T> {
    // Unsure about rustc's optimization. The alternative would be:
    // elt(list, 0)

    match (list)  {
      @Nil => {
        None
      }
      @Cons(e, _) => {
        Some(e)
      }
    }
}

pub fn rest<T>(list: @List<T>) -> @List<T> {
    match (list)  {
      @Nil => {
        @Nil
      }
      @Cons(_, rest) => {
        rest
      }
    }
}

pub fn cons<T>(e: @T, list: @List<T>) -> @List<T> {
    @Cons(e, list)
}

pub fn append<T>(list1: @List<T>, list2: @List<T>) -> @List<T>{
    match (list1) {
      @Nil => { list2 }
      @Cons(e, rest) => { cons(e, append(rest, list2)) }
    }
}


pub fn elt<T>(list: @List<T>, idx: u64) -> Option<@T> {
    elt_direct(list, idx, 0)
}

// tail recursive
fn elt_direct<T>(list: @List<T>, idx: u64, counter: u64) -> Option<@T> {
    match(list) {
      @Nil => {None}
      @Cons(e, rest) => {
        if (counter == idx) {
            Some(e)
        }
        else {
            elt_direct(rest, idx, counter + 1)
        }
      }
    }
}


pub fn length<T>(list: @List<T>) -> u64 {
    match (list)
    {
      @Nil => {
        0
      }
      @Cons(_, rest) => {
        length(rest) + 1
      }
    }
}


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
#[test]
fn check_delete() {
    assert delete(@Cons(@10, @Nil), 0) ==  @Nil;
    let three_long = cons(@10, cons(@20, cons(@30, @Nil)));

    assert delete(three_long, 0) ==  cons(@20, cons(@30, @Nil));

    // non-mutating
    assert three_long == cons(@10, cons(@20, cons(@30, @Nil)));
}
#[test]
fn check_cons() {
    assert @Cons(@10, @Nil) == cons(@10, @Nil)
}

#[test]
fn check_len() {
    let three_long = cons(@10, cons(@20, cons(@30, @Nil)));
    let mut len : u64 = length(three_long);

    assert len == 3;

    let list : @List<u64> = @Nil;
    len = length(list);

    assert len == 0;
}

#[test]
fn check_elt() {
    let three_long = cons(@10, cons(@20, cons(@30, @Nil)));


    assert elt(three_long, 0) == Some(@10);
    assert elt(three_long, 1) == Some(@20);
    assert elt(three_long, 2) == Some(@30);

    assert elt(three_long, -1) == None;
    assert elt(three_long, 4) == None;
}