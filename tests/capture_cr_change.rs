use dychess::prelude::*;

#[test]
fn capture_cr_change() {
    let mut board = Board::from_epd(false, "r3k2r/8/1N4N1/8/8/1n4n1/8/R3K2R b KQkq - 0 1").unwrap();

    board.make_move(Move::new(Square::G3, Square::H1, None));
    assert!(!board.white_castle_rights().king_side());

    board.make_move(Move::new(Square::G6, Square::H8, None));
    assert!(!board.black_castle_rights().king_side());

    board.make_move(Move::new(Square::B3, Square::A1, None));
    assert!(!board.white_castle_rights().queen_side());

    board.make_move(Move::new(Square::B6, Square::A8, None));
    assert!(!board.black_castle_rights().queen_side());
}
