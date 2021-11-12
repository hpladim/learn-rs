#![allow(unused)]
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t = true;

    let f: bool = false; // with explicit type annotation


    // characters 
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    let tup = (500, 6.4, 1);
    // destructuring to variables
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // more tuple
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("The value of five_hundred is: {}", five_hundred);

    // array
    let a: [i32; 5] = [1, 2, 3, 4, 5]; //semicolon here

    let first = a[0]; 
    println!("first {}", first);

    let a = [3; 7]; //equal: assignment here
   
    let second = a[1];
    println!("second {}", second);

    //Invalid Array Element Access
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    //let element = a[index]; //fails at run time

    //println!("The value of element is: {}", element);



}
