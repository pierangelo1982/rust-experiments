const PI: f64 = 3.14;

fn main() {
    println!("Hello, world!");
    let person: String = full_name("napoleone", "bonaparte");
    println!("{}", person);

    let a: i32 = area(10, 20);
    println!("L'area è di: {}", a);

    let c_area: f64 = circle_area(33.00);
    println!("l'area del cerchio è: {:.2}", c_area);
}

fn full_name(name: &str, lastname: &str) -> String  {
    let x: String = format!("{} {}", name, lastname);
    return x;
}

fn area(x: i32, y: i32) -> i32 {
    return x * y
}

fn circle_area(x: f64) -> f64 {
    let area: f64 = (x * x) * PI;
    return area;
}