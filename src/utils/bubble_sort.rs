use crate::utils::__swap_in_array__;

pub fn bubble_sort<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) {
    let len: usize = arr.len();
    let mut is_sorted: bool = true;
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                __swap_in_array__(&mut arr[j..=j + 1]);
                is_sorted = false;
            }
        }
        
        if is_sorted { return; }
    }
}


