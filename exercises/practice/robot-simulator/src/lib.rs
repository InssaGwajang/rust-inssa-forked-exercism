use self::Direction::*;

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    pub fn turn_right(self) -> Self {
        Self {
            direction: self.direction.rotate_clockwise(),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            direction: self.direction.rotate_counterclockwise(),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Self {
            x: match self.direction {
                East => self.x + 1,
                West => self.x - 1,
                _ => self.x,
            },
            y: match self.direction {
                North => self.y + 1,
                South => self.y - 1,
                _ => self.y,
            },
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => robot,
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
