Enums, or Enumerations, allow you to define a type by enumerating possible variants of the type.

### Declaring an Enum
Enums are a way of saying that a value is one of a possible set of values.

To show this, we'll use an example of IP addresses.

```Rust
enum IpAddrKind {
	V4,
	V6,
}
```

This `IpAddrKind` is now a custom data type that we can use elsewhere.

<br><br>
### Instantiating an Enum
You can create an instance of an enum variable like this

```Rust
let four: IpAddrKind = IpAddrKind::V4;
let six: IpAddrKind = IpAddrKind::V6;
```

Each value under the enum is namespaced using the `::` operator, signifying that they are instances of the larger class `IpAddrKind`.

Because these are all classed under the same enum type, we can do something such as declaring a function that takes a `IpAddrKind` type, but since each value inside the enum is namespaced under the larger `IpAddrKind` enum type, we can send any of these variables into the function.

```Rust
fn route(ip_kind: IpAddrKind) {}
```

We could pass this function either `four` or `six`, as both fall under the `IpAddrKind` enum type.

We can also store an enum inside of a `struct`, just like any other type.

```Rust
struct IpAddr {
	kind: IpAddrKind,
	address: String,
}
```

So far though, we've only stored what *type* of IP we have, but we don't know the type, so we could make something like a struct to store what *kind* of address we have and what the actual *address* is.

```Rust
let home = IpAddr {
	kind: IpAddrKind::V4,
	address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
	kind: IpAddrKind::V6,
	address: String::from("::1"),
}
```

This would work for keeping track of what data a `V4` or `V6` type would represent, but we could make it better.

In the above example, we have our string in `address` separate from the address `kind`, but we can take this further by storing the string data directly *in the enum itself*.

We can do this by putting a `()` next to the enum variant.

```Rust
enum IpAddr {
	V4(String),
	V6(String),
}
```

We can now instantiate these enum values with data

```Rust
let home = IpAddr::V4 (String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

In this version of the program, we attach each variant directly to the enum value, so there is no reason to create the struct.

We can also see closer how enums work. We can see that the enum name also becomes a function that constructs an instance of the enum.
