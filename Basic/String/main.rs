fn main() {
    //String literal
    let name:&str = "Roseedee";
    println!("{}", name);

    //Static string
    let fname:&'static str = "Roseedee";
    let lname:&'static str = "Cehlaeh";
    println!("{} {}", fname, lname);

    //String Object
    let empty_string = String::new();
    let content_string = String::from("Roseedee");
    println!("{}", empty_string.len());
    println!("{}", content_string.len());
}