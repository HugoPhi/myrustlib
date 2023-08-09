#![allow(dead_code)]

pub struct SourceLinkList<T: std::default::Default + std::fmt::Debug> {
    pub val: T,
    next: Option<Box<SourceLinkList<T>>>,
}


impl<T: std::default::Default + std::fmt::Debug> SourceLinkList<T> {
    pub fn new() -> Self {
        SourceLinkList {
            val: Default::default(),
            next: Option::None,
        }
    }
    
    pub fn push(&mut self, value: T) {
        if self.next.is_none() {
            self.next = Option::Some(Box::new(SourceLinkList::new()));
            self.next.as_mut().unwrap().val = value;
        } else {
            self.next.as_mut().unwrap().push(value);
        }
    }
    
    pub fn pop(&mut self) -> bool {
        if self.next.is_none() {
            return false;
        } else {
            if self.next.as_mut().unwrap().next.is_none() {
                self.next = Option::None;
                return true;
            } else {
                self.next.as_mut().unwrap().pop()
            }
        }
    }
    
    fn __assist_show_format__(&self) {
        if self.next.is_some() {
            print!("{:?}, ", self.val);
            self.next.as_ref().unwrap().__assist_show_format__();
        } else {
            print!("{:?}]", self.val);
        }
    }
    
    pub fn __show_format__(&self) {
        print!("[");
        self.__assist_show_format__();
    }
    
    pub fn end_value(&mut self) -> Option<&mut T> {
        if self.next.is_none() {
            Some(&mut self.val)
        } else {
            self.next.as_mut().unwrap().end_value()
        }
    }
}


impl<T: std::default::Default + std::fmt::Debug> std::fmt::Debug for SourceLinkList<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.__show_format__();
        Ok(())
    }
}


