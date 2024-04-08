fn main() {
    // We can use scalar values that are stored only on the stack
    // After reassignment

    let x = 5;
    let y = x;

    println!("The value of x is still {x} and y is {y}");

    // We cannot use values stored on the heap after they are moved.
    let my_str = String::from("hi");

    // To make this valid, we need to clone the reference to the string so we can still use str later
    // This means that there is a second copy of this data on the heap.
    random_func(my_str.clone());

    println!("the string is out of scope");
    println!("string's value is {my_str}");
}

fn random_func(my_str: String) {
    println!("running random func");
    println!("the value of the string here is {my_str}");
}
