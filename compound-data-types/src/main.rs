fn main() {
    // Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tuple);

    // Array
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("Array: {:?}", array);
    // String Array
    let string_array: [&str; 3] = ["Hello", "Rust", "World"];
    println!("String Array: {:?}", string_array);

    // Vector
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", vector);

    // String
    let string: String = String::from("Hello, Rust!");
    println!("String: {}", string);

    // Struct
    struct Person {
        name: String,
        age: u8,
    }

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("Struct: {} is {} years old", person.name, person.age);

    // Enum
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    let direction = Direction::Up;
    match direction {
        Direction::Up => println!("Enum: Up"),
        Direction::Down => println!("Enum: Down"),
        Direction::Left => println!("Enum: Left"),
        Direction::Right => println!("Enum: Right"),
    }
}
