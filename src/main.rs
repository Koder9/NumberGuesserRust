use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::time::Duration;

fn read_input(prompt: &str) -> Result<u32, String> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;
    input.trim().parse().map_err(|_| "Invalid input".to_string())
}

fn main() {
    print!("{esc}c", esc = 27 as char);
    println!("Guess the number!\n");

    // Take integer delay input
    let delay = match read_input("Please input the delay in seconds.") {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    print!("{esc}c", esc = 27 as char);
    std::thread::sleep(Duration::from_secs(delay.into()));

    // Take range
    let startrange: u32 = match read_input("Enter the start range") {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    print!("{esc}c", esc = 27 as char);
    std::thread::sleep(Duration::from_secs(delay.into()));

    let endrange = match read_input("Enter the end range") {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    print!("{esc}c", esc = 27 as char);
    std::thread::sleep(Duration::from_secs(delay.into()));

    let secret_number = rand::thread_rng().gen_range(startrange..=endrange);

    loop {
        println!("Please input your guess.");

        let guess = match read_input("") {
            Ok(num) => num,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!();
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
        
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("The correct number was {}", secret_number);
                println!("You win!");
                break;
            }
        }
        print!("{esc}c", esc = 27 as char);
        std::thread::sleep(Duration::from_secs(delay.into()));
    }
}
