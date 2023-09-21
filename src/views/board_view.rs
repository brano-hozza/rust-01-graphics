use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

pub struct Renderer {
    pub screen_area: Rect,
    pub empty_color: Color
}

impl Renderer {
    pub fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.empty_color);
        canvas.fill_rect(self.screen_area).unwrap();

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

}