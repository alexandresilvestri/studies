use std::io::{self, Write};

struct Node {
    number: i32,
    next: Option<Box<Node>>,
}

fn print_stored_numbers(head: &Option<Box<Node>>) {
    let mut ptr = head;
    let mut counter = 1;

    while let Some(node) = ptr {
        println!("Value on node {} is: {}", counter, node.number);
        ptr = &node.next;
        counter += 1;
    }
}

fn read_int(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    let mut list: Option<Box<Node>> = None;

    let qty = read_int("Type the quantity of numbers you want to store: ");

    for i in 0..qty {
        let current = i + 1;
        let carrier = read_int(&format!("Type the number {} to store: ", current));

        let tmp = Box::new(Node {
            number: carrier,
            next: list,
        });
        list = Some(tmp);
    }

    print_stored_numbers(&list);
}

