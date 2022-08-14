// Tuple = group of values
// max 12 elements

pub fn tuples() {
    let person: (&str, &str, i8) = ("John Doe", "America", 32);

    println!(
        "{name} is from {location} and is {age}",
        name = person.0,
        location = person.1,
        age = person.2
    );
}
