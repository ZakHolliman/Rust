### [Working] Intro
The `unwrap()` method is a method that you can call on several types in Rust to `extract the value inside of the type`. They are most prominently used in extracting values from `Option<T>` or `Result` types.

# Example
```Rust
// main.rs

fn main(){
	let result: Option<i32> = Some(42);
	print_type_of(&result);
	
	let value = result.unwrap();
	print_type_of(&value);
	
	println!("The value is: {}", value);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
```

### Output
```
core::option::Option
i32
The value is: 42
```

# Breakdown

# Return Type
The `unwrap` method will attempt to extract a value from any given type. In the case above, we extract the value of `42` from our `result` variable, and assign it to `value`. If, however, our value has no extractable data, such as an `Option<T>::None` or `Result::Err`, then the program will panic. To avoid this, it's better to either use a `match` statement to handle the possible error case, or use one of the alternative `unwrap` methods such as `unwrap_or()` or `unwrap_or_else()`.