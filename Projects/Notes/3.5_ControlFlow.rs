~~~~~ Control Flow ~~~~~
Now we'll cover all of the flow control options that Rust provides such as
    "if" and "loop" expressions.



~~~ If Expression ~~~
An if statement is one of the most common programming concepts. It's a
    conditional statement that evaluates one of multiple ways based on a
    boolean condition.

Here's how they are implemented in Rust.

    fn main(){
        let number = 3;

        if number < 5{
            println!("Condition was true");
        } else {
            println!("Condition was false");
        }
    }

They work the same as an if statement in any other language. The statement takes
    the conditional statement and evaluates it to determine whether a path
    should be taken.



~~~ Handling multiple conditions with else if ~~~
We also have the "else if" otpion for if statements with more than 2 conditions.

    fn main(){
        let number = 6;

        if number % 4 == 0{
            println!("Number is divisible by 4");
        } else if number % 3 == 0 {
            println!("Number is divisible by 3");
        } else if number % 2 == 0 {
            println!("Number is divisible by 2")
        } else if number % 3 == 0 {
            println!("Number is not divisible by 4, 3, or 2");
        }
    }

This if statement has 4 possible branches.



~~~ Using "if" in a "let" statement ~~~
Because "if" is an Expression, we can use it on the right side of a "let"
    Statement to assign the outcome as a variable

    fn main(){
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }

In this code, the "number" variable will be bound to the outcome of the if 
    statement.

Running the code, we see "The value of number is 5" which is due to "condition"
    being initialized as true. If we were to change "condition"'s value to 
    false, then the code would print "The value of number is 6".

Remember that blocks of code evaluate all of the expressions in them. In the
    case above, we have two different possible paths/arms for the code to 
    take, and each has integer numbers 5 and 6 respectively. These values both
    hold the same type. If we were to change one of them to a different
    type, such as a string, the compiler would throw an error.

    fn main(){
        let condition = true;
        let number = if condition { 5 } else { "Six" };

        println!("The value of number is: {number}");
    }

This code would throw a type mismatch error because the first value is an i32
    and the second is String.



~~~ Repetition with Loops ~~~
Rust also has the standard loops that you would expect in a programming language

It has three: loop, while, and for.



~~~ Loop ~~~
The "loop" keyword executes a block of code over and over again forever or until
    it is told to stop.

    fn main() {
        loop{
            println!("again!")l;
        }
    }

When run will produce:

    again!
    again!
    again!
    ...

In order to break out of a loop, you use the "break" keyword. There is also
    the "continue" keyword, which tells the program to skip over any remaining
    code in the loop, and start from the top of the loop again.

    

~~~ Returning values from loops ~~~
You may want to use a loop to continuously poll for something such as user
    input. In this case, you will want the loop to return a value.

To return a value in a loop, we can place the value we want to return after
    the "break" statement.

    fn main(){
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

In this code segment, we declare the "counter" variable (mutable, so that its
    value can be changed) and then declare another variable "result" to be
    assigned the value of whatever the "loop" expression evaluates to.



~~~ Loop labels ~~~
If you have multiple loops, the "break" and "continue" keywords will apply to the
    innermost loop. Sometimes you may want to break out of a higher level loop
    or something similar. In this case you would use a "loop label" in order
    to distinguish specific loops. We declare a loop label with a single quote 

    fn main(){
        let mut count = 5;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");

                if remaining == 9 {
                    break;
                } if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
            count += 1;
        }
        println!("End count = {count}");
    }

This code will run two nested loops, breaking conditionally based on values
    that are being altered in both.



~~~ While Loops ~~~
Sometimes you want to run a loop conditionally, until a certain condition is
    met or unmet. For this we use a "while" loop.

    fn main(){
        let mut number = 3;

        while number != 0 {
            println!("{number}");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

This program defines a variable "number" and sets it to 3. Inside the loop we
    have defined the condition as "While number != 0" meaning that the loop will
    run until the "number" variable can be evanluated to 0. Inside the loop it
    prints the number's value, and then decreases it by 1. It does this
    repeatedly until number = 0, at which point it terminates the loop and moves
    on to the next piece of code, printing in total:

    3
    2
    1
    LIFTOFF!!!



~~~ Looping through collections with For ~~~
When we have a collection of items, such as a tuple or array, we may want to
    loop over each element in that collection. For this we use a "For" loop.

    fn main() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        for element in a {
            println!("The value is: {element}");
        }
    }

When we run this code, we loop through every element in "a" and can access it
    inside the loop.

We can also use a For loop to write safer versions of the code we wrote before
    with the While loop with a "Range" value.

    fn main() {
        for number in (1..4).rev(){
            println!("{number}");
        }
        println!("LIFTOFF!!!");
    }
