use crate::data;
use crate::external;
use crate::system::{ElementConfigType, ElementConfigUnion};
use std::os::raw::{c_int, c_void};

pub type OnHoverCallback = extern "C" fn(data::ElementId, data::PointerData, *const c_int);

pub struct Builder(pub(crate) ());

impl Builder {
    // clay: CLAY macro
    pub fn build<F: FnOnce(&Self)>(&self, items: &[Item<'_>], build_children: F) {
        unsafe { external::Clay__OpenElement() };
        for item in items {
            match *item {
                Item::Id(id) => unsafe {
                    external::Clay__AttachId(external::Clay__HashString(id, 0, 0))
                },
                Item::IdI(id, i) => unsafe {
                    external::Clay__AttachId(external::Clay__HashString(id, i, 0))
                },
                Item::Layout(layout) => unsafe {
                    external::Clay__AttachLayoutConfig(external::Clay__StoreLayoutConfig(layout))
                },
                Item::Rectangle(rectangle) => unsafe {
                    external::Clay__AttachElementConfig(
                        ElementConfigUnion {
                            rectangle_element_config: external::Clay__StoreRectangleElementConfig(
                                rectangle,
                            ),
                        },
                        ElementConfigType::Rectangle,
                    )
                },
                Item::Text(text, config) => unsafe {
                    external::Clay__OpenTextElement(
                        text,
                        external::Clay__StoreTextElementConfig(config),
                    )
                },
                Item::Image(image) => unsafe {
                    external::Clay__AttachElementConfig(
                        ElementConfigUnion {
                            image_element_config: external::Clay__StoreImageElementConfig(image),
                        },
                        ElementConfigType::Image,
                    )
                },
                Item::Floating(floating) => unsafe {
                    external::Clay__AttachElementConfig(
                        ElementConfigUnion {
                            floating_element_config: external::Clay__StoreFloatingElementConfig(
                                floating,
                            ),
                        },
                        ElementConfigType::FloatingContainer,
                    )
                },
                Item::Custom(custom) => unsafe {
                    external::Clay__AttachElementConfig(
                        ElementConfigUnion {
                            custom_element_config: external::Clay__StoreCustomElementConfig(custom),
                        },
                        ElementConfigType::Custom,
                    )
                },
                Item::Scroll(scroll) => unsafe {
                    external::Clay__AttachElementConfig(
                        ElementConfigUnion {
                            scroll_element_config: external::Clay__StoreScrollElementConfig(scroll),
                        },
                        ElementConfigType::ScrollContainer,
                    )
                },
                Item::Border(border) => unsafe {
                    external::Clay__AttachElementConfig(
                        ElementConfigUnion {
                            border_element_config: external::Clay__StoreBorderElementConfig(border),
                        },
                        ElementConfigType::BorderContainer,
                    )
                },
            }
        }

        unsafe { external::Clay__ElementPostConfiguration() };

        build_children(self);

        unsafe { external::Clay__CloseElement() };
    }

    // clay: Clay_OnHover
    pub fn set_on_hover_callback(on_hover: OnHoverCallback, user_data: isize) {
        unsafe { external::Clay_OnHover(on_hover, user_data) };
    }

    // clay: Clay_Hovered
    pub fn hovered(&self) -> bool {
        unsafe { external::Clay_Hovered() }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Item<'a> {
    Id(data::String<'a>),
    IdI(data::String<'a>, u32),
    Layout(Layout),
    Rectangle(Rectangle),
    Text(data::String<'a>, Text),
    Image(Image),
    Floating(Floating),
    Custom(Custom),
    Scroll(Scroll),
    Border(Border),
}

#[derive(Debug, Copy, Clone)]
// clay: CLAY_ID
pub struct Id<'a>(pub data::String<'a>);

impl<'a> From<Id<'a>> for Item<'a> {
    fn from(id: Id<'a>) -> Self {
        Item::Id(id.0)
    }
}

#[derive(Debug, Copy, Clone)]
// clay: CLAY_IDI
pub struct IdI<'a>(pub data::String<'a>, pub u32);

impl<'a> From<IdI<'a>> for Item<'a> {
    fn from(id: IdI<'a>) -> Self {
        Item::IdI(id.0, id.1)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_LayoutConfig
// clay: CLAY_LAYOUT
pub struct Layout {
    pub sizing: data::Sizing,
    pub padding: data::Padding,
    pub child_gap: u16,
    pub child_alignment: data::ChildAlignment,
    pub layout_direction: data::LayoutDirection,
}

impl From<Layout> for Item<'_> {
    fn from(layout: Layout) -> Self {
        Item::Layout(layout)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_RectangleElementConfig
// clay: CLAY_RECTANGLE
pub struct Rectangle {
    pub color: data::Color,
    pub corner_radius: data::CornerRadius,
    // CLAY_EXTEND_CONFIG_RECTANGLE
}

impl From<Rectangle> for Item<'_> {
    fn from(rectangle: Rectangle) -> Self {
        Item::Rectangle(rectangle)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_TextElementConfig
// clay: CLAY_TEXT_CONFIG
pub struct Text {
    pub text_color: data::Color,
    pub font_id: u16,
    pub font_size: u16,
    pub letter_spacing: u16,
    pub line_height: u16,
    pub wrap_mode: data::TextWrapMode,
    // CLAY_EXTEND_CONFIG_TEXT
}

impl Text {
    // clay: CLAY_TEXT
    pub fn with<'a>(&self, text: data::String<'a>) -> Item<'a> {
        Item::Text(text, *self)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_ImageElementConfig
// clay: CLAY_IMAGE
pub struct Image {
    pub image_data: *const c_void, // XXX fix
    pub source_dimensions: data::Dimensions,
    // CLAY_EXTEND_CONFIG_IMAGE
}

impl Default for Image {
    fn default() -> Self {
        Self {
            image_data: std::ptr::null(),
            source_dimensions: data::Dimensions::default(),
        }
    }
}

impl From<Image> for Item<'_> {
    fn from(image: Image) -> Self {
        Item::Image(image)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_FloatingElementConfig
// clay: CLAY_FLOATING
pub struct Floating {
    pub offset: data::Vector2,
    pub expand: data::Dimensions,
    pub z_index: u16,
    pub parent_id: u32,
    pub attachment: data::FloatingAttachPoints,
    pub pointer_capture_mode: data::PointerCaptureMode,
}

impl From<Floating> for Item<'_> {
    fn from(floating: Floating) -> Self {
        Item::Floating(floating)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_CustomElementConfig
// clay: CLAY_CUSTOM_ELEMENT
pub struct Custom {
    custom_data: *const c_void,
    // CLAY_EXTEND_CONFIG_CUSTOM
}

impl From<Custom> for Item<'_> {
    fn from(custom: Custom) -> Self {
        Item::Custom(custom)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_ScrollElementConfig
// clay: CLAY_SCROLL
pub struct Scroll {
    pub horizontal: bool,
    pub vertical: bool,
}

impl From<Scroll> for Item<'_> {
    fn from(scroll: Scroll) -> Self {
        Item::Scroll(scroll)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_BorderElementConfig
// clay: CLAY_BORDER
pub struct Border {
    pub left: data::BorderStyle,
    pub right: data::BorderStyle,
    pub top: data::BorderStyle,
    pub bottom: data::BorderStyle,
    pub between_children: data::BorderStyle,
    pub corner_radius: data::CornerRadius,
}

impl Border {
    // clay: CLAY_BORDER_OUTSIDE
    pub fn outside(
        left: data::BorderStyle,
        right: data::BorderStyle,
        top: data::BorderStyle,
        bottom: data::BorderStyle,
    ) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
            ..data::default()
        }
    }
    // clay: CLAY_BORDER_OUTSIDE_RADIUS
    pub fn outside_radius(width: u32, color: data::Color, radius: f32) -> Self {
        Self {
            left: data::BorderStyle { width, color },
            right: data::BorderStyle { width, color },
            top: data::BorderStyle { width, color },
            bottom: data::BorderStyle { width, color },
            corner_radius: data::CornerRadius {
                top_left: radius,
                top_right: radius,
                bottom_left: radius,
                bottom_right: radius,
            },
            ..data::default()
        }
    }
    // clay: CLAY_BORDER_ALL
    pub fn all(style: data::BorderStyle) -> Self {
        Self {
            left: style,
            right: style,
            top: style,
            bottom: style,
            between_children: style,
            ..data::default()
        }
    }
    // clay: CLAY_BORDER_ALL_RADIUS
    pub fn all_radius(width: u32, color: data::Color, radius: f32) -> Self {
        let style = data::BorderStyle { width, color };
        Self {
            left: style,
            right: style,
            top: style,
            bottom: style,
            between_children: style,
            corner_radius: data::CornerRadius {
                top_left: radius,
                top_right: radius,
                bottom_left: radius,
                bottom_right: radius,
            },
        }
    }
}

impl From<Border> for Item<'_> {
    fn from(border: Border) -> Self {
        Item::Border(border)
    }
}
