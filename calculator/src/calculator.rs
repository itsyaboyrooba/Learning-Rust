#[allow(clippy::needless_return)] 
/* 
Why? What? I don't get this warning,
it's super cool 'n' all that u can return values 
without the "return statement", but like, what am i supposed to do???? :D
*/

pub fn calculator(num1: f64, num2: f64, op: char) -> f64 {

    match op{ 
        '+' => return add(num1, num2),
        '-' => return subtract(num1, num2),
        '*' => return multiply(num1, num2),
        '/' => return divide(num1, num2),
        _ => {
            println!("Unknown operator {}!", op);
            std::process::exit(1)
        }
    };
}

fn add(num1: f64, num2: f64) -> f64 {
    let sum: f64 = num1 + num2;
    sum
}

fn subtract(num1: f64, num2: f64) -> f64 {
    let sum: f64 = num1 - num2;
    sum
}

fn multiply(num1: f64, num2: f64) -> f64 {
    let sum: f64 = num1 * num2;
    sum
}

fn divide(num1: f64, num2: f64) -> f64 {
    let sum: f64 = num1 / num2;
    sum
}