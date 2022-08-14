// Ints: in -> n = amount of bits
// Floats: f32, f64
// Boolean (bool)
// Characters (char)

pub fn types() {
    // ints & floats
    let x: i32 = 100_00;
    let age: i32 = 17;

    let float_y: f64 = 1.5;

    let max_i32 = std::i32::MAX;

    println!("{}", max_i32);

    // boolean
    let is_active = false;
    let is_adult: bool = true;
    let is_age_greater_than_ten = age > 10;

    println!(
        "{:?}",
        (
            x,
            float_y,
            max_i32,
            is_active,
            is_adult,
            is_age_greater_than_ten
        )
    );

    // characters
    let face: char = '\u{1F600}';

    println!("{}", face);
}
