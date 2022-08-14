// pub = public function
// it will "export" the function (export)

pub fn print() {
    // print to console
    println!("Hello world");

    // print a non-string (formatting)
    println!("Number: {}, Object: ", 53,);

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Casper", "BE", "Code"
    );

    // named arguments
    println!(
        "{name} likes to play {activity}",
        name = "Casper",
        activity = "Code"
    );

    // :b -> binary
    // :x -> hexadecimal
    // :o -> octal
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 08);

    // debugging
    println!("Debug: {:?}", (12, true, "hello"));
}
