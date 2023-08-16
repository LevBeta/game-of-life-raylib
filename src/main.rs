pub mod game;

use game::GameOfLife;
use raylib::prelude::*;

fn main() {

    let cell_size: i32  = 5;
    let cols: i32 = 250;
    let rows: i32 = 175;

    let width: i32 = (cols - 1) * cell_size;
    let height: i32 = (rows -1) * cell_size;

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Game of life")
        .vsync()
        .build();

    rl.set_target_fps(60);

    let mut gof = GameOfLife::new(cols as usize, rows as usize);

    gof.random_cells();
    while !rl.window_should_close() {

        if rl.is_key_down(KeyboardKey::KEY_R) { gof.random_cells() }

        let mut d = rl.begin_drawing(&thread);    
        for (row_idx, row) in gof.board.iter().enumerate() {
            for (col_idx, &cell) in row.iter().enumerate() {
                let x = col_idx * cell_size as usize;
                let y = row_idx * cell_size as usize;

                if cell {
                    d.draw_rectangle(x as i32, y as i32, cell_size, cell_size, Color::WHITE);
                } else {
                    d.draw_rectangle(x as i32, y as i32, cell_size, cell_size, Color::BLACK);
                }
            }
        }
        gof.next_seed();
    }
}
