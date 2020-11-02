pub mod renderer {
    use sdl2::rect::{Point, Rect};
    use sdl2::render::Canvas;
    use sdl2::video::Window;
    pub struct Renderer {}

    impl Renderer {
        pub fn rect(
            canvas: &mut Canvas<Window>,
            x: i32,
            y: i32,
            width: u32,
            height: u32,
        ) -> Result<(), String> {
            canvas.fill_rect(Rect::new(x, y, width, height))
        }

        pub fn draw_circle(
            canvas: &mut Canvas<Window>,
            center: Point,
            radius: i32,
        ) -> Result<(), String> {
            let mut x = radius;
            let mut y = 0;
            let mut re = x * x + y * y - radius * radius;
            while x >= y {
                canvas.draw_point(Point::new(center.x() + x, center.y() + y))?;
                canvas.draw_point(Point::new(center.x() + y, center.y() + x))?;
                canvas.draw_point(Point::new(center.x() - x, center.y() + y))?;
                canvas.draw_point(Point::new(center.x() - y, center.y() + x))?;
                canvas.draw_point(Point::new(center.x() - x, center.y() - y))?;
                canvas.draw_point(Point::new(center.x() - y, center.y() - x))?;
                canvas.draw_point(Point::new(center.x() + x, center.y() - y))?;
                canvas.draw_point(Point::new(center.x() + y, center.y() - x))?;
                if 2 * (re + 2 * y + 1) + 1 - 2 * x > 0 {
                    re += 1 - 2 * x;
                    x -= 1;
                }
                re += 2 * y + 1;
                y += 1;
            }
            Ok(())
        }
    }
}
