use std::io;
fn main() {
    println!("enter a number: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("numbers only!");

    let hello: i32 = num.trim().parse().expect("num bitches");
    if hello > 18 {
     println!("you are eligible to watch porn");
    } else {
        println!("go watch some cartoons!");
    }
    match hello {
        1 => println!("meow"),
        2 => println!("hiss"),
        _ => println!("hell nah"),
    }

    let mut numbers = [1,2,3,4,5];

    for i in 0..numbers.len(){
        println!("{}", numbers[i]);
    }

    numbers[0] = 27;
    println!("{:?}", numbers);

    let mut cars = vec!["volvo", "mercedes", "maruti"];
    cars.push("suzuki");

    println!("{:?}", cars); 
    
    

}
