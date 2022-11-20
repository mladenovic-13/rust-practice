fn main() {
    let mut counter = 0;

    // Return value from loop
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // If we have loops within loops, break and continue
    // applay to the innermost loop at the point
    // in this case we can specify "loop label"
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

    // Conditional while loop
    let a = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("The value is {}", a[index]);

        index += 1;
    }

    // For loop
    for element in a {
        println!("The value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
