use std::io;

fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("无法读取行");

    let num: u32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    if num % 4 == 0 {
        println!("number is divisible by 4!");
    } else if num % 3 == 0 {
        println!("number is divisible by 3!");
    } else if num % 2 == 0 {
        println!("number is divisible by 2!");
    } else {
        println!("number is not divisible by 4, 3 or 2!");
    }
}
