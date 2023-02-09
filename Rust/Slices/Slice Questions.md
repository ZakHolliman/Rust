# Question 1

Consider the variables `s2` and `s3` in the following program. These two variables will be located in memory within the stack frame for `main`. Each variable has a size in memory on the stack, _not_ including the size of pointed data. Which statement is true about the sizes of `s2` and `s3`?

```Rust
fn main(){
	let s = String::from("hello"); // Initialize a String variable
	let s2: &String = &s; // Initialize a String reference with 's' as its ref
	let s3: &str = &s[..] // Init an s3 variable as a slice of s
}
```

### Answer
`s3` has *more* bytes than `s2`. This is because the type of `s2` is *Just* a pointer, as `s2` only has the value of a reference to `s`. `s3` on the other hand, is of a *Slice* type, meaning it has a reference pointer *and* a size value.





# Question 2

Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

```Rust
fn main() {
	let mut s = String::from("hello");
	for &item in s.as_bytes().iter() {
		if item == b'l' {
			s.push_str(" world");
		}
	}
	
	println!("{s}");
}
```

### Answer
This program **does not** compile. This is because `s.as_bytes()` produces an **immutable** reference to `s`, and it is illegal to mutate `s` inside the for-loop.