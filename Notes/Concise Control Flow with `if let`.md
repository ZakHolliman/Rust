The `if let` syntax lets you combine `if` and `let` into a single, less verbose way to handle `values that match one pattern, while ignoring the rest`.

<br><br>
## Example `match`
Let's take an example program

```Rust
// main.rs

fn main() {
	let config_max = Some(3u8);

	match config_max {
		Some(max) => println!("The maximum is configured to be {}", max), 
		_ => (),
	}
}
```

Output

```
The maximum is configured to be 3
```

Here we first declare a variable `config_max` using an `Option<T>`. The value of this is `Some(3u8)` which means we want the value to be `3` stored as a `u8` type. Next is the `match` statement. We pass `config_max` as the pattern, and it matches the first `Some` pattern, binding the value stored in `config_max` to the `max` variable, and then prints it.

<br><br>
## Switching to `if let`
In this `match` statement, we only want to do something if our `Some` value has something stored in it. In order to satisfy the `match` requirements however, we also have to add `_ => ()`. We can circumvent this by using `if let`.

```Rust
// main.rs

fn main() {
	let config_max = Some(3u8);

	if let Some(max) = config_max {
		println!("The maximum is configured to be {}", max);
	}
}
```

Output

```
The maximum is configured to be 3
```

The syntax `if let` takes a pattern and an expression separated by an equal sign. In our example, `Some(max)` is our pattern and `config_max` is the expression being evaluated. When this arm is matched, `max` is bound to the value stored inside the `config_max` variable that we've passed in.

Choosing `if let` over `match` means less typing and more readable code, but we do `lose the exhaustive pattern checking` that `match` offers.

`if let` should basically be used for when we want a `match` statement that runs code when the `value matches one pattern, and ignores all other values`.

<br><br>
## Using `else` with `if let`
Just like a normal `if` statement, we can also use the `else` keyword with an `if let`.

```Rust
// main.rs

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
```

Output

```
State quarter from Alabama!
```

This is the same as this

```Rust
// main.rs

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
```

Output

```
State quarter from Alabama!
```

<br><br>
# Question 1
Which control flow construct would be most idiomatic to use in the following function?

```Rust
enum Location {
	Point(i32),
	Range(i32, i32)
}

fn print_range_max(loc: &Location){
	// Print the second field of Range, if loc is a Range
}
```

Response
* match
* if let

##### Answer
`if let` should be used, because the function only has an effect in one condition.

<br><br>
# Question 2


```Rust
enum Location {
	Point(i32),
	Range(i32, i32)
}

fn print_range_max(loc: &Location){
	// Return the first field of Range, or the only field in Point
}
```

Response
* match
* if let

##### Answer
This function needs to return a value for each condition, so we should use `match`.
