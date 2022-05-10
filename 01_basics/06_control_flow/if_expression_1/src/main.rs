fn main() {
    
    let number = 3;
    

    if number == 3 {
        println!("{} è il numero perfetto", number);
    } else {
        println!("{} non è il numero perfetto", number);
    }

    let condition = true;
    let mynumber = if condition { 5 } else { 6 };
    println!("il valore di mynumber è {}", mynumber);
    
}
