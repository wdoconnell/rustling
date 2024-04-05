use rand::Rng;

fn main() {
    loop_testing();
    if_testing();
    while_testing();
    for_testing();
}

fn if_testing() {
    println!("\nIF TESTING");
    let number = rand::thread_rng().gen_range(1..=5);

    println!("the value of the randomly generated number is {number}");

    // Condition is required to be a bool (no 'truthy'/'falsy')
    if number < 5 {
        println!("the number was less than 5");
    } else {
        println!("the number was not less than 5");
    }

    println!("Some experiments with let and if");

    let result = if number < 5 {
        "lessthanfive"
    } else {
        "fiveorgreater"
    };
    println!("The result is {result}");
}

fn loop_testing() {
    println!("\nLOOP TESTING");
    let mut counter = 0;
    println!("this code should continue until the counter hits five, then we'll break");

    let end_result = loop {
        counter += 1;
        println!("the value of counter is {counter}");

        // Can return an expression from break
        if counter == 5 {
            break counter;
        }
    };

    println!("the value of counter when we broke out of the loop was {end_result}");

    println!("by default, loops will break out of the innermost loop, but we can use labels");

    let mut outer_counter = 0;

    'outerloop: loop {
        outer_counter += 1;

        let mut inner_counter = 0;
        loop {
            inner_counter += 1;

            // Break entirely after 20 * 3.
            if outer_counter == 3 {
                break 'outerloop;
            }

            // Break default (inner loop only) after 20.
            if inner_counter == 20 {
                break;
            }
            println!("outer counter: {outer_counter}, inner counter: {inner_counter}");
        }
    }

    println!(
        "we broke out of the outer loop from the inner loop at outer_counter: {outer_counter}"
    );
}

fn while_testing() {
    println!("\nWHILE TESTING");
    println!("further testing with a while loop");

    let mut while_counter = 0;

    while while_counter != 3 {
        while_counter += 1;
        println!("looping through while with counter of value {while_counter}");
    }
    println!("exiting while loop with counter at value {while_counter}");
}

fn for_testing() {
    println!("\nFOR TESTING");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("this is the value of the current element in the array {element}");
    }
    println!("exited the for loop");

    println!("here's a countdown example with .rev");
    for number in (1..=10).rev() {
        println!("{number}");
    }
}
