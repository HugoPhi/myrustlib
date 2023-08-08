mod stdstruct;
mod utils;


fn main() {
    // test for SourceLinkList 
    // let mut list = SourceLinkList::new();
    // list.push(1);
    // list.push(2);
    // list.push(3);
    // list.push(4);
    // list.push(5);
    // list.push(6);
    // println!("{:?}", list);
    // while list.pop() {
    //     println!("{:?}", list);
    // }
    

    // test for stack 
    let mut stack: stdstruct::stack::Stack<i32> = stdstruct::stack::Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.push(5);
    stack.push(6);
    println!("{:?}", stack);
    while !stack.empty() {
        println!("{:?}", stack);
        stack.pop();
    }
    println!("{:?}", stack);
}

