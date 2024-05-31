# Constant
variable name could upper case
> ### Syntax
```
    // const VARIABLE_NAME = value;
    // const VARIABLE_NAME:datatype = value;
    const PI = 3.14;
    println!("Value PI is : {}", PI);
``` 

> ### Shadowsing Variable
```
    let n = 10;
    let n = 20;
    println!("{}", n);
```
output : 20

```
    let name = "Roseedee"
    let name = name.len();
    println!("{}", name)
```
Output: 8
```
   const NAME:&str = "Roseedee";
   const NAME:usize = NAME.len(); 
   //Error : `NAME` already defined
   println!("name changed to integer : {}",NAME);
```