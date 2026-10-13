use std::io;

fn calc_fibonacci(n: i8) -> i64 {
    if n == 0 { return 0 };
    if n == 1 { return 1 };

    let mut acc: i64 = 1;
    let mut previous: i64 = 0;
    let mut i: i8 = 1;

    loop {
        if i == n { break; }
        let tmp = acc;
        acc += previous;
        previous = tmp;
        i += 1; 
    }

    return acc;
}

fn main() {
    println!("Type the number to calc fibonacci: ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the response");
    let number: i8 = number.trim().parse::<i8>().expect("Número deve ser no máximo 255.");
    let response: i64 = calc_fibonacci(number);
    println!("{response}");
}

