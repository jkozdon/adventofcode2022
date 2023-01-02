use std::env;
use std::fs;

fn snafu_to_decimal(line: &str) -> i64 {
    let mut base = 1;
    let mut num = 0;
    for c in line.trim().chars().rev() {
        num += base
            * match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            };
        base *= 5;
    }
    num
}

fn decimal_to_snafu(mut num: i64) -> String {
    let mut snafu = "".to_string();
    let mut next = 0;
    while num > 0 || next > 0 {
        let val = next + num % 5;
        let val = if val > 4 {
            next = 1;
            val - 5
        } else {
            next = 0;
            val
        };

        next += if val <= 2 {
            snafu = val.to_string() + &snafu;
            0
        } else {
            if val == 3 {
                snafu = "=".to_string() + &snafu;
            } else if val == 4 {
                snafu = "-".to_string() + &snafu;
            } else {
                panic!("bad val: {}", val);
            }
            1
        };
        num /= 5;
    }
    snafu
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut total: i64 = 0;
    for line in file.lines() {
        total += snafu_to_decimal(line);
    }

    let snafu = decimal_to_snafu(total);
    assert!(total == snafu_to_decimal(&snafu));
    println!("sum in SNAFU: {}", snafu);
}
