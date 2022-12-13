use std::collections::VecDeque;
use std::env;
use std::fs;

enum Operation {
    Add(i64),
    Mul(i64),
    OMul(),
}

struct Monkey {
    items_a: VecDeque<i64>,
    items_b: VecDeque<i64>,
    operation: Operation,
    test: i64,
    neighbor: (usize, usize),
    inspected_a: i64,
    inspected_b: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut scale = 1;
    for monkey in file.split("\n\n") {
        let monkey: Vec<&str> = monkey.split("\n").collect();
        let items: VecDeque<i64> = monkey[1]
            .trim()
            .splitn(3, ' ')
            .last()
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse().expect("number"))
            .collect();
        let num = monkey[2].rsplit_once(' ').unwrap().1.trim().parse::<i64>();
        let operation = match num {
            Ok(num) => match monkey[2].find("+") {
                Some(_) => Operation::Add(num),
                None => Operation::Mul(num),
            },
            _ => Operation::OMul(),
        };
        let test: i64 = monkey[3]
            .rsplit_once(' ')
            .unwrap()
            .1
            .trim()
            .parse()
            .expect("Number");
        scale *= test;
        let neighbor: (usize, usize) = (
            monkey[4]
                .rsplit_once(' ')
                .unwrap()
                .1
                .trim()
                .parse()
                .expect("Number"),
            monkey[5]
                .rsplit_once(' ')
                .unwrap()
                .1
                .trim()
                .parse()
                .expect("Number"),
        );
        let monkey = Monkey {
            items_a: items.clone(),
            items_b: items,
            operation: operation,
            test: test,
            neighbor: neighbor,
            inspected_a: 0,
            inspected_b: 0,
        };
        monkeys.push(monkey);
    }

    // PART A
    for _ in 0..20 {
        for k in 0..monkeys.len() {
            let (a, b) = monkeys[k].neighbor;
            let test = monkeys[k].test;
            while !monkeys[k].items_a.is_empty() {
                monkeys[k].inspected_a += 1;
                let item = monkeys[k].items_a.pop_front().unwrap();
                let item = match monkeys[k].operation {
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                    Operation::OMul() => item * item,
                };
                let item = item / 3;
                if item % test == 0 {
                    monkeys[a].items_a.push_back(item);
                } else {
                    monkeys[b].items_a.push_back(item);
                }
            }
        }
    }
    let mut max = (0, 0);
    for monkey in &monkeys {
        if max.0 <= monkey.inspected_a {
            max.1 = max.0;
            max.0 = monkey.inspected_a;
        } else if max.1 <= monkey.inspected_a {
            max.1 = monkey.inspected_a;
        }
    }
    println!("divide by 3: {}", max.0 * max.1);

    // PART B
    for _ in 0..10_000 {
        for k in 0..monkeys.len() {
            let (a, b) = monkeys[k].neighbor;
            let test = monkeys[k].test;
            while !monkeys[k].items_b.is_empty() {
                monkeys[k].inspected_b += 1;
                let item = monkeys[k].items_b.pop_front().unwrap();
                let item = match monkeys[k].operation {
                    Operation::Add(n) => item + n,
                    Operation::Mul(n) => item * n,
                    Operation::OMul() => item * item,
                };
                let item = item % scale;
                if item % test == 0 {
                    monkeys[a].items_b.push_back(item);
                } else {
                    monkeys[b].items_b.push_back(item);
                }
            }
        }
    }
    let mut max = (0, 0);
    for monkey in &monkeys {
        if max.0 <= monkey.inspected_b {
            max.1 = max.0;
            max.0 = monkey.inspected_b;
        } else if max.1 <= monkey.inspected_b {
            max.1 = monkey.inspected_b;
        }
    }
    println!("not reduced: {}", max.0 * max.1);
}
