use self::Direction::*;
#[derive(PartialEq, Debug)]
// #[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

// #[derive(Clone, Copy)]
pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}
impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x: x, y: y, d: d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            d: self.d.turn_right(),
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            d: self.d.turn_left(),
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            x: match self.d {
                East => self.x + 1,
                West => self.x - 1,
                _ => self.x,
            },
            y: match self.d {
                North => self.y + 1,
                South => self.y - 1,
                _ => self.y,
            },
            ..self
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |robot, c| match c {
            'L' => robot.turn_left(),
            'R' => robot.turn_right(),
            'A' => robot.advance(),
            _ => robot,
        })

        // let mut robot = Self { ..self };

        // instructions.chars().for_each(|c| match c {
        //     'L' => robot = robot.turn_left(),
        //     'R' => robot = robot.turn_right(),
        //     'A' => robot = robot.advance(),
        //     _ => panic!("invalid instruction"),
        // });

        // robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
