use std::collections::HashMap;
use std::env;
use std::fs;

fn has_humn(name: &str, data: &HashMap<&str, &str>) -> bool {
    if name == "humn" {
        true
    } else {
        let expr = data.get(name).unwrap().trim();
        match expr.parse::<u64>() {
            Ok(n) => false,
            _ => {
                let tok: Vec<&str> = expr.splitn(3, ' ').collect();
                has_humn(tok[0], data) || has_humn(tok[2], data)
            }
        }
    }
}

fn get_humn(cur_res: u64, name: &str, data: &HashMap<&str, &str>) -> u64 {
    if name == "humn" {
        return cur_res;
    }
    let expr = data.get(name).unwrap().trim();
    let tok: Vec<&str> = expr.splitn(3, ' ').collect();
    let a = tok[0];
    let b = tok[2];
    let first = has_humn(a, data);
    let new_res = if first {
        get_result(b, data)
    } else {
        get_result(a, data)
    };
    if name == "root" {
        get_humn(new_res, if first { a } else { b }, data)
    } else if tok[1] == "+" {
        get_humn(cur_res - new_res, if first { a } else { b }, data)
    } else if tok[1] == "-" {
        if first {
            get_humn(cur_res + new_res, a, data)
        } else {
            get_humn(new_res - cur_res, b, data)
        }
    } else if tok[1] == "*" {
        get_humn(cur_res / new_res, if first { a } else { b }, data)
    } else if tok[1] == "/" {
        if first {
            get_humn(cur_res * new_res, a, data)
        } else {
            panic!()
        }
    } else {
        panic!()
    }
}

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

    println!("humn: {}", get_humn(0, "root", &data));
}
