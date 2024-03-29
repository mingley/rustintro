pub fn run(){
    //print to console
    println!("Hello from the print.rs file");
    //basic formatting
    println!("{} is from {}", "Mike", "Florida");
    //positional arugemnts
    println!("{0} is from {1} and {0} likes to {2}", "Mike","FL","code");
    //named arguments
    println!("{name} likes to play {activity}", name="Mike", activity="Baseball");
    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10,10,10 );
    //placeholder for debug trait
    println!("{:?}", (12,true,"hello"));
    //basic math
    println!("10+10={}", 10+10);
}