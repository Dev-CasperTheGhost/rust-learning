// Arrays = fixed length, same data types
use std::mem;

pub fn arrays() {
    let numbers: [i32; 3] = [1, 2, 3];

    println!("{:?}", (numbers));
    println!("First item: {}", numbers[0]);

    let mut names: [&str; 3] = ["John", "Casper", "Jane"];

    println!("{:?}", (names));

    // change value
    names[1] = "Jane";
    names[2] = "Casper";

    println!("{:?}", (names));

    println!("Array length: {}", names.len());
    println!("Memory used: {}", mem::size_of_val(&names));

    let slice: &[i32] = &numbers[0..1];
    println!("{:?}", (slice));
}
