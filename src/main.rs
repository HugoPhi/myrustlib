#![allow(dead_code)]

mod linear;
mod utils;

fn main() {
    // test_stack();
    // test_sourcelink_list();
    // test_sort_funtions();
    test_queue();
}


fn test_queue() {
    // test for queue
    let mut queue: linear::Queue<i32> = linear::Queue::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    queue.push(4);
    queue.push(5);
    println!("{:?}", queue);
    println!("length of queue is: {:?}", queue.len());
    *queue.front().unwrap() = 6;
    println!("{:?}", queue);
    while !queue.empty() {
        println!("{:?}", queue);
        queue.pop();
    }
    println!("{:?}", queue);
}


fn test_stack() {
    // test for stack 
    let mut stack: linear::Stack<i32> = linear::Stack::new();
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

fn test_sourcelink_list() {
     // test for SourceLinkList 
    let mut list = linear::SourceLinkList::new(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    list.push(6);
    println!("{:?}", list);
    println!("length of list is: {:?}", list.len());
    while list.pop() {
        println!("{:?}", list);
        println!("length of list is: {:?}", list.len());
    }
    list.insert(6, 4);
    println!("{:?}", list);
    loop {
        list.erase(0);
        println!("{:?}", list);
    }   
}

fn test_sort_funtions() {
    // test for sort functions
    let mut arr: [i32; 10] = [8, 9, 5, 7, 10, 6, 1, 4, 2, 3];
    // utils::bubble_sort(&mut arr);
    // utils::insertion_sort(&mut arr);
    // utils::quick_sort(&mut arr);
    // utils::merge_sort(&mut arr);
    utils::heap_sort(&mut arr);
    println!("{:?}", arr);
}



