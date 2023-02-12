`match` is an extremely powerful flow control construct in Rust. It allows you to compare a value against a series of patterns and then execute code based on the pattern's match. `match` is extremely useful in that it has a wide diversity of patterns, as well as safety in the compiler knowing that all possible cases are handled.

### Match Syntax
Here's an example of a `match` statement.

```Rust
enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => 1,
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter => 25,
	}
}
```

Let's break down this `match` statement. First, we list the `match` keyword, followed by an expression. In this case we have the value `coin`. This is similar to an `if` statement, except in an `if` statement, the expression must return a boolean, but here it can be any type.

### Arms
Next we have what are called `arms` in the match statement. Arms are made of two parts: A pattern and some code. The first `arm` that we have looks like this

```Rust
Coin::Penny => 1,
```

This has a patttern of `Coin::Penny` that we want to match against. The `=>` separates the pattern from the code that we want to run. In this case, the only code we want to run is the value `1`. We separate these `arms` with a comma.

When a `match` statement executes, it compares the given value against each of its arms, in order. If a pattern in an arm matches the value, the arm is executed.

The code in each arm is an expression, and the resulting value of that expression in the arm is the value that is returned for the entire `match` expression. If you want to run multiple lines of code in one arm, you can add curly braces.

```Rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### Match Variable Binding
`match` statements can also bind to the parts of the values that match the pattern. This is how we extract values out of enum variants.

To show this, we'll expand our `Coin` enum to include a US state for quarters.

```Rust
#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
	// ...
}

enum Coin {
	Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
```

and then say we were trying to collect quarters with all 50 states.

In our match statement, we can add a value to our `Coin::Quarter` arm that will match to the state value of the enum value, we'll call this `state`. When our `Coin::Quarter` is matched by the code, and thus that arm is executed, this `state` variable will bind to the `UsState` value attached to `Coin::Quarter`. We can then use this in the code executed by the `match` statement.

```Rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

Now, when we pass a quarter into this match statement and get to the `Coin::Quarter` arm, we can bind its value to our arm's and use it in the arm's code.

### Handling `Option<T>`
We can use a `match` statement to extract the `T` value from a `Some` enum that is created using `Option<T>`.

```Rust
fn get_value(x: Option<i32>) -> i32 {
    match x {
        None => {
            println!("Err");
            0
        },
        Some(i) => i,
    }
}

fn main(){
    let mut test: Option<i32> = Some(5);

    println!("{}", get_value(test));
}
```

This `get_value` function will take in an input of `Option<i32>` and return only the `i32` value. If there is a value, we return it. If there is no value, we return `0` and print `"Err"`.

Let's say we wanted to write a funciton that takes an `Option<i32>`, and if there's a value, adds 1 to that value. If there is no value, then it does nothing.

```Rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

Stepping through this, we can first look at the `plus_one` function. It takes a parameter that we call `x` that is of type `Option<i32>`, and returns another `Option<i32>`.

Then inside is our `match x` statement, which uses `x` as a value to match all of its arms against.

For the line

```Rust
let six = plus_one(five);
```

The `match` statement will use `five` as `x`. In the first arm, `None` does not match `Option<i32>`, so it moves on. Next is the arm `Some(i)` which *does* match `Option<i32>`, so this code executes, binding `i` to the `i32` value stored in our `x`. In this case, it creates a new `Some` value using this `i` and makes it equal to `i + 1`, and then returns it.

`match` statement arms must cover *all possibilities* for a piece of code. If they don't, then the compiler will throw an error.

E.g. in this code, we don't handle the `None` case, so it does no compile

```Rust
fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		Some(i) => Some(i + 1),
	}
}
```

### Default Cases
In a `match` statement, we can also cover special cases, and then leave all other cases to a default. Say we were rolling a dice, and when we roll a `5`, then we've defined that as a lucky number, and something special happens. We can make that `match` statement like so

```Rust
fn main() {
	let dice_roll = 4;
	
	match dice_roll {
		5 => println!("Lucky number!"),
		other => println!("{:?}", other),
	}
}
```

Instead of having to list all of the other numbers that would effectively do the same thing, the `other` pattern covers all of these for us in one arm.

### Catch-all Field
Rust also has a pattern that we can use as a catch-all when we don't want to actually use the variable which is the `_` character. `_` is a special pattern that matches any value and does not bind to that result.

```Rust
fn main() {
	let dice_roll = 4;
	
	match dice_roll {
		5 => println!("Lucky number!"),
		_ => println!("Roll again!"),
	}
}
```

### Ownership with Matches
If an enum contains non-copyable data such as a String, then we should take special note how how these values are being passed into our `match` functions.

For  example, this will compile.

```Rust
fn main() {
	let opt: Option<String> = Some(String::from("Hello"));

	match opt {
		Some(_) => println!("Some!"),
		None => println!("None!"),
	}

	println!("{:?}", opt);
}
```

Because we have our `Some(_)` and `None` defined, and `_` is a placeholder for "any" within that `Some` type. If we were to instead use something like `s` 

```Rust
fn main() {
	let opt: Option<String> = Some(String::from("Hello world"));
	
	match opt {
	    // _ became s
	    Some(s) => println!("Some: {}", s),
	    None => println!("None!")
	};
	
	println!("{:?}", opt);
}
```

This would not compile. We first declare a normal enum `opt`, and it is the *value* of `Option<String>` not a reference like `&Option<String>`. This means that when the value would be read in by the `match opt` statement, it will *move* the data. Since the data is moved, `opt` is no longer valid, and when we get to the `println!` statement, it will be illegal to use `opt` there.

If we did want to use `opt` without moving things, we would use a reference, like so

```Rust
fn main() {
	let opt: Option<String> = Some(String::from("Hello world"));
	
	match &opt {
	    Some(s) => println!("Some: {}", s),
	    None => println!("None!")
	};
	
	println!("{:?}", opt);
}
```

When we pass the entire enum as a reference like this, all of the enum's values also inherit the reference tag, so `s` has the type `&String`, and `opt` can be used after the match.

# Question 1
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

```Rust
enum Location {
	Point(i32),
	Range(i32, i32)
}

