use crate::utils::__swap_in_array__;

fn three_way_adapt<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) {
    let len: usize = arr.len();
    let mid: usize = (len - 1) / 2;
    if arr[0] < arr[mid] { __swap_in_array__(&mut arr[0..=mid]); }
    if arr[0] > arr[len - 1] { __swap_in_array__(&mut arr[0..=len - 1]); }
    if arr[0] < arr[mid] { __swap_in_array__(&mut arr[0..=mid]); }
}


pub fn partition<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) -> usize {
    let len =arr.len();
    let pivot: T = arr[0].clone();
    let mut id1: usize = 0;
    let mut id2: usize = len - 1;
    while id1 < id2 {
        while (id1 < id2) && (arr[id2] >= pivot) { id2 = id2 - 1; }
        if id1 < id2 { arr[id1] = arr[id2].clone(); id1 = id1 + 1; }
        while (id1 < id2) && (arr[id1] <= pivot) { id1 = id1 + 1; }
        if id1 < id2 { arr[id2] = arr[id1].clone(); id2 = id2 - 1; }
    }
    arr[id1] = pivot;
    return id1;
}


pub fn quick_sort<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) {
    let len: usize = arr.len();
    if len > 1 {
        three_way_adapt(arr);
        let i = partition(arr);
        quick_sort(&mut arr[..i]);
        quick_sort(&mut arr[i + 1..]);
    }
}


