~~~~~ References and Borrowing ~~~~~
We've seen how to pass a variable by value, but we can also pass it by
    reference. Unlike a pointer, a reference is guaranteed to point to a valid
    value for the life of that reference.

    fn main(){
        let s1 = String::new("hello");  // Init a new 's1' variable and
                                    // initialize it to a String dynamic type

        let len = calculate_length(&s1);    // Init another new variable 'len'
                                    // whose value is the length of s1. s1 is
                                    // passed as a reference, denoted by the '&' 

        println!("The legth of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

Note how in this code we pass the variable s1 with an ampersand (&). This is
    passing the variable s1 as a REFERENCE, meaning that we can refer to its
    value in the funciton without taking ownership of it.

In this piece of code

    let len = calculate_length(&s1);

We use "&s1" to refer to our reference. This refers to the value of s1, but does
    not own it, so the value is not dropped when this function exits.

Inside the function, we also declare the parameter with a & to indicate that we
    need the variable to be passed as a reference.

    fn calculate_length(s: &String) -> usize { // s is a reference to a String
        s.len()
    }   // Here, s goes out of scope. But because it does not have ownership
    // of what it refers to, it is not dropped.

We call the action of creating a new reference "borrowing". Just like in real
    life, if you borrow something, you don't own it and have to give it back.

So what if w try to modify something we're borrowing?

    fn main(){
        let s = String::from("hello");

        change(&s);
    }

    fn change(some_string: &String) {
        some_string.push_str(", world!");
    }

This will throw an error when we try to compile. The compiler will tell us that
    we are trying to modify a reference variable, so just like variables are
    immutable by default, so are References



Question 1
Determine whether the program will pass the compiler. If it passes, write 
    the expected output of the program if it were executed.
    
    fn print(s: &String) {
      println!("{s}");
    }

    fn main() {
      let mut s = String::from("hello");
      print(&s);
      s.push_str(" world");
      print(&s);
    }

This will compile. It will print this

    hello
    hello world

The references to "s" do not last longer than the call to "print", so it's valid
    to mutate it between calls.





Question 2
Consider a move of a value of type &String versus a value of type String. The 
    move of &String will copy:

Response
    * The same number of bytes
    * More bytes
    * Fewer bytes

The &String will allocate LESS BYTES than the String itself

The String struct contains a pointer, length, and capacity. The &String type is
    just a pointer, so it's a shallow copy.



~~~ Mutable References ~~~
If we *do* want to mutate a reference, we can pass a "mutable reference" to
    allow a function to edit the value.

    fn main(){
        let mut s = String::from("hello");  // Create mutable string 's'

        change(&mut s); // Call 'change' function with a mutable ref of s
    }

    fn change(some_string: &mut String) { // Accepts a reference of a mutable
                                            // String
        some_string.push_str(", world");    // Edits the string
    }

Since a reference is immutable by default, the way we add "mut" as a keyword
    explicitly denotes that we are going to be changing the reference's value
    inside the function.

Also note the difference between using the "&" operator and the "&mut".

The expression "&s" creates an IMMUTABLE reference of type "&String"
The expression "&mut" creates a MUTABLE reference of type "&mut String"

Mutable references do have one restriction, however, in that you can only
    create one mutable referenec at a time.

For instance, this code:

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

Will give the following error.

    $ cargo run
    Compiling ownership v0.1.0 (file:///projects/ownership)
    error[E0499]: cannot borrow `s` as mutable more than once at a time
    --> src/main.rs:5:14
    |
    4 |     let r1 = &mut s;
    |              ------ first mutable borrow occurs here
    5 |     let r2 = &mut s;
    |              ^^^^^^ second mutable borrow occurs here
    6 | 
    7 |     println!("{}, {}", r1, r2);
    |                        -- first borrow later used here

This error tells us that the code is invalid, as we cannot borrow "s" as
    mutable more than once. This restriction allows us to still make references
    to variables, but in a more controlled manner. The benefit of this is
    preventing "data races" at compile time, where two separate parts of a
    program try to edit data at the same time.

As before, we can use curly braces to scope multiple mutable references, but
    not SIMULTANEOUS mutable references.

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, allowing for a new reference

    let r2 = &mut s;

There is also a similar rule for combining mutable and immutable refs

    let mut s = String::from("hello");

    let r1 = &s; // Fine
    let r2 = &s; // Fine
    let r3 = &mut s; // <-- Not allowed

    println!("{}, {}, and {}", r1, r2, r3);

This will give the following error

    $ cargo run
    Compiling ownership v0.1.0 (file:///projects/ownership)
    error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    --> src/main.rs:6:14
    |
    4 |     let r1 = &s; // no problem
    |              -- immutable borrow occurs here
    5 |     let r2 = &s; // no problem
    6 |     let r3 = &mut s; // BIG PROBLEM
    |              ^^^^^^ mutable borrow occurs here
    7 | 
    8 |     println!("{}, {}, and {}", r1, r2, r3);
    |                                -- immutable borrow later used here

We also cannot have a mutable reference while we have an immutable one of the
    same value. Users of immutable data don't expect them to randomly 
    change, but multiple readers of the same data are fine, as they are not
    altering the data.

A reference's scope starts where it is introduced, and continues until the last
    time it is used.

This code, for example

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

WILL compile, because our last usage of the immutable refs comes before the
    mutable ref is created.



Question 1
Determine whether the program will pass the compiler. If it passes, write the 
    expected output of the program if it were executed.
    
    fn incr(n: &mut i32) {  // Takes a mutable i32 reference
        *n += 1;    // Dereferences the value to += 1?
    }

    fn main() {
        let mut n = 1;  // Creates value n
        incr(&n);   // Passes n as a ref
        println!("{n}");    // Print
    }

This will NOT compile. Even though n is marked as "mut", the reference to n
    must also be marked as "mut".



Question 2
Determine whether the program will pass the compiler. If it passes, write the 
    expected output of the program if it were executed.
    
    fn main() {
        let mut s = String::from("hello");  // Define mut string
        let s2 = &s;    // create immut ref to s
        let s3 = &mut s;    // Create a mut ref to s
        s3.push_str(" world");  // Last usage of s3
        println!("{s2}");   // Last usage of s2
    }

This will NOT compile. The scope of s3 eclipses s2, and thus throws an error.



~~~ The Rules of References ~~~
Recap of references
    * At any given time, you can have EITHER ONE MUTABLE reference or ANY NUMBER
        OF IMMMUTABLE references
    * References must always be valid