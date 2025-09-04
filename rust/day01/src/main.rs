use std::io;

fn cel_to_far() {
    println!("cel_to_far");
}

fn far_to_cel() {
    println!("fat_to_cel");
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
