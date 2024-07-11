fn main() {
    // Variables and Mutability
    let mut x = 5;

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant value = {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "  ";
    let spaces_len = spaces.len();

    println!("Some space '{spaces}' of length {spaces_len}");
}
