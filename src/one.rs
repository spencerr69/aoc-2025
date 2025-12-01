use std::fs;
use std::path;


pub fn run() {
    println!("Day 1");

    let input = get_input();

    let mut dial = Dial::new();

    input.lines().map(Instruction::from).for_each(dial.update_current_pos());

    println!("Password: {}", dial.password)

}

pub fn get_input() -> String {
    let path = path::Path::new("src/inputs/1.txt");

   let input = fs::read_to_string(path).expect("Couldn't find file 1.txt");
    input
}

enum Direction {
    LEFT,
    RIGHT
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        if value == "L" {
            Direction::LEFT
        } else {
            Direction::RIGHT
        }
    }
}

struct Instruction {
    direction: Direction,
    amount: i32
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let (dir, amount) = value.split_at(1);

        let direction = Direction::from(dir);
        let amount = amount.parse::<i32>().unwrap();

        Instruction {
            direction,
            amount
        }
    }
}

struct Dial {
    current_pos: i32,
    pub password: i32
}

impl Dial {
    pub fn new() -> Self {
        Self {
            current_pos: 50,
            password: 0
        }
    }

    pub fn update_current_pos(&mut self) -> impl FnMut(Instruction) {
        |instruction| {
            self.rotate(instruction);
        }
    }

    pub fn rotate(&mut self, instruction: Instruction) {
        match instruction.direction {
            Direction::LEFT => {
                self.current_pos -= instruction.amount;
                while self.current_pos < 0 {
                    self.password += 1;
                    self.current_pos += 100
                }
            }
            Direction::RIGHT => {
                self.current_pos += instruction.amount;
                while self.current_pos > 99 {
                    self.password += 1;
                    self.current_pos -= 100
                }
            }
        }
        // if self.current_pos == 0 {
        //     self.password += 1
        // }
    }
}