~ Data Types ~
Every value is Rust has a data type, and Rust is a statically typed language.

Rust has two major data type subsets: Scalar and Compound.





~~~ Scalar Types ~~~
A Scalar type is on that represents a single value. Like most other languages
    Rust has four major scalar types: Integer, Float, Boolean, and Character.



~~~~~ Integers ~~~~~
An Integer is a number without a fractional component; A whole number.

The naming convention that Rust uses for integer values is "iXX" for a signed
    integer, and a "uXX" for an unsigned integer, used with the values 
    of: 8, 16, 32, 64, and 128. E.g. i32, u16, u128, etc...

Additionally there is one other type called arch whos size depends on the
    machine running the code. It's values are isize and usize.

There are also a few other literal that you can use to declare numbers

    Number literal      Example
    Decimal             98_222
    Hex                 0xff
    Octal               0o77
    Binary              0b11011010
    Byte (u8 only)      b'A'



~~~~~ Floating Point Numbers ~~~~~
Rust also has two primitive types for floats: f32 and f64. Rust defaults to f64.

    fn main(){
        let x = 6.0; // f64

        let y: f32 = 3.0; // f32
    }



~~~~~ Numeric Operators ~~~~~
Rust supports the basic operators that you would expect

    fn main() {
        // addition
        let sum = 5 + 10;

        // subtraction
        let difference = 95.5 - 4.3;

        // multiplication
        let product = 4 * 30;

        // division
        let quotient = 56.7 / 32.2;
        let floored = 2 / 3; // Results in 0

        // remainder
        let remainder = 43 % 5;
    }



~~~~~ Boolean ~~~~~
Rust also has Boolean values, which are values that can be either true of false.
Booleans are one byte in size.

    fn main(){
        let t = true;

        let f: bool = false;
    }



~~~~~ Character ~~~~~
Characters are single line alphabetic types, denotes with ''.

    fn main() {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
    }

Char type is four bytes in size.





~~~ Compound Types ~~~
Compound types are able to group multiple values together. Rust has two types
    of Compounds: Tuples and Arrays.



~~~~~ The Tuple Type ~~~~~
A tuple is a general way of grouping variables with different types together.
    They are fixed in length and cannot be changed once they are created. The 
    types contained in a tuple do not have to match.

    fn main(){
        let tup: (i32, f64, u8) = (500, 6.4, 1);
    }

This "tup" variable binds all of these values to itself as one group.

To get the values out of a tuple we can use pattern matching to deconstruct it.

    fn main(){
        let tup: = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {y}");
    }

First, this program creates a tuple and binds the given values to it. Since the
    types are not explicitely declared, they are infered.

It then uses a pattern with the "let" keyword to deconstruct it into three 
    separate parts.

Alternatively, we can access a tuple's elements with a "." operator followed
    by the index of what we want to access.
    
    fn main(){
        let tup: = (500, 6.4, 1);

        let five_hundred = tup.0;

        let six_point_four = tup.1;

        let one = tup.2;
    }



~~~~~ The Array Type ~~~~~
The Array type is a way to group several values of the SAME type.

    fn main(){
        let array = [1, 2, 3, 4, 5];
    }

Arrays are not allowed to change size once declared.

Array types can be declared like so

    let a: [i32, 5] = [1, 2, 3, 4, 5];

This array has the TYPE of i32 and a SIZE of 5.

Array elements can be indexed like normal

    let a: = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];