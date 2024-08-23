use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("数を当てて");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("予想して");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("読み込みに失敗");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("次のように予想した: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("ちいさすぎ"),
            Ordering::Greater => println!("大きすぎ"),
            Ordering::Equal => {
                println!("あなたのかち");
                break;
            }
        }
    }
}