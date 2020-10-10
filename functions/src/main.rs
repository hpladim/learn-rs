fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1 // No semicolon here,  or use return...
    };

    println!("The value of y is: {}, and x is {}", y, x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);

    let x = plus_one_plus_two(5);

    println!("The value of x is: {} and {}", x.0, x.1);
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // alternative:
    //return x+1;
}

fn plus_one_plus_two(x: i32) -> (i32, i32) {
    (x + 1, x + 2)
}
