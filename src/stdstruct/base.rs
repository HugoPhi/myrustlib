
#![allow(dead_code)]


struct SourceLinkList<T: std::default::Default + std::fmt::Debug> {
    val: T,
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

// ------------------------------------------------------------------------------------------ // 


struct Stack<T: std::default::Default + std::fmt::Debug> {
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

