use std::env;
use std::fs;

fn decreaselvl(b: &[u8], alvl: i32, blvl: &mut i32, j: &mut usize) {
    while alvl != *blvl {
        if b[*j] == b'[' {
            *blvl += 1;
        } else if b[*j] == b']' {
            *blvl -= 1;
        }
        *j += 1;
    }
}

fn verify(a: &[u8], b: &[u8]) -> bool {
    let (mut i, mut j) = (0, 0);
    let (mut alvl, mut blvl) = (0, 0);
    while i < a.len() && j < b.len() {
        // move up a level
        if a[i] == b'[' && b[j] == b'[' {
            alvl += 1;
            blvl += 1;
        // left is shorter which is ok
        } else if a[i] == b']' {
            alvl -= 1;
            decreaselvl(b, alvl, &mut blvl, &mut j)
        // right is shorter which is bad
        } else if b[i] == b']' {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    for l in file.split("\n\n") {
        let (a, b) = l.split_once("\n").unwrap();
        println!("{}", a);
        println!("{}", b);
        println!("{}", verify(a.as_bytes(), b.as_bytes()));
        println!("----");
    }
}
