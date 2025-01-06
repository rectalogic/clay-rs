use crate::data;
use crate::external;
use crate::system::{ElementConfigType, ElementConfigUnion};
use std::marker::PhantomData;
use std::os::raw::c_float;
use std::os::raw::c_void;

pub trait Element {
    fn attach(&self, builder: &crate::ui::Builder);
}

pub struct Builder<'a>((), PhantomData<&'a ()>);

impl<'a> Builder<'a> {
    pub(crate) fn new() -> Self {
        Self((), PhantomData)
    }

    // clay: CLAY macro
    pub fn build<FI, FC>(&self, items: FI, children: FC)
    where
        FI: FnOnce(&Self),
        FC: FnOnce(&Self),
    {
        unsafe { external::Clay__OpenElement() };

        items(self);

        unsafe { external::Clay__ElementPostConfiguration() };

        children(self);

        unsafe { external::Clay__CloseElement() };
    }

    // https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/
    unsafe extern "C" fn hover_trampoline<F>(
        element_id: ElementId,
        pointer_data: data::PointerData,
        user_data: isize,
    ) where
        F: FnMut(ElementId, data::PointerData),
    {
        let user_data = &mut *(user_data as *mut F);
        user_data(element_id, pointer_data);
    }

    fn get_hover_trampoline<F>(_closure: &F) -> external::OnHoverCallback
    where
        F: FnMut(ElementId, data::PointerData),
    {
        Self::hover_trampoline::<F>
    }

    // clay: Clay_OnHover
    pub fn set_on_hover_callback<F>(&'a self, on_hover: &mut F)
    where
        F: FnMut(ElementId, data::PointerData) + 'a,
    {
        let mut on_hover = on_hover;
        let trampoline = Self::get_hover_trampoline(&on_hover);
        unsafe {
            external::Clay_OnHover(trampoline, &mut on_hover as *mut _ as isize);
        }
    }

    // clay: Clay_OnHover
    pub fn set_on_hover_callback_raw<T>(
        &self,
        on_hover: unsafe extern "C" fn(ElementId, data::PointerData, isize),
        user_data: &T,
    ) {
        let user_data_ptr: isize = user_data as *const T as isize;
        unsafe {
            external::Clay_OnHover(on_hover, user_data_ptr);
        }
    }

    // clay: Clay_Hovered
    pub fn is_hovered() -> bool {
        unsafe { external::Clay_Hovered() }
    }

    // clay: Clay_SetPointerState
    pub fn set_pointer_state(position: data::Vector2, pointer_down: bool) {
        unsafe { external::Clay_SetPointerState(position, pointer_down) };
    }
    // clay: Clay_UpdateScrollContainers
    pub fn update_scroll_containers(
        enable_drag_scrolling: bool,
        scroll_delta: data::Vector2,
        delta_time: f32,
    ) {
        unsafe {
            external::Clay_UpdateScrollContainers(
                enable_drag_scrolling,
                scroll_delta,
                delta_time as c_float,
            )
        };
    }
}

pub fn no_children(_: &Builder) {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_ElementId
// clay: CLAY_ID
// clay: CLAY_IDI
pub struct ElementId<'a> {
    id: u32,
    offset: u32,
    base_id: u32,
    string_id: data::String<'a>,
}

impl<'a> ElementId<'a> {
    pub fn new_id(string_id: data::String<'a>) -> Self {
        unsafe { external::Clay__HashString(string_id, 0, 0) }
    }
    pub fn new_idi(string_id: data::String<'a>, offset: u32) -> Self {
        unsafe { external::Clay__HashString(string_id, offset, 0) }
    }
    pub fn offset(&self) -> u32 {
        self.offset
    }
    pub fn string_id(&'a self) -> data::String<'a> {
        self.string_id
    }
    pub fn find(id: data::String) -> Self {
        unsafe { external::Clay_GetElementId(id) }
    }
    // clay: Clay_PointerOver
    pub fn is_pointer_over(&self) -> bool {
        unsafe { external::Clay_PointerOver(*self) }
    }
    // clay: Clay_GetScrollContainerData
    pub fn get_scroll_container_data(&self) -> data::ScrollContainerData<'_> {
        unsafe { external::Clay_GetScrollContainerData(*self) }
    }
}

impl Element for ElementId<'_> {
    fn attach(&self, _builder: &Builder) {
        unsafe { external::Clay__AttachId(*self) };
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

impl Element for Layout {
    fn attach(&self, _builder: &Builder) {
        unsafe { external::Clay__AttachLayoutConfig(external::Clay__StoreLayoutConfig(*self)) };
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

impl Element for Rectangle {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__AttachElementConfig(
                ElementConfigUnion {
                    rectangle_element_config: external::Clay__StoreRectangleElementConfig(*self),
                },
                ElementConfigType::Rectangle,
            )
        };
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
    pub fn with<'a>(&self, text: data::String<'a>) -> (data::String<'a>, Text) {
        (text, *self)
    }
}

impl Element for (data::String<'_>, Text) {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__OpenTextElement(self.0, external::Clay__StoreTextElementConfig(self.1))
        };
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

impl Element for Image {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__AttachElementConfig(
                ElementConfigUnion {
                    image_element_config: external::Clay__StoreImageElementConfig(*self),
                },
                ElementConfigType::Image,
            )
        };
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

impl Element for Floating {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__AttachElementConfig(
                ElementConfigUnion {
                    floating_element_config: external::Clay__StoreFloatingElementConfig(*self),
                },
                ElementConfigType::FloatingContainer,
            )
        };
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

impl Element for Custom {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__AttachElementConfig(
                ElementConfigUnion {
                    custom_element_config: external::Clay__StoreCustomElementConfig(*self),
                },
                ElementConfigType::Custom,
            )
        };
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

impl Element for Scroll {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__AttachElementConfig(
                ElementConfigUnion {
                    scroll_element_config: external::Clay__StoreScrollElementConfig(*self),
                },
                ElementConfigType::ScrollContainer,
            )
        };
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

impl Element for Border {
    fn attach(&self, _builder: &Builder) {
        unsafe {
            external::Clay__AttachElementConfig(
                ElementConfigUnion {
                    border_element_config: external::Clay__StoreBorderElementConfig(*self),
                },
                ElementConfigType::BorderContainer,
            )
        };
    }
}
