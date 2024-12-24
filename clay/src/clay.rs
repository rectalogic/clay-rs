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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
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
#[derive(Copy, Clone, Default)]
pub enum LayoutDirection {
    #[default]
    LeftToRight,
    TopToBottom,
}

#[packed_enum]
#[derive(Copy, Clone, Default)]
pub enum LayoutAlignmentX {
    #[default]
    Left,
    Right,
    Center,
}

#[packed_enum]
#[derive(Copy, Clone, Default)]
pub enum LayoutAlignmentY {
    #[default]
    Top,
    Bottom,
    Center,
}

#[packed_enum]
#[derive(Copy, Clone, Default)]
pub enum SizingType {
    #[default]
    Fit,
    Grow,
    Percent,
    Fixed,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct ChildAlignment {
    pub x: LayoutAlignmentX,
    pub y: LayoutAlignmentY,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
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

impl SizingAxis {
    fn fit(min: f32, max: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Fit,
            size: SizeUnion {
                size_minmax: SizingMinMax {
                    min: min as c_float,
                    max: max as c_float,
                },
            },
        }
    }
    fn grow(min: f32, max: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Grow,
            size: SizeUnion {
                size_minmax: SizingMinMax {
                    min: min as c_float,
                    max: max as c_float,
                },
            },
        }
    }
    fn fixed(size: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Fixed,
            size: SizeUnion {
                size_minmax: SizingMinMax {
                    min: size as c_float,
                    max: size as c_float,
                },
            },
        }
    }
    fn percent(percent: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Percent,
            size: SizeUnion {
                size_percent: percent as c_float,
            },
        }
    }
}
impl Default for SizingAxis {
    fn default() -> Self {
        Self {
            r#type: SizingType::Fit,
            size: SizeUnion {
                size_minmax: SizingMinMax::default(),
            },
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
// Clay_Sizing
pub struct Sizing {
    pub width: SizingAxis,
    pub height: SizingAxis,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Padding {
    pub x: u16,
    pub y: u16,
}

#[packed_enum]
#[derive(Copy, Clone, Default)]
// Clay_TextElementConfigWrapMode
pub enum TextWrapMode {
    #[default]
    ClayTextWrapWords,
    ClayTextWrapNewlines,
    ClayTextWrapNone,
}

#[packed_enum]
#[derive(Copy, Clone, Default)]
pub enum FloatingAttachPointType {
    #[default]
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
#[derive(Copy, Clone, Default)]
pub struct FloatingAttachPoints {
    pub element: FloatingAttachPointType,
    pub parent: FloatingAttachPointType,
}

#[packed_enum]
#[derive(Copy, Clone, Default)]
pub enum PointerCaptureMode {
    #[default]
    Capture,
    Passthrough,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
// Clay_Border
pub struct BorderStyle {
    pub width: u32,
    pub color: Color,
}

// XXX figure this out
#[repr(C)]
#[derive(Copy, Clone)]
union ElementConfigUnion<'a> {
    rectangle_element_config: &'a ui::Rectangle,
    text_element_config: &'a ui::Text,
    image_element_config: &'a ui::Image,
    floating_element_config: &'a ui::Floating,
    custom_element_config: &'a ui::Custom,
    scroll_element_config: &'a ui::Scroll,
    border_element_config: &'a ui::Border,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct ElementConfig<'a> {
    r#type: ElementConfigType,
    config: ElementConfigUnion<'a>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ScrollContainerData<'a> {
    scroll_position: &'a Vector2, // XXX
    scroll_container_dimensions: Dimensions,
    content_dimensions: Dimensions,
    config: ui::Scroll,
    found: bool,
}

trait Configure {
    fn configure(&self);
}

pub mod ui {
    use super::*;

    pub struct Id<'a>(pub String<'a>);

    // CLAY_ID
    impl Configure for Id<'_> {
        fn configure(&self) {
            unsafe {
                Clay__AttachId(Clay__HashString(self.0, 0, 0));
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone, Default)]
    // Clay_LayoutConfig
    pub struct Layout {
        pub sizing: Sizing,
        pub padding: Padding,
        pub child_gap: u16,
        pub child_alignment: ChildAlignment,
        pub layout_direction: LayoutDirection,
    }

    impl Configure for Layout {
        fn configure(&self) {
            unsafe {
                Clay__AttachLayoutConfig(Clay__StoreLayoutConfig(*self));
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_RectangleElementConfig
    pub struct Rectangle {
        pub color: Color,
        pub corner_radius: CornerRadius,
        // CLAY_EXTEND_CONFIG_RECTANGLE
    }

    impl Configure for Rectangle {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    rectangle_element_config: Clay__StoreRectangleElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::Rectangle);
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_TextElementConfig
    pub struct Text {
        pub text_color: Color,
        pub font_id: u16,
        pub font_size: u16,
        pub letter_spacing: u16,
        pub line_height: u16,
        pub wrap_mode: TextWrapMode,
        // CLAY_EXTEND_CONFIG_TEXT
    }

    //XXX Text takes a String and a TextElementConfig
    //XXX CLAY_TEXT(CLAY_STRING("FOO"), CLAY_TEXT_CONFIG({...}))
    // #define CLAY_TEXT(text, textConfig) Clay__OpenTextElement(text, textConfig)
    impl Configure for Text {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    text_element_config: Clay__StoreTextElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::Text);
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_ImageElementConfig
    pub struct Image {
        pub image_data: *const c_void, // XXX fix
        pub source_dimensions: Dimensions,
        // CLAY_EXTEND_CONFIG_IMAGE
    }

    impl Configure for Image {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    image_element_config: Clay__StoreImageElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::Image);
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_FloatingElementConfig
    pub struct Floating {
        pub offset: Vector2,
        pub expand: Dimensions,
        pub z_index: u16,
        pub parent_id: u32,
        pub attachment: FloatingAttachPoints,
        pub pointer_capture_mode: PointerCaptureMode,
    }

    impl Configure for Floating {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    floating_element_config: Clay__StoreFloatingElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::FloatingContainer);
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_CustomElementConfig
    pub struct Custom {
        custom_data: *const c_void,
        // CLAY_EXTEND_CONFIG_CUSTOM
    }

    impl Configure for Custom {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    custom_element_config: Clay__StoreCustomElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::Custom);
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_ScrollElementConfig
    pub struct Scroll {
        pub horizontal: bool,
        pub vertical: bool,
    }

    impl Configure for Scroll {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    scroll_element_config: Clay__StoreScrollElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::ScrollContainer);
            }
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    // Clay_BorderElementConfig
    pub struct Border {
        pub left: BorderStyle,
        pub right: BorderStyle,
        pub top: BorderStyle,
        pub bottom: BorderStyle,
        pub between_children: BorderStyle,
        pub corner_radius: CornerRadius,
    }

    impl Configure for Border {
        fn configure(&self) {
            unsafe {
                let config = ElementConfigUnion {
                    border_element_config: Clay__StoreBorderElementConfig(*self),
                };
                Clay__AttachElementConfig(config, ElementConfigType::BorderContainer);
            }
        }
    }
    // XXX add Border impl helpers
    // #define CLAY_BORDER(...) Clay__AttachElementConfig(CLAY__CONFIG_WRAPPER(Clay_ElementConfigUnion, { .borderElementConfig = Clay__StoreBorderElementConfig(CLAY__INIT(Clay_BorderElementConfig) __VA_ARGS__) }, CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER))
    // #define CLAY_BORDER_OUTSIDE(...) Clay__AttachElementConfig(CLAY__CONFIG_WRAPPER(Clay_ElementConfigUnion, { .borderElementConfig = Clay__StoreBorderElementConfig(CLAY__INIT(Clay_BorderElementConfig) { .left = __VA_ARGS__, .right = __VA_ARGS__, .top = __VA_ARGS__, .bottom = __VA_ARGS__ }) }, CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER))
    // #define CLAY_BORDER_OUTSIDE_RADIUS(width, color, radius) Clay__AttachElementConfig(CLAY__CONFIG_WRAPPER(Clay_ElementConfigUnion, { .borderElementConfig = Clay__StoreBorderElementConfig(CLAY__INIT(Clay_BorderElementConfig) { .left = { width, color }, .right = { width, color }, .top = { width, color }, .bottom = { width, color }, .cornerRadius = { radius, radius, radius, radius } })}, CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER))
    // #define CLAY_BORDER_ALL(...) Clay__AttachElementConfig(CLAY__CONFIG_WRAPPER(Clay_ElementConfigUnion, { .borderElementConfig = Clay__StoreBorderElementConfig(CLAY__INIT(Clay_BorderElementConfig) { .left = __VA_ARGS__, .right = __VA_ARGS__, .top = __VA_ARGS__, .bottom = __VA_ARGS__, .betweenChildren = __VA_ARGS__ }) }, CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER))
    // #define CLAY_BORDER_ALL_RADIUS(width, color, radius) Clay__AttachElementConfig(CLAY__CONFIG_WRAPPER(Clay_ElementConfigUnion, { .borderElementConfig = Clay__StoreBorderElementConfig(CLAY__INIT(Clay_BorderElementConfig) { .left = { width, color }, .right = { width, color }, .top = { width, color }, .bottom = { width, color }, .betweenChildren = { width, color }, .cornerRadius = { radius, radius, radius, radius }}) }))
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
    fn Clay__CloseElement();
    fn Clay__StoreLayoutConfig<'a>(config: ui::Layout) -> &'a ui::Layout;
    fn Clay__ElementPostConfiguration();
    fn Clay__AttachId(id: ElementId);
    fn Clay__AttachLayoutConfig(config: &ui::Layout);
    fn Clay__AttachElementConfig(config: ElementConfigUnion, r#type: ElementConfigType);
    fn Clay__StoreRectangleElementConfig<'a>(config: ui::Rectangle) -> &'a ui::Rectangle;
    fn Clay__StoreTextElementConfig<'a>(config: ui::Text) -> &'a ui::Text;
    fn Clay__StoreImageElementConfig<'a>(config: ui::Image) -> &'a ui::Image;
    fn Clay__StoreFloatingElementConfig<'a>(config: ui::Floating) -> &'a ui::Floating;
    fn Clay__StoreCustomElementConfig<'a>(config: ui::Custom) -> &'a ui::Custom;
    fn Clay__StoreScrollElementConfig<'a>(config: ui::Scroll) -> &'a ui::Scroll;
    fn Clay__StoreBorderElementConfig<'a>(config: ui::Border) -> &'a ui::Border;
    fn Clay__HashString(key: String, offset: u32, seed: u32) -> ElementId;
}

struct ParentElement;

impl ParentElement {
    fn new() {
        unsafe {
            Clay__OpenElement();
        }
    }
}

impl Drop for ParentElement {
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
            let _parent = ParentElement;
            $(
                $expression.configure();
            )+
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

    #[test]
    fn simple_ui() {
        clay!((
            ui::Id(String::from("HeroBlob")),
            ui::Layout {
                sizing: Sizing {
                    width: SizingAxis::grow(0.0, 480.0),
                    ..Default::default()
                },
                padding: Padding { x: 16, y: 16 },
                child_gap: 16,
                child_alignment: ChildAlignment {
                    y: LayoutAlignmentY::Center,
                    ..Default::default()
                },
                ..Default::default()
            }
        ));
        // CLAY(CLAY_IDI("HeroBlob", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_GROW({ .max = 480 }) }, .padding = {16, 16}, .childGap = 16, .childAlignment = {.y = CLAY_ALIGN_Y_CENTER} }), CLAY_BORDER_OUTSIDE_RADIUS(2, color, 10)) {
        //     CLAY(CLAY_IDI("CheckImage", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_FIXED(32) } }), CLAY_IMAGE({ .sourceDimensions = { 128, 128 }, .sourceURL = imageURL })) {}
        //     CLAY_TEXT(text, CLAY_TEXT_CONFIG({ .fontSize = fontSize, .fontId = FONT_ID_BODY_24, .textColor = color }));
        // }
    }
}
