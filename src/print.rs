pub fn run() {
    //Print to console
    println!("Hello From the print rs file");

    //Basic Formatting
    println!("{} is from {}", "Damola", "Earth");

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Damola", "Earth", "Code"
    );

    //Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Damola",
        activity = "Tennis"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 10, 10, 10);

    //Placeholder for debug trait
    println!("{:?}", (12, true, "Hello"));

    //Basic math
    println!("10 +10 = {}", 10 + 10);
}
