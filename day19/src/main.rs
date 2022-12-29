use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Robot {
    count: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug)]
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

#[derive(Debug)]
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

    pub fn update(&mut self, (ore, clay, obsidian, geode): (u32, u32, u32, u32)) {
        self.ore += ore;
        self.clay += clay;
        self.obsidian += obsidian;
        self.geode += geode;
    }

    pub fn buy_robot(&mut self, robot: &mut Robot) {
        self.ore -= robot.ore;
        self.clay -= robot.clay;
        self.obsidian -= robot.obsidian;
        robot.count += 1;
    }

    pub fn can_buy(&self, robot: &Robot) -> bool {
        self.ore >= robot.ore && self.clay >= robot.clay && self.obsidian >= robot.obsidian
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = &args[1];
    let file = fs::read_to_string(file).expect("Should have been able to read the file");

    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.\n").unwrap();

    for (i, blueprint) in re.captures_iter(&file).enumerate() {
        assert!(i + 1 == blueprint[1].parse::<usize>().expect("blueprint num"));
        let mut team = RoboTeam::new(blueprint);
        assert!(team.obsidian.ore < team.obsidian.clay);
        assert!(team.geode.ore < team.geode.obsidian);
        let mut pack = Items::new();
        for time in 0..24 {
            let old = (
                team.ore.count,
                team.clay.count,
                team.obsidian.count,
                team.geode.count,
            );

            if pack.can_buy(&team.geode) {
                pack.buy_robot(&mut team.geode);
            }
            let next_geode = if team.obsidian.count == 0 {
                100000
            } else {
                (team.geode.obsidian + team.obsidian.count - 1 - pack.obsidian)
                    / team.obsidian.count
            };

            if pack.can_buy(&team.obsidian)
                && next_geode + pack.ore >= team.obsidian.ore + team.geode.ore
            {
                pack.buy_robot(&mut team.obsidian);
            }
            let next_obsidian = if team.clay.count == 0 {
                100000
            } else if pack.clay > team.obsidian.clay {
                0
            } else {
                (team.obsidian.clay + team.clay.count - 1 - pack.clay) / team.clay.count
            };
            if pack.can_buy(&team.clay) && team.clay.ore <= next_obsidian + pack.ore {
                pack.buy_robot(&mut team.clay);
            }
            pack.update(old);
        }
        println!("{:?}", pack.geode);
    }
}
