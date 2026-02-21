pub use crate::CarouselMotion;
pub use crate::{
    CarouselItem, CarouselItemResolved, CarouselOrientation, CarouselPartState,
    CarouselPartStateInput, CarouselSlot,
};
pub use ui_headless::A11yDirection;

pub const DEFAULT_ID_BASE: &str;
pub const DEFAULT_ARIA_LABEL: &str;
pub const DEFAULT_CONTROLS_ARIA_LABEL: &str;
pub const DEFAULT_INDICATORS_ARIA_LABEL: &str;
pub const DEFAULT_PREVIOUS_LABEL: &str;
pub const DEFAULT_NEXT_LABEL: &str;
pub const DEFAULT_INDICATOR_ARIA_LABEL_TEMPLATE: &str;
pub const DEFAULT_ORIENTATION: crate::CarouselOrientation;
pub const DEFAULT_LOOP_NAVIGATION: bool;

pub fn sanitize_motion(motion: crate::CarouselMotion) -> crate::CarouselMotion;

pub fn attach_carousel_indicator_motion(
    container_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    highlight_ref: leptos::prelude::NodeRef<leptos::html::Div>,
    active_index: leptos::prelude::ReadSignal<usize>,
    option_id: leptos::prelude::Callback<usize, String>,
    motion: crate::CarouselMotion,
);

pub fn Carousel(
    id_base: String,
    items: Vec<crate::CarouselItem>,
    selected_index: Option<leptos::prelude::Signal<Option<usize>>>,
    default_selected_index: Option<usize>,
    on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>,
    orientation: crate::CarouselOrientation,
    is_loop_navigation: bool,
    motion: crate::CarouselMotion,
    aria_label: Option<String>,
    controls_aria_label: Option<String>,
    indicators_aria_label: Option<String>,
    previous_label: Option<String>,
    next_label: Option<String>,
    indicator_aria_label_template: Option<String>,
    lang: Option<String>,
    dir: Option<ui_headless::A11yDirection>,
    class_name: Option<String>,
) -> impl leptos::prelude::IntoView;
