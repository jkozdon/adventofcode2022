use std::collections::HashMap;
use std::env;
use std::fs;

fn show_chamber(chamber: &Vec<u8>, top: usize) {
    for i in (0..top + 1).rev() {
        print!("|");
        for j in (0..7).rev() {
            if chamber[i] & (1 << j) == 0 {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("|");
    }
}

fn valid_move(shape: &Vec<u8>, chamber: &Vec<u8>, bottom: usize) -> bool {
    let mut valid = true;
    for (i, block) in (*shape).iter().enumerate() {
        valid = valid && (*block & chamber[bottom + i] == 0);
    }
    valid
}

fn get_shape(num: u128) -> (Vec<u8>, (u8, u8)) {
    let num = num % 5;
    if num == 0 {
        (
            vec![(1 << 4) | (1 << 3) | (1 << 2) | (1 << 1)], //
            (1, 4),
        )
    } else if num == 1 {
        (
            vec![
                1 << 3,                         //
                (1 << 4) | (1 << 3) | (1 << 2), //
                1 << 3,
            ], //
            (2, 4),
        )
    } else if num == 2 {
        (
            vec![
                (1 << 4) | (1 << 3) | (1 << 2), //
                (1 << 2),                       //
                (1 << 2),
            ], //
            (2, 4),
        )
    } else if num == 3 {
        (
            vec![
                (1 << 4), //
                (1 << 4), //
                (1 << 4), //
                (1 << 4),
            ], //
            (4, 4),
        )
    } else if num == 4 {
        (
            vec![
                (1 << 4) | (1 << 3), //
                (1 << 4) | (1 << 3),
            ], //
            (3, 4),
        )
    } else {
        panic!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let first_shape: u128 = 2022;
    let last_shape: u128 = 1000000000000;
    let mut chamber = vec![0 as u8; (last_shape as usize) * 4];
    chamber[0] = !0;
    let mut stack_top = 0 as usize;

    let mut shape_num = 0 as u128;
    let wind: Vec<char> = file.trim().chars().collect();
    let mut wind_num = 0 as usize;

    let mut cache = HashMap::<(u128, usize), (u128, usize, Vec<u8>)>::new();
    let mut check = true;

    let mut shape_shift = 0 as u128;
    let mut stack_shift = 0 as usize;
    let look_back = 100;
    loop {
        if check {
            if cache.contains_key(&(shape_num % 5, wind_num)) {
                let (a_shape_num, a_stack_top, a_c) =
                    cache.get(&(shape_num % 5, wind_num)).unwrap();
                let mut matching = 0;
                for i in 0..a_c.len() {
                    if a_c[i] == chamber[stack_top - look_back + i] {
                        matching += 1;
                    }
                }
                if matching != a_c.len() {
                    cache.insert(
                        (shape_num % 5, wind_num),
                        (
                            shape_num,
                            stack_top,
                            chamber[stack_top - look_back..stack_top + 1].to_vec(),
                        ),
                    );
                } else {
                    let rem = last_shape - shape_num;
                    let diff = shape_num - a_shape_num;
                    let scale = rem / diff;
                    stack_shift = (scale as usize) * (stack_top - a_stack_top);
                    shape_shift = (scale as u128) * diff;
                    check = false;
                    if shape_num + shape_shift == last_shape {
                        break;
                    }
                }
            } else {
                if stack_top > look_back {
                    cache.insert(
                        (shape_num % 5, wind_num),
                        (
                            shape_num,
                            stack_top,
                            chamber[stack_top - look_back..stack_top + 1].to_vec(),
                        ),
                    );
                }
            }
        }
        let (mut shape, mut sides) = get_shape(shape_num);
        shape_num += 1;
        let mut bottom = stack_top + 4;
        loop {
            if wind[wind_num] == '<' {
                if sides.1 < 6 {
                    for block in shape.iter_mut() {
                        *block <<= 1;
                    }
                    if valid_move(&shape, &chamber, bottom) {
                        sides = (sides.0 + 1, sides.1 + 1);
                    } else {
                        for block in shape.iter_mut() {
                            *block >>= 1;
                        }
                    }
                }
            } else {
                if sides.0 > 0 {
                    for block in shape.iter_mut() {
                        *block >>= 1;
                    }
                    if valid_move(&shape, &chamber, bottom) {
                        sides = (sides.0 - 1, sides.1 - 1);
                    } else {
                        for block in shape.iter_mut() {
                            *block <<= 1;
                        }
                    }
                }
            }
            wind_num = (wind_num + 1) % wind.len();

            if valid_move(&shape, &chamber, bottom - 1) {
                bottom -= 1;
            } else {
                break;
            }
        }
        for (i, block) in shape.iter_mut().enumerate() {
            chamber[bottom + i] |= *block;
        }
        stack_top = std::cmp::max(stack_top, bottom + shape.len() - 1);
        //show_chamber(&chamber, stack_top);
        if shape_num + shape_shift == first_shape {
            println!(
                "height at {}:          {}",
                first_shape,
                stack_top + stack_shift
            );
        }
        if shape_num + shape_shift == last_shape {
            println!("height at {}: {}", last_shape, stack_top + stack_shift);
            return;
        }
    }
}
