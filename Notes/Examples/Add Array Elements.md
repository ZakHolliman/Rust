### Done
Small program to showcase how you would add up all of the elements in an array.

<br><br>
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

# Breakdown
There isn't much to talk about here, the only important thing to note is the

```Rust
array.iter().sum();
```

This `iterates` over all of the elements in the array. `Iterator` in Rust has a helpful method attached to it called `sum()` that can be called on Iterator's that are iterating over numeric values. If we call this, they all get summed up for us, and then returned into the `total` variable that we defined.