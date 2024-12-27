use crate::clay::ClayArray;
use clay_macros::packed_enum;
use core::slice;
use std::{
    fmt,
    marker::PhantomData,
    os::raw::{c_char, c_float, c_int},
};

#[repr(C)]
#[derive(Copy, Clone)]
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

type StringArray<'a> = ClayArray<'a, String<'a>>;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
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
pub struct Vector2 {
    pub x: c_float,
    pub y: c_float,
}

// XXX can we make this (and some other structs) tuple structs? Color(0., 128, 255., 255.) - is tht same C layout?
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// Clay_Color
pub struct Color {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct BoundingBox {
    pub x: c_float,
    pub y: c_float,
    pub width: c_float,
    pub height: c_float,
}

#[repr(C)]
#[derive(Clone)]
// Clay_ElementId
pub struct ElementId<'a> {
    id: u32,
    offset: u32,
    base_id: u32,
    string_id: String<'a>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct CornerRadius {
    pub top_left: c_float,
    pub top_right: c_float,
    pub bottom_left: c_float,
    pub bottom_right: c_float,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
pub enum LayoutDirection {
    #[default]
    LeftToRight,
    TopToBottom,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
pub enum LayoutAlignmentX {
    #[default]
    Left,
    Right,
    Center,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
pub enum LayoutAlignmentY {
    #[default]
    Top,
    Bottom,
    Center,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
pub enum SizingType {
    #[default]
    Fit,
    Grow,
    Percent,
    Fixed,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct ChildAlignment {
    pub x: LayoutAlignmentX,
    pub y: LayoutAlignmentY,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct SizingMinMax {
    pub min: c_float,
    pub max: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
union SizeUnion {
    size_minmax: SizingMinMax,
    size_percent: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SizingAxis {
    size: SizeUnion,
    r#type: SizingType,
}

impl SizingAxis {
    pub fn fit(min: f32, max: f32) -> Self {
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
    pub fn grow(min: f32, max: f32) -> Self {
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
    pub fn fixed(size: f32) -> Self {
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
    pub fn percent(percent: f32) -> Self {
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

impl fmt::Debug for SizingAxis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SizingAxis")
            .field("type", &self.r#type)
            .field(
                "size",
                match self.r#type {
                    SizingType::Percent => unsafe { &self.size.size_percent },
                    _ => unsafe { &self.size.size_minmax },
                },
            )
            .finish()
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// Clay_Sizing
pub struct Sizing {
    pub width: SizingAxis,
    pub height: SizingAxis,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct Padding {
    pub x: u16,
    pub y: u16,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
// Clay_TextElementConfigWrapMode
pub enum TextWrapMode {
    #[default]
    ClayTextWrapWords,
    ClayTextWrapNewlines,
    ClayTextWrapNone,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
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
pub struct FloatingAttachPoints {
    pub element: FloatingAttachPointType,
    pub parent: FloatingAttachPointType,
}

#[packed_enum]
#[derive(Debug, Copy, Clone, Default)]
pub enum PointerCaptureMode {
    #[default]
    Capture,
    Passthrough,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
// Clay_Border
pub struct BorderStyle {
    pub width: u32,
    pub color: Color,
}
