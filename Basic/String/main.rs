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

    //Common methods
    // new()
    let mut fname = String::new();
    fname.push_str("Roseedee");
    println!("{}", fname);

    // to_string();
    let name1 = "Roseedee".to_string();
    println!("{}", name1);

    // replace()
    let name2 = "Hello World".to_string();  //convert string to object
    let name3 = name2.replace("World", "Me");
    println!("{} \n{}", name2, name3);
}