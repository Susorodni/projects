fn main() {
    let guy = String::from("bruh");
    println!("{guy}");
    println!("{guy}");

    { // s not valid
        let s = "Hello"; // valif now
        let stuff = String::from("hello"); // same, but dealocated from heap
    } // s no long valid

    let x = 5;
    let y= x; // this is a copy on the stack

    let s1 = "hello"; // literal string
    let sthing = String::from("stuff");
    let s2 = sthing; // s1 is MOVED to s2
    // can't use s1 anymore since it doesn't have the copy trait
    // println!("{s1}");

    let sokay = s2.clone();
    println!("{sokay}");

    let amogus = String::from("Stuff");
    take_ownership(&amogus);
    println!("{amogus}");

    epic_stuff();

    let mut s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[..];

    println!("{}, so we got, {hello}, {world}", first_word(&s));

    let mut sus = "amogus".to_own;

    let mut thingy = String::from("asdasdasd");

    thingy.push_str("string");

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// OWNERSHIP RULES
// 1. Each value in rust has a variable called its OWNER
// 2. There can only be ONE owner at a time
// 3. When the owner goes out of scope, THE VALUE
// WILL BE DROPPED

// REFERENCE RULES
// 1. At any time

// when something's not passed in as a referene it stays there
// forever, for any function

// You can only have ONE mutable reference to a mutable variable at a time
// let mut s = String::from("hello");
// let r1 = &mut s;
// let r2 = &mut s; <--- won't work!

// Can't have both a mutable AND an immutable reference in the same scope
// since the immutable reference does not expect the value to change!

// let r1 = &s;
// let r2 = &s;
// let r3 = &mut s;
// println!("{r1}, {r2}, {r3}");

// You can do this though, after the last line when r1 and r2 are used 

// let r1 = &s;
// let r2 = &s;
// println!("{r1}, {r2}");
// let r3 = &mut s;

// You can have multiple immutable references

fn take_ownership(some_string: &String) {
    println!("{some_string}");
}

fn epic_stuff() {
    let mut x = 5;
    let mut y = &mut x;
    
    *y += 1;

    println!("y: {}", *y);
    println!("x: {x}");
}