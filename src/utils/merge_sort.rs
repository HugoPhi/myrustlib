// arr [0 .. mid - 1]  and  arr [mid .. len - 1] , includes edge   
pub fn merge_two<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T], mid: usize) {
    let len: usize = arr.len();
    let mut i: usize = 0;
    let mut j: usize = mid;
    let mut k: usize = 0;
    let mut tmp: Vec<T> = vec![];
    for id in 0..len {
        tmp.push(arr[id].clone());
    }
    while i < mid && j < len {
        if tmp[i] <= tmp[j] {
            arr[k] = tmp[i].clone();
            i = i + 1; k = k + 1;
        } else {
            arr[k] = tmp[j].clone();
            j = j + 1; k = k + 1;
        }
    }
    while i < mid { arr[k] = tmp[i].clone(); i = i + 1; k = k + 1; }
    while j < len { arr[k] = tmp[j].clone(); j = j + 1; k = k + 1; }
}


pub fn merge_sort<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) {
    let len: usize = arr.len();
    if len > 1 {
        let mid: usize = len / 2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge_two(arr, mid);
    }
}


