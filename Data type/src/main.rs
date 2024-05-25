fn main() {
    let number = 10;        //integer
    let floating = 3.14;    //floating point
    let boolean = true;     //boolean
    let string = "String";  //string
    let character = '♀';    //character
    
    println!("{}", number);     //prints 10
    println!("{}", floating);   //prints 3.14
    println!("{}", boolean);    //prints true
    println!("{}", string);     //prints String
    println!("{}", character);  //prints ♀ 

    let ni8_min:i8 = -128;
    let ni8_max:i8 = 127;
    let nu8_min:u8 = 0;
    let nu8_max:u8 = 255;

    println!("integer signed min {}", ni8_min);
    println!("integer signed max {}", ni8_max);
    println!("integer unsigned max {}", nu8_min);
    println!("integer unsigned max {}", nu8_max);
    
    let c = '♠';
    let c1:char = 'A';

    println!("{}", c);
    println!("{}", c1);


    
}