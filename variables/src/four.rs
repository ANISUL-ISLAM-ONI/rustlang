fn main(){
    println!("Hello, world! I am from Main function");
    another_function(2345);
}

fn another_function(x: i32){
    println!("I am from another function. Parameter passed as {}", x);
}
