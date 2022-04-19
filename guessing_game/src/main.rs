use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the num");

    let ans = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 여기서 중복선언 왜 가능? => shadowing, 타입 바꿀 때.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("숫자를 입력해");
                continue;
            }
        };

        match guess.cmp(&ans) { // &는 뭐하는 놈이냐 => reference. ch4
            Ordering::Less => println!("작아"),
            Ordering::Greater => println!("커"),
            Ordering::Equal => {
                println!("정답");
                break;
            }
        }
    }
}