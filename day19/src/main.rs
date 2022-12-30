use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Robot {
    count: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct RoboTeam {
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
}

impl RoboTeam {
    pub fn new(blueprint: regex::Captures<'_>) -> Self {
        let ore = Robot {
            count: 1,
            ore: blueprint[2].parse::<u32>().expect("ore ore"),
            clay: 0,
            obsidian: 0,
        };
        let clay = Robot {
            count: 0,
            ore: blueprint[3].parse::<u32>().expect("ore ore"),
            clay: 0,
            obsidian: 0,
        };
        let obsidian = Robot {
            count: 0,
            ore: blueprint[4].parse::<u32>().expect("ore ore"),
            clay: blueprint[5].parse::<u32>().expect("ore ore"),
            obsidian: 0,
        };
        let geode = Robot {
            count: 0,
            ore: blueprint[6].parse::<u32>().expect("ore ore"),
            clay: 0,
            obsidian: blueprint[7].parse::<u32>().expect("ore ore"),
        };
        Self {
            ore,
            clay,
            obsidian,
            geode,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Items {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Items {
    pub fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    pub fn update(&mut self, team: &RoboTeam) {
        self.ore += team.ore.count;
        self.clay += team.clay.count;
        self.obsidian += team.obsidian.count;
        self.geode += team.geode.count;
    }

    pub fn buy_robot(&mut self, robot: &mut Robot) {
        self.ore -= robot.ore;
        self.clay -= robot.clay;
        self.obsidian -= robot.obsidian;
        robot.count += 1;
    }

    pub fn sell_robot(&mut self, robot: &mut Robot) {
        self.ore += robot.ore;
        self.clay += robot.clay;
        self.obsidian += robot.obsidian;
        robot.count -= 1;
    }

    pub fn can_buy(&self, robot: &Robot) -> bool {
        self.ore >= robot.ore && self.clay >= robot.clay && self.obsidian >= robot.obsidian
    }
}

fn step(
    time: u32,
    team: &mut RoboTeam,
    pack: &Items,
    seen: &mut HashSet<(u32, Items, RoboTeam)>,
    mut max_geodes: u32,
) -> u32 {
    if seen.contains(&(time, *pack, *team)) {
        return 0;
    }
    seen.insert((time, pack.clone(), team.clone()));
    let time = time + 1;
    let cur_geodes = pack.geode + team.geode.count;
    let rem_time = 24 - time;
    let most_geodes = rem_time * (rem_time + 1) / 2 + cur_geodes + team.geode.count * rem_time;
    if time == 24 {
        cur_geodes
    } else if most_geodes > max_geodes {
        if pack.can_buy(&team.ore) {
            let mut new_pack = pack.clone();
            new_pack.update(&team);
            new_pack.buy_robot(&mut team.ore);
            max_geodes = std::cmp::max(step(time, team, &new_pack, seen, max_geodes), max_geodes);
            new_pack.sell_robot(&mut team.ore);
        }
        if pack.can_buy(&team.clay) {
            let mut new_pack = pack.clone();
            new_pack.update(&team);
            new_pack.buy_robot(&mut team.clay);
            max_geodes = std::cmp::max(step(time, team, &new_pack, seen, max_geodes), max_geodes);
            new_pack.sell_robot(&mut team.clay);
        }
        if pack.can_buy(&team.obsidian) {
            let mut new_pack = pack.clone();
            new_pack.update(&team);
            new_pack.buy_robot(&mut team.obsidian);
            max_geodes = std::cmp::max(step(time, team, &new_pack, seen, max_geodes), max_geodes);
            new_pack.sell_robot(&mut team.obsidian);
        }
        if pack.can_buy(&team.geode) {
            let mut new_pack = pack.clone();
            new_pack.update(&team);
            new_pack.buy_robot(&mut team.geode);
            max_geodes = std::cmp::max(step(time, team, &new_pack, seen, max_geodes), max_geodes);
            new_pack.sell_robot(&mut team.geode);
        }
        let mut new_pack = pack.clone();
        new_pack.update(&team);
        std::cmp::max(step(time, team, &new_pack, seen, max_geodes), max_geodes)
    } else {
        max_geodes
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.\n").unwrap();

    let mut quality = 0;
    for (i, blueprint) in re.captures_iter(&file).enumerate() {
        assert!(i + 1 == blueprint[1].parse::<usize>().expect("blueprint num"));
        let mut team = RoboTeam::new(blueprint);
        let pack = Items::new();
        let mut seen = HashSet::<(u32, Items, RoboTeam)>::new();
        let geodes = step(0, &mut team, &pack, &mut seen, 0);
        println!("{}", geodes);
        quality += (i as u32 + 1) * geodes;
    }
    println!("{}", quality);
}
