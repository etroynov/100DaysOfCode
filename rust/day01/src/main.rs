use std::io;

fn cel_to_far() {
    println!("Enter temperature");

    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("");

    let temp: f64 = match str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input value");
            return;
        }
    };

    println!("{:.2}C -> {:.2}F", temp, (temp * 9.0 / 5.0) + 32.0);
}

fn far_to_cel() {
    println!("Enter temperature");

    let mut str = String::new();

    io::stdin().read_line(&mut str).expect("");

    let temp: f64 = match str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input value");
            return;
        }
    };

    println!("{:.2}F -> {:.2}C", temp, (temp - 32.0) * 5.0 / 9.0);
}

fn main() {
    println!("Temperature converter");
    println!("1: Celsius -> Fahrenheit");
    println!("2: Fahrenheit -> Celsius ");
    println!("Please select an option (1 or 2)");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let value: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice pls use 1 or 2");
            return;
        }
    };

    if value == 1 {
        cel_to_far();
    } else if value == 2 {
        far_to_cel();
    } else {
        println!("Invalid choice pls use 1 or 2");
    }
}
