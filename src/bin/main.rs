use anyhow::Result;
use chers::game_state::Game;
use chers::game_state::GameViewer;
use chers::game_state::RenderType;

fn main() -> Result<()> {
    let mut game = Game::new();
    let game_viewer = GameViewer::new().with_render_type(RenderType::Normal);

    game.natural_apply_move("e2e4")?;
    game.natural_apply_move("d7d5")?;
    game.natural_apply_move("e4d5")?;
    game.natural_apply_move("d8d5")?;
    game.natural_apply_move("a2a4")?;
    game.natural_apply_move("d5d3")?;
    game.natural_apply_move("a1a3")?;
    game.natural_apply_move("d3d6")?;
    game.natural_apply_move("a3d4")?;
    game.natural_apply_move("d6d8")?;
    game_viewer.print(&game);

    game_viewer.print_square_idxs();
    game_viewer.print_legal_moves(&game, 27);

    Ok(())
}
