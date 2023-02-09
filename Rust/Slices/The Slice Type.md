Slices in Rust let you reference contiguous spaces of memory without accessing the **entire** memory space of the data. 

❗❗❗ **Slices are a type of REFERENCE, so they DO NOT take ownership.**

The most common type of slice is a *String Slice* which lets you access a portion of a given string. If we were to declare a string such as this.

```Rust
let s = String::from("hello world");
```

Then in memory, this is stored as

| 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 |
| - | - | - | - | - | - | - | - | - |  - | - |
| h | e | l | l | o |   | w | o | r |  l | d |

And we can see that the starting index of "hello" is 0 (inclusively), and the ending is 5 (exclusively). The same applies to "world" where the starting index is 6 and the end is 11.

Using the range syntax, we can specify these ranged of the strings

```Rust
let hello = &s[0..5];
let world = &s[6..11];
```

And easily index the part of a string that we want, without referencing the entire thing.

A Slice, as a data structure, consists of a **pointer** and a **size** value. The pointer points to the start of the slice, and the size tells it how far from the pointer's start we go to get the full slice. In the example above, we would now have two **Slice** data structures, one assigned to each variable.

The "hello" slice would look like :

| pointer | size |
|-|-|
| 0 | 5 |

and the "world" slice would look like :

| pointer | size |
|-|-|
| 6 | 11 |

Of course, in real memory, the pointer addresses are more complex, but the core concept remains the same.

# Other Slice Types

You can also use Slices to index other data types, such as an array

```Rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2..3]);
```


❗❗❗ **Here, the slice has the TYPE of : &[i32]**

It works the same way a string slice does, by storing a reference to the first element, along with a length.

