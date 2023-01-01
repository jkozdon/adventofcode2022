use std::env;
use std::fs;

fn do_move_a(
    (x, y, d): (usize, usize, usize),
    num: u32,
    map: &Vec<Vec<char>>,
) -> (usize, usize, usize) {
    let mut x = x;
    let mut y = y;
    for _ in 0..num {
        if d == 0 {
            if x + 1 < map[y].len() && map[y][x + 1] != ' ' {
                if map[y][x + 1] == '.' {
                    x += 1;
                } else {
                    break;
                }
            } else {
                let mut xx = 0;
                while map[y][xx] == ' ' {
                    xx += 1;
                }
                if map[y][xx] == '.' {
                    x = xx;
                } else {
                    break;
                }
            }
        } else if d == 1 {
            if y + 1 < map.len() && x < map[y + 1].len() && map[y + 1][x] != ' ' {
                if map[y + 1][x] == '.' {
                    y += 1;
                } else {
                    break;
                }
            } else {
                let mut yy = 0;
                while map[yy][x] == ' ' {
                    yy += 1;
                }
                if map[yy][x] == '.' {
                    y = yy;
                } else {
                    break;
                }
            }
        } else if d == 2 {
            if x > 0 && map[y][x - 1] != ' ' {
                if map[y][x - 1] == '.' {
                    x -= 1;
                } else {
                    break;
                }
            } else {
                let mut xx = map[y].len() - 1;
                while map[y][xx] == ' ' {
                    xx -= 1;
                }
                if map[y][xx] == '.' {
                    x = xx;
                } else {
                    break;
                }
            }
        } else if d == 3 {
            if y > 0 && map[y - 1][x] != ' ' {
                if map[y - 1][x] == '.' {
                    y -= 1;
                } else {
                    break;
                }
            } else {
                let mut yy = map.len() - 1;
                while map[yy].len() <= x || map[yy][x] == ' ' {
                    yy -= 1;
                }
                if map[yy][x] == '.' {
                    y = yy;
                } else {
                    break;
                }
            }
        } else {
            panic!("{}", d);
        }
    }
    (x, y, d)
}

fn get_panel4(x: usize, y: usize, sz: usize) -> usize {
    if y < sz {
        1
    } else if x < sz {
        2
    } else if x < 2 * sz {
        3
    } else if y < 2 * sz {
        4
    } else if x < 3 * sz {
        5
    } else {
        6
    }
}

