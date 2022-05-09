fn main() {
    // let guess: u32 = "42".parse().expect("Not a number");
    let x = 2.0; // f64
    let y: f32 = 3.0;

    let sum = 5 + 10;
    println!("sum = {}", sum);

    let substraction = 95.5 - 10.4;
    println!("substraction = {}", substraction);

    let multiplication = 3 * 5;
    println!("multiplication = {}", multiplication);

    let quotient = 15 / 3;
    println!("quotient = {}", quotient);

    let floored = 2 / 3;
    println!("floored = {}", floored);

    let remainder = 43 % 5;
    println!("remainder = {}", remainder);

    let t = true;
    println!("t = {}", t);

    let f: bool = false;
    println!("f = {}", f);

    println!("**** charactered type");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("charactered type c = {}", c);
    println!("charactered type z = {}", z);
    println!("charactered type = {}", heart_eyed_cat);

    println!("**** tuple");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup = {:?}", tup);
    println!("tup = {:#?}", tup);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("x = {:?}", x);
    println!("five_hundred  = {:?}", five_hundred);
    println!("six_point_four  = {:?}", six_point_four);
    println!("one = {:?}", one);

    println!("**** array");
    let a = [1, 2, 3, 4, 5];
    println!("a = {:?}", a);
    println!("a = {:#?}", a);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months = {:?}", months);

    let a_two: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a_two = {:?}", a_two);

    let a_three = [3; 5];
    println!("a_three = {:?}", a_three);

    let a_four = [1, 2, 3, 4, 5];

    let first = a_four[0];
    let second = a_four[1];
    println!("first = {}", first);
    println!("second = {}", second);





    
}