fn main(){
    let mut x: i32 = 1;
    let y: i32 = 10;
    loop {
        if x <= y {
            println!("{}", x);
            x = x + 1;
        }
        else {
            break;
        }
    }
}
