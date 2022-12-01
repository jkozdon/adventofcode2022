use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut sum = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for line in file.lines() {
        if line == "" {
            if sum >= a {
                c = b;
                b = a;
                a = sum;
            } else if sum >= b {
                c = b;
                b = sum;
            } else if sum >= c {
                c = sum;
            }
            sum = 0;
        } else {
            let val: u32 = line.trim().parse().expect("Wanted a number");
            sum = sum + val;
        }
    }

    println!("maximum value:    {}", a);
    println!("sum of top three: {}", a + b + c);
}
