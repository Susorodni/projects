fn main() {
    let x = 5;
    
    let x = x + 1;

    {
        // this will stay 12 since it's within a different scopeyou
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is: {x}");
}
