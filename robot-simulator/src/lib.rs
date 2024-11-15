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
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        // todo!("Create a robot at (x, y) ({x}, {y}) facing {d:?}")
        Self {
            position: (x, y),
            direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            direction: match self.direction {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            },
            ..self
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            direction: match self.direction {
                Direction::East => Direction::North,
                Direction::South => Direction::East,
                Direction::West => Direction::South,
                Direction::North => Direction::West,
            },
            ..self
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let (x, y) = self.position;
        let position = match &self.direction {
            Direction::East => (x + 1, y),
            Direction::South => (x, y - 1),
            Direction::West => (x - 1, y),
            Direction::North => (x, y + 1),
        };
        Self { position, ..self }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        // todo!("Follow the given sequence of instructions: {instructions}")
        let mut result = self;
        for ins in instructions.bytes() {
            result = match ins as char {
                'R' => result.turn_right(),
                'L' => result.turn_left(),
                'A' => result.advance(),
                ins => unreachable!("{ins:?}",),
            };
        }

        result
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
