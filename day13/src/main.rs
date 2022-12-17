use std::env;
use std::fs;

fn decreaselvl(rhs: &[u8], lhslvl: i32, rhslvl: &mut i32, rhs_iter: &mut usize) {
    while lhslvl != *rhslvl {
        if rhs[*rhs_iter] == b'[' {
            *rhslvl += 1;
        } else if rhs[*rhs_iter] == b']' {
            *rhslvl -= 1;
        }
        *rhs_iter += 1;
    }
    *rhs_iter -= 1;
}

fn increaselvl(rhs: &[u8], rhslvl: &mut i32, rhs_iter: &mut usize) {
    while rhs[*rhs_iter] == b'[' {
        *rhslvl += 1;
        *rhs_iter += 1;
    }
}

fn getnum(rhs: &[u8], rhs_iter: &mut usize) -> i32 {
    let rhs_start = *rhs_iter;
    while rhs[*rhs_iter + 1] != b']' && rhs[*rhs_iter + 1] != b',' {
        *rhs_iter += 1;
    }
    String::from_utf8_lossy(&rhs[rhs_start..(*rhs_iter + 1)])
        .parse()
        .expect("number")
}

fn verify(lhs: &[u8], rhs: &[u8]) -> bool {
    let (mut lhs_iter, mut rhs_iter) = (0, 0);
    let (mut lhslvl, mut rhslvl) = (0, 0);
    while lhs_iter < lhs.len() && rhs_iter < rhs.len() {
        // move up level
        if lhs[lhs_iter] == b'[' && rhs[rhs_iter] == b'[' {
            lhslvl += 1;
            rhslvl += 1;
        // left is shorter which is ok
        } else if lhs[lhs_iter] == b']' {
            if rhs[rhs_iter] == b']' {
                lhslvl -= 1;
                decreaselvl(rhs, lhslvl, &mut rhslvl, &mut rhs_iter);
            } else {
                return true;
            }
            // right is shorter which is bad
        } else if rhs[rhs_iter] == b']' {
            return false;
            // left is single value
        } else if rhs[rhs_iter] == b'[' {
            increaselvl(rhs, &mut rhslvl, &mut rhs_iter);
            if rhs[rhs_iter] == b']' {
                return false;
            } else {
                let l = getnum(lhs, &mut lhs_iter);
                let r = getnum(rhs, &mut rhs_iter);
                if l < r {
                    return true;
                } else if l > r {
                    return false;
                }
            }
            if rhs[rhs_iter + 1] != b']' {
                return true;
            }
            decreaselvl(rhs, lhslvl, &mut rhslvl, &mut rhs_iter);
            // right is single value
        } else if lhs[lhs_iter] == b'[' {
            increaselvl(lhs, &mut lhslvl, &mut lhs_iter);
            if lhs[lhs_iter] == b']' {
                return true;
            } else {
                let l = getnum(lhs, &mut lhs_iter);
                let r = getnum(rhs, &mut rhs_iter);
                if l < r {
                    return true;
                } else if l > r {
                    return false;
                }
            }
            if lhs[lhs_iter + 1] != b']' {
                return false;
            }
            decreaselvl(lhs, rhslvl, &mut lhslvl, &mut lhs_iter);
            // comma so move on
        } else if lhs[lhs_iter] == b',' && lhs[lhs_iter] == b',' {
            // should never get here
        } else if lhs[lhs_iter] == b',' || lhs[lhs_iter] == b',' {
            panic!();
            // compare results
        } else {
            let l = getnum(lhs, &mut lhs_iter);
            let r = getnum(rhs, &mut rhs_iter);
            if l < r {
                return true;
            } else if l > r {
                return false;
            }
        }
        lhs_iter += 1;
        rhs_iter += 1;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut sum = 0;
    let mut two = 1;
    let mut six = 2;
    for (i, l) in file.split("\n\n").enumerate() {
        let (lhs, rhs) = l.split_once("\n").unwrap();
        let (lhs, rhs) = (lhs.trim(), rhs.trim());
        if verify(lhs.as_bytes(), rhs.as_bytes()) {
            sum += i + 1;
        }
        if verify(lhs.as_bytes(), "[[2]]".as_bytes()) {
            two += 1;
        }
        if verify(lhs.as_bytes(), "[[6]]".as_bytes()) {
            six += 1;
        }
        if verify(rhs.as_bytes(), "[[2]]".as_bytes()) {
            two += 1;
        }
        if verify(rhs.as_bytes(), "[[6]]".as_bytes()) {
            six += 1;
        }
    }
    println!("sum of pairs:   {}", sum);
    println!("special values: {}", two * six);
}
