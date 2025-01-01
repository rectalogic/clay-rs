use crate::{data, external, ui};
use clay_macros::packed_enum;
use std::{
    fmt,
    marker::PhantomData,
    os::raw::{c_float, c_int, c_void},
};

pub type MeasureTextCallback = extern "C" fn(&data::String, &ui::Text) -> data::Dimensions;
pub type QueryScrollOffsetCallback = extern "C" fn(u32) -> data::Vector2;

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

pub trait Renderer {
    fn get_layout_dimensions(&self) -> data::Dimensions;
    fn render(&self, render_commands: &mut RenderCommandIter<'_>);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_Arena
pub(crate) struct ArenaInternal {
    next_allocation: u64,
    capacity: u64,
    memory: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
pub struct Arena<'a> {
    memory: &'a [u8],
    internal: ArenaInternal,
    render_commands: RenderCommandIter<'a>,
}

impl<'a> Arena<'a> {
    // clay: Clay_CreateArenaWithCapacityAndMemory
    pub fn new(memory: &'a [u8]) -> Arena<'a> {
        Arena {
            memory,
            internal: unsafe {
                external::Clay_CreateArenaWithCapacityAndMemory(
                    memory.len() as u32,
                    memory.as_ptr() as *const c_void,
                )
            },
            render_commands: Default::default(),
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
    pub fn initialize(&self, layout_dimensions: data::Dimensions, error_handler: ErrorHandler) {
        unsafe { external::Clay_Initialize(self.internal, layout_dimensions, error_handler) }
    }
    // clay: Clay_SetMeasureTextFunction
    pub fn set_measure_text_callback(callback: MeasureTextCallback) {
        unsafe { external::Clay_SetMeasureTextFunction(callback) };
    }
    // clay: Clay_SetQueryScrollOffsetFunction
    pub fn set_query_scroll_offset_callback(
        query_scroll_offset_callback: QueryScrollOffsetCallback,
    ) {
        unsafe { external::Clay_SetQueryScrollOffsetFunction(query_scroll_offset_callback) };
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
    // clay: Clay_SetLayoutDimensions
    fn set_layout_dimensions(dimensions: data::Dimensions) {
        unsafe { external::Clay_SetLayoutDimensions(dimensions) };
    }

    // clay: Clay_BeginLayout/Clay_EndLayout
    pub fn render<'b, F: FnOnce(&ui::Builder)>(&'b mut self, renderer: &impl Renderer, ui: F) {
        Arena::set_layout_dimensions(renderer.get_layout_dimensions());
        unsafe { external::Clay_BeginLayout() };
        ui(&ui::Builder(()));
        self.render_commands = unsafe { external::Clay_EndLayout() }.into_iter();
        renderer.render(&mut self.render_commands);
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
enum RenderCommandType {
    None,
    Rectangle,
    Border,
    Text,
    Image,
    ScissorStart,
    ScissorEnd,
    Custom,
}

#[derive(Debug, Copy, Clone)]
pub enum RenderCommandElement {
    None,
    Rectangle(ui::Rectangle),
    Border(ui::Border),
    Text(ui::Text),
    Image(ui::Image),
    ScissorStart,
    ScissorEnd,
    Custom(ui::Custom),
}

#[repr(C)]
// clay: Clay_RenderCommand
pub struct RenderCommand<'a> {
    pub bounding_box: data::BoundingBox,
    config: ElementConfigUnion<'a>,
    pub text: data::String<'a>,
    pub id: u32,
    command_type: RenderCommandType,
}

impl RenderCommand<'_> {
    pub fn element(&self) -> RenderCommandElement {
        match self.command_type {
            RenderCommandType::Rectangle => {
                RenderCommandElement::Rectangle(unsafe { *self.config.rectangle_element_config })
            }
            RenderCommandType::Text => {
                RenderCommandElement::Text(unsafe { *self.config.text_element_config })
            }
            RenderCommandType::Border => {
                RenderCommandElement::Border(unsafe { *self.config.border_element_config })
            }
            RenderCommandType::Image => {
                RenderCommandElement::Image(unsafe { *self.config.image_element_config })
            }
            RenderCommandType::Custom => {
                RenderCommandElement::Custom(unsafe { *self.config.custom_element_config })
            }
            RenderCommandType::None => RenderCommandElement::None,
            RenderCommandType::ScissorStart => RenderCommandElement::ScissorStart,
            RenderCommandType::ScissorEnd => RenderCommandElement::ScissorEnd,
        }
    }
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
// clay: Clay_RenderCommandArray
pub(crate) struct RenderCommandArray<'a> {
    capacity: u32,
    length: u32,
    internal_array: *const RenderCommand<'a>,
    _lifetime_marker: PhantomData<&'a RenderCommand<'a>>,
}

impl Default for RenderCommandArray<'_> {
    fn default() -> Self {
        Self {
            capacity: 0,
            length: 0,
            internal_array: std::ptr::null(),
            _lifetime_marker: PhantomData,
        }
    }
}

#[derive(Default)]
pub struct RenderCommandIter<'a> {
    array: RenderCommandArray<'a>,
    index: i32,
}

impl fmt::Debug for RenderCommandIter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.array).finish()
    }
}

impl<'a> Iterator for RenderCommandIter<'a> {
    type Item = &'a RenderCommand<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.array.length as i32 {
            None
        } else {
            let item = unsafe { external::Clay_RenderCommandArray_Get(&self.array, self.index) };
            self.index += 1;
            Some(item)
        }
    }
}

impl<'a> IntoIterator for RenderCommandArray<'a> {
    type Item = &'a RenderCommand<'a>;
    type IntoIter = RenderCommandIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RenderCommandIter {
            array: self,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_arena() {
        let memory = [0u8; 1024];
        let arena = Arena::new(&memory);
        assert_eq!(arena.internal.capacity, memory.len() as u64);
    }

    #[test]
    fn initialize_arena() {
        let size: u32 = Arena::min_memory_size();
        let memory = vec![0u8; size as usize];
        let arena = Arena::new(&memory);
        assert_eq!(arena.internal.capacity, memory.len() as u64);
        let dimensions = data::Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions, data::default());
    }
}
