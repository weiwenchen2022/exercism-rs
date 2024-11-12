#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }

        Some(Self(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.0 == other.position.0
            || self.position.1 == other.position.1
            || (self.position.0 - other.position.0).abs()
                == (self.position.1 - other.position.1).abs()
    }
}
