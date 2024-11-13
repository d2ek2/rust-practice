use std::io;

fn check_fizzbuzz(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".to_string(),
        (0, _) => "Fizz".to_string(),
        (_, 0) => "Buzz".to_string(),
        (_, _) => n.to_string(),
    }
}

fn main() {
    println!("数値を入力してください：");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("入力の読み取りに失敗しました");

    match input.trim().parse::<i32>() {
        Ok(number) => {
            if number <= 0 {
                println!("正の整数を入力してください");
                return;
            }
            let result = check_fizzbuzz(number);
            println!("入力値: {}", number);
            println!("結果: {}", result);
        }
        Err(_) => {
            println!("有効な整数を入力してください");
            return;
        }
    }
}

#[test]
fn test_fizzbuzz() {
    assert_eq!(check_fizzbuzz(1), "1");
    assert_eq!(check_fizzbuzz(3), "Fizz");
    assert_eq!(check_fizzbuzz(5), "Buzz");
    assert_eq!(check_fizzbuzz(225), "FizzBuzz");
    assert_eq!(check_fizzbuzz(7), "7");
}
