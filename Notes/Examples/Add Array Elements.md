### Done
Small program to showcase how you would add up all of the elements in an array.

# Program
```Rust
//main.rs

fn main() {
	let array: [i32; 5] = [1, 2, 3, 4, 5];
	
	let total: i32 = array.iter().sum();
	
	println!("The total is: {}", total);
}
```

### Output
```
The total is: 15
```