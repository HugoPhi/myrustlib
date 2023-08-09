use crate::utils::__swap_in_array__;

fn maxtopheap_shiftdown<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T], index: usize, bottom: usize) {
    let mut par = index;
    let mut son = 2*par + 1;
    
    while son <= bottom {
        if (son + 1 <= bottom) && (arr[son + 1] > arr[son]) { son = son + 1; }
        if arr[par] >= arr[son] {
            return;
        } else {
            __swap_in_array__(&mut arr[par..=son]);
        }
        par = son;
        son = 2*par + 1;
    }
}


pub fn heap_sort<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) {
    let len: usize = arr.len();
    let parent: usize = len/2 - 1;
    let mut i: usize = parent;
    loop {
        maxtopheap_shiftdown(&mut arr[..], i, len - 1);
        if i > 0 { i = i - 1; } else { break; }
    }
    i = len - 1;
    while i > 0 {
        __swap_in_array__(&mut arr[0..=i]);
        maxtopheap_shiftdown(&mut arr[..], 0, i - 1);
        i = i - 1;
    }
}


