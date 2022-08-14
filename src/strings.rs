// Primitive str = Immutable, fixed-length
// String = Growable, use when you need to modify

pub fn strings() {
    // let hello_prim = "Hello"; // Primitive str
    let mut hello = String::from("Hello"); // String

    // get length of string
    println!("Length: {}", hello.len());

    // push string
    hello.push_str(" World");
    println!("Push: {}", hello);

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Is Empty
    println!("Is empty: {}", String::from("").is_empty());

    // contains
    println!("Contains: {}", hello.contains("World"));

    // replace
    println!("Replace: {}", hello.replace("World", "Casper"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Assertions
    let a = String::from("a");
    let b = String::from("b");

    if a == b {
        println!("A equals b");
    } else {
        println!("A does not equal b");
    }
}
