use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число!");
    
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Пожалуйста, введи предположение:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось прочитать строку(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Строка не явяляется числом(");
                continue;
            },
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Меньше загаданного"),
            Ordering::Greater => println!("Больше загаданного"),
            Ordering::Equal => {
                println!("Ты угадал: {}", guess);
                break;
            }
        }
    }
}
