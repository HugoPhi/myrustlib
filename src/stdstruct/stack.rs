#![allow(dead_code)]
use super::SourceLinkList;


pub struct Stack<T: std::default::Default + std::fmt::Debug> {
    size: usize,
    arr: SourceLinkList<T>,
}

impl<T: std::default::Default + std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Stack {
            size: 0,
            arr: SourceLinkList::new(),
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
            println!("stack is empty!!❌");
            return false;
        } else {
            self.size -= 1;
            self.arr.pop()
        }
    }

    pub fn top(&mut self) -> Option<&mut T> {
        if self.empty() {
            None
        } else {
            self.arr.end_value()
        }
    }
}


impl<T: std::default::Default + std::fmt::Debug> std::fmt::Debug for Stack<T> {
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


