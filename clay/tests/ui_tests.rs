#[test]
fn test_simple_ui() {
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
    let render_commands = arena.render(|| {
        let color = clay::Color {
            r: 240.,
            g: 189.,
            b: 100.,
            a: 255.,
        };
        const FONT_ID_BODY_24: u16 = 2;

        clay::clay!((
            clay::IdI(clay::String::from("HeroBlob"), 1),
            clay::Layout {
                sizing: clay::Sizing {
                    width: clay::SizingAxis::grow(0.0, 480.0),
                    ..clay::default()
                },
                padding: clay::Padding { x: 16, y: 16 },
                child_gap: 16,
                child_alignment: clay::ChildAlignment {
                    y: clay::LayoutAlignmentY::Center,
                    ..clay::default()
                },
                ..clay::default()
            },
            clay::Border::outside_radius(2, color, 10.0)
        ) {
            clay::clay!(
                (
                    clay::Id(clay::String::from("CheckImage")),
                    clay::Layout {
                        sizing: clay::Sizing { width: clay::SizingAxis::fixed(32.), ..clay::default() },
                        ..clay::default()
                    },
                    clay::Image { source_dimensions: clay::Dimensions { width: 128., height: 128.}, ..clay::default() } // XXX need extended sourceUrl
                ) {
                    println!("children");
                }
            );
            clay::Text {
                font_size: 18,
                font_id: FONT_ID_BODY_24,
                text_color: color,
                ..clay::default()
            }.with(clay::String::from("Some text here"));
        });
        // CLAY(CLAY_IDI("HeroBlob", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_GROW({ .max = 480 }) }, .padding = {16, 16}, .childGap = 16, .childAlignment = {.y = CLAY_ALIGN_Y_CENTER} }), CLAY_BORDER_OUTSIDE_RADIUS(2, color, 10)) {
        //     CLAY(CLAY_IDI("CheckImage", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_FIXED(32) } }), CLAY_IMAGE({ .sourceDimensions = { 128, 128 }, .sourceURL = imageURL })) {}
        //     CLAY_TEXT(text, CLAY_TEXT_CONFIG({ .fontSize = fontSize, .fontId = FONT_ID_BODY_24, .textColor = color }));
        // }
    });
    for command in render_commands {
        dbg!(command);
    }
}
