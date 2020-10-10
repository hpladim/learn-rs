#![allow(unused)]

const MAX_POINTS : u32 = 100_000;


fn main() {



    //Mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    // Shadowing
    let y = 5;

    let y = y + 1;

    let y = y * 2;

    println!("The value of y is: {}", y);

    //Shadow types
    let spaces = "   ";
    let _spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");
}
