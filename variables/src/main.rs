use std::io;

fn main() {
    /* Constants */
    // Naming convention for constants is allcaps, snake case
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3; // const must specify type, no runtime comp
    println!("Three hours in seconds is {THREE_HOURS_IN_SECONDS}");

    /* Mutable variables */
    let mut x = 5; // let is immutable unless `mut` is specified
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    } // Shadowed variable in inner scope is handled separately from outer-scope x.

    // This will retain the original value.
    println!("Back in the outer scope, the value of x is {x}");

    // Difference between shadowing and mutation is that variable is in fact different
    // I.e., when shadowing (as opposed to mutating), could have a different type.
    let name = "hollow knight";
    let name = name.len();

    println!("Length of this string is {name}");

    println!("Let's try some integer overflow");

    // This code panics in debug mode, but will compile.
    // Rust uses `twos complement wrapping`, which will allow value wrap-arounds.
    // This unsigned 8-bit integer overflows to 119.
    // let mut overflow: u8 = 255;
    // overflow += 120;
    // println!("The value of overflow is {overflow}")

    /* Floating-point numbers */
    let x = 2.0; // f64 by default due to cpu

    let y: f32 = 3.0; // specify f32.

    let sum = x + y; // but compiler treats x as f32 by inference here.
                     // This would not work if we added an f64 type annotation to x.
    println!("This is the sum of two floating points: {sum}");

    // Integer addition
    let sum = 5 + 10;
    println!("The sum of 5 and 10 is {sum}");

    // Float subtraction
    let diff = 10.2 - 9.1;
    println!("the difference between 10.2 and 9.1 is {diff}");

    // Int multiplication
    let prod = 2 * 3;
    println!("the product of 2 and 3 is {prod}");

    // Float division
    let div = 243.3 / 9.2;
    println!("243.3 divided by 9.2 is {div}");

    // Integer division with truncation
    // Truncates towards 0 to the nearest integer
    let int_div = 10 / 3;
    println!("the result of integer dividing 10 by 3 is: {int_div}");

    //  Remainder
    let remainder = 43 % 5;
    println!("the remainder of 43 mod 5 is: {remainder}");

    // Bool
    // Represented by one byte; can use type annotation.
    let t = true;
    let f: bool = false;
    let equal = t == f;
    println!("Is true false? {equal}");

    // Character types
    // This is four bytes in size, so can represent more than just ASCII.
    let emoji: char = '🐕';
    println!("This is a dog emoji character: {emoji}");

    /* Primitive Compound Types */

    // Tuple
    // Declare with parens (like python)
    // Fixed length; don't need all values to be of same type.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Tuples support destructuring
    let (_, b, _) = tup;

    println!("the middle value of the tuple is {b}");

    // Tuples also support . accession
    let first_val = tup.0;
    println!("the first value of the tuple is {first_val}");

    // Tuple without any values is "unit", which is the implicit return type of expressions.
    let _ = ();

    // Array
    // Arrays in rust actually have as fixed length.
    // Use this when you want data allocated on stack rather than heap
    // or when you want to ensure you always have a fixed number of elements.
    // Not as flexible as the vector type.

    let arr = [1, 2, 3, 4, 5];
    let second_val = arr[1];
    println!("the second value of the array is {second_val}");

    // Some fixed vals.
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("Listing the months in a year:");
    for month in months {
        println!("{month}");
    }

    let second_arr = [1, 2, 3, 4, 5];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("The index entered was not a number");

    let element = second_arr[index];

    // Picking an out-of-bounds index will panic,
    // but unlike in otherlanguages, won't let you access invalid memory.
    println!("The value of the element at index {index} is: {element}");
}
