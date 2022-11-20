fn main() {
    // variables are immutable by default
    // we can deffine mutable varible with "let mut"
    let x = 5;
    println!("Value of x is: {x}");

    // if we deffine variable with existing name
    // it's called "shadowing" variable
    // it means that we replace old one
    let x = 6;
    println!("Value of x is: {x}");

    // The Tuple Type
    // Tuple is type where we group multiple type
    // into one type, by writing a comma-separated
    // list of values inside parentheses
    // Each position in tuple has a type
    // Type annotations are optional
    let tup = (500, 6.4, 1);
    // We can access values by destucturing
    let (_, y, z) = tup;
    // Also can access directly by using "."
    let first = tup.0;
    println!("Value of tup is: {first} {y} {z}");

    // The tuple without any values has a special name, "unit"
    // It's written () and represent an empty value or an
    // empty return type, expressions implicitly return unit
    // if they don't return any other value

    // Array Type
    // Unlike arrays in some other languages, arrays
    // in Rust have a fixed length

    // let array: [i32; 5] = [1, 2, 3, 4, 5]

    let _array = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March"];
}
