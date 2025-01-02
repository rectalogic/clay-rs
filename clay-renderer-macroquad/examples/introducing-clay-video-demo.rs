use macroquad::prelude::*;

// Based on https://github.com/nicbarker/clay/tree/main/examples/introducing-clay-video-demo

struct Document<'a> {
    title: clay::String<'a>,
    contents: clay::String<'a>,
}

#[macroquad::main("Introducing Clay Demo")]
async fn main() {
    let documents =     [
        Document{ title: "Squirrels".into(), contents: r#"The Secret Life of Squirrels: Nature's Clever Acrobats\n""Squirrels are often overlooked creatures, dismissed as mere park inhabitants or backyard nuisances. Yet, beneath their fluffy tails and twitching noses lies an intricate world of cunning, agility, and survival tactics that are nothing short of fascinating. As one of the most common mammals in North America, squirrels have adapted to a wide range of environments from bustling urban centers to tranquil forests and have developed a variety of unique behaviors that continue to intrigue scientists and nature enthusiasts alike.\n""\n""Master Tree Climbers\n""At the heart of a squirrel's skill set is its impressive ability to navigate trees with ease. Whether they're darting from branch to branch or leaping across wide gaps, squirrels possess an innate talent for acrobatics. Their powerful hind legs, which are longer than their front legs, give them remarkable jumping power. With a tail that acts as a counterbalance, squirrels can leap distances of up to ten times the length of their body, making them some of the best aerial acrobats in the animal kingdom.\n""But it's not just their agility that makes them exceptional climbers. Squirrels' sharp, curved claws allow them to grip tree bark with precision, while the soft pads on their feet provide traction on slippery surfaces. Their ability to run at high speeds and scale vertical trunks with ease is a testament to the evolutionary adaptations that have made them so successful in their arboreal habitats.\n""\n""Food Hoarders Extraordinaire\n""Squirrels are often seen frantically gathering nuts, seeds, and even fungi in preparation for winter. While this behavior may seem like instinctual hoarding, it is actually a survival strategy that has been honed over millions of years. Known as \"scatter hoarding,\" squirrels store their food in a variety of hidden locations, often burying it deep in the soil or stashing it in hollowed-out tree trunks.\n""Interestingly, squirrels have an incredible memory for the locations of their caches. Research has shown that they can remember thousands of hiding spots, often returning to them months later when food is scarce. However, they don't always recover every stash some forgotten caches eventually sprout into new trees, contributing to forest regeneration. This unintentional role as forest gardeners highlights the ecological importance of squirrels in their ecosystems.\n""\n""The Great Squirrel Debate: Urban vs. Wild\n""While squirrels are most commonly associated with rural or wooded areas, their adaptability has allowed them to thrive in urban environments as well. In cities, squirrels have become adept at finding food sources in places like parks, streets, and even garbage cans. However, their urban counterparts face unique challenges, including traffic, predators, and the lack of natural shelters. Despite these obstacles, squirrels in urban areas are often observed using human infrastructure such as buildings, bridges, and power lines as highways for their acrobatic escapades.\n""There is, however, a growing concern regarding the impact of urban life on squirrel populations. Pollution, deforestation, and the loss of natural habitats are making it more difficult for squirrels to find adequate food and shelter. As a result, conservationists are focusing on creating squirrel-friendly spaces within cities, with the goal of ensuring these resourceful creatures continue to thrive in both rural and urban landscapes.\n""\n""A Symbol of Resilience\n""In many cultures, squirrels are symbols of resourcefulness, adaptability, and preparation. Their ability to thrive in a variety of environments while navigating challenges with agility and grace serves as a reminder of the resilience inherent in nature. Whether you encounter them in a quiet forest, a city park, or your own backyard, squirrels are creatures that never fail to amaze with their endless energy and ingenuity.\n""In the end, squirrels may be small, but they are mighty in their ability to survive and thrive in a world that is constantly changing. So next time you spot one hopping across a branch or darting across your lawn, take a moment to appreciate the remarkable acrobat at work a true marvel of the natural world.\n"#.into() },
        Document{ title: "Lorem Ipsum".into(), contents: r#"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."#.into() },
        Document{ title: "Vacuum Instructions".into(), contents: r#"Chapter 3: Getting Started - Unpacking and Setup\n""\n""Congratulations on your new SuperClean Pro 5000 vacuum cleaner! In this section, we will guide you through the simple steps to get your vacuum up and running. Before you begin, please ensure that you have all the components listed in the \"Package Contents\" section on page 2.\n""\n""1. Unboxing Your Vacuum\n""Carefully remove the vacuum cleaner from the box. Avoid using sharp objects that could damage the product. Once removed, place the unit on a flat, stable surface to proceed with the setup. Inside the box, you should find:\n""\n""    The main vacuum unit\n""    A telescoping extension wand\n""    A set of specialized cleaning tools (crevice tool, upholstery brush, etc.)\n""    A reusable dust bag (if applicable)\n""    A power cord with a 3-prong plug\n""    A set of quick-start instructions\n""\n""2. Assembling Your Vacuum\n""Begin by attaching the extension wand to the main body of the vacuum cleaner. Line up the connectors and twist the wand into place until you hear a click. Next, select the desired cleaning tool and firmly attach it to the wand's end, ensuring it is securely locked in.\n""\n""For models that require a dust bag, slide the bag into the compartment at the back of the vacuum, making sure it is properly aligned with the internal mechanism. If your vacuum uses a bagless system, ensure the dust container is correctly seated and locked in place before use.\n""\n""3. Powering On\n""To start the vacuum, plug the power cord into a grounded electrical outlet. Once plugged in, locate the power switch, usually positioned on the side of the handle or body of the unit, depending on your model. Press the switch to the \"On\" position, and you should hear the motor begin to hum. If the vacuum does not power on, check that the power cord is securely plugged in, and ensure there are no blockages in the power switch.\n""\n""Note: Before first use, ensure that the vacuum filter (if your model has one) is properly installed. If unsure, refer to \"Section 5: Maintenance\" for filter installation instructions."#.into() },
        Document{ title: "Article 4".into(), contents: "Article 4".into() },
        Document{ title: "Article 5".into(), contents: "Article 5".into() },
    ];

    let size: u32 = clay::Arena::min_memory_size();
    let memory = vec![0u8; size as usize];
    let arena = clay::Arena::new(&memory);
    let dimensions = clay::Dimensions::new(1024.0, 768.0);
    arena.initialize(dimensions, clay::default());
    let font = load_ttf_font(
        "clay/clay/examples/introducing-clay-video-demo/resources/Roboto-Regular.ttf",
    )
    .await
    .unwrap();
    let font_id_body_16 = clay_renderer_macroquad::add_font(font);

    let mut arena = arena;
    let renderer = clay_renderer_macroquad::MacroquadRenderer::new();

    let layout_expand = clay::Sizing {
        width: clay::SizingAxis::grow(0., 0.),
        height: clay::SizingAxis::grow(0., 0.),
    };
    let content_background = clay::Rectangle {
        color: clay::Color::rgb(90., 90., 90.),
        corner_radius: clay::CornerRadius::new(8.),
    };

    let render_dropdown_menu_item = |builder: &clay::Builder, text: clay::String| {
        builder.build(
            &[clay::Layout {
                padding: clay::Padding { x: 16, y: 16 },
                ..clay::default()
            }
            .into()],
            |builder| {
                builder.build(
                    &[clay::Text {
                        font_id: font_id_body_16,
                        font_size: 16,
                        text_color: clay::Color::rgb(255., 255., 255.),
                        ..clay::default()
                    }
                    .with(text)],
                    |_| {},
                );
            },
        );
    };

    let render_header_button = |builder: &clay::Builder, text: clay::String| {
        builder.build(
            &[
                clay::Layout {
                    padding: clay::Padding { x: 16, y: 8 },
                    ..clay::default()
                }
                .into(),
                clay::Rectangle {
                    color: clay::Color::rgb(140., 140., 140.),
                    corner_radius: clay::CornerRadius::new(5.),
                }
                .into(),
            ],
            |builder| {
                builder.build(
                    &[clay::Text {
                        font_id: font_id_body_16,
                        font_size: 16,
                        text_color: clay::Color::rgb(255., 255., 255.),
                        ..clay::default()
                    }
                    .with(text)],
                    |_| {},
                );
            },
        );
    };

    loop {
        let mut selected_document_index = 0;

        let handle_sidebar_interaction =
            |_element_id: clay::ElementId, pointer_data: clay::PointerData, user_data: isize| {
                if pointer_data.state == clay::PointerDataInteractionState::PressedThisFrame
                    && user_data >= 0
                    && user_data < documents.len() as isize
                {
                    selected_document_index = user_data as usize;
                }
            };

        arena.render(&renderer, |builder| {
            builder.build(
                &[
                    clay::Id("OuterContainer".into()).into(),
                    clay::Rectangle {
                        color: clay::Color::rgb(43., 41., 51.),
                        ..clay::default()
                    }
                    .into(),
                    clay::Layout {
                        layout_direction: clay::LayoutDirection::TopToBottom,
                        sizing: layout_expand,
                        padding: clay::Padding { x: 16, y: 16 },
                        child_gap: 16,
                        ..clay::default()
                    }
                    .into(),
                ],
                |builder| {
                    builder.build(
                        &[
                            clay::Id("HeaderBar".into()).into(),
                            content_background.into(),
                            clay::Layout {
                                sizing: clay::Sizing {
                                    width: clay::SizingAxis::fixed(60.),
                                    height: clay::SizingAxis::grow(0., 0.),
                                },
                                padding: clay::Padding { x: 16, y: 0 },
                                child_gap: 16,
                                child_alignment: clay::ChildAlignment {
                                    y: clay::LayoutAlignmentY::Center,
                                    ..clay::default()
                                },
                                ..clay::default()
                            }
                            .into(),
                        ],
                        |builder| {
                            builder.build(
                                &[
                                    clay::Id("FileButton".into()).into(),
                                    clay::Layout {
                                        padding: clay::Padding { x: 16, y: 16 },
                                        ..clay::default()
                                    }
                                    .into(),
                                    clay::Rectangle {
                                        color: clay::Color::rgb(140., 140., 140.),
                                        corner_radius: clay::CornerRadius::new(5.),
                                    }
                                    .into(),
                                ],
                                |builder| {
                                    clay::Text {
                                        font_id: font_id_body_16,
                                        font_size: 16,
                                        text_color: clay::Color::rgb(255., 255., 255.),
                                        ..clay::default()
                                    }
                                    .with("File".into());

                                    let file_menu_visible =
                                        clay::ElementId::find("FileButton".into())
                                            .is_pointer_over()
                                            || clay::ElementId::find("FileMenu".into())
                                                .is_pointer_over();

                                    if file_menu_visible {
                                        builder.build(
                                        &[
                                            clay::Id("FileMenu".into()).into(),
                                            clay::Floating {
                                                attachment: clay::FloatingAttachPoints {
                                                    parent:
                                                        clay::FloatingAttachPointType::LeftBottom,
                                                    ..clay::default()
                                                },
                                                ..clay::default()
                                            }
                                            .into(),
                                            clay::Layout {
                                                padding: clay::Padding { x: 0, y: 8 },
                                                ..clay::default()
                                            }
                                            .into(),
                                        ],
                                        |builder| {
                                            builder.build(
                                                &[
                                                    clay::Layout {
                                                        layout_direction:
                                                            clay::LayoutDirection::TopToBottom,
                                                        sizing: clay::Sizing {
                                                            width: clay::SizingAxis::fixed(200.),
                                                            ..clay::default()
                                                        },
                                                        ..clay::default()
                                                    }
                                                    .into(),
                                                    clay::Rectangle {
                                                        color: clay::Color::rgb(40., 40., 40.),
                                                        corner_radius: clay::CornerRadius::new(8.),
                                                    }
                                                    .into(),
                                                ],
                                                |builder| {
                                                    // Render dropdown items here
                                                    render_dropdown_menu_item(
                                                        builder,
                                                        "New".into(),
                                                    );
                                                    render_dropdown_menu_item(
                                                        builder,
                                                        "Open".into(),
                                                    );
                                                    render_dropdown_menu_item(
                                                        builder,
                                                        "Close".into(),
                                                    );
                                                },
                                            );
                                        },
                                    );
                                    }
                                },
                            );
                        },
                    );
                    render_header_button(builder, "Edit".into());
                    builder.build(
                        &[clay::Layout {
                            sizing: clay::Sizing {
                                width: clay::SizingAxis::grow(0., 0.),
                                ..clay::default()
                            },
                            ..clay::default()
                        }
                        .into()],
                        |_| {},
                    );
                    render_header_button(builder, "Upload".into());
                    render_header_button(builder, "Media".into());
                    render_header_button(builder, "Support".into());
                },
            );

            builder.build(
                &[
                    clay::Id("LowerContent".into()).into(),
                    clay::Layout {
                        sizing: layout_expand,
                        child_gap: 16,
                        ..clay::default()
                    }
                    .into(),
                ],
                |builder| {
                    builder.build(
                        &[
                            clay::Id("Sidebar".into()).into(),
                            content_background.into(),
                            clay::Layout {
                                layout_direction: clay::LayoutDirection::TopToBottom,
                                padding: clay::Padding { x: 16, y: 16 },
                                child_gap: 8,
                                sizing: clay::Sizing {
                                    width: clay::SizingAxis::fixed(250.),
                                    height: clay::SizingAxis::grow(0., 0.),
                                },
                                ..clay::default()
                            }
                            .into(),
                        ],
                        |builder| {
                            for (i, document) in documents.iter().enumerate() {
                                let sidebar_button_layout = clay::Layout {
                                    sizing: clay::Sizing {
                                        width: clay::SizingAxis::grow(0., 0.),
                                        ..clay::default()
                                    },
                                    padding: clay::Padding { x: 16, y: 16 },
                                    ..clay::default()
                                };

                                let rectangle_fn = || {
                                    if i == selected_document_index {
                                        Some(clay::Item::Rectangle(clay::Rectangle {
                                            color: clay::Color::rgb(120., 120., 120.),
                                            corner_radius: clay::CornerRadius::new(8.),
                                        }))
                                    } else {
                                        // XXX can't use closures with FFI
                                        // clay::Item::set_on_hover_callback(
                                        //     handle_sidebar_interaction,
                                        //     i as isize,
                                        // );
                                        if clay::Item::is_hovered() {
                                            // XXX this runs in the wrong element context
                                            Some(clay::Item::Rectangle(clay::Rectangle {
                                                color: clay::Color::rgba(120., 120., 120., 120.),
                                                corner_radius: clay::CornerRadius::new(8.),
                                            }))
                                        } else {
                                            None
                                        }
                                    }
                                };

                                builder.build(
                                    &[
                                        sidebar_button_layout.into(),
                                        (&clay::Deferred(&rectangle_fn)).into(),
                                    ],
                                    |builder| {
                                        builder.build(
                                            &[clay::Text {
                                                font_id: font_id_body_16,
                                                font_size: 20,
                                                text_color: clay::Color::rgb(255., 255., 255.),
                                                ..clay::default()
                                            }
                                            .with(document.title)],
                                            |_| {},
                                        );
                                    },
                                );
                            }
                        },
                    );

                    builder.build(
                        &[
                            clay::Id("MainContent".into()).into(),
                            content_background.into(),
                            clay::Scroll {
                                vertical: true,
                                ..clay::default()
                            }
                            .into(),
                            clay::Layout {
                                layout_direction: clay::LayoutDirection::TopToBottom,
                                child_gap: 16,
                                padding: clay::Padding { x: 16, y: 16 },
                                sizing: layout_expand,
                                ..clay::default()
                            }
                            .into(),
                        ],
                        |builder| {
                            let selected_document = &documents[selected_document_index];
                            builder.build(
                                &[
                                    clay::Text {
                                        font_id: font_id_body_16,
                                        font_size: 24,
                                        text_color: clay::Color::rgb(255., 255., 255.),
                                        ..clay::default()
                                    }
                                    .with(selected_document.title),
                                    clay::Text {
                                        font_id: font_id_body_16,
                                        font_size: 24,
                                        text_color: clay::Color::rgb(255., 255., 255.),
                                        ..clay::default()
                                    }
                                    .with(selected_document.contents),
                                ],
                                |_| {},
                            );
                        },
                    );
                },
            );
        });

        next_frame().await
    }
}
