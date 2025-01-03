use crate::data;
use crate::system::{
    ArenaInternal, ElementConfigType, ElementConfigUnion, ErrorData, ErrorHandler,
    MeasureTextCallback, QueryScrollOffsetCallback, RenderCommand, RenderCommandArray,
};
use crate::ui;
use std::os::raw::{c_float, c_void};

pub(crate) type OnHoverCallback = unsafe extern "C" fn(data::ElementId, data::PointerData, isize);

#[link(name = "clay")]
extern "C" {
    pub(crate) fn Clay_MinMemorySize() -> u32;
    pub(crate) fn Clay_CreateArenaWithCapacityAndMemory(
        capacity: u32,
        offset: *const c_void,
    ) -> ArenaInternal;
    pub(crate) fn Clay_Initialize(
        arena: ArenaInternal,
        layout_dimensions: data::Dimensions,
        error_handler: ErrorHandler,
    );
    pub(crate) fn Clay_SetPointerState(position: data::Vector2, pointer_down: bool);
    pub(crate) fn Clay_UpdateScrollContainers(
        enable_drag_scrolling: bool,
        scroll_delta: data::Vector2,
        delta_time: c_float,
    );
    pub(crate) fn Clay_SetLayoutDimensions(dimensions: data::Dimensions);

    pub(crate) fn Clay_Hovered() -> bool;
    pub(crate) fn Clay_OnHover(on_hover_callback: OnHoverCallback, user_data: isize);
    pub(crate) fn Clay_PointerOver(element_id: data::ElementId) -> bool;
    pub(crate) fn Clay_GetScrollContainerData(id: data::ElementId) -> data::ScrollContainerData;
    pub(crate) fn Clay_SetQueryScrollOffsetFunction(
        query_scroll_offset_callback: QueryScrollOffsetCallback,
    );
    pub(crate) fn Clay_SetCullingEnabled(enabled: bool);
    pub(crate) fn Clay_SetMaxElementCount(max_element_count: u32);
    pub(crate) fn Clay_SetMaxMeasureTextCacheWordCount(max_measure_text_cache_word_count: u32);

    pub(crate) fn Clay__ErrorHandlerFunctionDefault(error_text: ErrorData);
    pub(crate) fn Clay_SetMeasureTextFunction(measure: MeasureTextCallback);
    pub(crate) fn Clay_BeginLayout();
    pub(crate) fn Clay_EndLayout<'a>() -> RenderCommandArray<'a>;
    pub(crate) fn Clay__OpenElement();
    pub(crate) fn Clay__OpenTextElement<'a>(text: data::String, config: &'a ui::Text);
    pub(crate) fn Clay__CloseElement();
    pub(crate) fn Clay__StoreLayoutConfig<'a>(config: ui::Layout) -> &'a ui::Layout;
    pub(crate) fn Clay__ElementPostConfiguration();
    pub(crate) fn Clay__AttachId(id: data::ElementId);
    pub(crate) fn Clay__AttachLayoutConfig<'a>(config: &'a ui::Layout);
    pub(crate) fn Clay__AttachElementConfig(config: ElementConfigUnion, r#type: ElementConfigType);
    pub(crate) fn Clay__StoreRectangleElementConfig<'a>(config: ui::Rectangle)
        -> &'a ui::Rectangle;
    pub(crate) fn Clay__StoreTextElementConfig<'a>(config: ui::Text) -> &'a ui::Text;
    pub(crate) fn Clay__StoreImageElementConfig<'a>(config: ui::Image) -> &'a ui::Image;
    pub(crate) fn Clay__StoreFloatingElementConfig<'a>(config: ui::Floating) -> &'a ui::Floating;
    pub(crate) fn Clay__StoreCustomElementConfig<'a>(config: ui::Custom) -> &'a ui::Custom;
    pub(crate) fn Clay__StoreScrollElementConfig<'a>(config: ui::Scroll) -> &'a ui::Scroll;
    pub(crate) fn Clay__StoreBorderElementConfig<'a>(config: ui::Border) -> &'a ui::Border;
    pub(crate) fn Clay__HashString<'a>(
        key: data::String,
        offset: u32,
        seed: u32,
    ) -> data::ElementId<'a>;
    pub(crate) fn Clay_GetElementId<'a>(id: data::String) -> data::ElementId<'a>;
    pub(crate) fn Clay_RenderCommandArray_Get<'a>(
        array: &RenderCommandArray<'a>,
        index: i32,
    ) -> &'a RenderCommand<'a>;
}
