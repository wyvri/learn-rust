fn main() {
    let my_string = String::from("meow");
    println!("{my_string}");

    let your_string = yum(my_string);

    println!("{your_string}");
}

fn yum(my_string:String) -> String {
    let your_string = my_string;
    your_string
}