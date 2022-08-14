pub fn variables() {
    // let
    let name = "Casper";
    let mut age = 17;

    println!("{name} is {age} years old", name = name, age = age);

    // change age
    age = 20;

    println!("{name} is {age} years old", name = name, age = age);

    let (name2, age2) = ("Casper", 17);

    println!("{name2} is {age2} years old", name2 = name2, age2 = age2);

    // const
    const MAX_POINTS: i32 = 100;
    println!("Max points per player: {points}", points = MAX_POINTS);
}
