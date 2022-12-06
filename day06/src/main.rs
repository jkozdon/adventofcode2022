use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let data = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut pkt = vec![0 as usize; 4];
    let mut msg = vec![0 as usize; 14];
    let mut pkt_found = false;
    for (i, c) in data.trim().chars().enumerate() {
        if !pkt_found {
            for j in 0..3 {
                pkt[j] = pkt[j + 1];
            }
            pkt[3] = 1 << ((c as usize) - ('a' as usize));
            let mut chk = pkt[0];
            if chk > 0 {
                for j in 1..4 {
                    if chk & pkt[j] != 0 {
                        chk = 0;
                        break;
                    }
                    chk = chk | pkt[j];
                }
            }
            if chk != 0 {
                pkt_found = true;
                println!("packet:  {}", i + 1);
            }
        }
        for j in 0..13 {
            msg[j] = msg[j + 1];
        }
        msg[13] = 1 << ((c as usize) - ('a' as usize));
        let mut chk = msg[0];
        if chk > 0 {
            for j in 1..14 {
                if chk & msg[j] != 0 {
                    chk = 0;
                    break;
                }
                chk = chk | msg[j];
            }
        }
        if chk != 0 {
            println!("message: {}", i + 1);
            break;
        }
    }
}