fn do_move_b4(
    (x, y, d): (usize, usize, usize),
    num: u32,
    map: &Vec<Vec<char>>,
) -> (usize, usize, usize) {
    let mut x = x;
    let mut y = y;
    let mut d = d;
    let sz = map.len() / 3;
    for _ in 0..num {
        if d == 0 {
            if x + 1 < map[y].len() && map[y][x + 1] != ' ' {
                if map[y][x + 1] == '.' {
                    x += 1;
                } else {
                    break;
                }
            } else {
                let dy = y % sz;
                let (xx, yy, dd) = match get_panel4(x, y, sz) {
                    1 => (4 * sz - 1, 2 * sz - 1 - dy, 2), // 6
                    4 => (4 * sz - 1 - dy, 2 * sz, 1),     // 6
                    6 => (3 * sz - 1, sz - 1 - dy, 2),     // 1
                    _ => panic!(),
                };
                if map[yy][xx] == '.' {
                    (x, y, d) = (xx, yy, dd);
                } else if map[yy][xx] == ' ' {
                    panic!();
                } else {
                    break;
                };
            }
        } else if d == 1 {
            if y + 1 < map.len() && x < map[y + 1].len() && map[y + 1][x] != ' ' {
                if map[y + 1][x] == '.' {
                    y += 1;
                } else {
                    break;
                }
            } else {
                let dx = x % sz;
                let (xx, yy, dd) = match get_panel4(x, y, sz) {
                    2 => (3 * sz - 1 - dx, 3 * sz - 1, 3), // 5
                    3 => (2 * sz, 3 * sz - 1 - dx, 0),     // 5
                    5 => (sz - 1 - dx, 2 * sz - 1, 3),     // 2
                    6 => (0, 2 * sz - 1 - dx, 0),          // 2
                    _ => panic!(),
                };
                if map[yy][xx] == '.' {
                    (x, y, d) = (xx, yy, dd);
                } else if map[yy][xx] == ' ' {
                    panic!();
                } else {
                    break;
                };
            }
        } else if d == 2 {
            if x > 0 && map[y][x - 1] != ' ' {
                if map[y][x - 1] == '.' {
                    x -= 1;
                } else {
                    break;
                }
            } else {
                let dy = y % sz;
                let (xx, yy, dd) = match get_panel4(x, y, sz) {
                    1 => (sz + dy, sz, 1),                 // 3
                    2 => (4 * sz - 1 - dy, 3 * sz - 1, 3), // 6
                    5 => (3 * sz - 1 - dy, 2 * sz - 1, 3), // 3
                    _ => panic!(),
                };
                if map[yy][xx] == '.' {
                    (x, y, d) = (xx, yy, dd);
                } else if map[yy][xx] == ' ' {
                    panic!();
                } else {
                    break;
                };
            }
        } else if d == 3 {
            if y > 0 && map[y - 1][x] != ' ' {
                if map[y - 1][x] == '.' {
                    y -= 1;
                } else {
                    break;
                }
            } else {
                let dx = x % sz;
                let (xx, yy, dd) = match get_panel4(x, y, sz) {
                    1 => (sz - 1 - dx, sz, 1),             // 2
                    2 => (3 * sz - 1 - dx, 0, 1),          // 1
                    3 => (2 * sz, dx, 0),                  // 1
                    6 => (3 * sz - 1, 2 * sz - 1 - dx, 2), // 4
                    _ => panic!(),
                };
                if map[yy][xx] == '.' {
                    (x, y, d) = (xx, yy, dd);
                } else if map[yy][xx] == ' ' {
                    panic!();
                } else {
                    break;
                };
            }
        } else {
            panic!("{}", d);
        }
    }
    (x, y, d)
}

fn get_panel(x: usize, y: usize, sz: usize) -> usize {
    if y < sz {
        if x < 2 * sz {
            1
        } else {
            2
        }
    } else if y < 2 * sz {
        3
    } else if y < 3 * sz {
        if x < sz {
            4
        } else {
            5
        }
    } else {
        6
    }
}

