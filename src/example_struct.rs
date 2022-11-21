// fn main() {
//     let width = 30;
//     let height = 50;

//     println!(
//         "The are of rectangle is {} square pixels.",
//         area(width, height)
//     );
// }
// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// REFACTORING WITH TUPLES
// fn main() {
//     let rect = (30, 50);

//     println!("Area is: {}", area(rect));
// }
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// REFACTORING WITH STRUCTS
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area is: {:#?}", rect);
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
