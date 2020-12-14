use std::fs;

fn main() {
    let instructions: Vec<Instruction> = fs::read_to_string("input")
        .expect("unable to read input file")
        .split_whitespace()
        .map(|s| str_to_inst(s))
        .collect();
    let result = part_1(&instructions);
    println!("{}", result);
}

struct Instruction {
    op: Operation,
    arg: isize,
}

enum Operation {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Ship {
    north_pos: isize,
    east_pos: isize,
    waypoint_north: isize,
    waypoint_east: isize,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            north_pos: 0,
            east_pos: 0,
            waypoint_north: 1,
            waypoint_east: 10,
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.north_pos.abs() + self.east_pos.abs()
    }

    fn execute(&mut self, instruction: &Instruction) -> () {
        match instruction.op {
            Operation::North => {
                self.move_waypoint(instruction.arg, 0);
            }
            Operation::South => {
                self.move_waypoint(-instruction.arg, 0);
            }
            Operation::East => {
                self.move_waypoint(0, instruction.arg);
            }
            Operation::West => {
                self.move_waypoint(0, -instruction.arg);
            }
            Operation::Left => {
                self.rotate_waypoint(-instruction.arg);
            }
            Operation::Right => {
                self.rotate_waypoint(instruction.arg);
            }
            Operation::Forward => {
                self.move_to_waypoint(instruction.arg);
            }
        }
    }

    fn move_waypoint(&mut self, north: isize, east: isize) -> () {
        self.waypoint_north += north;
        self.waypoint_east += east;
    }

    fn rotate_waypoint(&mut self, degrees: isize) -> () {
        match degrees % 360 {
            0 => (),
            90 => self.rotate_waypoint_90(),
            -270 => self.rotate_waypoint_90(),
            180 => self.rotate_waypoint_180(),
            -180 => self.rotate_waypoint_180(),
            270 => self.rotate_waypoint_270(),
            -90 => self.rotate_waypoint_270(),
            _ => (),
        }
    }

    fn rotate_waypoint_90(&mut self) -> () {
        let prev_n = self.waypoint_north;
        let prev_e = self.waypoint_east;
        self.waypoint_north = -prev_e;
        self.waypoint_east = prev_n;
    }

    fn rotate_waypoint_180(&mut self) -> () {
        self.rotate_waypoint_90();
        self.rotate_waypoint_90();
    }

    fn rotate_waypoint_270(&mut self) -> () {
        self.rotate_waypoint_90();
        self.rotate_waypoint_90();
        self.rotate_waypoint_90();
    }

    fn move_to_waypoint(&mut self, times: isize) -> () {
        self.north_pos += self.waypoint_north * times;
        self.east_pos += self.waypoint_east * times;
    }
}

fn str_to_inst(input: &str) -> Instruction {
    Instruction {
        op: match input.chars().nth(0).unwrap() {
            'N' => Operation::North,
            'S' => Operation::South,
            'E' => Operation::East,
            'W' => Operation::West,
            'L' => Operation::Left,
            'R' => Operation::Right,
            _ => Operation::Forward,
        },
        arg: input[1..].parse().unwrap(),
    }
}

fn part_1(input: &[Instruction]) -> isize {
    let mut ship = Ship::new();
    for instruction in input.iter() {
        ship.execute(instruction);
    }
    ship.manhattan_distance()
}
