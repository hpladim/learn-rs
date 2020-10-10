fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition { "five" } else { "six" };

    println!("The value of number is: {}", number);


    // loop with break
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    // while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    let a = [10, 20, 30, 40, 50];
    let mut index = 4;

    while index != 0  {
        println!("the value is: {}", a[index]);

        index -= 1;
    }

    // iterate
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }


    for element in (1..100).rev() {
        println!("the value is: {}", element);
    }

    println!("LIFTOFF!!!");

    let n = 7;

    println!("{}",fibo(n))
}


fn fibo(n:u32) -> u64{
    let mut index = 1;
    let mut prev = 0;
    let mut now = 1;
    while index < n {
        let temp = now;
        now = now + prev;
        prev = temp;
        index += 1;
    }
    now
}
