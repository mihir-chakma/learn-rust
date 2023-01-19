fn main() {

    // Variables 
    let mut x = 5;
    println!("The value of x is : {x}");

    // change the x value to 6 
    x = 6;
    println!("The value of x is : {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds : {THREE_HOURS_IN_SECONDS}");

    // Shadowing 
    let a = 5;
    let a = a + 5;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {a}");
    }

    println!("The value of a is {a}");


    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}", spaces);


}
