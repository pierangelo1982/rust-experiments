fn main() {
    
    let mut number = 3;

    while number != 0 {

        println!("{}!", number);
        number -= 1;

    }

    println!("LIFTOFF!!!");

    println!("**********************************");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!("***********************************");

    for element in a {
        println!("the value is: {}", element);
    }

    println!("************************************");

    for n in (1..4).rev() {
        println!("the value is: {}", n);
    }
    println!("LIFTOFF!!!");
}
