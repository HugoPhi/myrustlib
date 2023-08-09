mod datas;
mod utils;


fn main() {
//     // test for SourceLinkList 
//     let mut list = SourceLinkList::new();
//     list.push(1);
//     list.push(2);
//     list.push(3);
//     list.push(4);
//     list.push(5);
//     list.push(6);
//     println!("{:?}", list);
//     while list.pop() {
//         println!("{:?}", list);
//     }
    
    
    // test for stack 
    let mut stack: datas::Stack<i32> = datas::Stack::new();
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
    
//     // test for sort functions
//     let mut arr: [i32; 10] = [8, 9, 5, 7, 10, 6, 1, 4, 2, 3];
//     println!("{:?}", arr);
//     utils::bubble_sort(&mut arr); // √
//     utils::insertion_sort(&mut arr); // √
//     utils::quick_sort(&mut arr); // √
//     utils::merge_sort(&mut arr); // √
//     utils::heap_sort(&mut arr);  // √
//     println!("{:?}", arr);
}

