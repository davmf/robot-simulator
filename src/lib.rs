// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone)]
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
        let mut robot = Robot {
            x: self.x,
            y: self.y,
            d: self.d,
        };

        for c in instructions.chars() {
            let new_robot = robot.apply_instruction(c);
            robot = new_robot;
        };

        robot
    }
    
    fn apply_instruction(self, instruction: char) -> Self {
        match instruction {
            'L' => self.turn_left(),
            'R' => self.turn_right(),
            'A' => self.advance(),
             _ => self,
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_apply_instruction() {
        let robot = Robot::new(-1, -1, Direction::South).apply_instruction('L');
        assert_eq!(robot.position(), (-1, -1));
        assert_eq!(robot.direction(), &Direction::East);
    }
}