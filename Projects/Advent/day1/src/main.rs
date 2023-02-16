use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();


    let mut index: i32 = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(n) => println!("{}", n),
            Err(e) => println!("Error"),
        }
    }


}

// fn print_type_of<T>(_: &T){
//     println!("Type is {}", std::any::type_name::<T>());
// }


// If we find a ' ' empty space, increase index by 1
// Also find out how to use lists