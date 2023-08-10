#![allow(dead_code)]

use super::base::SourceLinkList;

pub struct Queue<T: std::default::Default + std::fmt::Debug + std::clone::Clone> {
    size: usize,
    arr: SourceLinkList<T>,
}

impl<T: std::default::Default + std::fmt::Debug + std::clone::Clone> Queue<T> {
    pub fn new() -> Self {
        Queue {
            size: 0,
            arr: SourceLinkList::new(Default::default()),
        }
    }

    pub fn empty(&self) -> bool {
        if self.size == 0 {
            return true;
        } else {
            return false;
        }
    }

    pub fn push(&mut self, value: T) {
        if self.empty() {
            self.size = 1;
            self.arr.val = value;
        } else {
            self.arr.push(value);
            self.size += 1;
        }
    }

    pub fn pop(&mut self) -> bool {
        if self.empty() {
            return false;
        } else {
            self.size -= 1;
            self.arr.erase(0)
        }
    }

    pub fn front(&mut self) -> Option<&mut T> {
        if self.empty() {
            None
        } else {
            Option::Some(self.arr.at(0))
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<T: std::default::Default + std::fmt::Debug + std::clone::Clone> std::fmt::Debug for Queue<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.empty() {
            print!("[]");
            Ok(())
        } else {
            print!("{:?}", self.arr);
            Ok(())
        }
    }
}



