// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}


impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {x, y, d}
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot {x: self.x, y: self.y, d}
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot {x: self.x, y: self.y, d}
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y, d) = match self.d {
            Direction::North => (self.x, self.y + 1, Direction::North),
            Direction::East => (self.x + 1, self.y, Direction::East),
            Direction::South => (self.x, self.y - 1, Direction::South),
            Direction::West => (self.x - 1, self.y, Direction::West),
        };
        Robot {x, y, d}
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {

        let robot = Robot {
            x: self.x,
            y: self.y,
            d: self.d,
        };

        let x = instructions.chars();

        for c in instructions.chars() {
            let new_robot = match c {
                'L' => robot.turn_left(),
                'R' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            };
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
