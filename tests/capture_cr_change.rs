use dychess::prelude::*;

#[test]
fn capture_cr_change() {
    let mut board = Board::from_epd(false, "8/1k6/8/8/8/8/6p1/R3K2R b KQ - 0 1").unwrap();
    board.make_move(Move::new(Square::G2, Square::H1, Some(Piece::Queen)));
    assert!(!board.white_castle_rights().king_side());
}
