use std::collections::BTreeSet;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let data: Vec<(usize, usize, usize)> = file
        .lines()
        .map(|l| {
            let mut d = l.splitn(3, ',');
            (
                // add one to create an edge buffer
                d.next().unwrap().parse::<usize>().expect("value 1") + 1,
                d.next().unwrap().parse::<usize>().expect("value 2") + 1,
                d.next().unwrap().parse::<usize>().expect("value 3") + 1,
            )
        })
        .collect();

    let (mut xmax, mut ymax, mut zmax) = (0 as usize, 0 as usize, 0 as usize);
    data.iter().for_each(|(x, y, z)| {
        // add one to allow for full range (zero indexing)
        // add one to create an edge buffer
        xmax = std::cmp::max(xmax, *x + 1 as usize + 2);
        ymax = std::cmp::max(ymax, *y + 1 as usize + 2);
        zmax = std::cmp::max(zmax, *z + 1 as usize + 2);
    });
    assert!(xmax < 32);
    let mut pixels = vec![0 as u32; ymax * zmax];

    let mut sides: u32 = 0;
    for (x, y, z) in data.iter() {
        sides += 6;
        let p = (1 as u32) << x;
        pixels[y + ymax * z] |= p;
        if (p << 1) & pixels[y + ymax * z] != 0 {
            sides -= 2;
        }
        if (p >> 1) & pixels[y + ymax * z] != 0 {
            sides -= 2;
        }
        if *y > 0 && pixels[(y - 1) + ymax * z] & p != 0 {
            sides -= 2;
        }
        if y + 1 < ymax && pixels[(y + 1) + ymax * z] & p != 0 {
            sides -= 2;
        }
        if *z > 0 && pixels[y + ymax * (z - 1)] & p != 0 {
            sides -= 2;
        }
        if z + 1 < zmax && pixels[y + ymax * (z + 1)] & p != 0 {
            sides -= 2;
        }
    }
    println!("all surfaces:      {}", sides);

    let mut outer = vec![0 as u32; ymax * zmax];
    let mut stack: BTreeSet<(usize, usize, usize)> = BTreeSet::new();
    stack.insert((0, 0, 0));
    fn check_vals(
        x: usize,
        y: usize,
        z: usize,
        pixels: &Vec<u32>,
        outer: &Vec<u32>,
        stack: &mut BTreeSet<(usize, usize, usize)>,
        ymax: usize,
    ) -> u32 {
        let p: u32 = 1 << x;
        if p & pixels[y + ymax * z] != 0 {
            1
        } else if p & outer[y + ymax * z] == 0 {
            stack.insert((x, y, z));
            0
        } else {
            0
        }
    }
    let mut sides: u32 = 0;
    while let Some((x, y, z)) = stack.pop_first() {
        let p = (1 as u32) << x;
        if outer[y + ymax * z] & p != 0 {
            continue;
        }
        outer[y + ymax * z] |= p;
        if x > 0 {
            sides += check_vals(x - 1, y, z, &pixels, &outer, &mut stack, ymax);
        }
        if x + 1 < xmax {
            sides += check_vals(x + 1, y, z, &pixels, &outer, &mut stack, ymax);
        }
        if y > 0 {
            sides += check_vals(x, y - 1, z, &pixels, &outer, &mut stack, ymax);
        }
        if y + 1 < ymax {
            sides += check_vals(x, y + 1, z, &pixels, &outer, &mut stack, ymax);
        }
        if z > 0 {
            sides += check_vals(x, y, z - 1, &pixels, &outer, &mut stack, ymax);
        }
        if z + 1 < zmax {
            sides += check_vals(x, y, z + 1, &pixels, &outer, &mut stack, ymax);
        }
    }
    println!("external surfaces: {}", sides);
}
