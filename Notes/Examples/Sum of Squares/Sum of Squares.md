# [Done] Intro
Write a function called `sum_of_squares` that takes two integers `a` and `b` as 
parameters and `returns the sum of squares of all numbers between a and b` 
(inclusive). If a is greater than b, return 0.

# Program 1

```Rust
// main.rs

fn main() {
	// Test 1
    let (num1, num2) : (i32, i32) = (7, 5);
    let sum = sum_of_squares(num1, num2);
    println!("Sum of squares from {} to {} is {}", num1, num2, sum);

	// Test 2
    let (num1, num2) : (i32, i32) = (3, 9);
    let sum = sum_of_squares(num1, num2);
    println!("Sum of squares from {} to {} is {}", num1, num2, sum);
}

fn sum_of_squares(a: i32, b: i32) -> i32 {
    // Return 0 if a > b
    if a > b { return 0 }

    // Do program
    let mut sum = 0;

    for num in a..=b {
        sum += i32::pow(num, 2);
    }

    return sum;
}
```

This can be done a bit more succinctly using some Rust features such as `match` statements.

# Program 2
``` Rust
// main.rs

fn main() {
	// Test 1
    let (num1, num2) : (i32, i32) = (7, 5);
    let sum = sum_of_squares(num1, num2);
    match sum {
        Some(sum) => println!("Sum of squares between {} and {} is {}", num1, num2, sum),
        None => println!("num1({}) was bigger than num2({})", num1, num2),
    }

	// Test 2
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
```