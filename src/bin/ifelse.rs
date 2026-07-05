

fn main () {
    let x: i32 = rand::random();
    println!("Random number: {}", x);
    if x > 18 {
        println!("Safe");
    }
    else { 
        println!("not safe");
    }
}
