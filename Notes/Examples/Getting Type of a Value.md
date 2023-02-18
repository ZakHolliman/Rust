Sometimes you want to find out what a variable's type is so that you can ensure it is acting as you expect. For this you can use this

```Rust
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

Output
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

Looking at it, we can see it comes from the `std` library and 