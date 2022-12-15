use std::env;
use std::fs;
fn verify(a: &[u8], b: &[u8]) -> bool {
    let (mut i, mut j) = (0, 0);
    if a[i] == '[' as u8 && b[j] == '[' as u8 {
    } else if a[i] == '[' as u8 {
    } else if b[j] == '[' as u8 {
    } else {
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
