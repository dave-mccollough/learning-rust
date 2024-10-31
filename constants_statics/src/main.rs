// Constant variable
// Constants are defined with the const keyword
// Constant variables can never be mutated
// Naming convention for constant variables is SCREAMING_SNAKE_CASE
// Constants require an explicit type annotation
// Constants can be declared in any scope including the global scope
// Default to using constants unless you need a static variable

const MAX_PLAYERS: u8 = 10;

// Static variables
// Static variables are declared with the static keyword
// Static variables naming convention is SCREAMING_SNAKE_CASE
// Static variables require an explicit type annotation
// Static variables can be declared in any scope
// Use static variables when storing large amounts of data
static GAME_NAME: &str = "Rust Poker";
// Static variables can be marked as mutable
static mut NEW_GAME_NAME: &str = "Rust Hold'em";
// Accessing or muting a static variable is unsafe and must be done in an unsafe block

fn main() {
    println!("Hello, world!");

    // Using constants
    // Constants do not occupy a specific place in memory
    let a: u8 = MAX_PLAYERS;
    let b: u8 = MAX_PLAYERS;

    // Using static variables
    let x: &str = GAME_NAME;
    let y: &str = GAME_NAME;
}
