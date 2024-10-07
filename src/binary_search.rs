use std::usize;

pub fn binary_search (arr: &[i32], x: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high = arr.len().try_into().unwrap();
    high = high-1;

    while low <= high {
        let mid = (low + high) / 2;
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

#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn nominal_array() {
        let result = binary_search(&[1,2,3,4,5,6,7,8,9], 5);
        assert_eq!(result,4);

    }
}