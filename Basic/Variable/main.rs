fn main() {
    //Immutable
    let n = 10;
    // n = 11;  Error
    println!("{}", n);

    //Mutable
    let mut n1 = 10;
    println!("{}", n1);
    n1 = 20;
    println!("{}", n1);
}