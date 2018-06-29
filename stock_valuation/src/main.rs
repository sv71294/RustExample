use std::collections::HashMap;
use std::io;
fn main() {
    let mut stock_sum_map: HashMap<String, Vec<i32>> = HashMap::new();

    while check_io_flag() {
        let (name, value) = read_io();
        stock_sum_map.entry(name).or_insert(Vec::new()).push(value);
    }

    for (name, vector) in stock_sum_map {
        let total_value: i32 = vector.iter().sum();
        println!("Stock name: {}, total Value: {}", name, total_value);
    }
}

fn check_io_flag() -> bool {
    loop {
        println!("Do you want to add stock? y/n");
        let mut check = String::new();
        io::stdin()
            .read_line(&mut check)
            .expect("Failed to read response");

        if check.trim() == "n" {
            println!("Getting Result");
            return false;
        } else if check.trim() != "y" {
            continue;
        }
        return true;
    }
}

fn read_io() -> (String, i32) {
    println!("Please enter stock name:");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    let value: i32;
    loop {
        println!("Please enter stock value:");
        let mut value_str = String::new();
        io::stdin()
            .read_line(&mut value_str)
            .expect("Failed to read the value");

        value = match value_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Value is not a number!!");
                continue;
            }
        };
        break;
    }
    (name.trim().to_string(), value)
}
