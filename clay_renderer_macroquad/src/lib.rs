use clay;
use macroquad::prelude::*;

pub struct MacroquadRenderer;

impl clay::Renderer for MacroquadRenderer {
    fn get_layout_dimensions(&self) -> clay::Dimensions {
        clay::Dimensions {
            width: screen_width(),
            height: screen_height(),
        }
    }

    fn render(&self, render_commands: &mut clay::RenderCommandIter<'_>) {
        for command in render_commands {
            match command.element() {
                clay::RenderCommandElement::Rectangle(rectangle) => {
                    // XXX handle rectangle.corner_radius https://github.com/not-fl3/macroquad/pull/448/files#diff-7287b746975aadc977a38a86ee3163de1d9030d2902e510ba9d955e5f9b6f3c1
                    macroquad::shapes::draw_rectangle(
                        command.bounding_box.x,
                        command.bounding_box.y,
                        command.bounding_box.width,
                        command.bounding_box.height,
                        Color(rectangle.color).into(),
                    );
                }
                _ => {}
            }
        }
    }
}

struct Color(clay::Color);

impl From<Color> for macroquad::color::Color {
    fn from(color: Color) -> Self {
        macroquad::color::Color {
            r: color.0.r,
            g: color.0.g,
            b: color.0.b,
            a: color.0.a,
        }
    }
}
