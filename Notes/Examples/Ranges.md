### Done
Several programs showcasing the different ways that you can specify `range slices`

<br><br>
# Value (Inclusive) -> Value (Exclusive)
```Rust
// main.rs

fn main(){
	let array: [i32; 5] = [0, 1, 2, 3, 4];

	for int in &array[1..3]{
        println!("Value is: {}", int);
    }
}
```

### Output
```
Value is: 1
Value is: 2
```

Simply putting two values in the range of a slice will range it `inclusively` from the first value *up to, but not including* the max range value.

This shows a slice range that goes from an `inclusive` value of `1` to an `exclusive` value of `3`, so it only prints `1` and `2`.

<br><br>
# Value (Inclusive) -> Value (Inclusive)
```Rust
// main.rs

fn main(){
	let array: [i32; 5] = [0, 1, 2, 3, 4];

	for int in &array[1..=3]{
        println!("Value is: {}", int);
    }
}
```

### Output
```
Value is: 1
Value is: 2
Value is: 3
```

If we *did* want to include the max of the range in our iteration, we can use the `..=x` syntax.

This shows a slice range that goes from an `inclusive` value of `1` to an `inclusive` value of `3`, now printing `1, 2 and 3`.
<br><br>
# Beginning -> Value (Exclusive)
```Rust
// main.rs

fn main(){
	let array: [i32; 5] = [0, 1, 2, 3, 4];

	for int in &array[..3]{
        println!("Value is: {}", int);
    }
}
```

### Output
```
Value is: 0
Value is: 1
Value is: 2
```

We can also `drop the value` from the beginning altogether to specify a slice from the `beginning of the array` up until, including the max value.

This shows a slice range that goes from the `beginning` of the array to an `exclusive` value of `3`, printing `0, 1, and 2`.
<br><br>
# Value (Inclusive) -> End
```Rust
// main.rs

fn main(){
	let array: [i32; 5] = [0, 1, 2, 3, 4];

	for int in &array[1..]{
        println!("Value is: {}", int);
    }
}
```

### Output
```
Value is: 1
Value is: 2
Value is: 3
Value is: 4
```

`Dropping the value` from the `end` of the syntax specifies the range to go until the `end` of the specified list. 

This shows a slice range that goes from `1` to the `end of the array`, printing `1, 2, 3, and 4`.