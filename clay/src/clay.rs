use clay_macros::packed_enum;
use std::{
    marker::PhantomData,
    os::raw::{c_char, c_float, c_int, c_void},
};

#[repr(C)]
#[derive(Clone)]
pub struct ClayArray<'a, T> {
    capacity: u32,
    length: u32,
    internal_array: *const T,
    _lifetime_marker: PhantomData<&'a T>,
}

#[repr(C)]
#[derive(Clone)]
pub struct String<'a> {
    length: c_int,
    chars: *const c_char,
    _lifetime_marker: PhantomData<&'a c_char>,
}

impl<'a> From<&'a str> for String<'a> {
    #[inline]
    fn from(s: &'a str) -> String<'a> {
        Self {
            length: s.len() as c_int,
            chars: s.as_ptr() as *const c_char,
            _lifetime_marker: PhantomData,
        }
    }
}

type StringArray<'a> = ClayArray<'a, &'a String<'a>>;

#[repr(C)]
pub struct Arena<'a> {
    pub label: String<'a>,
    next_allocation: u64,
    capacity: u64,
    memory: *mut c_void,
    _lifetime_marker: PhantomData<&'a c_void>,
}

impl<'a> Arena<'a> {
    pub fn new(memory: &'a [u8]) -> Arena<'a> {
        unsafe {
            Clay_CreateArenaWithCapacityAndMemory(
                memory.len() as u32,
                memory.as_ptr() as *const c_void,
            )
        }
    }
    pub fn min_memory_size() -> u32 {
        unsafe { Clay_MinMemorySize() }
    }
    pub fn initialize(self, layout_dimensions: Dimensions) {
        unsafe { Clay_Initialize(self, layout_dimensions) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Dimensions {
    pub width: c_float,
    pub height: c_float,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BoundingBox {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
}

#[repr(C)]
#[derive(Clone)]
pub struct ElementId<'a> {
    id: u32,
    offset: u32,
    base_id: u32,
    string_id: String<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CornerRadius {
    pub top_left: c_float,
    pub top_right: c_float,
    pub bottom_left: c_float,
    pub bottom_right: c_float,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum ElementConfigType {
    Rectangle = 1,
    BorderContainer = 2,
    FloatingContainer = 4,
    ScrollContainer = 8,
    Image = 16,
    Text = 32,
    Custom = 64,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum LayoutDirection {
    LeftToRight,
    TopToBottom,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum LayoutAlignmentX {
    Left,
    Right,
    Center,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum LayoutAlignmentY {
    Top,
    Bottom,
    Center,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum SizingType {
    Fit,
    Grow,
    Percent,
    Fixed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ChildAlignment {
    pub x: LayoutAlignmentX,
    pub y: LayoutAlignmentY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SizingMinMax {
    pub min: c_float,
    pub max: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union SizeUnion {
    pub size_minmax: SizingMinMax,
    pub size_percent: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SizingAxis {
    pub size: SizeUnion,
    pub r#type: SizingType,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sizing {
    pub width: SizingAxis,
    pub height: SizingAxis,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Padding {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LayoutConfig {
    pub sizing: Sizing,
    pub padding: Padding,
    pub child_gap: u16,
    pub child_alignment: ChildAlignment,
    pub layout_direction: LayoutDirection,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RectangleElementConfig {
    pub color: Color,
    pub corner_radius: CornerRadius,
    // CLAY_EXTEND_CONFIG_RECTANGLE
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum TextElementConfigWrapMode {
    ClayTextWrapWords,
    ClayTextWrapNewlines,
    ClayTextWrapNone,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TextElementConfig {
    pub text_color: Color,
    pub font_id: u16,
    pub font_size: u16,
    pub letter_spacing: u16,
    pub line_height: u16,
    pub wrap_mode: TextElementConfigWrapMode,
    // CLAY_EXTEND_CONFIG_TEXT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ImageElementConfig {
    pub image_data: *const c_void, // XXX fix
    pub source_dimensions: Dimensions,
    // CLAY_EXTEND_CONFIG_IMAGE
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum FloatingAttachPointType {
    LeftTop,
    LeftCenter,
    LeftBottom,
    CenterTop,
    CenterCenter,
    CenterBottom,
    RightTop,
    RightCenter,
    RightBottom,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FloatingAttachPoints {
    pub element: FloatingAttachPointType,
    pub parent: FloatingAttachPointType,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum PointerCaptureMode {
    Capture,
    Passthrough,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FloatingElementConfig {
    pub offset: Vector2,
    pub expand: Dimensions,
    pub z_index: u16,
    pub parent_id: u32,
    pub attachment: FloatingAttachPoints,
    pub pointer_capture_mode: PointerCaptureMode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CustomElementConfig {
    custom_data: *const c_void,
    // CLAY_EXTEND_CONFIG_CUSTOM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScrollElementConfig {
    pub horizontal: bool,
    pub vertical: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Border {
    pub width: u32,
    pub color: Color,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BorderElementConfig {
    pub left: Border,
    pub right: Border,
    pub top: Border,
    pub bottom: Border,
    pub between_children: Border,
    pub corner_radius: CornerRadius,
}

// XXX figure this out
#[repr(C)]
#[derive(Copy, Clone)]
pub union ElementConfigUnion<'a> {
    rectangle_element_config: &'a RectangleElementConfig,
    text_element_config: &'a TextElementConfig,
    image_element_config: &'a ImageElementConfig,
    floating_element_config: &'a FloatingElementConfig,
    custom_element_config: &'a CustomElementConfig,
    scroll_element_config: &'a ScrollElementConfig,
    border_element_config: &'a BorderElementConfig,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElementConfig<'a> {
    r#type: ElementConfigType,
    config: ElementConfigUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScrollContainerData<'a> {
    scroll_position: &'a Vector2, // XXX
    scroll_container_dimensions: Dimensions,
    content_dimensions: Dimensions,
    config: ScrollElementConfig,
    found: bool,
}

#[packed_enum]
#[derive(Copy, Clone)]
pub enum RenderCommandType {
    None,
    Rectangle,
    Border,
    Text,
    Image,
    ScissorStart,
    ScissorEnd,
    Custom,
}

#[repr(C)]
#[derive(Clone)]
pub struct RenderCommand<'a> {
    bounding_box: BoundingBox,
    config: ElementConfigUnion<'a>,
    text: String<'a>, // XXX fix
    id: u32,
    command_type: RenderCommandType,
}

pub type RenderCommandArray<'a> = ClayArray<'a, &'a RenderCommand<'a>>;

#[link(name = "clay")]
extern "C" {
    fn Clay_MinMemorySize() -> u32;
    fn Clay_CreateArenaWithCapacityAndMemory<'a>(capacity: u32, offset: *const c_void)
        -> Arena<'a>;
    fn Clay_Initialize(arena: Arena, layout_dimensions: Dimensions);

    fn Clay__OpenElement();
    fn Clay__ElementPostConfiguration();
    fn Clay__CloseElement();
}

struct Element;

impl Element {
    fn new() {
        unsafe {
            Clay__OpenElement();
        }
    }
}

impl Drop for Element {
    fn drop(&mut self) {
        unsafe {
            Clay__CloseElement();
        }
    }
}

#[macro_export]
macro_rules! clay {
    ( ( $( $expression:expr ),+ ) $( $children:block )? ) => {
        {
            let _guard = Element;
            $( $expression );+;
            unsafe { Clay__ElementPostConfiguration() ; }
            $( $children )?
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_arena() {
        let memory = [0u8; 1024];
        let arena = Arena::new(&memory);
        assert_eq!(arena.capacity, memory.len() as u64);
    }

    #[test]
    fn initialize_arena() {
        let size: u32 = Arena::min_memory_size();
        let memory = vec![0u8; size as usize];
        let arena = Arena::new(&memory);
        assert_eq!(arena.capacity, memory.len() as u64);
        let dimensions = Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions);
    }

    #[test]
    fn initialize_arena_label() {
        let size: u32 = Arena::min_memory_size();
        let memory = vec![0u8; size as usize];
        let mut arena = Arena::new(&memory);
        arena.label = "Main Arena".into();
        assert_eq!(arena.capacity, memory.len() as u64);
        let dimensions = Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions);
        // XXX assert on label
    }
}
