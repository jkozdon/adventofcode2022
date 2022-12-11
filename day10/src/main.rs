use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut cycle: i32 = 1;
    let mut total: i32 = 1;
    let mut result: i32 = 0;
    let mut check: i32 = 20;
    for line in file.lines() {
        cycle += 1;
        if cycle == check {
            result += cycle * total;
        }
        if line.len() > 4 {
            let num: i32 = line.split_once(' ').unwrap().1.parse().unwrap();
            total += num;
            cycle += 1;
            if cycle == check {
                result += cycle * total;
            }
        }
        if cycle >= check {
            check += 40;
        }
        if cycle >= 220 {
            break;
        }
    }
    println!("strength sum: {}", result);

    let mut pixel: i32 = 0;
    let mut X: i32 = 1;
    for line in file.lines() {
        if X - 1 == pixel || X == pixel || X + 1 == pixel {
            print!("#");
        } else {
            print!(" ");
        }
        pixel += 1;
        if pixel % 40 == 0 {
            println!("");
            pixel = 0;
        }
        if line.len() > 4 {
            let num: i32 = line.split_once(' ').unwrap().1.parse().unwrap();
            if X - 1 == pixel || X == pixel || X + 1 == pixel {
                print!("#");
            } else {
                print!(" ");
            }
            pixel += 1;
            if pixel % 40 == 0 {
                println!("");
                pixel = 0;
            }
            X += num;
        }
    }
}
