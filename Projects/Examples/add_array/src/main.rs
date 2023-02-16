fn main() {
    let (a, b) : (i32, i32) = (9, 5);

    let sum = sum_of_squares(a, b);

    println!("Sum of squares from {} to {} is {}", a, b, sum);
}

fn sum_of_squares(a: i32, b: i32) -> i32 {
    if a > b
        return 0

    let mut sum = 0;

    for num in a..=b {
        sum += i32::pow(num, 2);
    }

    return sum
}