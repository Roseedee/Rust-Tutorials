# Variables

> ### Syntax
```
    let variable_name = value;          //no type specified
    let variable_name:datetype = value  //type specified
```

> ### Immutable
In Rust default variable are immutable or read only
```
    let age = 10;
    // age = 11; Error : you cannot assign values twice to immutable variable
```

> ### Mutable
use mut keywork before variable name for make it mutable
```
    let mut age = 10;
    println!("{}", age);
    age = 11;
    println!("{}", age);
```