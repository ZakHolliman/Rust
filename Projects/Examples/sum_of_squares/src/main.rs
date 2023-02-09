/*
Write a function called sum_of_squares that takes two integers a and b as 
parameters and returns the sum of squares of all numbers between a and b 
(inclusive). If a is greater than b, return 0.
*/

fn main() {
    let a: i32 = 2;
    let b: i32 = 5;

    let sum = sum_of_squares(a, b);

    println!("Sum of squares from {} to {} is {}", a, b, sum);
}

fn sum_of_squares(a: i32, b: i32) -> i32 {
    // For each number in the range of a to b
    let mut sum = 0;

    for num in a..=b {
        sum += i32::pow(num, 2);
    }

    return sum
}