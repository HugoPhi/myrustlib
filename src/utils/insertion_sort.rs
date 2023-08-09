pub fn insertion_sort<T: std::cmp::PartialOrd + std::clone::Clone>(arr: &mut [T]) {
    let len: usize = arr.len();
    for i in 1..len {
        let mut is_alloc: bool = false;
        let mut j = i - 1;
        let tmp = arr[i].clone();
        loop {
            if arr[j] > tmp { 
                arr[j + 1] = arr[j].clone(); 
            } else { 
                arr[j + 1] = tmp.clone();
                is_alloc = true;
                break;
            }
            if j > 0 { j = j - 1; } else { break; }
        }
        if !is_alloc { arr[0] = tmp; }
    }
}


