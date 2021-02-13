use std::io::{Read, Write};

fn main() {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    print!("Введите строку: ");
    stdout.flush().expect("Failed to flush stdout");

    let mut string = String::new();
    stdin.read_line(&mut string).expect("Failed to read a line");

    print!("Введите символ для поиска: ");
    stdout.flush().expect("Failed to flush stdout");

    let mut character = String::new();
    stdin.read_line(&mut character).expect("Failed to read a line");
    let character = character.chars().nth(0).expect("Invalid input");

    let mut result: i32 = 0;
    for char in string.chars() {
        if char == character {
            result += 1;
        }
    }
    println!("Символ встречается в строке {} раз", result);

    /* 
        In Functional Programming style
        let mut result: i32 = string.chars().fold(0, |acc, x| if x == character { acc + 1 } else { acc });
    */
}
