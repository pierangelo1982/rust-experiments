fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    println!("x is not mutable, will remain,  {}", x);
    let mut y = 10;
    println!("The value of y is {}", y);
    y = 15;
    println!("The value of y è mut(able) is {}", y);

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of const THREE_HOURS_IN_SECONDS {}", THREE_HOURS_IN_SECONDS);

    println!("*******************");
    println!("**** shadowing ****");
    println!("*******************");
    let z = 5;
    let z = z + 5;
    {
        let z = z * 2;
        println!("The value of z is {}", z);
    }

    println!("The value of z is {}", z);


}