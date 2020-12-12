use std::println;

mod utils;

fn main() {
    let data = utils::load_input("./data/day_12.txt").unwrap();

    let instructions = data.lines().map(|l| {
        let (o, v) = l.split_at(1);
        Instruction {
            order: o.chars().next().unwrap(),
            value: v.parse().unwrap(),
        }
    });

    let mut ship = Ship::new();
    // let mut ship_2 = Ship::new();

    for i in instructions {
        match i.order {
            'L' | 'R' => {
                ship.rotate(i.order, i.value as f32);
                // ship_2.rotate(i.order, i.value as f32)
            }
            _ => {
                // ship_2.navigate_with_waypoint(&i);
                ship.navigate(&i)
            }
        }
    }

    println!("Answer 1/2 : {}", ship.get_manhattan_distance());
    //println!("Answer 2/2 : {}", ship_2.get_manhattan_distance());
}

#[derive(Debug)]
struct Ship {
    position: Position,
    orientation: Orientation,
    waypoint: Waypoint,
}
#[derive(Debug)]
struct Instruction {
    order: char,
    value: u32,
}
#[derive(Debug)]
struct Position {
    north: isize,
    east: isize,
}
#[derive(Debug)]
struct Orientation {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Waypoint {
    north: isize,
    east: isize,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Position { north: 0, east: 0 },
            orientation: Orientation { x: 1.0, y: 0.0 },
            waypoint: Waypoint { north: 1, east: 10 },
        }
    }

    // fn navigate_with_waypoint(&mut self, instruction: &Instruction) {
    //     let v = instruction.value as isize;

    //     match instruction.order {
    //         'F' => {
    //             self.position.east += v * self.waypoint.east;
    //             self.position.north += v * self.waypoint.north;
    //         }
    //         'N' => self.waypoint.north += v,
    //         'S' => self.waypoint.north -= v,
    //         'E' => self.waypoint.east += v,
    //         'W' => self.waypoint.east -= v,
    //         _ => panic!("unsupported order"),
    //     }
    // }

    fn navigate(&mut self, instruction: &Instruction) {
        let v = instruction.value as isize;

        match instruction.order {
            'F' => {
                self.position.east += self.orientation.x as isize * v;
                self.position.north += self.orientation.y as isize * v;
            }
            'N' => self.position.north += v,
            'S' => self.position.north -= v,
            'E' => self.position.east += v,
            'W' => self.position.east -= v,
            _ => panic!("unsupported order"),
        };
    }

    fn rotate(&mut self, direction: char, angle: f32) {
        let angle = angle.to_radians();
        let cur_angle = (self.orientation.y).atan2(self.orientation.x);

        let new_angle = match direction {
            'L' => cur_angle + angle,
            'R' => cur_angle - angle,
            _ => panic!("unsupported direction"),
        };

        self.orientation.x = new_angle.cos().round();
        self.orientation.y = new_angle.sin().round();
    }

    fn get_manhattan_distance(&self) -> isize {
        self.position.north.abs() + self.position.east.abs()
    }
}
