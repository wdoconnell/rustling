use std::io;

fn main() {
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Cannot read line");

    // Parse string to i32
    let index: i32 = index.trim().parse().unwrap();

    let nth_fib = compute_fib(index);

    println!("The fibonacci number at position {index} is {nth_fib}");
    println!("Hello, world!");
}

fn compute_fib(index: i32) -> i32 {
    if index <= 1 {
        return index;
    }

    return compute_fib(index - 1) + compute_fib(index - 2);
}
