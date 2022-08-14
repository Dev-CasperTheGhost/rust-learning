// Vectors = Resizeable array

pub fn vectors() {
    let mut numbers: Vec<i32> = vec![1, 2, 3];

    println!("{:?}", (numbers));
    println!("First item: {}", numbers[0]);

    let mut names: [&str; 3] = ["John", "Casper", "Jane"];

    println!("{:?}", (names));

    // change value
    names[1] = "Jane";
    names[2] = "Casper";

    println!("{:?}", (names));

    // loops

    for name in names.iter() {
        println!("{}", name);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", (numbers));
}
