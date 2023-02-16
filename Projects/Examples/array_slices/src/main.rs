fn main() {
    let mut total: i32 = 0;

    let array = [3, 7, 8, 2, 4];

    // Slices
    for int in &array[1..3]{
        println!("Value is: {}", int);
    }

    println!("");

    for int in &array[..3]{
        println!("Value is: {}", int);
    }

    println!("");

    for int in &array[1..=3]{
        println!("Value is: {}", int);
    }

    println!("");

    for int in &array[1..]{
        println!("Value is: {}", int);
    }

    println!("");

    for int in &array[..]{
        println!("Value is: {}", int);
    }

    println!("");

    // Iter
    for int in array.iter(){
        println!("Value is: {}", int);
    }

    println!("");

    for int in array.iter(){
        total += int;
        // println!("Value is: {}", int);
    }

    println!("Total is: {}", total);

    // Enumerate
    // Enumerate is a method that can be called from an `iterator` that 
    println!("");

    for (index, value) in array.iter().enumerate(){
        println!("Index is: #{}, Value is: {}", index, value);
    }

    println!("");

    // Summing arrays with .iter().sum()
    let sum: i32 = array.iter().sum();
    println!("Sum is: {}", sum);

}

fn add_array(slice: &[i32]){
    for int in slice {
        println!("{}", int)
    }
}