fn main() {
	let l: Location = Location::Range(0, 5);
	let n = match l {
		Location::Point(_) => -1,
		Location::Range(_, other) => other,
		Location::Range(0, _) => 0,
		_ => -2
	};
	
	println!("{n}");
}
```

##### Answer
Does compile. We use a `match` on `l` to return a value that `n` takes. The match's arms are read from top to bottom, and it finds a pattern that matches on the second arm, `Location::Range(_, other) => other,` so it uses that and returns `other` which is the second number in our `Range(i32, i32)` pair.

# Question 2
Consider this method implemented for the `Option` type:

```Rust
impl<T> Option<T> {
	fn unwrap_or(self, other: T) -> T {
		match self {
			Some(t) => t,
			None => other
		}
	}
}
```

Which sentence best describes the behavior of this function?
* Returns a new option containing the object inside `self` if it exists, and `other` otherwise
* Returns the object inside `self` if it exists, and `other` otherwise
* Returns a reference to the object inside `self` if it exists, and `other` otherwise
* Inserts `other` into `self` if `self` does not already contain a value

##### Answer
B. This function "unwraps" the option by taking ownership of it and retrieving the value inside, but if no value exists, it falls back by returning `other`.

# Question 3
Determine whether the program will pass the compiler. If it passes, write the expected output of the program if it were executed.

```Rust
#[derive(Debug)]
enum Either {
	Left(usize),
	Right(String)
}

fn main() {
	let x = Either::Right(String::from("Hello world"));
	let value = match x {
		Either::Left(n) => n,
		Either::Right(s) => s.len()
	};
	
	println!("{x:?} {value}");
}
```

##### Answer
Does NOT Compile. The match arm `Either::Right(s)` moves the filed `s`, so `x` cannot be used in the `println!` statement.

# Question 4
Consider these two implementations of a function to decrement an unsigned number twice.

```Rust
fn decr_twice_v1(n: u32) -> Option<u32> {
	match n {
		0 => None,
		1 => None,
		n2 => Some(n2 - 2)
	}
}

fn decr_twice_v2(n: u32) -> Option<u32> {
	if n == 0 {
		None
	} else if n == 1 {
		None
	} else {
		Some(n - 2)
	}
}
```

The functions have the same behavior for:
* No inputs
* All inputs
* Some, but not all inputs

##### Answer
All inputs. The `match` and `if` perform the same function here.