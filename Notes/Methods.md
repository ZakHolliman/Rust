Methods are similar to functions in that they are declared with `fn` and have a name, have parameters and return values, and contain code that runs when the method is called.

Unlike functions, however, methods are defined within the context of a complex data structure such as a Struct or Enum.

Let's say we have a struct called `Rectangle` and an `area` function that takes a reference to this Rectangle, and returns its area

```Rust
#[derive(Debug)]
struct Rectangle { // Declare a struct with values `width` and `height`
	width: u32,
	height: u32,
}
```

We can add a method to this struct by adding an `impl` (Implementation block) to `Rectangle`.

```Rust
impl Rectangle {
	fn area(&self) -> u32 { // Define a function `area` that only takes a reference to the struct it is implemented onto. Returns a u32.
	
		self.width * self.height // Return width * height
	}
}
```

This defines a block of code where everything inside is associated with Rectangle. Note how it does not need to be contained within the struct itself, but instead acts on its own, and is simply attached to the `Rectangle` struct.

```Rust
#[derive(Debug)]
struct Rectangle{...} // Here we declare the Rectangle struct

impl Rectangle {...} // Here we attach a method to the Rectangle
```

We can then use this method by using the dot syntax

```Rust
fn main(){
	let rec1 = Rectangle {
		width: 30,
		height: 50,
	};
	
	println!(
		"The area of the rectangle is {} square pixels",
		rect1.area()
	);
}
```

In this `area` method that we defined, notice how we defined the parameter as `&self`. This is because, just as if we passed `&Rectangle`, we do not want to take ownership of the data being passed in. We simply want to read it, do an operation with it, and then return it.

### Method Mutation
If we *did* want to mutate the data, we could call the `self` with the `&mut` keyword. For this example we'll use a `Square` struct.

```Rust
impl Square {
	// -- Snippit --

	fn set_width(&mut self, width: u32){ // `self` is called with `&mut`, specifying that we are going to want to mutate the data
		self.width = width;
	}
}
```

And when we instantiate our Square, we add the `mut` keyword.

```Rust
fn main() {
	let mut sq1 = Square { // `mut` is also added here to create the `sq1` variable as mutatable in the first place
		width: 30,
	};
	
	// -- Snippit --
}
```

Now that we've done this, we can use it in an actual program :

```Rust
#[derive(Debug)]
struct Square {
	width: u32,
}

impl Square {
	fn area(&self) -> u32{
		self.width * self.width
	}

	fn set_width(&mut self, width: u32) {
		self.width = width;
	}
}

fn main() {
	let mut sq1 = Square {
		width: 30,
	};

	println!("The area of the square is {} square pixels", sq1.area()); // Call read-only area method
	
	sq1.set_width(5); // Use mutating method
	
	println!("The area of the square is {} square pixels", sq1.area()); // Call read-only area method
}
```

And this would print :

```
The area of the square is 900 square pixels
The area of the square is 25 square pixels
```

So here we can see how we have mutated the data of the struct through a method.

### Methods Taking Ownership
Methods *can* take ownership of the passed paramter, but it is rare, usually reserved for when we want to change an instance of one struct into another.

As an example, take a look at the program below.

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Square {
    length: u32,
}

impl Rectangle {
    fn make_square(self) -> Square {
        let square = Square {
            length: self.width,
        };

        square
    }
}

fn main(){
    let rect = Rectangle{
        width: 50,
        height: 30,
    };

    println!("{:?}", rect);

    let square = rect.make_square();

    println!("{:?}", square);
}
```

This program defines a `Square` and `Rectangle` struct and attaches a `make_square()` method to `Rectangle` that turns the `Rectangle` into a `Square`. In the `main` we first instantiate a `Rectangle`, then print it to the console. After that it then creates a `Square` with the `make_square()` method that we defined and attached to `Rectangle`, and then calls the `println!` again on the `square`. With this, we can see how Rust treats the ownership of `Rectangle` and `Square` before and after the method call.

##### Rectangle and Square declaration
Here we've got two structs `Rectangle` and `Square` that we've defined.

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Square {
    length: u32,
}
```

