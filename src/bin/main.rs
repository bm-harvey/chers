use anyhow::Result;
use chers::chess::{BoardState, Game};
use chers::core::bits;
use chers::view;
use chers::view::{GameViewer, RenderType};
use rand::prelude::*;

fn main() -> Result<()> {
    let mut game = Game::new();
    let game_viewer = GameViewer::new().with_render_type(RenderType::ASCIIGreek);
    game_viewer.print_square_idxs();

    for _ in 0..11 {
        game.do_random_pseudo_legal_move()?;
    }
    game_viewer.print(&game);

    //

    game_viewer.print_legal_moves(&game);


    Ok(())
}

fn run_time_test() -> Result<()> {
    let mut total_eval = 0_f32;
    let num_iters = 100_000;


    let start = std::time::Instant::now();
    for _ in 0..num_iters {
        let mut game = Game::new();
        for _ in 0..100 {
            game.do_random_pseudo_legal_move()?;
        }
        total_eval += game.board_state().material_value();
    }

    println!("Avg. Eval : {}", total_eval / num_iters as f32);
    println!("Time : {}", start.elapsed().as_secs_f32());
    Ok(())
}
