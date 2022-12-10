use std::cmp;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let grid: Vec<Vec<u32>> = file
        .lines()
        .map(|x| x.chars().map(|x| x.to_string().parse().unwrap()).collect())
        .collect();

    let width = grid[0].len();
    let height = grid.len();
    let mut visible = vec![vec![false; width]; height];
    let mut scenic = vec![vec![1 as u32; width]; height];

    let mut count: u32 = 0;
    for h in 0..height {
        // left -> right
        let mut view = vec![1 as u32; 10];
        let mut min_tree = grid[h][0];
        count += (!visible[h][0]) as u32;
        visible[h][0] = true;
        scenic[h][0] = 0;
        for w in 1..width {
            let tree = grid[h][w];
            if min_tree < tree {
                min_tree = tree;
                count += (!visible[h][w]) as u32;
                visible[h][w] = true;
            }
            scenic[h][w] *= view[tree as usize];
            for (i, v) in view.iter_mut().enumerate() {
                if (i as u32) <= tree {
                    *v = 1;
                } else {
                    *v += 1;
                }
            }
        }

        // right -> left
        let mut view = vec![1 as u32; 10];
        let mut min_tree = grid[h][width - 1];
        count += (!visible[h][width - 1]) as u32;
        visible[h][width - 1] = true;
        scenic[h][width - 1] = 0;
        for w in (0..width - 1).rev() {
            let tree = grid[h][w];
            if min_tree < tree {
                min_tree = tree;
                count += (!visible[h][w]) as u32;
                visible[h][w] = true;
            }
            scenic[h][w] *= view[tree as usize];
            for (i, v) in view.iter_mut().enumerate() {
                if (i as u32) <= tree {
                    *v = 1;
                } else {
                    *v += 1;
                }
            }
        }
    }

    let mut most_scenic: u32 = 0;
    for w in 0..width {
        // top -> bottom
        let mut view = vec![1 as u32; 10];
        let mut min_tree = grid[0][w];
        count += (!visible[0][w]) as u32;
        visible[0][w] = true;
        scenic[0][w] = 0;
        for h in 1..height {
            let tree = grid[h][w];
            if min_tree < tree {
                min_tree = tree;
                count += (!visible[h][w]) as u32;
                visible[h][w] = true;
            }
            scenic[h][w] *= view[tree as usize];
            for (i, v) in view.iter_mut().enumerate() {
                if (i as u32) <= tree {
                    *v = 1;
                } else {
                    *v += 1;
                }
            }
        }

        // bottom -> top
        let mut view = vec![1 as u32; 10];
        let mut min_tree = grid[height - 1][w];
        count += (!visible[height - 1][w]) as u32;
        visible[height - 1][w] = true;
        scenic[height - 1][w] = 0;
        for h in (0..height - 1).rev() {
            let tree = grid[h][w];
            if min_tree < tree {
                min_tree = tree;
                count += (!visible[h][w]) as u32;
                visible[h][w] = true;
            }
            scenic[h][w] *= view[tree as usize];
            most_scenic = cmp::max(most_scenic, scenic[h][w]);
            for (i, v) in view.iter_mut().enumerate() {
                if (i as u32) <= tree {
                    *v = 1;
                } else {
                    *v += 1;
                }
            }
        }
    }
    println!("visible trees: {}", count);
    println!("scenic score:  {}", most_scenic);
}
