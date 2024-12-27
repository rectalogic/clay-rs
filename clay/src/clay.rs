use crate::{data, external, ui};
use clay_macros::packed_enum;
use std::{marker::PhantomData, os::raw::c_void};

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

#[repr(C)]
pub struct Arena<'a> {
    pub label: data::String<'a>,
    next_allocation: u64,
    capacity: u64,
    memory: *mut c_void,
    _lifetime_marker: PhantomData<&'a c_void>,
}

impl<'a> Arena<'a> {
    pub fn new(memory: &'a [u8]) -> Arena<'a> {
        unsafe {
            external::Clay_CreateArenaWithCapacityAndMemory(
                memory.len() as u32,
                memory.as_ptr() as *const c_void,
            )
        }
    }
    pub fn min_memory_size() -> u32 {
        unsafe { external::Clay_MinMemorySize() }
    }
    pub fn initialize(self, layout_dimensions: data::Dimensions) {
        unsafe { external::Clay_Initialize(self, layout_dimensions) }
    }
    pub fn render<F: FnOnce()>(ui: F) -> RenderCommandArray<'a> {
        unsafe {
            external::Clay_BeginLayout();
        }
        ui();
        unsafe { external::Clay_EndLayout() }
    }
}

#[packed_enum]
#[derive(Copy, Clone)]
pub(crate) enum ElementConfigType {
    Rectangle = 1,
    BorderContainer = 2,
    FloatingContainer = 4,
    ScrollContainer = 8,
    Image = 16,
    Text = 32,
    Custom = 64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) union ElementConfigUnion<'a> {
    pub(crate) rectangle_element_config: &'a ui::Rectangle,
    pub(crate) text_element_config: &'a ui::Text,
    pub(crate) image_element_config: &'a ui::Image,
    pub(crate) floating_element_config: &'a ui::Floating,
    pub(crate) custom_element_config: &'a ui::Custom,
    pub(crate) scroll_element_config: &'a ui::Scroll,
    pub(crate) border_element_config: &'a ui::Border,
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
    scroll_position: &'a data::Vector2, // XXX
    scroll_container_dimensions: data::Dimensions,
    content_dimensions: data::Dimensions,
    config: ui::Scroll,
    found: bool,
}

#[packed_enum]
#[derive(Debug, Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct RenderCommand<'a> {
    bounding_box: data::BoundingBox,
    config: ElementConfigUnion<'a>,
    text: data::String<'a>, // XXX fix
    id: u32,
    command_type: RenderCommandType,
}

pub type RenderCommandArray<'a> = ClayArray<'a, RenderCommand<'a>>;

pub mod internal {
    use super::external;

    pub struct ParentElement;

    impl ParentElement {
        pub fn new() -> Self {
            unsafe {
                external::Clay__OpenElement();
            }
            Self
        }
        pub fn post_configuration(&self) {
            unsafe {
                external::Clay__ElementPostConfiguration();
            }
        }
    }

    impl Drop for ParentElement {
        fn drop(&mut self) {
            unsafe {
                external::Clay__CloseElement();
            }
        }
    }

    pub trait Configure {
        fn configure(&self);
    }
}

#[macro_export]
macro_rules! clay {
    ( ( $( $expression:expr ),+ ) $( $children:block )? ) => {
        {
            let parent = $crate::internal::ParentElement::new();
            $( $crate::internal::Configure::configure(&$expression); )+
            parent.post_configuration();
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
        let dimensions = data::Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions);
    }

    #[test]
    fn initialize_arena_label() {
        let size: u32 = Arena::min_memory_size();
        let memory = vec![0u8; size as usize];
        let mut arena = Arena::new(&memory);
        arena.label = "Main Arena".into();
        assert_eq!(arena.capacity, memory.len() as u64);
        let dimensions = data::Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions);
        // XXX assert on label
    }
}
