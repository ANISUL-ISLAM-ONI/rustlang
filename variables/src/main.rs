fn main(){
    let number: i32 = 234;
    println!("The next value after {} is {}", number, plus_one(number));
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
