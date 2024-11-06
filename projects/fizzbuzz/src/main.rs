use std::io;

fn main() {
    println!("数値を入力してください：");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    let num: i32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("有効cd .な数値を入力してください");
            return;
        }
    };

    for i in 1..=num {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            (_, _) => println!("{}", i),
        }
    }
}
