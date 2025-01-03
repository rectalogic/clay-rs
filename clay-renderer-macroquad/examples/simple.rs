use clay::Builder;
use macroquad::prelude::*;

#[macroquad::main("Simple")]
async fn main() {
    let font = load_ttf_font(
        "clay/clay/examples/introducing-clay-video-demo/resources/Roboto-Regular.ttf",
    )
    .await
    .unwrap();
    let font_id = clay_renderer_macroquad::add_font(font);

    let size: u32 = clay::Arena::min_memory_size();
    let memory = vec![0u8; size as usize];
    let arena = clay::Arena::new(&memory);
    let dimensions = clay::Dimensions::new(300.0, 300.0);
    arena.initialize(dimensions, clay::default());
    let mut arena = arena;
    let renderer = clay_renderer_macroquad::MacroquadRenderer::new();
    loop {
        arena.render(&renderer, |builder| {
            builder.build(
                |builder| {
                    builder.attach(
                        clay::Layout {
                            layout_direction: clay::LayoutDirection::TopToBottom,
                            padding: clay::Padding { x: 16, y: 16 },
                            child_gap: 16,
                            ..clay::default()
                        }
                        .into(),
                    );
                    builder.attach(
                        clay::Rectangle {
                            color: clay::Color::rgb(255., 0., 0.),
                            ..clay::default()
                        }
                        .into(),
                    );
                },
                |builder| {
                    child_rect(builder, clay::Color::rgb(0., 255., 0.), font_id);
                    child_rect(builder, clay::Color::rgb(0., 0., 255.), font_id);
                    child_rect(builder, clay::Color::rgb(255., 0., 255.), font_id);
                },
            )
        });

        next_frame().await
    }
}

fn child_rect(builder: &Builder, color: clay::Color, font_id: u16) {
    builder.build(
        |builder| {
            builder.attach(
                clay::Layout {
                    padding: clay::Padding { x: 16, y: 16 },
                    sizing: clay::Sizing {
                        height: clay::SizingAxis::fixed(80.),
                        ..clay::default()
                    },
                    ..clay::default()
                }
                .into(),
            );
            builder.attach(
                clay::Rectangle {
                    color,
                    ..clay::default()
                }
                .into(),
            );
        },
        |builder| {
            builder.attach(
                clay::Text {
                    font_id,
                    font_size: 18,
                    text_color: clay::Color::rgb(0., 0., 0.),
                    ..clay::default()
                }
                .with("Foobar".into()),
            );
        },
    );
}
