use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use views::board_view;

use crate::model::game::Game;

mod views;
mod model;

fn main() -> Result<(), String> {
    let screen_width = 800;
    let screen_height = 600;

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Rust!", screen_width, screen_height)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .unwrap();
    let board_view = board_view::Renderer {
        screen_area: Rect::new(0, 0, screen_width, screen_height),
        empty_color: Color::RGB(64, 192, 255),
    };
    canvas.set_draw_color(board_view.empty_color);

    let mut game = Game {
        board: model::game::make_empty_board(),
    };

    game.jumble_board();

    let mut running = true;
    let mut event_queue = sdl_context.event_pump().unwrap();

    let mut is_red = true;

    while running {
        for event in event_queue.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    running = false;
                }
                Event::MouseButtonDown { x, y, .. } => {
                    let row: usize = (5 * x / board_view.screen_area.w).try_into().unwrap();
                    let col: u32 = (5 * y / board_view.screen_area.h).try_into().unwrap();
                    println!("Mouse button down at: ({}, {})", row, col);
                    match is_red {
                        true => game.place_piece(row, col as usize, model::game::BoardPiece::Red),
                        false => game.place_piece(row, col as usize, model::game::BoardPiece::Black)
                    };
                    is_red = !is_red;
                }
                _ => {}
            }
        }

        board_view.render(&mut canvas, &game.board);
        canvas.present();
    }

    game.print_board();


    Ok(())
}