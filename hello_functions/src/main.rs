fn main() {
    print_labeled_measurement(5, 'h');
    
    let x = plus_one(five());
    println!("The value of x is {x}.");

    true_or_false();

    variable_true();

    lets_loop();

    lets_while_loop();

    lets_for_loop();
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}.");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// hello comments!
// and hello if statements

fn true_or_false() {
    let number = 7;

    if number < 5 {
        println!("The condition was true");
    } else {
        println!("The condition was false");
    }
}

fn variable_true() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}.")
}

fn lets_loop() {
    let mut loop_time = 2;

    loop {
        if loop_time > 0 {
            println!("Hi!");
            loop_time = loop_time - 1
        } else {
            break;
        }
    }
}

fn lets_while_loop() {
    let mut loop_time = 3;

    while loop_time > 0 {
        println!("Hello!");
        loop_time = loop_time - 1
    }

    println!("Go!");
}

fn lets_for_loop() {
    let loop_time = [1, 2, 3, 4];

    for number in loop_time {
        println!("Greetings!");
    }
}