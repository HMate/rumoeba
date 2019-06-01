use rumoeba;

#[test]
fn game_not_finished_initially() {
    let player1 = rumoeba::Player::new("SeeZee");
    let player2 = rumoeba::Player::new("Moak");
    let game = rumoeba::XOGame::new(3, &player1, &player2);

    assert_ne!(game.finished(), true);
}

#[test]
fn play_game() {
    let player1 = rumoeba::Player::new("SeeZee");
    let player2 = rumoeba::Player::new("Moak");
    let game = rumoeba::XOGame::new(3, &player1, &player2);

    game.place(&player1, 1, 1);
    game.place(&player2, 1, 2);
    game.place(&player1, 0, 1);
    game.place(&player2, 2, 1);
    game.place(&player1, 0, 0);
    game.place(&player2, 2, 2);
    game.place(&player1, 0, 2);
    assert!(game.finished());
}

// TODO: error when placing mark where somebody already placed
// TODO: try game.finished before ending game