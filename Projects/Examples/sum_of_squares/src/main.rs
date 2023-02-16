fn main() {
    let (num1, num2) : (i32, i32) = (7, 5);
    let sum = sum_of_squares(num1, num2);
    match sum {
        Some(sum) => println!("Sum of squares between {} and {} is {}", num1, num2, sum),
        None => println!("num1({}) was bigger than num2({})", num1, num2),
    }

    let (num1, num2) : (i32, i32) = (3, 9);
    let sum = sum_of_squares(num1, num2);
    match sum {
        Some(sum) => println!("Sum of squares between {} and {} is {}", num1, num2, sum),
        None => println!("num1({}) was bigger than num2({})", num1, num2),
    }
}

fn sum_of_squares(a: i32, b: i32) -> Option<i32> {
    // Return 0 if a > b
    if a > b { return None }

    // Do program
    let mut sum = 0;

    for num in a..=b {
        sum += i32::pow(num, 2);
    }

    return Some(sum);
}