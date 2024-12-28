use crate::{external, ui};
use clay_macros::packed_enum;
use core::slice;
use std::{
    fmt,
    marker::PhantomData,
    os::raw::{c_char, c_float, c_int},
};

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClayArray<'a, T> {
    capacity: u32,
    length: u32,
    internal_array: *const T,
    _lifetime_marker: PhantomData<&'a T>,
}

pub struct ClayArrayIter<'a, T> {
    pub(crate) array: ClayArray<'a, T>,
    pub(crate) index: i32,
    pub(crate) getter: unsafe extern "C" fn(&ClayArray<'a, T>, i32) -> &'a T,
}

impl<'a, T> Iterator for ClayArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.length as i32 {
            None
        } else {
            let item = unsafe { (self.getter)(&self.array, self.index) };
            self.index += 1;
            Some(item)
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
// clay: CLAY_STRING
pub struct String<'a> {
    length: c_int,
    chars: *const c_char,
    _lifetime_marker: PhantomData<&'a c_char>,
}

impl String<'_> {
    pub fn len(&self) -> usize {
        self.length as usize
    }
}

impl fmt::Debug for String<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = if self.length == 0 {
            ""
        } else {
            let bytes =
                unsafe { slice::from_raw_parts(self.chars as *const u8, self.length as usize) };
            std::str::from_utf8(bytes).unwrap_or(&"<invalid UTF-8")
        };
        f.debug_struct("String").field("chars", &s).finish()
    }
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

// clay: Clay__StringArray
type StringArray<'a> = ClayArray<'a, String<'a>>;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_Dimensions
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
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_Vector2
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

// XXX can we make this (and some other structs) tuple structs? Color(0., 128, 255., 255.) - is tht same C layout?
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_Color
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_BoundingBox
pub struct BoundingBox {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_PointerDataInteractionState
pub enum PointerDataInteractionState {
    PressedThisFrame,
    Pressed,
    ReleasedThisFrame,
    Released,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_PointerData
pub struct PointerData {
    position: Vector2,
    state: PointerDataInteractionState,
}

pub type OnHoverCallback = fn(ElementId, PointerData, *const c_int);
pub type QueryScrollOffsetCallback = fn(u32) -> Vector2;

#[repr(C)]
#[derive(Copy, Clone)]
// clay: Clay_ElementId
pub struct ElementId<'a> {
    id: u32,
    offset: u32,
    base_id: u32,
    string_id: String<'a>,
}

impl ElementId<'_> {
    // clay: Clay_Hovered
    pub fn hovered() -> bool {
        unsafe { external::Clay_Hovered() }
    }
    // clay: Clay_PointerOver
    pub fn pointer_over(&self) -> bool {
        unsafe { external::Clay_PointerOver(*self) }
    }
    // clay: Clay_GetScrollContainerData
    pub fn get_scroll_container_data(&self) -> ScrollContainerData<'_> {
        unsafe { external::Clay_GetScrollContainerData(*self) }
    }
    // clay: Clay_OnHover
    pub fn on_hover(on_hover: OnHoverCallback, user_data: *const c_int) {
        unsafe {
            external::Clay_OnHover(on_hover, user_data);
        }
    }
    // clay: Clay_SetQueryScrollOffsetFunction
    pub fn set_query_scroll_offset_callback(
        query_scroll_offset_callback: QueryScrollOffsetCallback,
    ) {
        unsafe {
            external::Clay_SetQueryScrollOffsetFunction(query_scroll_offset_callback);
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
// clay: Clay_ScrollContainerData
pub struct ScrollContainerData<'a> {
    scroll_position: &'a Vector2,
    scroll_container_dimensions: Dimensions,
    content_dimensions: Dimensions,
    config: ui::Scroll,
    found: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_CornerRadius
pub struct CornerRadius {
    pub top_left: c_float,
    pub top_right: c_float,
    pub bottom_left: c_float,
    pub bottom_right: c_float,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_LayoutDirection
pub enum LayoutDirection {
    #[default]
    LeftToRight,
    TopToBottom,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_LayoutAlignmentX
pub enum LayoutAlignmentX {
    #[default]
    Left,
    Right,
    Center,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_LayoutAlignmentY
pub enum LayoutAlignmentY {
    #[default]
    Top,
    Bottom,
    Center,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay__SizingType
pub enum SizingType {
    #[default]
    Fit,
    Grow,
    Percent,
    Fixed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_ChildAlignment
pub struct ChildAlignment {
    pub x: LayoutAlignmentX,
    pub y: LayoutAlignmentY,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_SizingMinMax
pub struct SizingMinMax {
    pub min: c_float,
    pub max: c_float,
}

impl Default for SizingMinMax {
    fn default() -> Self {
        Self {
            min: default(),
            max: f32::MAX,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
union SizeUnion {
    minmax: SizingMinMax,
    percent: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
// clay: Clay_SizingAxis
pub struct SizingAxis {
    size: SizeUnion,
    r#type: SizingType,
}

impl SizingAxis {
    // clay: CLAY_SIZING_FIT
    pub fn fit(min: f32, max: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Fit,
            size: SizeUnion {
                minmax: SizingMinMax {
                    min: min as c_float,
                    max: max as c_float,
                },
            },
        }
    }
    // clay: CLAY_SIZING_GROW
    pub fn grow(min: f32, max: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Grow,
            size: SizeUnion {
                minmax: SizingMinMax {
                    min: min as c_float,
                    max: max as c_float,
                },
            },
        }
    }
    // clay: CLAY_SIZING_FIXED
    pub fn fixed(size: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Fixed,
            size: SizeUnion {
                minmax: SizingMinMax {
                    min: size as c_float,
                    max: size as c_float,
                },
            },
        }
    }
    // clay: CLAY_SIZING_PERCENT
    pub fn percent(percent: f32) -> Self {
        SizingAxis {
            r#type: SizingType::Percent,
            size: SizeUnion {
                percent: percent as c_float,
            },
        }
    }
}

impl Default for SizingAxis {
    fn default() -> Self {
        Self {
            r#type: SizingType::Fit,
            size: SizeUnion { minmax: default() },
        }
    }
}

impl fmt::Debug for SizingAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SizingAxis")
            .field("type", &self.r#type)
            .field(
                "size",
                match self.r#type {
                    SizingType::Percent => unsafe { &self.size.percent },
                    _ => unsafe { &self.size.minmax },
                },
            )
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_Sizing
pub struct Sizing {
    pub width: SizingAxis,
    pub height: SizingAxis,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_Padding
pub struct Padding {
    pub x: u16,
    pub y: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_TextElementConfigWrapMode
pub enum TextWrapMode {
    #[default]
    ClayTextWrapWords,
    ClayTextWrapNewlines,
    ClayTextWrapNone,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_FloatingAttachPointType
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
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_FloatingAttachPoints
pub struct FloatingAttachPoints {
    pub element: FloatingAttachPointType,
    pub parent: FloatingAttachPointType,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_PointerCaptureMode
pub enum PointerCaptureMode {
    #[default]
    Capture,
    Passthrough,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// clay: Clay_Border
pub struct BorderStyle {
    pub width: u32,
    pub color: Color,
}
