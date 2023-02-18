`Slices` in Rust let you reference contiguous spaces of memory without accessing the **entire** memory space of the data. 

`Slices` are a type of *reference*, so they DO NOT take ownership.

<br><br>
## String Slices
The most common type of slice is a `String Slice` which lets you access a portion of a given string. If we were to declare a string such as this.

```Rust
let s = String::from("hello world");
```

Then in memory, this is stored as

| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| - | - | - | - | - | - | - | - | - |  - | - |
| h | e | l | l | o |   | w | o | r |  l | d |

And we can see that the starting index of "hello" is 0 (inclusively), and the ending is 5 (exclusively). The same applies to "world" where the starting index is 6 and the end is 11.

Using the `range syntax`, we can specify these ranged of the strings

```Rust
let hello = &s[0..5];
let world = &s[6..11];
```

And easily index the part of a string that we want, without referencing the entire thing.

<br><br>
## Slice 
A `Slice`, as a data structure, consists of a **pointer** and a **size** value. The pointer points to the start of the slice, and the size tells it how far from the pointer's start we go to get the full slice. In the example above, we would now have two **Slice** data structures, one assigned to each variable.

The "hello" slice would look like :

| pointer | size |
|-|-|
| 0 | 5 |

and the "world" slice would look like :

| pointer | size |
|-|-|
| 6 | 11 |

Of course, in real memory, the pointer addresses are more complex, but the core concept remains the same.

<br><br>
## Other Slice Types

You can also use Slices to index other data types, such as an array

```Rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2..3]);
```


Here, the slice has the TYPE of `&[i32]`

It works the same way a string slice does, by storing a reference to the first element, along with a length.


# Slice Ranges



# Slice Into Functions
You can pass a `slice` into a function

<br><br>
# Questions
## Question 1
Consider the variables `s2` and `s3` in the following program. These two variables will be located in memory within the stack frame for `main`. Each variable has a size in memory on the stack, _not_ including the size of pointed data. Which statement is true about the sizes of `s2` and `s3`?

```Rust
fn main(){
	let s = String::from("hello"); // Initialize a String variable
	let s2: &String = &s; // Initialize a String reference with 's' as its ref
	let s3: &str = &s[..] // Init an s3 variable as a slice of s
}
```

##### Answer
`s3` has *more* bytes than `s2`. This is because the type of `s2` is *Just* a pointer, as `s2` only has the value of a reference to `s`. `s3` on the other hand, is of a *Slice* type, meaning it has a reference pointer *and* a size value.

<br><br>
## Question 2
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

##### Answer
This program **does not** compile. This is because `s.as_bytes()` produces an **immutable** reference to `s`, and it is illegal to mutate `s` inside the for-loop.