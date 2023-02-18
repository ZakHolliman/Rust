### [Working] Intro
The `as_bytes()` method is a method available on `str` types, which `converts the string into an array of bytes.`

# Example
```Rust
// main.rs

fn main(){
	let message = "Hello";
	let bytes = message.as_bytes();

	println!("{:?}", bytes);
}
```

### Output
```
[72, 101, 108, 108, 111]
```

# Breakdown
