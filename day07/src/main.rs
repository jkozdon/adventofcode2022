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
    let mut stack = vec!["/".to_string()];
    dirs.insert("/".to_string(), Directory::new());

    for line in file.lines().skip(1) {
        if line.chars().nth(0).unwrap() == '$' {
            match line {
                "$ ls" => {}
                "$ cd .." => {
                    stack.pop();
                }
                line => {
                    let mut newdir = stack.last().unwrap().clone();
                    newdir.push_str(line.splitn(3, ' ').nth(2).unwrap());
                    newdir.push_str("/");
                    stack.push(newdir);
                }
            };
        } else if line.chars().nth(0).unwrap() == 'd' {
            let mut newdir = stack.last().unwrap().clone();
            newdir.push_str(line.splitn(2, ' ').nth(1).unwrap());
            newdir.push_str("/");
            dirs.insert(newdir.clone(), Directory::new());
            dirs.get_mut(stack.last().unwrap())
                .unwrap()
                .add_subdir(newdir);
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
    let mut delsize = dirs.get_mut(&"/".to_string()).unwrap().size;
    let required = delsize - 40000000;
    for dir in dirs.values() {
        if dir.size <= 100000 {
            sum = sum + dir.size;
        }
        if dir.size >= required && dir.size < delsize {
            delsize = dir.size;
        }
    }
    println!("sum of dir <= 100000: {}", sum);
    println!("delete dir size:      {}", delsize);
}
