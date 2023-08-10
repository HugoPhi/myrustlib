#![allow(dead_code)]

pub struct SourceLinkList<T: std::default::Default + std::fmt::Debug + std::clone::Clone> {
    pub val: T,
    next: Option<Box<SourceLinkList<T>>>,
}


impl<T: std::default::Default + std::fmt::Debug + std::clone::Clone> SourceLinkList<T> {
    pub fn new(init_val: T) -> Self {
        SourceLinkList {
            val: init_val,
            next: Option::None,
        }
    }
    
    pub fn push(&mut self, value: T) {
        if self.next.is_none() {
            self.next = Option::Some(Box::new(SourceLinkList::new(value)));
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
    
    pub fn len(&self) -> usize {
        let cnt: usize = 1;
        if self.next.is_none() {
            return cnt;
        } else {
            return self.next.as_ref().unwrap().len() + 1;
        }
    }

    pub fn at(&mut self, index: usize) -> &mut T {
        if index == 0 {
            &mut self.val
        } else if self.next.is_none() {
            panic!("index out of range");
        } else {
            self.next.as_mut().unwrap().at(index - 1)
        }
    }

    pub fn insert(&mut self, mut index: usize, value: T) {
        let mut ptr = self;
        if index == 0 { 
            let tmp = ptr.val.clone();
            ptr.val = value.clone();
            ptr.next = Option::Some(Box::new(SourceLinkList { val: tmp, next: ptr.next.take() }));
            return;
        }
        while index > 1 {
            if ptr.next.is_none() { panic!("index out of range"); }
            ptr = ptr.next.as_mut().unwrap();
            index = index - 1;
        }
        ptr.next = Option::Some(Box::new(SourceLinkList { val: value, next: ptr.next.take() }));
    }


    pub fn erase(&mut self, mut index: usize) -> bool {
        let mut ptr = self;
        if index == 0 {
            if ptr.next.is_none() { 
                return false;
            } else {
                ptr.val = ptr.next.as_mut().unwrap().val.clone();
                ptr.next = ptr.next.as_mut().unwrap().next.take();
                return true;
            }
        }
        while index > 1 {
            if ptr.next.is_none() { panic!("index out of range!") }
            ptr = ptr.next.as_mut().unwrap();
            index = index - 1;
        }
        ptr.next = ptr.next.as_mut().unwrap().next.take();
        return true;
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


impl<T: std::default::Default + std::fmt::Debug + std::clone::Clone> std::fmt::Debug for SourceLinkList<T> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.__show_format__();
        Ok(())
    }
}


