#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self){
        println!("{:?}", self);
    }
}

fn main(){
	let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}