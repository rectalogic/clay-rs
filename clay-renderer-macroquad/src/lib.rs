use macroquad::prelude::*;

// Clay_SetMeasureTextFunction takes no userdata, so this needs to be global
static mut FONTS: Vec<macroquad::text::Font> = Vec::new();

pub fn add_font(font: macroquad::text::Font) -> u16 {
    unsafe {
        FONTS.push(font);
        (FONTS.len() - 1) as u16
    }
}

fn get_font(font_id: u16) -> &'static macroquad::text::Font {
    unsafe { &FONTS[font_id as usize] }
}

extern "C" fn measure_text(text: &clay::String, config: &clay::Text) -> clay::Dimensions {
    let font = unsafe { &FONTS[config.font_id as usize] };
    let size = macroquad::text::measure_text((*text).into(), Some(font), config.font_size, 1.0);
    clay::Dimensions {
        width: size.width,
        height: size.height,
    }
}

pub struct MacroquadRenderer(());

impl MacroquadRenderer {
    pub fn new() -> Self {
        clay::Arena::set_measure_text_callback(measure_text);
        Self(())
    }
}

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
                    draw_rectangle(
                        command.bounding_box.x,
                        command.bounding_box.y,
                        command.bounding_box.width,
                        command.bounding_box.height,
                        Color(rectangle.color).into(),
                    );
                }
                clay::RenderCommandElement::Text(text) => {
                    draw_text_ex(
                        command.text.into(),
                        command.bounding_box.x,
                        command.bounding_box.y,
                        TextParams {
                            font_size: text.font_size,
                            font: Some(get_font(text.font_id)),
                            color: Color(text.text_color).into(),
                            ..Default::default()
                        },
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
