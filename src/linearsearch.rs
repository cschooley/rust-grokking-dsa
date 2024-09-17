

pub fn linearsearch ( arr: &[i32], x: i32) -> i32 {

    //let arr_iter = arr.iter();

    for val in arr.iter() {
        if *val == x {
            return *val;
        }
    }
    return -1;
}