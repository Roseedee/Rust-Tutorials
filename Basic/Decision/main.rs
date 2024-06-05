fn main() {
    let n1:i32 = 10;
    if n1 > 0 {
        println!("number is positive");
    }

    let n2:i32 = -10;
    if n2 > 0 {
        println!("number is positive");
    }else {
        println!("number is negative or zero");
    }

    let n3:i32 = 0;
    if n3 > 0 {
        println!("number is positive");
    }else if n3 < 0 {
        println!("number is negative");
    }else {
        println!("number is zero");
    }

    let n = 2;
    let str = match n{
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Unknow"
    };
    println!("{} is {}", n, str);
}