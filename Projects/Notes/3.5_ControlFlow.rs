

~~~ While Loops ~~~


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
