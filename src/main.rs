mod linear_search;
mod binary_search;

fn main() {
    let arr: [i32; 5] = [3, 5, 7, 9, 11]; 

    let r = linear_search::linear_search( &arr, 5);
    println!("{r}");

    let r = binary_search::binary_search(&arr, 5);
    println!("{r}");



}
