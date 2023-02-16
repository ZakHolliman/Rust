# Question 1

```Rust
/// Makes a string to separate lines of text, 
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> &str { // Creates a function and takes a string reference, returns a string reference
    if user_str == "" { // If the string is empty
        let default = "=".repeat(10); // Make the string ten = signs
        &default // Return a reference to this string
    } else {
        user_str // Return the string back
    }
}
```

If you tried to compile this function, which of the following best describes the compiler error you would get?
* function `make_separator` cannot return a reference of type `&str`
* cannot return reference to local variable default
* `user_str` does not live long enough
* function `make_separator` cannot return two different references

##### Answer

# Question 2
```Rust
/// Makes a string to separate lines of text, 
/// returning a default if the provided string is blank
fn make_separator(user_str: &str) -> &str {
    if user_str == "" {
        let default = "=".repeat(10);
        &default
    } else {
        user_str
    }
}
```

Normally if you try to compile this function, the compiler returns the following error:
```Rust

error[E0515]: cannot return reference to local variable `default`
 --> test.rs:6:9
  |
6 |         &default
  |         ^^^^^^^^ returns a reference to data owned by the current function
```

Assume that the compiler did NOT reject this function. Which (if any) of the following programs would (1) pass the compiler, and (2) possibly violate memory safety or cause a data race if executed? Check each program that satisfies both criteria, OR check "None of these programs" if none are satisfying.

println!("{}", make_separator("Hello world!"));

let s = make_separator("");
println!("{s}");

let s = make_separator("");

None of these programs

##### Answer


# Question 3
