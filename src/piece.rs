#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl Piece {
    /// All of the pieces with ascending indices.
    pub const ALL: [Self; 6] = [
        Self::Pawn, Self::Knight, Self::Bishop, Self::Rook, Self::Queen, Self::King
    ];
}
