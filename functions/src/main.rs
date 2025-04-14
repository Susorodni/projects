fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let _y = 6;

    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");

    let stuff = five();

    println!("The value of stuff is: {stuff}");
    println!("The value of amogus is: {}", plus_one(4));
}

fn another_function(x: i32) {
    println!("The value of x is : {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements: instructions that perform an action and do
// not return a value
// expressions: evaluate a resultant value

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}