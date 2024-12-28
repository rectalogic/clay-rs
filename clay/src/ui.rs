use crate::arena::internal::Configure;
use crate::arena::{ElementConfigType, ElementConfigUnion};
use crate::data;
use crate::external;
use std::os::raw::c_void;

#[derive(Debug, Copy, Clone)]
// clay: CLAY_ID
pub struct Id<'a>(pub data::String<'a>);

impl Configure for Id<'_> {
    fn configure(&self) {
        unsafe {
            external::Clay__AttachId(external::Clay__HashString(self.0, 0, 0));
        }
    }
}

#[derive(Debug, Copy, Clone)]
// clay: CLAY_IDI
pub struct IdI<'a>(pub data::String<'a>, pub u32);

impl Configure for IdI<'_> {
    fn configure(&self) {
        unsafe {
            external::Clay__AttachId(external::Clay__HashString(self.0, self.1, 0));
        }
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

impl Configure for Layout {
    fn configure(&self) {
        unsafe {
            external::Clay__AttachLayoutConfig(external::Clay__StoreLayoutConfig(*self));
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_RectangleElementConfig
// clay: CLAY_RECTANGLE
pub struct Rectangle {
    pub color: data::Color,
    pub corner_radius: data::CornerRadius,
    // CLAY_EXTEND_CONFIG_RECTANGLE
}

impl Configure for Rectangle {
    fn configure(&self) {
        unsafe {
            let config = ElementConfigUnion {
                rectangle_element_config: external::Clay__StoreRectangleElementConfig(*self),
            };
            external::Clay__AttachElementConfig(config, ElementConfigType::Rectangle);
        }
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
    pub fn with(&self, text: data::String) {
        unsafe {
            external::Clay__OpenTextElement(text, external::Clay__StoreTextElementConfig(*self));
        }
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

impl Configure for Image {
    fn configure(&self) {
        unsafe {
            let config = ElementConfigUnion {
                image_element_config: external::Clay__StoreImageElementConfig(*self),
            };
            external::Clay__AttachElementConfig(config, ElementConfigType::Image);
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
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

impl Configure for Floating {
    fn configure(&self) {
        unsafe {
            let config = ElementConfigUnion {
                floating_element_config: external::Clay__StoreFloatingElementConfig(*self),
            };
            external::Clay__AttachElementConfig(config, ElementConfigType::FloatingContainer);
        }
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

impl Configure for Custom {
    fn configure(&self) {
        unsafe {
            let config = ElementConfigUnion {
                custom_element_config: external::Clay__StoreCustomElementConfig(*self),
            };
            external::Clay__AttachElementConfig(config, ElementConfigType::Custom);
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_ScrollElementConfig
// clay: CLAY_SCROLL
pub struct Scroll {
    pub horizontal: bool,
    pub vertical: bool,
}

impl Configure for Scroll {
    fn configure(&self) {
        unsafe {
            let config = ElementConfigUnion {
                scroll_element_config: external::Clay__StoreScrollElementConfig(*self),
            };
            external::Clay__AttachElementConfig(config, ElementConfigType::ScrollContainer);
        }
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
impl Configure for Border {
    fn configure(&self) {
        unsafe {
            let config = ElementConfigUnion {
                border_element_config: external::Clay__StoreBorderElementConfig(*self),
            };
            external::Clay__AttachElementConfig(config, ElementConfigType::BorderContainer);
        }
    }
}
