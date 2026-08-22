use chers::game_state::GameState;

fn main() {
    let mut game = GameState::new();
    game.print();
    game.apply_move(4, 1, 4, 3);
    game.apply_move(3, 6, 3, 4);
    game.apply_move(4, 3, 3, 4);
    game.print();
}
