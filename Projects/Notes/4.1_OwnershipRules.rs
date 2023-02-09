~~~~~ Ownership Rules ~~~~~
There are some things to keep in mind when thinking about rules of ownership
    in Rust.

    * Each value in Rust has an owner
    * There can only be one owner at a time
    * When the owner goes out of scope, the value will be dropped



~~~ Variable Scope ~~~
As a first example of ownerhsip, we'll take a loop at the "scope" of some
    variables. The scope is the range within a program for which an item is
    valid.

    let s = "hello";

The variable "s" here refers to a string literal, which is hardcoded into the
    text of the program to refer to it. This variable is valid until the end of
    the current scope.

    {   // s is not valid here, as it has not been declared yet
        let s = "hello";    // s is valid from this point forward

        // do stuff with s
    }

    // This scope is now over, and s is no longer valid

There are two importantn points in time
    * s comes into scope, and is valid
    * s remains valid until it goes out of scope.



~~~ The String type ~~~
To illustrate the rules of scope, we need to use a data type that is more
    complex than the ones we've seen so far. The types we've used so far all
    have defined sizes, and thus can be easily stored on the stack for easy
    access. To look at how Rust stores data on the heap and cleans that data
    up, we can use something stored on the heap, such as a String.

Many times when using Strings, we will want them to be mutable, or even
    sometimes, such as when gathering input, the String may be undefined for a
    time.

You can create a string from a string literal using the "from" function.

    let s = String::from("hello");

The double colon indicates that "from" is in the namespace of "String".

This type of string can be mutated and changed.

    let mut s = String::from("hello"); // Creates an initial String

    s.push_str(", world"); // Pushes a literal onto the Strig

    println!("{}, s";) // Prints the String

The difference here between the string literal and String type is the way that
    they are stored in memory.



~~~ Memory and Allocation ~~~
In the case of a string literal, we know the contents at compile time, so it
    can thus be stored on the stack. For a String, it is dynamic and is thus
    stored on the Heap.

In Rust, when a dynamic variable is declared, the memory is automatically
    returned once the variable that owns it goes out of scope.

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }

    // The scope is now over, and s is no longer valid

When a variable goes out of scope, Rust calls a special function, "drop" which
    is where the author of "String" can put the code to return the memory.



~~~ Ways that variables and data interact: Move ~~~
Multiple variables can interact with the same data in different ways.

    let x = 5;
    let y = x;

This code creates two variables, binds 5 to "x" and "y" to the value of "x"
    which is 5. Since both of these data types are known, they are both pushed
    to the stack.

If we changed the code to

    let s1 = String::from("hello");
    let s2 = s1;

We now have code that looks similar, but does not behave the same.

In Rust, a String is made up of three parts: ptr, len, and capacity. This data
    is all stored on the Stack.

The "ptr" is the pointer that points to the data that is stored on the heap.
The "len" is how much memory the contents of the string are currently using.
We'll get back to "capacity".

When we says s2 = s1, what we are doing is copying everything in the Stack of
    s1 into s2. This means the pointer ADDRESS, len, and capacity are copied.
    The actual contents of the pointer address are not copied. Instead what
    happens is that we now have two pointers both pointing to the same place
    in the heap.

When a variable goes out of scope in Rust, it calls the "drop" function and
    cleans up the heap memory for that variable. When both of these variables
    go out of scope, it would normally cause an error known as a "double free"
    due to it trying to free the data twice. In order to ensure memory 
    safety then, Rust considers s1 as no longer active once s2 is assigned to 
    it, meaning it will not try to free anything when s1 goes out of scope.

If we were to try to use s1 after s2 was created, it would be invalid.

    let s1 = Sting::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);

This would give an error that tells us that the value was moved, where it was
    moved, and what kind of error that is giving.



~~~ Ways variables and data interact : Clone ~~~
So when trying to copy one variable to another it gets removed, but if we do
    want to deep copy the heap data of String we can use a method called "clone"

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

This will do the desired behaviour and deep copy the heap.

Rust has a special annotation called the "Copy" trait that we can place on
    types that are stored on the stack, such as integers and other rigidly
    defined types.



~~~ Ownership and Functions ~~~
The mechanics of passing a value to a function are similar to assigning a value
    to a variable. Passing a variable will move or copy.

    fn main(){
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the funciton
                                        // and is no longer valid here

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // x would move into the function
                                        // but i32 is Copy, so it's okay to
                                        // still use x after.
    }

    // Here x goes out of scope, then s. But s's value was moved so nothing
    // special happens

    fn takes_ownership(some_string: String){ // some_string comes into scope
        println!("{}", some_string);
    } // Here, some_string goes out of scope and 'drop' is called. The backing
    // memory is freed

    fn makes_copy(some_integer: i32){ // some_integer comes into scope
        println!("{}", some_integer);
    } // Here, some_integer goes out of scope. Nothing special happens

If we try to use "s" after takes_ownership, Rust would throw a compile error.



~~~ Return values and scope ~~~
Returning values can also transfer ownership.

    fn main(){
        let s1 = gives_ownership(); // gives_ownership moves its return value
                                    // into s1
        
        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and...
                                            // which also moves its return value
                                            // into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped

    fn gives_ownership() -> String { // gives_ownership will move its return
                                    // value into the function that calls it

        let some_string = String::from("yours"); // some_string enters scope

        some_string // some_string is returned and moves to the calling function
    }

    // This function takes a String and returns one
    fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                            // scope

        a_string    // a_string is returned and moves out to the calling function
    }

The ownership of a variable follows the same pattern every time:
    Assigning a value to another variable moves it. 
    
When a data that includes data on the heap goes out of scope, it is 'dropped'
    and the value will be cleaned up unless ownership has been moved.


~~~~~ Examples ~~~~~
1.
Determine whether the program will pass the compiler. If it passes, write the
    expected output.

    fn add_suffix(mut s: String){
        s.push_str(" world");
    }

    fn main(){
        let s = String::from("hello");
        add_suffix(s);
        println!("{s}");
    }

This program does not compile. The string "s" is moved into add_suffix after
    calling it, so it cannot be used later.




2.
What is the maximum number of times a heap allocation could occur in the
    following program? Write your answer using digits, like 1 or 2.

    fn print(s: String) {
        println!("{s}");
    }

    fn main() {
        let s = String::from("Hello");
        print(s.clone());
        print(s);
    }

This will have 2 heap allocations. First, the call to String::from causes an
    initial heap allocation. Then the call s.clone() does a deep copy that
    uses a second heap allocation. Moving a String does not cause a heap
    allocation.





3.
Determine whether the program will pass the compiler. If it passes, write the
    expected output.

    fn main() {
        let s = String::from("hello");
        let s2;
        let b = false;
        if b {
            s2 = s;
        }
        println!("{s}");
    }