### [Done] Intro
Sometimes you want to turn a string into a different data type, such as an integer or float. To do this, we can use the `parse()` method that is attached to `String`. Here's an example of using the `parse` method to do exactly that.

# Example
```Rust
// main.rs

fn main(){
	let string_num = "40"; // Number is initially declared as a string literal

	let num: i32 = string_num.parse().unwrap();

	print_type_of(&string_num);
	print_type_of(&num);
}

// Auxillary funciton, see `Getting Type of a Value` for more info
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
```

### Output
```
&str
i32
```

# Declare Variable
In this short program, we first declare a number `string_num` as a string literal `40`. 

```Rust
let string_num = "40"; // Number is initially declared as a string literal
```

# Parse method
In order to convert this to an `i32` value, we can call the `parse` method attached to `String`.

```Rust
let num: i32 = string_num.parse().unwrap();
```

# Return Type
The `parse` method will attempt to parse the given string into an `integer` or `float` value, returning a variant of the `Result` type with either an `Ok<i32>` variant that contains the parsed integer if successful, or an `Err<ParseIntError>` if it fails.

# Unwrap
In the case of a successful parse, we can use the `unwrap()` method on the `Ok` variant to extract the value. This is the value that will be stored in our variable `i32`.

# Error Handling
If we wanted to catch and handle the error in a case of possible failure, we can use a `match` statement.

```Rust
// main.rs

fn main(){
	let string_num = "40"; // Number is initially declared as a string literal

	let num: i32 = string_num.parse().unwrap();

	let num = match string_num.parse::<i32>(){
		Ok(parsed_num) => parsed_num,
	    Err(_) => {
	        println!("Error: Failed to parse integer from string");
	        return;
	    }
	};

	print_type_of(&string_num);
	print_type_of(&num);
}

// Auxillary funciton, see `Getting Type of a Value` for more info
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
```