fn do_move_b(
    (x, y, d): (usize, usize, usize),
    num: u32,
    map: &Vec<Vec<char>>,
) -> (usize, usize, usize) {
    let mut x = x;
    let mut y = y;
    let mut d = d;
    let sz = map.len() / 4;
    let st = [
        [0, 0, 0, 0],
        [sz, 2 * sz - 1, 0, sz - 1],
        [2 * sz, 3 * sz - 1, 0, sz - 1],
        [sz, 2 * sz - 1, sz, 2 * sz - 1],
        [0, sz - 1, 2 * sz, 3 * sz - 1],
        [sz, 2 * sz - 1, 2 * sz, 3 * sz - 1],
        [0, sz - 1, 3 * sz, 4 * sz - 1],
    ];
    for _ in 0..num {
        let (xx, yy, dd) = if d == 0 {
            if x + 1 < map[y].len() && map[y][x + 1] != ' ' {
                (x + 1, y, d)
            } else {
                let f = y % sz;
                let b = sz - 1 - f;
                match get_panel(x, y, sz) {
                    2 => (st[5][1], st[5][2] + b, 2), // 5
                    3 => (st[2][0] + f, st[2][3], 3), // 2
                    5 => (st[2][1], st[2][2] + b, 2), // 2
                    6 => (st[5][0] + f, st[5][3], 3), // 5
                    _ => panic!(),
                }
            }
        } else if d == 1 {
            if y + 1 < map.len() && x < map[y + 1].len() && map[y + 1][x] != ' ' {
                (x, y + 1, d)
            } else {
                let f = x % sz;
                let b = sz - 1 - f;
                match get_panel(x, y, sz) {
                    2 => (st[3][1], st[3][2] + f, 2), // 3
                    5 => (st[6][1], st[6][2] + f, 2), // 6
                    6 => (st[2][0] + f, st[2][2], 1), // 2
                    k => panic!("{}", k),
                }
            }
        } else if d == 2 {
            if x > 0 && map[y][x - 1] != ' ' {
                (x - 1, y, d)
            } else {
                let f = y % sz;
                let b = sz - 1 - f;
                match get_panel(x, y, sz) {
                    1 => (st[4][0], st[4][2] + b, 0), // 4
                    3 => (st[4][0] + f, st[4][2], 1), // 4
                    4 => (st[1][0], st[1][2] + b, 0), // 1
                    6 => (st[1][0] + f, st[1][2], 1), // 1
                    _ => panic!(),
                }
            }
        } else if d == 3 {
            if y > 0 && map[y - 1][x] != ' ' {
                (x, y - 1, d)
            } else {
                let f = x % sz;
                let b = sz - 1 - f;
                match get_panel(x, y, sz) {
                    1 => (st[6][0], st[6][2] + f, 0), // 6
                    2 => (st[6][0] + f, st[6][3], 3), // 6
                    4 => (st[3][0], st[3][2] + f, 0), // 3
                    _ => panic!(),
                }
            }
        } else {
            panic!("{}", d);
        };
        if map[yy][xx] == '.' {
            (x, y, d) = (xx, yy, dd);
        } else if map[yy][xx] == ' ' {
            panic!();
        } else {
            break;
        };
    }
    (x, y, d)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in file.lines() {
        if line.len() == 0 {
            break;
        }
        map.push(line.chars().collect());
    }
    let route = file.lines().last().unwrap();

    let mut pos_a: (usize, usize, usize) = (0, 0, 0);
    for (i, v) in map[0].iter().enumerate() {
        if *v == '.' {
            pos_a = (i, 0, 0);
            break;
        }
    }
    let mut pos_b = pos_a.clone();

    for mv in route.split_inclusive(|c: char| c == 'R' || c == 'L') {
        let (num, turn) = if mv.chars().last().unwrap() == 'R' {
            (mv.get(0..mv.len() - 1).unwrap(), Some('R'))
        } else if mv.chars().last().unwrap() == 'L' {
            (mv.get(0..mv.len() - 1).unwrap(), Some('L'))
        } else {
            (mv, None)
        };
        let num: u32 = num.trim().parse().expect("number");

        pos_a = do_move_a(pos_a, num, &map);
        if turn == Some('R') {
            pos_a.2 = if pos_a.2 < 3 { pos_a.2 + 1 } else { 0 };
        } else if turn == Some('L') {
            pos_a.2 = if pos_a.2 > 0 { pos_a.2 - 1 } else { 3 };
        }

        pos_b = if map.len() == 12 {
            do_move_b4(pos_b, num, &map)
        } else {
            do_move_b(pos_b, num, &map)
        };
        if turn == Some('R') {
            pos_b.2 = if pos_b.2 < 3 { pos_b.2 + 1 } else { 0 };
        } else if turn == Some('L') {
            pos_b.2 = if pos_b.2 > 0 { pos_b.2 - 1 } else { 3 };
        }
    }
    println!(
        "flat score: {:}",
        (pos_a.1 + 1) * 1000 + (pos_a.0 + 1) * 4 + pos_a.2
    );
    println!(
        "cube score: {:}",
        (pos_b.1 + 1) * 1000 + (pos_b.0 + 1) * 4 + pos_b.2
    );
}
