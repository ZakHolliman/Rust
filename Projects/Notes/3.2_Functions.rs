~~~~~ Functions ~~~~~
Functions are functions are functions. Every language has functions.

A function in Rust is declared with a "fn" keyword.

Rust code uses snake case as the convention for function and variable 
    names, meaning that all letters are lowercase, and separated by underscores.

    E.g.
    fn a_function_with_a_long_name(){
        println!("Wow, that name is really long!");
    }


Rust does not care where you define functions, the only thing that matters is
    the scope in which they are defined in. 



~~~ Parameters ~~~
Function paramater definition works as such:

    fn main(){
        another_function(5);
    }

    fn another_function(x: i32){
        println!("The value of x is: {x}");
    }

This program prints, as you would expect, "The value of x is: 5".

This function has one parameter, x, that is specified as an i32 type.



~~~ Statements and Expressions ~~~
Function bodies are made of statements that can optionally end in an expression.

A Statement is an instruction that performs an action and does not return a
    value.

An Expression evaluates to a resulting value.

Here's an example of a Statement:

    fn main(){
        let y = 6;
    }

In this function, "let y = 6" is a statement, as it does not return anything.

If we were to try something like:

    fn main(){
        let y = x = 6;
    }

This would not work however, because due to the way that Rust works, "x = 6" is
    the first assignment, and is a Statement and thus does not return a
    value, so the "y = " part of the expression binds to an invalid value, and
    throws a compilation error.

For Expressions, they are anything that evaluates into a value. This can be
    something like a function

    // A function that simply returns the value "5"
    fn five() -> i32 {
        5
    }

Or a block of code

    fn main(){
        let y = {
            let x = 3;
            x + 1
        };

        println!("The value of y is: {y}");
    }

Or a lot of other things, such as a macro

In the example code above, we have the Expression as this block of
    code:

    {
        let x = 3;
        x + 1
    }

This block of code evaluates to the value 4, which is then bound to y using the
    let keyword.

Also note that the "x + 1" does not have a semicolon at the end. This is because
    Expressions do not use semicolons. If you add a semicolon to an 
    Expression, it will turn it into a Statement.



~~~ Functions with return values ~~~
Functions can also, of course, return values for other sections of code to work
    with. Return values do not get names in Rust, but are denoted with a "->"
    arrow.

    fn five() -> i32 {
        5
    }

    fn main(){
        let x = five();

        println!("The value of x is: {x}");
    }

If we run this, we see "The value of x is: 5"

This is because we use a function call with "five()" and assign the returned
    value, which is simply just the number 5, to x.

Another example:

    fn main(){
        let x = plus_one(5);

        println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32{
        x + 1
    }

In this function we take an input parameter called "x" with the value of 
    i32, and simply add 1 to it. The return type is also specified as i32.