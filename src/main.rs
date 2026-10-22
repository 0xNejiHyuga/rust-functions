fn main() {
    another_function();
    funcx(10);
    println!("Function output: {}", output_func(10, 20));
}

// Just function
fn another_function() {
    println!("another_function!")
}
//Func's arguments
fn funcx(x: i32) {
    println!("Func: {x}")
}
// Output function
fn output_func(a: i32, b: i32) -> i32 {
    a + b
}