fn main() {
    println!("Hello, world!");

    let opt: Option<String> = Some(String::from("amongus"));

    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    };

    println!("{:?}", opt);
    println!("{:?}", &opt);

    let thingy = Location::Range(32, 46);
    let stuff = Location::Point(12);
    print_range_max(&stuff);
    get_start(&thingy);
}

enum Location {
    Point(i32),
    Range(i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

enum UsState {
    Alabama,
    Arkansas,
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_range_max(loc: &Location) {
    if let Location::Range(_, y) = loc {
        println!("Second field of range: {y}");
    }
}

fn get_start(loc: &Location) {
    match loc {
        Location::Range(first, _) => println!("First field of range: {first}"),
        Location::Point(point) => println!("Point is: {point}"),
        _ => ()
    }
}
