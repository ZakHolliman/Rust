### Done
The `Array` type, like any other language, is a way to group several values of the SAME type.

Arrays are not allowed to change size once declared.

Array elements can be indexed like normal

```rust

fn main(){
	let array1: = [1, 2, 3, 4, 5];
	let array2: [i32, 5] = [6, 7, 8, 9, 10];

	println!("First element in array 1 is {}", array1[0]);
	println!("Fourth element is array 2 {}", array1[4]);
}
```