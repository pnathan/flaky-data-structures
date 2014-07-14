/*
Circular queue, ring buffer, etc.

It's a finite data structure with good performance.

*/

//use std::vec::with_capacity;
use std::uint;
use std::vec;


pub struct Ring<T> {
    m_contents: Vec<T>,
    m_start_ptr: uint,
    m_end_ptr: uint,
    m_length: uint,
    m_max: uint
}

impl<T: Clone> Ring<T> {

    pub fn new(size: uint, default_value : T) -> Ring<T> {
        if (size == 0) {
            fail!("Ring buffers must have > 0 elements");
        }

        let mut temp = Vec::with_capacity(size);

        let mut i = 0;
        while i < size {
            // TODO: move this out of the loop.
            let dummy: T = default_value.clone();

            temp.push(dummy);

            i += 1;
        }

        Ring { m_contents : temp,
              m_length: 0,
              m_start_ptr: 0,
              m_end_ptr: 0,
              m_max: size
             }
    }

    pub fn size(&self) -> uint {
        self.m_length
    }

    // check the front of the ring
    pub fn peek(&self) -> Option<T> {
        if(self.m_length > 0) {
            Some(self.m_contents.get(self.m_start_ptr).clone())
        }
        else {
            None
        }
    }

    // enqueue
    pub fn push_back(&mut self,  data: T) -> () {
        *self.m_contents.get_mut(self.m_end_ptr) = data;
        self.m_end_ptr = (self.m_end_ptr + 1) % self.m_max;
        self.m_length += 1;
    }

    //dequeue
    pub fn pop(&mut self) -> Option<T> {
      if(self.m_length == 0) {
          None
        }
        else {
            let retval = self.m_contents.get(self.m_start_ptr).clone();

            self.m_start_ptr = (self.m_start_ptr + 1) % self.m_max;
            self.m_length -= 1;

            Some(retval)
        }
    }
}


#[test]
fn test_new() {
    let r : Ring<int> = Ring::new(1,0);
    assert!(r.size() == 0);

    let r : Ring<int> = Ring::new(128,0);
    assert!(r.size() == 0);
}

#[test]
#[should_fail]
fn test_fail_init() {
    let r : Ring<int> = Ring::new(0,0);
}
#[test]
fn test_push() {
    let mut r : Ring<int> = Ring::new(1, 0);
    r.push_back(10);
    assert!(r.size() == 1);
    assert!(r.peek() == Some(10));
}

#[test]
fn test_push_more() {
    let mut r : Ring<int> = Ring::new(5, 0);

    r.push_back(10);
    assert!(r.size() == 1);
    assert!(r.peek() == Some(10));

    r.push_back(20);
    assert!(r.size() == 2);
    assert!(r.peek() == Some(10));

    r.push_back(30);
    assert!(r.size() == 3);
    assert!(r.peek() == Some(10));

}

#[test]
fn test_push_pop() {
    let mut r : Ring<int> = Ring::new(5, 0);

    r.push_back(10);
    r.push_back(20);
    r.push_back(30);

    let val = r.pop();
    assert!(val == Some(10));
    let val = r.pop();
    assert!(val == Some(20));
    let val = r.pop();
    assert!(val == Some(30));
    assert!(r.peek() == None);
    assert!(r.size() == 0);
}
