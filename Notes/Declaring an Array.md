There are two ways that are most common for declaring `arrays` in Rust

# Method 1 - Implied
```Rust
// main.rs

fn main(){
	let array = [75, 23, 5, 92, 52];
}
```

This method of declaring arrays doesn't require any static type or size declarations, they are both implied based on the values that you give it.

# Method 2 - Static
```Rust
// main.rs

fn main(){
	let a: [i32; 5] = [75, 23, 5, 92, 52];
}
```

This method of declaring arrays requires you to specify both the `type` of data you will be storing in the array, and `the size` of the array. Here, we've defined our type as `i32` and `size` as `5`.