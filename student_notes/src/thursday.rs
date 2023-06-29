pub fn main() {
    //Rust Tuples
    let tuple2 = ("another tuple",);
    let my_tuple = (1, "hello", 3.14, "hello".to_string(), tuple2);

    // println!("{my_tuple:?}");

    // println!("{}", my_tuple.0);
    // println!("{}", my_tuple.1);
    // println!("{}", my_tuple.2);
    // println!("{}", my_tuple.3);
    // println!("{}", my_tuple.4 .0);

    // let (a, b, c, d, e) = my_tuple;
    // println!("{}{}{}{}", a, b, c, d);

    fn swap((x, y): (i32, i32)) -> (i32, i32) {
        (y, x)
    }

    let swapped = swap((1, 2));
    let (a, b) = swap((3, 4));

    println!("{swapped:?}");

    // Arrays and built-in funcs

    let arr1 = [1, 2, 3, 4, 5];

    let arr_length = arr1.len();

    println!("array is {} elements long", arr_length);

    let is_empty = arr1.is_empty();

    println!("the arr is empty: {}", is_empty);

    let contains = arr1.contains(&3);

    println!("Array containe 3?: {}", contains);

    let first = arr1.first();

    // Initalize matrix with rows and columns, the semi-colon

    let matx = [[0; 4]; 5];

    // Vector methods

    let mut vec1 = vec![3, 2, 1, 1];

    vec1.sort();
    // removes dups
    vec1.dedup();
    // Insert
    vec1.insert(1, 4);
}
