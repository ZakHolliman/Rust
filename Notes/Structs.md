A *Struct* is a data type that lets you package together multiple variables of various types into a single group.



### Declaring a Struct
Structs are similar to tuples, in that they can both hold mixed data sets, but a key difference is that each piece of data in struct is named. This makes structs more flexible than tuples, as the ordering of the data doesn't matter.

To declare a struct you use the keywword `struct` followed by a comma separated list of fields.

Declaring a tuple :

```Rust
let tuple = (bool, String, String, u64);
```

Declaring a struct :

```Rust
struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}
```



### Instantiating a Struct
To use this struct we can create an instance by specifying concrete values for each field. We do this with key:value pairs.

```Rust
fn main() {
	let user1 = User {
		active: true,
		username: String::from("someusername123"),
		email: String::from("someone@example.com"),
		sign_in_count: 1,
	}
}
```

This will create an instance of the struct `User` with the specified values.



### Accessing
If we wanted to access one of the parameters of this struct, we use the `dot` notation.

```Rust
fn main() {
	let mut user1 = User {
		active: true,
		username: String::from("someusername123"),
		email: String::from("someone@example.com"),
		sign_in_count: 1,
	}

	user1.email = String::from("anotheremail@example.com");
}
```

Note here how we declared the entire instance of the struct to be mutable, instead of one field.



### Returning a Struct
Also, as with any other data type, we can build and return a struct from a function as an expression.

```Rust
fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username: username,
		email: email,
		sign_in_count: 1,
	}
}
```



### Field Init
In our example above, our struct field names and parameter names are exactly the same, which led to a bit of redundancy in typing such as `username: username` or `email: email`. We can skip this redundancy by using a shorthand, simply stating the name of the parameter.

```Rust
fn build_user(email: String, username: String) -> User {
	User {
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}
```



### Struct Update
Sometimes you want to create a new instance of a struct that includes *some* of the values from another instance, **but** changes some of them. To do this, we use the *struct update* syntax.

If we wanted to do this *without* the struct update syntax, we would do something like this.

```Rust
fn main() {
	// -- Snippit --
	
	let user2 = {
		active: user1.active,
		username: user1.username,
		email: String::from("another@example.com"),
		sign_in_count: user1.sign_in_count,
	}
}
```

But with  `struct update` we can do the same thing with less code. 

```Rust
fn main(){
	// -- Snippit --

	let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};
}
```

This code will do the same thing, as the only parameter we change is the `email`, but the `..user1` syntax tells the compiler that we want everything else in this struct to be the same as in user1.

The `struct update` syntax uses the '=' syntax because it *moves* the data to a new variable. This means that `user1` is now *invalid* and cannot be accessed.

In our example, we have **two** strings in our struct. When we assign

```Rust
let user2 = User {
		email: String::from("another@example.com"),
		..user1
	};
```

We reassign the `email` field in the new `user2` variable, but we *move* all of the data from`user1`. The `active` and `sign_in_count` variables in `user1` are both primitive, so they implement `Copy` traits, but we *also* have the `username` field which is a `String`, and when Strings are moved, they are not copied, but instead moved. This in turn invalidates `user1`. If we were to, however, initialize both `email` *and* `username` in our new `user2` struct, then `user1` would remain valid.



### Structs with Unnamed Fields (Tuple Structs)
Rust also has the ability to create something called a "Tuple Struct", which is a struct that looks like a tuple, i.e. it does not have any names for its fields.

Tuple Structs are useful for when you want to give a name to a specific type of struct, but don't want to use the overhead of naming each field.

```Rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
}
```

Here, we first define our `tuple structs` at the top of our program. They are simply 3 parameters each, all `i32` values. After that we can `instantiate` a tuple of each type below in the main as normal.

Note that structs essentially implement their own `type`, i.e. you cannot assign a `Point` struct to a parameter that takes a `Color` struct, even though they are constructed out of the same types.



### Structs Without Any Fields (unit-like structs)
Structs can also not have any fields at all. These are called `unit-like` structs.

`unit-like` structs behave similarly to a `unit ()`. They can be used to implement traits onto a type without actually assigning any data to it.

```Rust
struct AlwaysEqual; // Creates an empty struct

fn main() {
	let subject = AlwaysEqual; // Instantiates the empty struct to a variable
}
```



### Ownership of Struct Data
Generally with a struct, we want it to *own* all of the data that it has instances of, for as long as the struct remains valid.

We can, however, store references to data that is owned by something else using "Lifetimes".
