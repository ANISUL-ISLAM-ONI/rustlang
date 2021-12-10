fn main(){
    println!("I am from Main");
    another_function(123, 'A');
}

fn another_function(x: i32, y: char){
    println!("You just passed {}{}", x, y);
}
