use crate::{
    data,
    external::{self, Clay_RenderCommandArray_Get},
    ui, ClayArray, ClayArrayIter,
};
use clay_macros::packed_enum;
use std::{
    fmt,
    marker::PhantomData,
    os::raw::{c_float, c_int, c_void},
};

pub type MeasureTextCallback = extern "C" fn(&data::String, &ui::Text) -> data::Dimensions;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_ErrorType
pub enum ErrorType {
    TextMeasurementFunctionNotProvided,
    ArenaCapacityExceeded,
    ElementsCapacityExceeded,
    TextMeasurementCapacityExceeded,
    DuplicateId,
    FloatingContainerParentNotFound,
    InternalError,
}

pub type ErrorHandlerCallback<'a> = extern "C" fn(ErrorData<'a>);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_ErrorData
pub struct ErrorData<'a> {
    error_type: ErrorType,
    error_text: data::String<'a>,
    user_data: *const c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_ErrorHandler
pub struct ErrorHandler<'a> {
    error_handler_callback: ErrorHandlerCallback<'a>,
    user_data: *const c_int,
}

extern "C" fn default_error_handler(error_data: ErrorData<'_>) {
    unsafe {
        external::Clay__ErrorHandlerFunctionDefault(error_data);
    }
}

impl Default for ErrorHandler<'_> {
    fn default() -> Self {
        Self {
            error_handler_callback: default_error_handler,
            user_data: std::ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_Arena
pub struct Arena<'a> {
    next_allocation: u64,
    capacity: u64,
    memory: *mut c_void,
    _lifetime_marker: PhantomData<&'a c_void>,
}

impl<'a> Arena<'a> {
    // clay: Clay_CreateArenaWithCapacityAndMemory
    pub fn new(memory: &'a [u8]) -> Arena<'a> {
        unsafe {
            external::Clay_CreateArenaWithCapacityAndMemory(
                memory.len() as u32,
                memory.as_ptr() as *const c_void,
            )
        }
    }
    // clay: Clay_MinMemorySize
    pub fn min_memory_size() -> u32 {
        unsafe { external::Clay_MinMemorySize() }
    }
    // clay: Clay_SetMaxElementCount
    pub fn set_max_element_count(max_element_count: u32) {
        unsafe { external::Clay_SetMaxElementCount(max_element_count) };
    }
    // clay: Clay_SetMaxMeasureTextCacheWordCount
    pub fn set_max_measure_text_cache_word_count(max_measure_text_cache_word_count: u32) {
        unsafe {
            external::Clay_SetMaxMeasureTextCacheWordCount(max_measure_text_cache_word_count)
        };
    }
    // clay: Clay_Initialize
    pub fn initialize(self, layout_dimensions: data::Dimensions, error_handler: ErrorHandler) {
        unsafe { external::Clay_Initialize(self, layout_dimensions, error_handler) }
    }
    // clay: Clay_SetMeasureTextFunction
    pub fn set_measure_text_callback(&self, callback: MeasureTextCallback) {
        unsafe { external::Clay_SetMeasureTextFunction(callback) };
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

    // clay: Clay_BeginLayout/Clay_EndLayout
    pub fn render<F: FnOnce()>(&self, ui: F) -> RenderCommandArray<'a> {
        unsafe { external::Clay_BeginLayout() };
        ui();
        unsafe { external::Clay_EndLayout() }
    }
}

#[packed_enum]
#[derive(Copy, Clone)]
// clay: Clay__ElementConfigType
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
// clay: Clay_ElementConfigUnion
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
#[derive(Debug, Copy, Clone)]
// clay: Clay_RenderCommandType
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
// clay: Clay_RenderCommand
pub struct RenderCommand<'a> {
    bounding_box: data::BoundingBox,
    config: ElementConfigUnion<'a>,
    text: data::String<'a>,
    id: u32,
    command_type: RenderCommandType,
}

impl fmt::Debug for RenderCommand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RenderCommand")
            .field("bounding_box", &self.bounding_box)
            .field(
                "config",
                match self.command_type {
                    RenderCommandType::Rectangle => unsafe {
                        &self.config.rectangle_element_config
                    },
                    RenderCommandType::Text => unsafe { &self.config.text_element_config },
                    RenderCommandType::Border => unsafe { &self.config.border_element_config },
                    RenderCommandType::Image => unsafe { &self.config.image_element_config },
                    RenderCommandType::Custom => unsafe { &self.config.custom_element_config },
                    RenderCommandType::None => &"None",
                    RenderCommandType::ScissorStart => &"ScissorStart",
                    RenderCommandType::ScissorEnd => &"ScissorEnd",
                },
            )
            .field("text", &self.text)
            .field("id", &self.id)
            .field("command_type", &self.command_type)
            .finish()
    }
}

// clay: Clay_RenderCommandArray
pub type RenderCommandArray<'a> = ClayArray<'a, RenderCommand<'a>>;
pub type RenderCommandIter<'a> = ClayArrayIter<'a, RenderCommand<'a>>;
impl<'a> IntoIterator for RenderCommandArray<'a> {
    type Item = &'a RenderCommand<'a>;
    type IntoIter = RenderCommandIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RenderCommandIter {
            array: self,
            index: 0,
            getter: Clay_RenderCommandArray_Get,
        }
    }
}

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

// clay: CLAY macro
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
        arena.initialize(dimensions, data::default());
    }
}
