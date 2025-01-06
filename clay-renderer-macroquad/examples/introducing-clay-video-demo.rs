use std::cell::Cell;

use clay::default as d;
use clay::prelude::*;
use macroquad::prelude::*;

// Based on https://github.com/nicbarker/clay/tree/main/examples/introducing-clay-video-demo

const WINDOW_SIZE: clay::Dimensions = clay::Dimensions {
    width: 1024.0,
    height: 768.0,
};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Introducing Clay Demo"),
        window_width: WINDOW_SIZE.width as i32,
        window_height: WINDOW_SIZE.height as i32,
        ..Default::default()
    }
}

struct Document<'a> {
    title: clay::String<'a>,
    contents: clay::String<'a>,
}

#[macroquad::main(window_conf)]
async fn main() {
    let documents =     [
        Document{ title: "Squirrels".into(), contents: "The Secret Life of Squirrels: Nature's Clever Acrobats\nSquirrels are often overlooked creatures, dismissed as mere park inhabitants or backyard nuisances. Yet, beneath their fluffy tails and twitching noses lies an intricate world of cunning, agility, and survival tactics that are nothing short of fascinating. As one of the most common mammals in North America, squirrels have adapted to a wide range of environments from bustling urban centers to tranquil forests and have developed a variety of unique behaviors that continue to intrigue scientists and nature enthusiasts alike.\n\nMaster Tree Climbers\nAt the heart of a squirrel's skill set is its impressive ability to navigate trees with ease. Whether they're darting from branch to branch or leaping across wide gaps, squirrels possess an innate talent for acrobatics. Their powerful hind legs, which are longer than their front legs, give them remarkable jumping power. With a tail that acts as a counterbalance, squirrels can leap distances of up to ten times the length of their body, making them some of the best aerial acrobats in the animal kingdom.\nBut it's not just their agility that makes them exceptional climbers. Squirrels' sharp, curved claws allow them to grip tree bark with precision, while the soft pads on their feet provide traction on slippery surfaces. Their ability to run at high speeds and scale vertical trunks with ease is a testament to the evolutionary adaptations that have made them so successful in their arboreal habitats.\n\nFood Hoarders Extraordinaire\nSquirrels are often seen frantically gathering nuts, seeds, and even fungi in preparation for winter. While this behavior may seem like instinctual hoarding, it is actually a survival strategy that has been honed over millions of years. Known as \"scatter hoarding,\" squirrels store their food in a variety of hidden locations, often burying it deep in the soil or stashing it in hollowed-out tree trunks.\nInterestingly, squirrels have an incredible memory for the locations of their caches. Research has shown that they can remember thousands of hiding spots, often returning to them months later when food is scarce. However, they don't always recover every stash some forgotten caches eventually sprout into new trees, contributing to forest regeneration. This unintentional role as forest gardeners highlights the ecological importance of squirrels in their ecosystems.\n\nThe Great Squirrel Debate: Urban vs. Wild\nWhile squirrels are most commonly associated with rural or wooded areas, their adaptability has allowed them to thrive in urban environments as well. In cities, squirrels have become adept at finding food sources in places like parks, streets, and even garbage cans. However, their urban counterparts face unique challenges, including traffic, predators, and the lack of natural shelters. Despite these obstacles, squirrels in urban areas are often observed using human infrastructure such as buildings, bridges, and power lines as highways for their acrobatic escapades.\nThere is, however, a growing concern regarding the impact of urban life on squirrel populations. Pollution, deforestation, and the loss of natural habitats are making it more difficult for squirrels to find adequate food and shelter. As a result, conservationists are focusing on creating squirrel-friendly spaces within cities, with the goal of ensuring these resourceful creatures continue to thrive in both rural and urban landscapes.\n\nA Symbol of Resilience\nIn many cultures, squirrels are symbols of resourcefulness, adaptability, and preparation. Their ability to thrive in a variety of environments while navigating challenges with agility and grace serves as a reminder of the resilience inherent in nature. Whether you encounter them in a quiet forest, a city park, or your own backyard, squirrels are creatures that never fail to amaze with their endless energy and ingenuity.\nIn the end, squirrels may be small, but they are mighty in their ability to survive and thrive in a world that is constantly changing. So next time you spot one hopping across a branch or darting across your lawn, take a moment to appreciate the remarkable acrobat at work a true marvel of the natural world.\n".into() },
        Document{ title: "Lorem Ipsum".into(), contents: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".into() },
        Document{ title: "Vacuum Instructions".into(), contents: "Chapter 3: Getting Started - Unpacking and Setup\n\nCongratulations on your new SuperClean Pro 5000 vacuum cleaner! In this section, we will guide you through the simple steps to get your vacuum up and running. Before you begin, please ensure that you have all the components listed in the \"Package Contents\" section on page 2.\n\n1. Unboxing Your Vacuum\nCarefully remove the vacuum cleaner from the box. Avoid using sharp objects that could damage the product. Once removed, place the unit on a flat, stable surface to proceed with the setup. Inside the box, you should find:\n\n    The main vacuum unit\n    A telescoping extension wand\n    A set of specialized cleaning tools (crevice tool, upholstery brush, etc.)\n    A reusable dust bag (if applicable)\n    A power cord with a 3-prong plug\n    A set of quick-start instructions\n\n2. Assembling Your Vacuum\nBegin by attaching the extension wand to the main body of the vacuum cleaner. Line up the connectors and twist the wand into place until you hear a click. Next, select the desired cleaning tool and firmly attach it to the wand's end, ensuring it is securely locked in.\n\nFor models that require a dust bag, slide the bag into the compartment at the back of the vacuum, making sure it is properly aligned with the internal mechanism. If your vacuum uses a bagless system, ensure the dust container is correctly seated and locked in place before use.\n\n3. Powering On\nTo start the vacuum, plug the power cord into a grounded electrical outlet. Once plugged in, locate the power switch, usually positioned on the side of the handle or body of the unit, depending on your model. Press the switch to the \"On\" position, and you should hear the motor begin to hum. If the vacuum does not power on, check that the power cord is securely plugged in, and ensure there are no blockages in the power switch.\n\nNote: Before first use, ensure that the vacuum filter (if your model has one) is properly installed. If unsure, refer to \"Section 5: Maintenance\" for filter installation instructions.".into() },
        Document{ title: "Article 4".into(), contents: "Article 4".into() },
        Document{ title: "Article 5".into(), contents: "Article 5".into() },
    ];

    let size: u32 = clay::Arena::min_memory_size();
    let memory = vec![0u8; size as usize];
    let arena = clay::Arena::new(&memory);
    arena.initialize(WINDOW_SIZE, d());
    let font = load_ttf_font(
        "clay/clay/examples/introducing-clay-video-demo/resources/Roboto-Regular.ttf",
    )
    .await
    .unwrap();
    let font_id_body_16 = clay_renderer_macroquad::add_font(font);

    let mut arena = arena;
    let renderer = clay_renderer_macroquad::MacroquadRenderer::new();

    let layout_expand = clay::Sizing {
        width: clay::SizingAxis::grow(0., f32::MAX),
        height: clay::SizingAxis::grow(0., f32::MAX),
    };
    let content_background = clay::Rectangle {
        color: clay::Color::rgb(90., 90., 90.),
        corner_radius: clay::CornerRadius::new(8.),
    };

    let render_dropdown_menu_item = |builder: &clay::Builder, text: clay::String| {
        builder.build(
            |builder| {
                clay::Layout {
                    padding: clay::Padding { x: 16, y: 16 },
                    ..d()
                }
                .attach(builder);
            },
            |builder| {
                clay::Text {
                    font_id: font_id_body_16,
                    font_size: 16,
                    text_color: clay::Color::rgb(255., 255., 255.),
                    ..d()
                }
                .with(text)
                .attach(builder);
            },
        );
    };

    let render_header_button = |builder: &clay::Builder, text: clay::String| {
        builder.build(
            |builder| {
                clay::Layout {
                    padding: clay::Padding { x: 16, y: 8 },
                    ..d()
                }.attach(builder);
                clay::Rectangle {
                    color: clay::Color::rgb(140., 140., 140.),
                    corner_radius: clay::CornerRadius::new(5.),
                }.attach(builder);
            },
            |builder| {
                clay::Text {
                    font_id: font_id_body_16,
                    font_size: 16,
                    text_color: clay::Color::rgb(255., 255., 255.),
                    ..d()
                }
                .with(text).attach(builder);
            },
        );
    };

    let current_selected_document_index = &Cell::new(0);
    // XXX can't get closures working...
    /*
    let mut hover_callback = 
    |element_id: clay::ElementId, pointer_data: clay::PointerData| {
        if pointer_data.state == clay::PointerDataInteractionState::PressedThisFrame {
            current_selected_document_index .set( element_id.offset() as usize);
        }
    };
    */
    unsafe  extern "C"  fn hover_callback_raw(element_id: clay::ElementId, pointer_data: clay::PointerData, user_data: isize) {
        if pointer_data.state == clay::PointerDataInteractionState::PressedThisFrame {
            unsafe {
                let ptr = user_data as *const &Cell<usize>;
                assert!(!ptr.is_null());
                let current_selected_document_index: &Cell<usize> = *ptr;
                current_selected_document_index .set( element_id.offset() as usize);
            }
        }
    }
    loop {
        arena.render(&renderer, |builder| {
            builder.build(
                |builder| {
                    clay::ElementId::new_id("OuterContainer".into()).attach(builder);
                    clay::Rectangle {
                        color: clay::Color::rgb(43., 41., 51.),
                        ..d()
                    }
                    .attach(builder);
                    clay::Layout {
                        layout_direction: clay::LayoutDirection::TopToBottom,
                        sizing: layout_expand,
                        padding: clay::Padding { x: 16, y: 16 },
                        child_gap: 16,
                        ..d()
                    }
                    .attach(builder);
                },
                |builder| {
                    builder.build(
                        |builder| {
                            clay::ElementId::new_id("HeaderBar".into()).attach(builder);
                            content_background.attach(builder);
                            clay::Layout {
                                sizing: clay::Sizing {
                                    height: clay::SizingAxis::fixed(60.),
                                    width: clay::SizingAxis::grow(0., f32::MAX),
                                },
                                padding: clay::Padding { x: 16, y: 0 },
                                child_gap: 16,
                                child_alignment: clay::ChildAlignment {
                                    y: clay::LayoutAlignmentY::Center,
                                    ..d()
                                },
                                ..d()
                            }
                            .attach(builder);
                        },
                        |builder| {
                            builder.build(
                                |builder| {
                                    clay::ElementId::new_id("FileButton".into()).attach(builder);
                                    clay::Layout {
                                        padding: clay::Padding { x: 16, y: 16 },
                                        ..d()
                                    }
                                    .attach(builder);
                                    clay::Rectangle {
                                        color: clay::Color::rgb(140., 140., 140.),
                                        corner_radius: clay::CornerRadius::new(5.),
                                    }
                                    .attach(builder);
                                },
                                |builder: &clay::Builder| {
                                    clay::Text {
                                        font_id: font_id_body_16,
                                        font_size: 16,
                                        text_color: clay::Color::rgb(255., 255., 255.),
                                        ..d()
                                    }
                                    .with("File".into()).attach(builder);
                               
                                    let file_menu_visible =
                                        clay::ElementId::find("FileButton".into())
                                            .is_pointer_over()
                                            || clay::ElementId::find("FileMenu".into())
                                                .is_pointer_over();

                                    if file_menu_visible {
                                        builder.build(
                                            |builder| {
                                                clay::ElementId::new_id("FileMenu".into()).attach(builder);
                                                clay::Floating {
                                                    attachment: clay::FloatingAttachPoints {
                                                        parent:
                                                            clay::FloatingAttachPointType::LeftBottom,
                                                        ..d()
                                                    },
                                                    ..d()
                                                }
                                                .attach(builder);
                                                clay::Layout {
                                                    padding: clay::Padding { x: 0, y: 8 },
                                                    ..d()
                                                }
                                                .attach(builder);
                                            },
                                            |builder: &clay::Builder| {
                                                builder.build(
                                                    |builder| {
                                                        clay::Layout {
                                                            layout_direction:
                                                                clay::LayoutDirection::TopToBottom,
                                                            sizing: clay::Sizing {
                                                                width: clay::SizingAxis::fixed(200.),
                                                                ..d()
                                                            },
                                                            ..d()
                                                        }
                                                        .attach(builder);
                                                        clay::Rectangle {
                                                            color: clay::Color::rgb(
                                                                40., 40., 40.,
                                                            ),
                                                            corner_radius:
                                                                clay::CornerRadius::new(8.),
                                                        }
                                                        .attach(builder);
                                                    },
                                                    |builder: &clay::Builder| {
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
                            render_header_button(builder, "Edit".into());
                            builder.build(
                                |builder| {
                                    clay::Layout {
                                        sizing: clay::Sizing {
                                            width: clay::SizingAxis::grow(0., f32::MAX),
                                            ..d()
                                        },
                                        ..d()
                                    }
                                    .attach(builder);
                                },
                                clay::no_children,
                            );
                            render_header_button(builder, "Upload".into());
                            render_header_button(builder, "Media".into());
                            render_header_button(builder, "Support".into());
                        },
                    );

                    builder.build(
                        |builder| {
                            clay::ElementId::new_id("LowerContent".into()).attach(builder);
                            clay::Layout {
                                sizing: layout_expand,
                                child_gap: 16,
                                ..d()
                            }
                            .attach(builder);
                        },
                        |builder| {
                            builder.build(
                                |builder| {
                                    clay::ElementId::new_id("Sidebar".into()).attach(builder);
                                    content_background.attach(builder);
                                    clay::Layout {
                                        layout_direction: clay::LayoutDirection::TopToBottom,
                                        padding: clay::Padding { x: 16, y: 16 },
                                        child_gap: 8,
                                        sizing: clay::Sizing {
                                            width: clay::SizingAxis::fixed(250.),
                                            height: clay::SizingAxis::grow(0., f32::MAX),
                                        },
                                        ..d()
                                    }
                                    .attach(builder);
                                },
                                |builder| {
                                    for (i, document) in documents.iter().enumerate() {
                                        let sidebar_button_layout = clay::Layout {
                                            sizing: clay::Sizing {
                                                width: clay::SizingAxis::grow(0., f32::MAX),
                                                ..d()
                                            },
                                            padding: clay::Padding { x: 16, y: 16 },
                                            ..d()
                                        };

                                        builder.build(
                                            |builder| {
                                                // Store index as IdI offset
                                                clay::ElementId::new_idi("DocumentButton".into(), i as u32).attach(builder);
                                                sidebar_button_layout.attach(builder);

                                                if i == current_selected_document_index.get() {
                                                    clay::Rectangle {
                                                        color: clay::Color::rgb(
                                                            120., 120., 120.,
                                                        ),
                                                        corner_radius: clay::CornerRadius::new(
                                                            8.,
                                                        ),
                                                    }
                                                    .attach(builder);
                                                } else {
                                                    // builder.set_on_hover_callback(&mut hover_callback);
                                                    builder.set_on_hover_callback_raw(hover_callback_raw, &current_selected_document_index);
                                                
                                                    if clay::Builder::is_hovered() {
                                                        clay::Rectangle {
                                                            color: clay::Color::rgba(
                                                                120., 120., 120., 120.,
                                                            ),
                                                            corner_radius:
                                                                clay::CornerRadius::new(8.),
                                                        }
                                                        .attach(builder);
                                                    }
                                                }
                                            },
                                            |builder| {
                                                builder.build(
                                                    |builder| {
                                                        clay::Text {
                                                            font_id: font_id_body_16,
                                                            font_size: 20,
                                                            text_color: clay::Color::rgb(
                                                                255., 255., 255.,
                                                            ),
                                                            ..d()
                                                        }
                                                        .with(document.title).attach(builder);
                                                    },
                                                    clay::no_children,
                                                );
                                            },
                                        );
                                    }
                                },
                            );

                            builder.build(
                                |builder| {
                                    clay::ElementId::new_id("MainContent".into()).attach(builder);
                                    content_background.attach(builder);
                                    clay::Scroll {
                                        vertical: true,
                                        ..d()
                                    }
                                    .attach(builder);
                                    clay::Layout {
                                        layout_direction: clay::LayoutDirection::TopToBottom,
                                        child_gap: 16,
                                        padding: clay::Padding { x: 16, y: 16 },
                                        sizing: layout_expand,
                                        ..d()
                                    }
                                    .attach(builder);
                                },
                                |builder| {
                                    let selected_document = &documents[current_selected_document_index.get()];
                                    builder.build(
                                        |builder| {
                                            clay::Text {
                                                font_id: font_id_body_16,
                                                font_size: 24,
                                                text_color: clay::Color::rgb(255., 255., 255.),
                                                ..d()
                                            }
                                            .with(selected_document.title).attach(builder);
                                            clay::Text {
                                                font_id: font_id_body_16,
                                                font_size: 24,
                                                text_color: clay::Color::rgb(255., 255., 255.),
                                                ..d()
                                            }
                                            .with(selected_document.contents).attach(builder);
                                        },
                                        clay::no_children,
                                    );
                                },
                            );
                        },
                    );
                },
            );
        });

        next_frame().await
    }
}
