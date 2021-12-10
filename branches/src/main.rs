use std::io;

fn main(){
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Wrong input");

    let number = number.trim().parse().expect("Couldn't parse");

    println!("The fibonacci number of {}th number is : {}", number, fibo(number));
}

fn fibo(x: i32) -> i32 {
    if x < 2 {
        x
    }
    else {
        fibo(x - 1) + fibo(x - 2)
    }
}
