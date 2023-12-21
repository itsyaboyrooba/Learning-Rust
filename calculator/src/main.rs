mod calculator;
mod userinput;

fn main() {
    let (num1, num2, op) = userinput::read();
    println!("Sum: {}", calculator::calculator(num1, num2, op));
}
