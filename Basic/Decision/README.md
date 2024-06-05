# Decision
- if statement
- if else statement
- nested if
- match statement

> ### if statement
__Syntax__
```
    if condition {
        //statement condition true
    }
```
__Example__
```
    let n:i32 = 10;
    if n > 0 {
        println!("number is positive");
    }
```
Output: number is positive

> ### if else statement
__Syntax__
```
    if condition {
        //statement condition true
    }else {
        //statement condition false
    }
```
__Example__
```
    let n:i32 = -10;
    if n > 0 {
        println!("number is positive");
    }else {
        println!("number is negative");
    }
```
Output: number is negative

> ### nested if
__Syntax__
```
    if condition {
        //statement
    }else if condition {
        //statement
    }else {
        //statement
    }
```
__Example__
```
    let n:i32 = 0;
    if n > 0 {
        println!("number is positive");
    }else if n < 0 {
        println!("number is negative");
    }else {
        println!("number is zero");
    }
```
Output: number is zero

> ### match statement
__Syntax__
```
    let expressionResult = match variable_expression{
        constant_expr1 => {
            //statement
        },
        constant_expr2 => {
            //statement
        },
        _ => {
            //default statement
        }
    }
```
__Example 1__
```
    let n = 2;
    let str = match n{
        0 => "Zero",
        1 => "One",
        2 => "Two",
        3 => "Three",
        _ => "Unknow"
    };
    println!("{} is {}", n, str);
```
Output: 2 is Two

__Example 2__
```
    let state_code = "MH";
    let state = match state_code {
        "MH" => {println!("Found match for MH"); "Maharashtra"},
        "KL" => "Kerala",
        "KA" => "Karnadaka",
        "GA" => "Goa",
        _ => "Unknown"
    };
    println!("State name is {}",state);
```
Output:
Found match for MH
State name is Maharashtra


