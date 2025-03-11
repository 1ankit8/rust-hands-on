use std::io;
const PI: f64 = 3.14159;
fn main() {
    println!("Hello, world!");
    println!("Please enter a number:");

    let (x, y) = (1, 2);
    println!("x: {}, y: {}", x, y);
    println!("PI: {}", PI);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please type a number!");
    println!("You entered: {}", number);

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();
    println!("Numbers: {:?}", numbers);

    for (index, num) in numbers.iter().enumerate() {
        if *num == 0 {
            println!("Index: {}, Number: {} is zero", index, num);
        } else if *num % 2 == 0 {
            if *num > 0 {
                println!("Index: {}, Number: {} is even and positive", index, num);
            } else {
                println!("Index: {}, Number: {} is even and negative", index, num);
            }
        } else if *num > 0 || *num < 0 {
            if *num > 0 {
                println!("Index: {}, Number: {} is odd and positive", index, num);
            } else {
                println!("Index: {}, Number: {} is odd and negative", index, num);
            }
        }
    }

    let length = numbers.len();
    println!("Length of array: {}", length);
}
