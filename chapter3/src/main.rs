// This is an example of a Rust comment. This is the same as most other languages.
fn main() {
    // If this weren't mutable, the x=6 redefinition would not be possible. Variables are immutable by default
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // let y = 3;
    // y is immutable, we can't say
    // y = 4;

    const THIS_IS_A_CONSTANT: u8 = 42; // const must be type annotated
    // constants are evaluated at compile time
    // other variables (including immutable vars) are evaluated at run-time
    println!("Here's a constant: {THIS_IS_A_CONSTANT}");

    // Shadowing
    let z = 5;

    let z = z + 1; // This overwrites the def of z

    {
        let z = z * 2; // We redefine z, but only within this scope
        println!("The value of x in the inner scope is: {z}");
    }

    println!("The value of x is: {z}");

    // Immutable variables allow type shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces");

    // DOESN'T work
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // Conditionals
    let num = 1;
    if num == 1 {
        println!("Num equals 1");
    }
    else if num == 2{
        println!("Num equals 2");
    }
    else {
        println!("Num is not 1 or 2");
    }

    // Conditionals in definitions
    let cond_num = if num == 1 { 42 } else { 10 };
    println!("The conditional number is: {cond_num}");

    // Loops
    // Infinite loop
    loop {
        println!("In the loop!");
        if num == 1 {
            break;
        }
    }

    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result: {result}");

    // Loop labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // While loop
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("WE HAVE LIFTOFF!!!");

    // For loop
    for item in (1..14) {
        print!("{item}");
        if item != 13 { print!(", "); }
    }
}