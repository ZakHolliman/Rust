// This is an extremely simple program that simply initializes an array and
// prints the total of the elements in the array.

fn main(){
    let mut total: i32 = 0;

    let arr = [1, 9, 4, 8, 5];

    for int in arr {
        total += int;
    }

    println!("Total is: {}", total);

    let mut total_2: i32 = 0;

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];

    for int2 in arr2 {
        println!("{}", int2);
    }

}