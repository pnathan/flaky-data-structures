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
    fn size(cont: @self) -> u64;
}

// A List; classic abstract datatype for a List
trait List<T> {
    fn head(&self) -> Option<@T>;
    fn length(&self) -> u64;
    fn cons(data: @T, seq: @self) -> @self;
    fn elt(&self, idx: u64) -> Option<@T>;
}

// A queue, which also is a kind of list
trait Queue<T> : List<T>{
    fn peek(seq: @self) -> Option<@T>;
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
    fn cons(data: @T, list: @List_Data<T>) -> @List_Data<T> {
        @Cons(data, list)
    }
    fn head(&self) -> Option<@T> {
        match (self)  {
          &Nil => {
            None
          }
          &Cons(e, _) => {
            Some(e)
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
impl<T> List_Data<T>: Container<T> {
    fn size (seq: @List_Data<T>) -> u64 {
        seq.length()
    }
}

// And it also implements the queue traits
impl<T> List_Data<T>: Queue<T> {

    fn peek(list : @List_Data<T>) -> Option<@T> {
        list.head()
    }

    // Returns the first element, along with a list that doesn't have
    // the first element.
    fn pop(list : @List_Data<T>) -> (Option<@T>, @List_Data<T>) {
        (None, list)
    }
}
/*

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

pub fn append<T>(list1: @List<T>, list2: @List<T>) -> @List<T>{
    match (list1) {
      @Nil => { list2 }
      @Cons(e, rest) => { cons(e, append(rest, list2)) }
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
*/

#[test]
fn check_cons() {
    let nilly : @List_Data<u64> = @Nil;
    let list : @List_Data<u64> = @Cons(@10, @Nil::<u64>);
    let otherlist : @List_Data<u64> = Nil.cons(@10, nilly);
    assert list == otherlist;
}

/*
#[test]
fn check_delete() {
    assert delete(@Cons(@10, @Nil), 0) ==  @Nil;
    let three_long = cons(@10, cons(@20, cons(@30, @Nil)));

    assert delete(three_long, 0) ==  cons(@20, cons(@30, @Nil));

    // non-mutating
    assert three_long == cons(@10, cons(@20, cons(@30, @Nil)));
}
*/

#[test]
fn check_head() {
// is the head still attached?
    let three_long : @List_Data<u64> = from_vec([10, 20, 30]);
    assert three_long.head() == Some(@10)

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
