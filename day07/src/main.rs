use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

struct Directory {
    subdirs: HashSet<String>,
    files: HashMap<String, u64>,
    size: u64,
}

impl Directory {
    fn new() -> Self {
        Directory {
            subdirs: HashSet::new(),
            files: HashMap::new(),
            size: 0,
        }
    }

    fn add_subdir(&mut self, dir: String) {
        self.subdirs.insert(dir.clone());
    }

    fn add_file(&mut self, name: String, size: u64) -> u64 {
        match self.files.entry(name.clone()) {
            Entry::Vacant(e) => {
                e.insert(size);
                self.size = self.size + size;
                size
            }
            _ => 0,
        }
    }

    fn add_size(&mut self, size: u64) {
        self.size = self.size + size;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let mut dirs = HashMap::new();
    let mut stack = vec!["/"];
    dirs.insert("/", Directory::new());

    for line in file.lines() {
        if line.chars().nth(0).unwrap() == '$' {
            match line {
                "$ ls" => {}
                "$ cd .." => {
                    stack.pop();
                }
                line => {
                    stack.push(line.splitn(3, ' ').nth(2).unwrap());
                }
            };
        } else if line.chars().nth(0).unwrap() == 'd' {
            let dir = line.splitn(2, ' ').nth(1).unwrap();
            dirs.insert(dir, Directory::new());
            dirs.get_mut(stack.last().unwrap())
                .unwrap()
                .add_subdir(dir.to_string());
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            let size: u64 = size.parse().expect("number");
            let size = dirs
                .get_mut(stack.last().unwrap())
                .unwrap()
                .add_file(name.to_string(), size);
            if size > 0 {
                for d in &stack {
                    dirs.get_mut(d).unwrap().add_size(size);
                }
            }
        }
    }

    let mut sum: u64 = 0;
    for dir in dirs.values() {
        if dir.size <= 100000 {
            sum = sum + dir.size;
        }
    }
    println!("{}", sum);
}
