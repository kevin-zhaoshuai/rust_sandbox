pub fn run() {
    // Print to console
    println!("Hello from the print.rs file")

    println!("{} is from: {}", "Brad", "Mass");

    //Positional Arguments
    println!("{0} is from {1} and {2} likes to {1}",
    "Tome", "1", "2");

    // Named Arguments
    println!("{name} likes to play {activity}",
    name = "John", activity = "baseball");

    // Placeholder traits
    println!("Binary, {:b} Hex: {:x}, Octol: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10)

}