### [Done] Intro
Oftentimes, you want to change the value of a variable inside a function. Normally in Rust, you cannot do this, if you were to, say, just pass the variable in as a normal parameter, even if the variable is declared as `mutable`. In order to actually edit our variable's value, we have to pass it in as a `mutable reference`.
# Program
```Rust
// main.rs

fn main(){
	let mut x: i32 = 5;
	
	println!("Value of x is: {}", x);
	
	change(&mut x);
	
	println!("Value of x is: {}", x);
}

fn change(val: &mut i32){
	*val = 20;
}
```

### Output
```
Value of x is: 5
Value of x is: 20
```
# Breakdown
There's a couple things going on here

# Declare variable as `mut`
The first is that we have to declare our variable `x` as `mut`.

```Rust
let mut x: i32 = 5;
```

If we don't do this, we won't be able to mutate it ever, regardless of whether we're in a function or not.

# Bring variable in as `reference`
The second is regarding our function's parameters. 

```Rust
fn change(val: &mut i32)
```

Notice here how we define the parameter `val` as a `&mut i32`. This defines it as a `mutable reference`, which means it takes a `reference` to a variable, and declares it as `mutable` within the scope of this function, and thus its value can be edited. 

# Pass variable in as `mut reference`
This means that we also have to `pass the value in` as a `&mut`.

```Rust
change(&mut x);
```

But we're still not done. There's still one piece that's easy to overlook. In order to actually *change* this variable, we have to `dereference` it with the `*` operator.

```Rust
*val = 20;
```

This accesses the actual *address* of this reference, and allows us to change its value.