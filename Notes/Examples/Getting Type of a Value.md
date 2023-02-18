### Done
Sometimes you want to find out what type a variable holds so that you can ensure it is acting as you expect, or perform some other operation on it. For this you can use this syntax

# Example
```Rust
// main.rs

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let s = "Hello";
    let i = 42;
    let j: u8 = 42;

    print_type_of(&s); // &str
    print_type_of(&i); // i32
	print_type_of(&j); // u8
    print_type_of(&main); // playground::main
    print_type_of(&print_type_of::<i32>); // playground::print_type_of<i32>
    print_type_of(&{ || "Hi!" }); // playground::main::{{closure}}
}
```

### Output
```
&str
i32
u8
playground::main
playground::print_type_of
playground::main::{{closure}}
```

# Breakdown
The only real thing we pay attention to here is the `print_type_of` function that we've defined. ``

Looking at the definition of the function

```Rust
fn print_type_of<T>(_: &T) {
```

We can see it uses `<T>` as its type, meaning that it can take *any* type. This is also specified in the parameters with `(_: &T)` which means we are being passed a `reference` of `T`.

Inside the function itself is this

```Rust
println!("{}", std::any::type_name::<T>())
```

This is a print statement that takes our `<T>` as a parameter. The value is assessed as a member of `std::any::type_name::<T>` which is a std library.