pub fn read() -> (f64, f64, char) {
    println!("Input the first number: ");
    let num1 = get_float();
    println!("Input the second number: ");
    let num2 = get_float();
    println!("Input the operation: ");
    let op = get_char();
    (num1, num2, op)
}

fn get_float() -> f64 {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input.trim().parse().expect("Input not a number");
}

fn get_char() -> char {
    let mut user_input = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    return user_input.trim().parse().expect("Input not a character");
}