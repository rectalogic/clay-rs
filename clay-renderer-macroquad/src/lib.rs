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

// Emulate raylib draw_ring using draw_arc
fn draw_ring(
    x: f32,
    y: f32,
    inner_radius: f32,
    outer_radius: f32,
    start_angle: f32,
    end_angle: f32,
    segments: u8,
    color: clay::Color,
) {
    draw_arc(
        x,                                   // x: break Vector2 into components
        y,                                   // y
        segments,                            // sides: use segments directly
        (inner_radius + outer_radius) / 2.0, // radius: use average of inner/outer as center radius
        start_angle,                         // rotation: use start_angle as base rotation
        outer_radius - inner_radius,         // thickness: difference between outer and inner radius
        end_angle - start_angle,             // arc: angle difference
        Color(color).into(),
    );
}

impl clay::Renderer for MacroquadRenderer {
    fn prepare_frame(&self) -> clay::Dimensions {
        let mouse_position = mouse_position();
        let scroll_delta = mouse_wheel();
        clay::Item::set_pointer_state(
            clay::Vector2 {
                x: mouse_position.0,
                y: mouse_position.1,
            },
            is_mouse_button_down(MouseButton::Left),
        );
        clay::Item::update_scroll_containers(
            true,
            clay::Vector2 {
                x: scroll_delta.0,
                y: scroll_delta.1,
            },
            get_frame_time(),
        );
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
                    let font = Some(get_font(text.font_id));
                    let size = macroquad::prelude::measure_text(
                        command.text.into(),
                        font,
                        text.font_size,
                        1.0,
                    );
                    draw_text_ex(
                        command.text.into(),
                        command.bounding_box.x,
                        command.bounding_box.y + size.offset_y, // draw_text_ex use baseline for y
                        TextParams {
                            font_size: text.font_size,
                            font,
                            color: Color(text.text_color).into(),
                            ..Default::default()
                        },
                    );
                }
                clay::RenderCommandElement::Border(border) => {
                    // Left border
                    if border.left.width > 0 {
                        draw_rectangle(
                            command.bounding_box.x,
                            command.bounding_box.y + border.corner_radius.top_left,
                            border.left.width as f32,
                            command.bounding_box.height
                                - border.corner_radius.top_left
                                - border.corner_radius.bottom_left,
                            Color(border.left.color).into(),
                        );
                    }
                    // Right border
                    if border.right.width > 0 {
                        draw_rectangle(
                            command.bounding_box.x + command.bounding_box.width
                                - border.right.width as f32,
                            command.bounding_box.y + border.corner_radius.top_right,
                            border.right.width as f32,
                            command.bounding_box.height
                                - border.corner_radius.top_right
                                - border.corner_radius.bottom_right,
                            Color(border.right.color).into(),
                        );
                    }
                    // Top border
                    if border.top.width > 0 {
                        draw_rectangle(
                            command.bounding_box.x + border.corner_radius.top_left,
                            command.bounding_box.y,
                            command.bounding_box.width
                                - border.corner_radius.top_left
                                - border.corner_radius.top_right,
                            border.top.width as f32,
                            Color(border.top.color).into(),
                        );
                    }
                    // Bottom border
                    if border.bottom.width > 0 {
                        draw_rectangle(
                            command.bounding_box.x + border.corner_radius.bottom_left,
                            command.bounding_box.y + command.bounding_box.height
                                - border.bottom.width as f32,
                            command.bounding_box.width
                                - border.corner_radius.bottom_left
                                - border.corner_radius.bottom_right,
                            border.bottom.width as f32,
                            Color(border.bottom.color).into(),
                        );
                    }
                    if border.corner_radius.top_left > 0. {
                        draw_ring(
                            command.bounding_box.x + border.corner_radius.top_left,
                            command.bounding_box.y + border.corner_radius.top_left,
                            border.corner_radius.top_left - border.top.width as f32,
                            border.corner_radius.top_left,
                            180.,
                            270.,
                            10,
                            border.top.color,
                        );
                    }
                    if border.corner_radius.top_right > 0. {
                        draw_ring(
                            command.bounding_box.x + command.bounding_box.width
                                - border.corner_radius.top_right,
                            command.bounding_box.y + border.corner_radius.top_right,
                            border.corner_radius.top_right - border.top.width as f32,
                            border.corner_radius.top_right,
                            270.,
                            360.,
                            10,
                            border.top.color,
                        );
                    }
                    if border.corner_radius.bottom_left > 0. {
                        draw_ring(
                            command.bounding_box.x + border.corner_radius.bottom_left,
                            command.bounding_box.y + command.bounding_box.height
                                - border.corner_radius.bottom_left,
                            border.corner_radius.bottom_left - border.top.width as f32,
                            border.corner_radius.bottom_left,
                            90.,
                            180.,
                            10,
                            border.bottom.color,
                        );
                    }
                    if border.corner_radius.bottom_right > 0. {
                        draw_ring(
                            command.bounding_box.x + command.bounding_box.width
                                - border.corner_radius.bottom_right,
                            command.bounding_box.y + command.bounding_box.height
                                - border.corner_radius.bottom_right,
                            border.corner_radius.bottom_right - border.bottom.width as f32,
                            border.corner_radius.bottom_right,
                            0.1,
                            90.,
                            10,
                            border.bottom.color,
                        );
                    }
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
