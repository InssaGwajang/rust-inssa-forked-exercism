#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(Self { x: rank, y: file }),
            _ => None,
        }
        // for number in [rank, file] {
        //     if number < 0 || number >= CHESSBOARD_SIZE {
        //         return None;
        //     }
        // }
        // Some(Self(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.x == other.position.x
            || self.position.y == other.position.y
            || (self.position.x - other.position.x).abs()
                == (self.position.y - other.position.y).abs()
    }
}
