use dychess::prelude::*;

#[test]
fn polyglot_hash() {
    let board = Board::from_epd(false, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(board.get_hash(), 0x463b96181691fc9c);

    let board = Board::from_epd(false, "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    assert_eq!(board.get_hash(), 0x823c9b50fd114196);

    let board = Board::from_epd(false, "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2").unwrap();
    assert_eq!(board.get_hash(), 0x0756b94461c50fb0);

    let board = Board::from_epd(false, "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2").unwrap();
    assert_eq!(board.get_hash(), 0x662fafb965db29d4);

    let board = Board::from_epd(false, "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPP1PPP/RNBQKBNR w KQkq f6 0 3").unwrap();
    assert_eq!(board.get_hash(), 0x22a48b5a8e47ff78);

    let board = Board::from_epd(false, "rnbqkbnr/ppp1p1pp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR b kq - 0 3").unwrap();
    assert_eq!(board.get_hash(), 0x652a607ca3f242c1);

    let board = Board::from_epd(false, "rnbq1bnr/ppp1pkpp/8/3pPp2/8/8/PPPPKPPP/RNBQ1BNR w - - 0 4").unwrap();
    assert_eq!(board.get_hash(), 0x00fdd303c946bdd9);

    let board = Board::from_epd(false, "rnbqkbnr/p1pppppp/8/8/PpP4P/8/1P1PPPP1/RNBQKBNR b KQkq c3 0 3").unwrap();
    assert_eq!(board.get_hash(), 0x3c8123ea7b067637);

    let board = Board::from_epd(false, "rnbqkbnr/p1pppppp/8/8/P6P/R1p5/1P1PPPP1/1NBQKBNR b Kkq - 0 4").unwrap();
    assert_eq!(board.get_hash(), 0x5c3f9b829b279560);
}

#[test]
fn zobrist_transposition() {
    let mut board = Board::default();
    board.make_move(Move::new(Square::E2, Square::E4, None));
    board.make_move(Move::new(Square::C7, Square::C5, None));
    board.make_move(Move::new(Square::B1, Square::C3, None));
    board.make_move(Move::new(Square::G8, Square::F6, None));
    let hash1 = board.get_hash();

    let mut board = Board::default();
    board.make_move(Move::new(Square::B1, Square::C3, None));
    board.make_move(Move::new(Square::G8, Square::F6, None));
    board.make_move(Move::new(Square::E2, Square::E4, None));
    board.make_move(Move::new(Square::C7, Square::C5, None));
    let hash2 = board.get_hash();

    assert_eq!(hash1, hash2);
}
