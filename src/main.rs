fn main() {
    let mut message = String::from("hi");

    let borrowed = &mut message;
    borrowed.push('!');

    println!("{message}"); // okay after the mutable borrow's last use
}
