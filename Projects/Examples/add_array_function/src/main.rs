// This program showcases two different ways of adding all of the elements in an 
// array together

fn main() {
    let array = [5, 2, 8, 19, 20];

    // Option 1
    let sum_v1 = add_array_v1(&array);
    println!("Sum v1 is: {}", sum_v1);

    let sum_v1 = add_array_v1(&array[1..=3]);
    println!("Sum v1 in array slice: {}", sum_v1);

    // Option 2
    let sum_v2 = add_array_v2(&array);
    println!("Sum v2 is: {}", sum_v2);

    let sum_v2 = add_array_v2(&array[1..=3]);
    println!("Sum v2 in array slice: {}", sum_v2);
}

fn add_array_v1(array: &[i32]) -> i32 {
    let mut total: i32 = 0;

    for int in array {
        total += int;
    }

    total
}

fn add_array_v2(array: &[i32]) -> i32 {
    array.iter().sum()
}