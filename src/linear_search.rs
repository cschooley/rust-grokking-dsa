pub fn linear_search ( arr: &[i32], x: i32) -> usize {

    for (inx, val) in arr.iter().enumerate() {
        if *val == x {
            return inx;
        }
    }
    return usize::MAX;

}

#[allow(dead_code)]
mod tests {
    use super::*;

    #[test]
    fn nominal_array() {
        let result = linear_search(&[1,2,3,4,5,6,7,8,9], 5);
        assert_eq!(result,4);

    }
}