fn main() {

    a3a(true);
    a3a(false);
    a3b(43);
    a4a(true);

    let input = 3;
    a4b(input);
    let res = sub(5, 2);
    println!("Hello, World! {:?}", res);
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn a3a(flag: bool) {
    if flag {
        println!("hello");
    } else {
        println!("goodbye");
    }
}

fn a3b(input: i32) {
    if input < 5 {
        println!("<5");
    } else if input == 5 {
        println!("=5")
    } else {
        println!(">5")
    }
}

fn a4a(flag: bool) {
    match flag {
        true => println!("it's true"),
        false => println!("it's false")
    }
}

fn a4b(input: i32){
    match input {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
}