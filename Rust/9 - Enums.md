Enums, or Enumerations, allow you to define a type by enumerating possible variants of the type.

### Declaring an Enum
Enums are a way of saying that a value is one of a possible set of values.

To declare the enum, we use the `enum` keyword, followed by the name of the enum, and then a `{}` contained list of possible variants. Here's an example.

```Rust
enum IpAddrKind {
	V4,
	V6,
}
```

This `IpAddrKind` is now a custom data type, and when we initialize it, we can set it equal to one of the variants we've specified : `V4` or `V6`.

### Instantiating an Enum
You can create an instance of an enum variable like this

```Rust
let four: IpAddrKind = IpAddrKind::V4;
let six: IpAddrKind = IpAddrKind::V6;
```

The Enum is declared as a type after the name of the variable, just as any other type would be. Each value under the enum is namespaced using the `::` operator, signifying that they are instances of the larger Enum type of `IpAddrKind`.

### Enum Function Parameters
Because these are all classed under the same enum type, we can do something such as declaring a function that takes a `IpAddrKind` type, but since each value inside the enum is namespaced under the larger `IpAddrKind` enum type, we can send any of these variables into the function.

```Rust
fn route(ip_kind: IpAddrKind) {}
```

We could pass this function either `four` or `six`, as both fall under the `IpAddrKind` enum type.

### Using Enums in Structs
We can also store an enum inside of a `struct`, just like any other type. So far, we've only stored what *type* of IP we have, but we don't know the actual data, so we could make something like a struct to store what *kind* of address we have and what the actual *address* is.

```Rust
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

fn main() {
	let home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};
	
	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};
}
```

Here, we've declared two structs, `home` and `loopback` that both have a `kind` which is represented by our enum, and an associated data type `address` which is a string. This would work for keeping track of what data a `V4` or `V6` type would represent, but we could make it better.

### Storing Data in Enums
In the above example, we have our String in `address` separate from the address `kind`, but we can actually store the String data directly *in the enum variant itself*.

We can do this by putting a `()` next to the enum variant.

```Rust
enum IpAddr {
	V4(String),
	V6(String),
}
```

We can now instantiate these enum values with data

```Rust
fn main(){
	let home = IpAddr::V4(String::from("127.0.0.1"));

	let loopback = IpAddr::V6(String::from("::1"));
}
```

In this version of the program, we attach each variant directly to the enum value, so there is no reason to create the struct.

We can also see closer how enums work. We can see that the enum name also becomes a function that constructs an instance of the enum. In this example, `IpAddr::V4()` is a function call that takes a `String` argument, and returns an instance of `IpAddr`.

### Storing Different Types of Data in an Enum
Enum variants can also store different types of data. For example, in our `IpAddrKid` Enum, we could define it like so

```Rust
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}
```

Now we've defined `V4` as being four 8-bit integers, and still kept `V6` as a String. We can then instantiate these with different values like so

```Rust
enum IpAddr {
	V4(u8, u8, u8, u8),
	V6(String),
}

fn main() {
	let home = IpAddr::V4(127, 0, 0, 1);

	let loopback = IpAddr::V6(String::from("::1"));
}
```

We don't have to stop at primitve data types, though. Enums can store *any* data type including Strings, Structs, or even other Enums.

### Complicated Enums
Let's look at another, more complicated enum example.

```Rust
enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
}
```

Here we've defined an Enum called `Message` that stores a few different variants, all with different data.

We could *technically* define multiple structs to get this same functionality, like so

```Rust
struct QuitMessage;
struct MoveMessage {
	x: i32,
	y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
```

But now if we wanted to pass any one of these into the same function, it would be difficult and obtrusive to work around the fact that they all exist as separate `struct` entities. With an `Enum` we can still define individual instances of these, but they all stay under the same `Message` name.

### Enum Methods
We can also define methods onto `Enums`, just as we did with `Structs`, using the `impl` keyword.

```Rust
impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}
```

And we can then call it like any other method.

```Rust
fn main(){
	let m = Message::Write(String::from("hello"));
    m.call();
}
```

### The 'Option' Enum
The `Option` Enum is a common and useful Enum that is defined by the standard library, to be used for the scenario in which a given value could be *something* or *nothing*.

For example, if you request something such as a list of items, and it has items in it, it will return the list of items, but if you request it and it's empty, it will return nothing. The `Option` Enum is used to ensure that, in either case, the scenario is accounted for in the program and everything can continue smoothly.

The `Option` Enum is defined in the standard library as

```Rust
enum Option<T> {
	None,
	Some(T),
}
```

It's so common that you can use `Option<T>` without importing the main library, and can even simply use `None` or `Some(T)` on their own without the `Option::` scoping.

Here's an example of using the `Option` Enum.

```Rust
fn main(){
	let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
```

Here, the `some_number` type is `Option<i32>` and the `some_char` is `Option<char>`. These types are automatically inferred by the compiler, so they don't have to be specified. For `absent_number`, however, the compiler doesn't know what it's going to expect just because of the `None` type, so we have to specify it as `Option<i32>`.

When we have a `Some` value, we know that a value is stored in that `Some`. Likewise, when we have a `None`, we know that nothing is there. It essentially means the same as null, but allows for greater error handling.

### The Death of Null
This has a few large advantages over simply just having a null value.

For one, we cannot use `Option<T>` directly as a value. If we were to try to do something like this, for example

```Rust
fn main(){
	let x: i8 = 5;
	let y: Option<i8> = Some(5);

	let sum = x + y;
}
```

Then we would get a large compiler error telling us that it does not know how to add `Option<i8>` and `i8` together. In the event that `y` was actually a `None` or "null" value, then this would have caused the program to crash.

Since we can never do operations on `Option<T>`, and must always convert `Option<T>` into `T`, it ensures that our value is always valid which eliminates a big problem that is always had with null, which is assuming that it's valid when it isn't.

In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value `Option<T>`. Then, when you use that value, you are required to explicitly handle the case when the value is null. Everywhere that a value has a type that isn’t an `Option<T>`, you _can_ safely assume that the value isn’t null. This was a deliberate design decision for Rust to limit null’s pervasiveness and increase the safety of Rust code.

In order to actually get a value out of `Option<T>`, there are a number of attached methods that you can call.

# Questions
### Question 1
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

```Rust
fn foo(x: &i32) { 
  println!("{x}");
}

fn main() {
  let x = null;
  foo(x);
}
```

##### Answer
No, Rust does not have null pointers, so `null` does not exist.

### Question 2
Consider these two representations of a Result type that contains a value T if a computation succeeds, or an error E if it fails.

```Rust
struct Result1<T, E> {
  ok: Option<T>,
  err: Option<E>,
}

enum Result2<T, E> {
  Ok(T),
  Err(E)
}
```

The enum `Result2` is considered more idiomatic than the struct `Result1` in Rust. Which statement below is NOT a valid reason why?
* The struct uses more space in memory at runtime than the enum
* The struct is more syntactically verbose to construct than the enum
* The struct could have `ok` and `err` both be `None`, while the enum must be at least one of them
* The struct contains `Option` types, which are only intended to wrap structs

##### Answer
D. It's okay to have structs contain `Option` types as a field, but if you are trying to construct a situation where you have two `Option` enum fields, and one must always be `Some`, then it's better to use an enum.

