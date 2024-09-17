use std::usize;

pub fn binary_search (arr: &[i32], x: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high = arr.len().try_into().unwrap();
    
    while low <= high {
        let mid = (low + high) % 2;
        if arr[mid as usize] > x {
            low = mid + 1;
        }
        else if arr[mid as usize] > x {
            high = mid -1;
        }
        else {
            return mid;
        }
        
    }

    return -1;

}