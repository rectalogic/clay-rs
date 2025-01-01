use clay::Builder;
use macroquad::prelude::*;

#[macroquad::main("Simple")]
async fn main() {
    // XXX set this in the renderer library
    extern "C" fn measure(text: &clay::String, config: &clay::Text) -> clay::Dimensions {
        clay::Dimensions {
            width: (text.len() * 10) as f32,
            height: config.font_size as f32,
        }
    }

    let size: u32 = clay::Arena::min_memory_size();
    let memory = vec![0u8; size as usize];
    let arena = clay::Arena::new(&memory);
    arena.set_measure_text_callback(measure);
    let dimensions = clay::Dimensions::new(300.0, 300.0);
    arena.initialize(dimensions, clay::default());
    let mut arena = arena;
    let renderer = clay_renderer_macroquad::MacroquadRenderer;
    loop {
        arena.render(&renderer, |builder| {
            builder.build(
                &[
                    clay::Layout {
                        layout_direction: clay::LayoutDirection::TopToBottom,
                        padding: clay::Padding { x: 16, y: 16 },
                        child_gap: 16,
                        ..clay::default()
                    }
                    .into(),
                    clay::Rectangle {
                        color: clay::Color::rgb(255., 0., 0.),
                        ..clay::default()
                    }
                    .into(),
                ],
                |builder| {
                    child_rect(builder, clay::Color::rgb(0., 255., 0.));
                    child_rect(builder, clay::Color::rgb(0., 0., 255.));
                    child_rect(builder, clay::Color::rgb(255., 0., 255.));
                },
            );
        });

        next_frame().await
    }
}

fn child_rect(builder: &Builder, color: clay::Color) {
    builder.build(
        &[
            clay::Layout {
                padding: clay::Padding { x: 16, y: 16 },
                sizing: clay::Sizing {
                    height: clay::SizingAxis::fixed(80.),
                    ..clay::default()
                },
                ..clay::default()
            }
            .into(),
            clay::Rectangle {
                color,
                ..clay::default()
            }
            .into(),
        ],
        |builder| {
            builder.build(
                &[clay::Text {
                    font_size: 18,
                    ..clay::default()
                }
                .with("Foobar".into())],
                |_| {},
            );
        },
    );
}
