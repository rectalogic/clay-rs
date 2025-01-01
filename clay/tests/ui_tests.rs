use std::iter::zip;

struct TestRenderer {
    pub expected: Vec<&'static str>,
}

impl clay::Renderer for TestRenderer {
    fn get_layout_dimensions(&self) -> clay::Dimensions {
        clay::Dimensions {
            width: 300.0,
            height: 300.0,
        }
    }

    fn render(&self, render_commands: &mut clay::RenderCommandIter<'_>) {
        for (expected_command, command) in zip(self.expected.iter(), render_commands) {
            assert_eq!(*expected_command, format!("{:?}", command));
        }
    }
}

#[test]
fn test_simple_ui() {
    extern "C" fn measure(text: &clay::String, config: &clay::Text) -> clay::Dimensions {
        clay::Dimensions {
            width: (text.len() * 10) as f32,
            height: config.font_size as f32,
        }
    }

    let renderer = TestRenderer {
        // Internally generated id's can change, not a good test...
        expected: vec![
            r#"RenderCommand { bounding_box: BoundingBox { x: 16.0, y: 16.0, width: 32.0, height: 32.0 }, config: Image { image_data: 0x0, source_dimensions: Dimensions { width: 128.0, height: 128.0 } }, text: String { chars: "" }, id: 1782946882, command_type: Image }"#,
            r#"RenderCommand { bounding_box: BoundingBox { x: 64.0, y: 23.0, width: 140.0, height: 18.0 }, config: Text { text_color: Color { r: 240.0, g: 189.0, b: 100.0, a: 255.0 }, font_id: 2, font_size: 18, letter_spacing: 0, line_height: 0, wrap_mode: ClayTextWrapWords }, text: String { chars: "Some text here" }, id: 78651382, command_type: Text }"#,
            r#"RenderCommand { bounding_box: BoundingBox { x: 0.0, y: 0.0, width: 300.0, height: 64.0 }, config: Border { left: BorderStyle { width: 2, color: Color { r: 240.0, g: 189.0, b: 100.0, a: 255.0 } }, right: BorderStyle { width: 2, color: Color { r: 240.0, g: 189.0, b: 100.0, a: 255.0 } }, top: BorderStyle { width: 2, color: Color { r: 240.0, g: 189.0, b: 100.0, a: 255.0 } }, bottom: BorderStyle { width: 2, color: Color { r: 240.0, g: 189.0, b: 100.0, a: 255.0 } }, between_children: BorderStyle { width: 0, color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 } }, corner_radius: CornerRadius { top_left: 10.0, top_right: 10.0, bottom_left: 10.0, bottom_right: 10.0 } }, text: String { chars: "" }, id: 2979443697, command_type: Border }"#,
        ],
    };

    let size: u32 = clay::Arena::min_memory_size();
    let memory = vec![0u8; size as usize];
    let arena = clay::Arena::new(&memory);
    arena.set_measure_text_callback(measure);
    let dimensions = clay::Dimensions::new(300.0, 300.0);
    arena.initialize(dimensions, clay::default());
    let mut arena = arena;
    arena.render(&renderer, |builder| {
        let color = clay::Color {
            r: 240.,
            g: 189.,
            b: 100.,
            a: 255.,
        };
        const FONT_ID_BODY_24: u16 = 2;

        // CLAY(CLAY_IDI("HeroBlob", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_GROW({ .max = 480 }) }, .padding = {16, 16}, .childGap = 16, .childAlignment = {.y = CLAY_ALIGN_Y_CENTER} }), CLAY_BORDER_OUTSIDE_RADIUS(2, color, 10)) {
        //     CLAY(CLAY_IDI("CheckImage", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_FIXED(32) } }), CLAY_IMAGE({ .sourceDimensions = { 128, 128 }, .sourceURL = imageURL })) {}
        //     CLAY_TEXT(text, CLAY_TEXT_CONFIG({ .fontSize = fontSize, .fontId = FONT_ID_BODY_24, .textColor = color }));
        // }
        builder.build(
            &[
                clay::IdI(clay::String::from("HeroBlob"), 1).into(),
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
                }
                .into(),
                clay::Border::outside_radius(2, color, 10.0).into(),
            ],
            |builder| {
                builder.build(
                    &[
                        clay::Id(clay::String::from("CheckImage")).into(),
                        clay::Layout {
                            sizing: clay::Sizing {
                                width: clay::SizingAxis::fixed(32.),
                                ..clay::default()
                            },
                            ..clay::default()
                        }
                        .into(),
                        clay::Image {
                            source_dimensions: clay::Dimensions {
                                width: 128.,
                                height: 128.,
                            },
                            ..clay::default()
                        }
                        .into(), // XXX need extended sourceUrl
                    ],
                    |_| {},
                );
                builder.build(
                    &[clay::Text {
                        font_size: 18,
                        font_id: FONT_ID_BODY_24,
                        text_color: color,
                        ..clay::default()
                    }
                    .with(clay::String::from("Some text here"))],
                    |_| {},
                );
            },
        );
    });
}
