
// Shift+ALT+A for block comments

struct Rectangle<T> {
    height: T,
    width: T,
}

struct Cube<T, U, V> {
    height: T,
    width: U,
    lenght: V,
}

fn struct_example() {
    let rect1 = Rectangle{height: 1, width: 2};
    let rect2 = Rectangle{height: 1.65, width: 2.22};
    
    let cube1 = Cube{ height: 1, width: 2, lenght: 3};
    let cube2 = Cube{ height: 1.54, width: 2, lenght: 3.75};
}


// Because its a generic, the data type isn't defined. We have to restrict our data type with "<T: std::ops::Mul<Output = T >>"
// ops::Mul = Operation::Multiply
fn sum_of_numbers<T: std::ops::Mul<Output = T >>(num1: T, num2: T) -> T {
    num1 * num2
}

fn lookup_datatype<T>(Object: T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    println!("{}", sum_of_numbers(1, 2));
    lookup_datatype(1);
    lookup_datatype(1.96);
    lookup_datatype("string_slice");
}