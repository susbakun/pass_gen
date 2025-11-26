#[allow(clippy::all, clippy::pedantic)]
mod passgen;

fn main() {
    let length:u8 = std::env::args().nth(1)
        .unwrap_or("12".to_string())
        .parse()
        .expect("Invalid number");

    let password = passgen::run(length);
    

    println!("Here is the password: {password}");
}