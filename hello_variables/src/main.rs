const ME : &str = "ren";

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    println!("I am: {}", ME);

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("y in the inner scope: {}", y);
    }

    println!("value of y: {}", y);
}
