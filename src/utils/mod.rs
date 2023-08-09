#![allow(dead_code)]

fn __swap_in_array__<T: std::clone::Clone>(arr: &mut[T]) {
    let len: usize = arr.len();
    let tmp: T = arr[0].clone();
    arr[0] = arr[len - 1].clone();
    arr[len - 1] = tmp;
}

pub mod bubble_sort;
pub use bubble_sort::bubble_sort;

pub mod insertion_sort;
pub use insertion_sort::insertion_sort;

pub mod quick_sort;
pub use quick_sort::quick_sort;

pub mod merge_sort;
pub use merge_sort::merge_sort;

pub mod heap_sort;
pub use heap_sort::heap_sort;


