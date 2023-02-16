### Done
Sometimes you might want to declare many variables at once. There are four methods of doing this.

<br><br>
# Method 1
```Rust
// main.rs

fn main(){
	let a = 5;
	let mut b: u8 = 20;
	let c: String = String::from("Hello");
}
```
<br><br>
# Method 2
```Rust
// main.rs

fn main(){
	let (a, mut b, c) = (5, 20, String::from("Hello"));
}
```
<br><br>
# Method 3
```Rust
// main.rs

fn main(){
	let (a, mut b, c) : (i32, u8, String) = (5, 20, String::from("Hello"));
}
```
<br><br>
# Method 4
```Rust
// main.rs

fn main(){
	let (a, mut b, c) = (5i32, 20u8, String::from("Hello"));
}
```