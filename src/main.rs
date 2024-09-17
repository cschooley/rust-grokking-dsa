mod linearsearch;

fn main() {
    let arr: [i32; 5] = [3, 5, 7, 9, 11]; 

    let r = linearsearch::linearsearch( &arr, 5);
    println!("{r}");
}
