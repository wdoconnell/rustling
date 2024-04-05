fn main() {
    // This is a statement - it ends in a semicolon, and returns nothing.
    println!("We are going to call another function, that will print 5 and a.");

    let result = other_function(5, 'a');
    println!("We are going to print the fn's return value, which should be six.");
    println!("the value returned from the function (1 higher) was {result}");

    // Even a number by itself could be a value return value when it is an expression.
    let five = return_five();
    println!("The value returned by return_five is: {five}.");
}

// type annotations are required in function parameters
fn other_function(num: i32, c: char) -> i32 {
    // These are statements; they end in semicolons, and return nothing.
    println!("another function called from main");
    println!("the value of the number (first arg) passed was {num}");
    println!("the value of the string (second arg) passed was {c}");

    // This is an expression - there is no semicolon, and it is implicitly returned from the fn.
    num + 1
    // We could specify 'return', but don't need to unless we're returning early.
}

fn return_five() -> i32 {
    5
}
