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

#### Common methods string object
##### new()
```
    let mut fname = String::new();
    fname.push_str("Roseedee");
    println!("{}", fname);
```
Ouput : Roseedee

##### to_string()
```
    let fname = "Roseedee".to_string(); //String Object
    println!("{}", fname);
```
Output: Roseedee

##### replace()
```
    let text = "Hello world".to_string();
    let text_replace = text.replace("world", "Me");
    println("{} \n{}", text, text_replace);
```
Output:
Hello world
Hello Me
