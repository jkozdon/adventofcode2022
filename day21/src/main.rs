use std::collections::HashMap;
use std::env;
use std::fs;

fn get_result(name: &str, data: &HashMap<&str, &str>) -> u64 {
    let expr = data.get(name).unwrap().trim();
    match expr.parse::<u64>() {
        Ok(n) => n,
        _ => {
            let tok: Vec<&str> = expr.splitn(3, ' ').collect();
            let a = get_result(tok[0], data);
            let b = get_result(tok[2], data);
            if tok[1] == "+" {
                a + b
            } else if tok[1] == "-" {
                a - b
            } else if tok[1] == "*" {
                a * b
            } else if tok[1] == "/" {
                a / b
            } else {
                panic!()
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut data = HashMap::new();
    for l in file.lines() {
        let (name, expr) = l.split_once(':').unwrap();
        data.insert(name, expr);
    }
    println!("root: {}", get_result("root", &data));
}
