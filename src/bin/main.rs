use anyhow::Result;
use chers::game_state::Game;
use chers::game_state::GameViewer;
use chers::game_state::RenderType;

fn main() -> Result<()> {
    let mut game = Game::new();
    let game_viewer = GameViewer::new().with_render_type(RenderType::ASCII);
    game_viewer.print(&game);
    game.natural_apply_move("e2e4")?;
    game.natural_apply_move("d7d5")?;
    game_viewer.print(&game);

    Ok(())
}
