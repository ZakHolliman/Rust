Here's an example of a program that uses Slices to return the first word of a given string.

```Rust
fn first_word(s: &String) -> &str { 
	let bytes = s.as_bytes(); 
	
	for (i, &item) in bytes.iter().enumerate() { 
		if item == b' ' { 
			return &s[0..i]; 
		} 
	}
	
	&s[..] 
}
```

Breaking it down, we start with the first line

```Rust
fn first_word(s: &String) -> &str { 
```

which declares a function "first_word" with a single parameter that takes a **string reference**. The function is also defined to **return** a string reference.

From there we have the line : 
```Rust
let bytes = s.as_bytes();
```

The "as_bytes" function is used to convert the string "s" into an array of bytes, so that we can iterate over the string and access it as if it were an array.

Then we have our for-loop:

```Rust
for (i, &item) in bytes.iter().enumerate() { 
		if item == b' ' { 
			return &s[0..i]; 
		} 
	} 
```

This for loop enumerates a **tuple** pair over this byte array, comparing each item (character) to `b' '` , and when it finds one it returns a slice of 0 up to where we found that `' '` character, giving us the first word of the string.