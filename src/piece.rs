#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    Pawn, Knight, Bishop, Rook, Queen, King
}

impl Piece {
    /// All of the pieces with ascending indices.
    pub const ALL: [Self; 6] = [
        Self::Pawn, Self::Knight, Self::Bishop, Self::Rook, Self::Queen, Self::King
    ];

    /// All of the pieces that a pawn can promote to with ascending indices.
    pub const PROMOTE_TO: [Self; 4] = [
        Self::Knight, Self::Bishop, Self::Rook, Self::Queen
    ];
}
