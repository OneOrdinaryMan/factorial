fn fact(num: usize) -> usize {
    if num == 0 {
        1
    } else {
        num * fact(num - 1)
    }
}
fn main() {
    let num: usize = 10;
    println!("The factorial of the number {} is {}", num, fact(num));
}