##### make_square
and following this, we have an `impl` block attached to Rectangle

```Rust
impl Rectangle { // Rectangle impl block
    fn make_square(self) -> Square { // Method that takes a parameter of `self` and returns a `Square`
    
        let square = Square { // Create a new `Square` type and set its length to self.width
            length: self.width,
        };

        square // Return the square
    } // Here we leave the method, unscoping our `self` and thus deprecating it, making it unable to be used again.
}
```

In this block, we take a direct value of `self` rather than a reference `&self`. This passes the ownership of our attached `Rectangle` to the method itself.

Inside the method itself, we simply create a new `Square` whose side length is the `width` of our current Rectangle, and then return it.

##### main
Once we leave this method, we return the new `square` type and unscope the `self`. This results in our new 

```Rust
fn main(){
	    let rect = Rectangle{ // Instantiate Rectangle
        width: 50,
        height: 30,
    };

    println!("{:?}", rect); // Call `println!` on rect

    let square = rect.make_square(); // Use our method to convert the `Rectangle` into a `Square`, destroying the `Rectangle` object

    println!("{:?}", square); // Call `println!` on square
}
```

And we can see how this code prints what we expected

```
Rectangle { width: 50, height: 30 }
Square { length: 50 }
```

We can see that the program actually is destroying our `Rectangle` type after the method call to `make_square` if we try to print `rect` *after* calling it, like so.

```Rust
fn main(){
    let rect = Rectangle{
        width: 50,
        height: 30,
    };

    let square = rect.make_square();

    println!("{:?}", rect);
    println!("{:?}", square);
}
```

and we get this *Borrow* error

```Rust
error[E0382]: borrow of moved value: `rect`
  --> src/main.rs:30:22
   |
23 |     let rect = Rectangle{
   |         ---- move occurs because `rect` has type `Rectangle`, which does not implement the `Copy` trait
...
28 |     let square = rect.make_square();
   |                       ------------- `rect` moved due to this method call
29 |
30 |     println!("{:?}", rect);
   |                      ^^^^ value borrowed here after move
   |
note: this function takes ownership of the receiver `self`, which moves `rect`
  --> src/main.rs:13:20
   |
13 |     fn make_square(self) -> Square {
   |                    ^^^^
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)

For more information about this error, try `rustc --explain E0382`.
error: could not compile `testing` due to previous error
```

This error tells us that we moved `rect` due to our method call, and never returned it, thus deprecating it. So when we try to call it with `println!` again after this method call, it is deprecated.

##### Method Name Sharing
We can also make a method that shares the name of a value stored in the same struct.

```Rust
impl Rectangle {
	fn width(&self) -> bool {
		self.wdith > 0
	}
}

fn main() {
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};
	
	if rect1.width() {
		println!("Rectangle has a nonzero width of {}", rect1.width);
	}
}
```

In this example, we can see how the `impl Rectangle` block implements a method called `width` which is the same as a value that `Rectangle` already implements. When we get to the `main` of the program, we see 

```Rust
if rect1.width(){ // Calls the `width()` method
	println!("Rectangle has a nonzero width of {}", rect1.width); // Uses the `width` variable
}
```

Since our `width` is followed by a `()`, the compiler knows that we want to use the `width()` method attached to `Rectangle`. If we were to instead not use the `()`, it would look for the variable named `width`. This can be used to easily access data values, such as creating a Getter function, that allows data to be kept private, but still interfaced with and read by API calls.

### Associated Functions
All functions defined within an `impl` block are called Associtaed Functions, because they are associated with the type named after `impl`. A function is considered an associated function if it is attached to a type, but does not have `self` as its first parameter. These are often used for constructors that will return a new instance of a struct.

E.g.
```Rust
impl Rectangle {
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}
```

In this function, we define a function inside the `impl Rectangle` block, but do not reference `self`, so it's considered an associated function. To call this associated function, we would use the `::` syntax.

```Rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main(){
    let rect = Rectangle::square(5);

    println!("{:?}", rect);
}
```

and this would print

```
Rectangle { width: 5, height: 5 }
```

