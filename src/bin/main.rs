use anyhow::Result;
use chers::chess::{BoardState, Game};
use chers::core::bits;
use chers::view;
use chers::view::{GameViewer, RenderType};
use rand::prelude::*;

fn main() -> Result<()> {
    let mut game = Game::new();
    let game_viewer = GameViewer::new().with_render_type(RenderType::ASCII);

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
    game.natural_apply_move("a4a5")?;
    game.natural_apply_move("b7b5")?;
    game_viewer.print(&game);

    game_viewer.print_square_idxs();
    //

    game_viewer.print_legal_moves(&game);

    //bits::dbg_mask(0x03_03_03_03_03_03_03_03);
    //bits::dbg_mask(0xc0_c0_c0_c0_c0_c0_c0_c0);
    //bits::dbg_mask(0x00_00_00_00_00_00_ff_ff);
    //bits::dbg_mask(0xff_ff_00_00_00_00_00_00);

    Ok(())
}

fn run_time_test() -> Result<()> {
    let mut total_eval = 0_f32;
    let num_iters = 1_000_000;

    let mut rng = rand::rng();

    let start = std::time::Instant::now();
    for _ in 0..num_iters {
        let mut game = Game::new();
        for _ in 0..100 {
            let file_1 = rng.random_range(0_usize..8);
            let file_2 = rng.random_range(0_usize..8);
            let rank_1 = rng.random_range(0_usize..8);
            let rank_2 = rng.random_range(0_usize..8);

            game.apply_move_by_coords(file_1, rank_1, file_2, rank_2, None)?;

            total_eval += game.board_state().material_value();
        }
    }

    println!("Avg. Eval : {}", total_eval / num_iters as f32);
    println!("Time : {}", start.elapsed().as_secs_f32());
    Ok(())
}
