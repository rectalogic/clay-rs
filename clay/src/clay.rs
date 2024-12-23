use clay_macros::packed_enum;
use std::{
    marker::PhantomData,
    os::raw::{c_char, c_float, c_int, c_void},
};
// XXX just use move instead of Copy, Clone for most of these?

#[repr(C)]
#[derive(Clone)]
pub struct String {
    length: c_int,
    chars: *const c_char,
    _owned: Option<Box<[u8]>>,
}

impl From<&str> for String {
    #[inline]
    fn from(s: &str) -> String {
        Self {
            length: s.len() as c_int,
            chars: s.as_ptr() as *const c_char,
            _owned: None,
        }
    }
}

impl From<std::string::String> for String {
    #[inline]
    fn from(s: std::string::String) -> String {
        let boxed: Box<[u8]> = Box::from(s.into_bytes());
        let ptr = boxed.as_ptr() as *const c_char;
        Self {
            length: boxed.len() as c_int,
            chars: ptr,
            _owned: Some(boxed),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StringArray {
    capacity: u32,
    length: u32,
    internal_array: &String,
}

#[repr(C)]
pub struct Arena<'a> {
    label: String,
    next_allocation: u64,
    capacity: u64,
    memory: *mut c_void,
    _marker: PhantomData<&'a c_void>,
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
    width: c_float,
    height: c_float,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vector2 {
    x: c_float,
    y: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color {
    r: c_float,
    g: c_float,
    b: c_float,
    a: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BoundingBox {
    x: c_float,
    y: c_float,
    width: c_float,
    height: c_float,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ElementId {
    id: u32,
    offset: u32,
    base_id: u32,
    string_id: String,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CornerRadius {
    top_left: c_float,
    top_right: c_float,
    bottom_left: c_float,
    bottom_right: c_float,
}

#[cfg(not(target_os = "windows"))]
type PackedEnum = u8;
#[cfg(target_os = "windows")]
type PackedEnum = u32;

#[packed_enum]
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
pub enum LayoutDirection {
    LeftToRight = 0,
    TopToBottom = 1,
}

#[packed_enum]
pub enum LayoutAlignmentX {
    Left = 0,
    Right = 1,
    Center = 2,
}

#[packed_enum]
pub enum LayoutAlignmentY {
    Top = 0,
    Bottom = 1,
    Center = 2,
}

#[packed_enum]
pub enum SizingType {
    Fit = 0,
    Grow = 1,
    Percent = 2,
    Fixed = 3,
}

#[link(name = "clay")]
extern "C" {
    fn Clay_MinMemorySize() -> u32;
    fn Clay_CreateArenaWithCapacityAndMemory(capacity: u32, offset: *const c_void) -> Arena;
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
        let memory = vec![0; size as usize];
        let arena = Arena::new(&memory);
        let dimensions = Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions);
        assert_eq!(arena.capacity, memory.len() as u64);
    }
}
