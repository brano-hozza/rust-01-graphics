use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

use crate::model::game::BoardPiece;

pub struct Renderer {
    pub screen_area: Rect,
    pub empty_color: Color,
}

impl Renderer {
    pub fn render(&self, canvas: &mut WindowCanvas, board: &[[BoardPiece; 5]; 5]) {
        canvas.set_draw_color(self.empty_color);
        canvas.fill_rect(self.screen_area).unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        self.draw_lines(canvas);
        self.draw_pieces(canvas, board);
    }

    pub fn draw_lines(&self, canvas: &mut WindowCanvas) {
        let cell_width: i32 = self.screen_area.w / 5;
        let cell_height: i32 = self.screen_area.h / 5;

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        for i in 0..5 {
            let x = i * cell_width;
            let y = i * cell_height;
            canvas.draw_line(Point::new(x, 0), Point::new(x, self.screen_area.h)).unwrap();
            canvas.draw_line(Point::new(0, y), Point::new(self.screen_area.w, y)).unwrap();
        }
    }

    pub fn draw_pieces(&self, canvas: &mut WindowCanvas, board: &[[BoardPiece; 5];5]){
        let cell_width: i32 = self.screen_area.w / 5;
        let cell_height: i32 = self.screen_area.h / 5;

        for i in 0..5 {
            let row: usize = i.try_into().unwrap();
            for j in 0..5 {
                let col: usize = j.try_into().unwrap();
                let x = i * cell_width;
                let y = j * cell_height;
                match board[row][col] {
                    BoardPiece::None => {}
                    BoardPiece::Red => {
                        canvas.set_draw_color(Color::RGB(255, 0, 0));
                        canvas.fill_rect(Rect::new(x, y, cell_width as u32, cell_height as u32)).unwrap();
                    }
                    BoardPiece::Black => {
                        canvas.set_draw_color(Color::RGB(0, 0, 0));
                        canvas.fill_rect(Rect::new(x, y, cell_width as u32, cell_height as u32)).unwrap();
                    }
                }
            }
        }

    }
}