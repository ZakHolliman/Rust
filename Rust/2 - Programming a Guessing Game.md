This chapter is going to work with a hands on project to learn more about Rust. It'll introduce a lot of important concepts such as let, match, methods, associated functions, and more.

It'll be a guessing game, where a number is randomly generated, and then a user has to guess what the number is.

### Setting up a new project
To set up a new project, go to projects and make a new project with Cargo

```Rust
$ cargo new guessing_game
$ cd guessing_game
```

`cargo new` takes the name of a project as the first argument, and creates a new Cargo directory for it.

### Processing a Guess
The first part of this program is going to be asking a user for input and storing a guess, which looks like this :

```Rust
use std::io;

fn main(){
	println!("Guess the number!");

	println!("Please input your guess!");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {guess}")
}
```

To start off, we have the line

```Rust
use std::io;
```

This indicates that we are goign to be importing and using the standard Rust library.

Next is the line

```Rust
fn main()
```

The `fn` indicates that this is a funciton, the rest is pretty standard.

After that we have

```Rust
println!("Guess the number!");
println!("Please input your guess.");
```

Which looks familiar, but note the `!` at the end of the line. This indicates that we are actually using a `macro` instead of a function, which we'll get into later, but is important to know for now.

### Storing Values with Variables
We can create a variable by using the `let` keyword.

```Rust
let x = 5;
```

Variables are *immutable* by default in Rust, meaning their values cannot be altered.

If we tried to alter the variables value, we would get this error

```Rust
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     x += 1;
  |     ^^^^^^ cannot assign twice to immutable variable
```

This error tells us that we are trying to assign a value to an immutable variable that has already been assigned.

In order to actually be able to change the data's value, we use the `mut` keyword.

```Rust
let mut x = 5;
```

With this, we can now mutate the data in the variable freely

```Rust
fn main(){
	let mut x = 5;
	x += 1;
}
```

This will compile fine.

Knowing this, we now know what the next line of code does.

```Rust
let mut guess = String::new();
```

We create a `guess` variable of type `String` that is mutatable.

### Receiving User Input
Our next piece of code looks like this

```Rust
io::stdin()
	.read_line(&mut guess)
```

This calls the `read_line` method which grabs user input