/*
    Prmitive data types in Rust:
    int, float, bool, char,

    Integers:
    Rust has signed and unsigned integers. Signed integers are represented using two's complement representation.
    Signed integers: i8, i16, i32, i64, i128, isize
    Unsigned integers: u8, u16, u32, u64, u128, usize
*/

/*
    Floats:
    rust has two types of floating point numbers: f32 and f64

    Boolean:
    bool: true, false

    Character:
    char: single unicode character
*/

fn main() {
    let x: i32 = -10;
    let y: u32 = 20;

    let fn1: f32 = 3.14;
    let fn2: f64 = 6.256;

    let isValid: bool = true;
    let isInvalid: bool = false;

    let ch: char = 'A';
    let ch2: char = 'B';

    println!("x: {}", x);
    println!("y: {}", y);
    println!("x: {}\ny: {}", x, y);
    // println!("x + y: {}", x + y);

    println!("fn1: {}", fn1);
    println!("fn2: {}", fn2);
    // println!("fn1 + fn2: {}", fn1 + fn2);
    // println!("fn1 * fn2: {}", fn1 * fn2);
    // println!("fn1 / fn2: {}", fn1 / fn2);
    // println!("fn1 - fn2: {}", fn1 - fn2);
    // println!("fn1 % fn2: {}", fn1 % fn2);
    println!("fn1.powi(2): {}", fn1.powi(2));
    println!("fn1.powf(2.0): {}", fn1.powf(2.0));
    println!("fn1.sqrt(): {}", fn1.sqrt());
    println!("fn1.cbrt(): {}", fn1.cbrt());
    println!("fn1.exp(): {}", fn1.exp());
    println!("fn1.exp_m1(): {}", fn1.exp_m1());
    println!("fn1.ln(): {}", fn1.ln());
    println!("fn1.log2(): {}", fn1.log2());
}
