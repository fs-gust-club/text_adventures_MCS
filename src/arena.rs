#![warn(missing_docs)]

use std::cell::RefCell;

pub struct Arena<T> {
    nodes: RefCell<Vec<T>>,
}

impl<T> Arena<T> {
    pub fn new(size: usize) -> Arena<T> {
        Arena {
            nodes: RefCell::new(Vec::<T>::with_capacity(size)),
        }
    }

    pub fn allocate(&self, node: T) -> &T {
        let mut nodes = self.nodes.borrow_mut();
        nodes.push(node);

        let value_ptr: *const T = nodes.last().unwrap();
        unsafe { &*value_ptr }
    }
}
