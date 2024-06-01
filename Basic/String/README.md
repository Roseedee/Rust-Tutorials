# String
> ### String Literal
Syntax
```
    let variable_name:&str = value;
```
Example
```
    let name:&str = "Roseedee";
    println!("{}", name);
```
Static String
```
    let fname:&'static str = "Roseedee";
    let lname:&'static str = "Cehlaeh";
    println!("{} {}", fname, lname);
```

> ### String Object
Syntax
```
    let empty_string = String::new();
    let content_string = String::from("Roseedee");

    println!("{}", empty_string.len());
    println!("{}", content_string.len());
```