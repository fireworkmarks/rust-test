use rand::Rng;  // 用于生成随机数的引用
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");
    let secret_number = rand::thread_rng().gen_range(01..101);
    println!("猜一个数：");

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        // shadow
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("猜测的数是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"), // arm
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("恭喜你，猜对了！");
                break;
            },
        }
    }
}
