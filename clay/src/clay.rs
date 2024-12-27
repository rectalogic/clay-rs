pub use crate::ui;
use crate::{data, external};
use clay_macros::packed_enum;
use std::{marker::PhantomData, os::raw::c_void};

#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}

#[repr(C)]
#[derive(Copy, Clone)]
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

pub(crate) trait Configure {
    fn configure(&self);
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
#[derive(Copy, Clone)]
pub struct RenderCommand<'a> {
    bounding_box: data::BoundingBox,
    config: ElementConfigUnion<'a>,
    text: data::String<'a>, // XXX fix
    id: u32,
    command_type: RenderCommandType,
}

pub type RenderCommandArray<'a> = ClayArray<'a, RenderCommand<'a>>;

struct ParentElement;

impl ParentElement {
    fn new() -> Self {
        unsafe {
            external::Clay__OpenElement();
        }
        Self
    }
}

impl Drop for ParentElement {
    fn drop(&mut self) {
        unsafe {
            external::Clay__CloseElement();
        }
    }
}

#[macro_export]
macro_rules! clay {
    ( ( $( $expression:expr ),+ ) $( $children:block )? ) => {
        {
            let _parent = ParentElement::new();
            $( $expression.configure(); )+
            unsafe { external::Clay__ElementPostConfiguration() ; }
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

    #[test]
    fn simple_ui() {
        extern "C" fn measure(text: &data::String, config: &ui::Text) -> data::Dimensions {
            data::Dimensions {
                width: (text.len() * 10) as f32,
                height: 18.,
            }
        }
        ui::set_measure_text_callback(measure);

        let size: u32 = Arena::min_memory_size();
        let memory = vec![0u8; size as usize];
        let arena = Arena::new(&memory);
        let dimensions = data::Dimensions::new(300.0, 300.0);
        arena.initialize(dimensions);
        let render_commands = Arena::render(|| {
            let color = data::Color {
                r: 240.,
                g: 189.,
                b: 100.,
                a: 255.,
            };
            const FONT_ID_BODY_24: u16 = 2;

            clay!((
                ui::IdI(data::String::from("HeroBlob"), 1),
                ui::Layout {
                    sizing: data::Sizing {
                        width: data::SizingAxis::grow(0.0, 480.0),
                        ..default()
                    },
                    padding: data::Padding { x: 16, y: 16 },
                    child_gap: 16,
                    child_alignment: data::ChildAlignment {
                        y: data::LayoutAlignmentY::Center,
                        ..default()
                    },
                    ..default()
                },
                ui::Border::outside_radius(2, color, 10.0)
            ) {
                clay!(
                    (
                        ui::Id(data::String::from("CheckImage")),
                        ui::Layout {
                            sizing: data::Sizing { width: data::SizingAxis::fixed(32.), ..default() },
                            ..default()
                        },
                        ui::Image { source_dimensions: data::Dimensions { width: 128., height: 128.}, ..default() } // XXX need extended sourceUrl
                    ) {
                        println!("children");
                    }
                );
                ui::Text {
                    font_size: 18,
                    font_id: FONT_ID_BODY_24,
                    text_color: color,
                    ..default()
                }.with(data::String::from("Some text here"));
            });
            // CLAY(CLAY_IDI("HeroBlob", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_GROW({ .max = 480 }) }, .padding = {16, 16}, .childGap = 16, .childAlignment = {.y = CLAY_ALIGN_Y_CENTER} }), CLAY_BORDER_OUTSIDE_RADIUS(2, color, 10)) {
            //     CLAY(CLAY_IDI("CheckImage", index), CLAY_LAYOUT({ .sizing = { CLAY_SIZING_FIXED(32) } }), CLAY_IMAGE({ .sourceDimensions = { 128, 128 }, .sourceURL = imageURL })) {}
            //     CLAY_TEXT(text, CLAY_TEXT_CONFIG({ .fontSize = fontSize, .fontId = FONT_ID_BODY_24, .textColor = color }));
            // }
        });
        println!("done");
    }
}
