fn main() {
    println!("Testing a loop");
    let first_element = "a";
    let second_element = "b";

    let elements = [first_element, second_element];
    for element in elements.iter() {
        println!("{}", &element)
    }
}
