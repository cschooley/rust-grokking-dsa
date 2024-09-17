pub fn linear_search ( arr: &[i32], x: i32) -> usize {

    //let arr_iter = arr.iter();

    for (inx, val) in arr.iter().enumerate() {
        if *val == x {
            return inx;
        }
    }
    return usize::MAX;

}