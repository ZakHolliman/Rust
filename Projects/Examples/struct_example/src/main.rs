enum Vowel {
    A, E, I, O, U,
}

fn print_vowel(vowel: Vowel){
    println!("{:?}", vowel)
}

fn main(){
    let a = Vowel::A;
    let e = Vowel::E;

    print_vowel(a);
    print_vowel(e);
}