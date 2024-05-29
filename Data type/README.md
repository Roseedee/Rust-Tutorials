# Data type
In Rust, variable types are automatically inferred by the compiler based on the value stored in the variable

> ### Example
```
    let number = 10;        //integer
    let floating = 3.14;    //floating point
    let boolean = true;     //boolean
    let string = "String";  //string
    let character = '♀';    //character
```

> ### print variable
use println! macro for print value in variable by using ```{}```
```
    let number = 10;
    println!("value in variable is {}", number)
```
Output : value in variable is 10

> ### Data Type

#### Integer
|No.    |Size   |Signed |Unsigned|
|-------|-------|-------|--------|
|1      |8 bit  |i8     |u8      |
|2      |16 bit |i16    |u16     |
|2      |32 bit |i32    |u32     |
|2      |128 bit|i128   |u128    |

- **Calculate can store numbers (Signed)**
```
    minimum : -(2^(nbit - 1))
    maximum : 2^(nbit - 1) - 1
    exp select 8bit
        min : -(2^(8-1))    = -128
        max : 2^(8-1)-1     = 127
```
- **Calculate can store numbers (Unsigned)**
Each unsigned variant can store numbers is 0 to ``` (2^nbit)-1 ```
```
    exp select 8bit
        min : 0
        max : (2^8)-1   = 255
```

- **how to use**
```
    let n = 10;         // default is i32 can store numbers -2147483648 to 2147483647
    let ni1:i8 = 10;    // range numbers -128 to 127    
    let nu1:u8 = 10;    // range numbers 0 to 255
```

- **Integer overflow**
```
    let n_max = 255;

    //can't run
    let n_maxo1 = 256; // overflow 0
    let n_maxo2 = 257; // overflow 1
```
#### Floating Point
classified as f32 and f64
**how to use**
```
    let f = 6000.57;     // default is f64
    let f1:f32 = 3.14;
```

#### Boolean
Boolean types has two possible values are true or false
```
    let b = true;        // boolean
    let b1:bool = true;
```
#### Character
Character supports numbers, alphabets, Unicode and special characters
```
    let c = 'C';
```








