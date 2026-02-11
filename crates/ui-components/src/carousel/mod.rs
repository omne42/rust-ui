mod logic;
pub mod styles;
mod view;

pub use crate::active_highlight::ActiveHighlightMotion as CarouselMotion;
pub use logic::{
    DEFAULT_ARIA_LABEL, DEFAULT_ID_BASE, DEFAULT_LOOP_NAVIGATION, DEFAULT_ORIENTATION,
};
pub use view::Carousel;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CarouselOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl CarouselOrientation {
    pub fn class_name(self) -> &'static str {
        match self {
            Self::Horizontal => "ui-carousel--horizontal",
            Self::Vertical => "ui-carousel--vertical",
        }
    }

    pub fn attr(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    pub fn prev_key(self) -> &'static str {
        match self {
            Self::Horizontal => "ArrowLeft",
            Self::Vertical => "ArrowUp",
        }
    }

    pub fn next_key(self) -> &'static str {
        match self {
            Self::Horizontal => "ArrowRight",
            Self::Vertical => "ArrowDown",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

impl CarouselItem {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            disabled: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CarouselItemResolved {
    pub id: String,
    pub slide_dom_id: String,
    pub dot_dom_id: String,
    pub title: String,
    pub description: Option<String>,
    pub disabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarouselSlot {
    Root,
    Viewport,
    Item,
    Controls,
    PrevButton,
    NextButton,
    Indicators,
    Indicator,
    IndicatorDot,
    IndicatorHighlight,
    Title,
    Description,
}

impl CarouselSlot {
    pub fn as_attr(self) -> &'static str {
        match self {
            CarouselSlot::Root => "carousel",
            CarouselSlot::Viewport => "carousel-viewport",
            CarouselSlot::Item => "carousel-item",
            CarouselSlot::Controls => "carousel-controls",
            CarouselSlot::PrevButton => "carousel-prev",
            CarouselSlot::NextButton => "carousel-next",
            CarouselSlot::Indicators => "carousel-indicators",
            CarouselSlot::Indicator => "carousel-indicator",
            CarouselSlot::IndicatorDot => "carousel-indicator-dot",
            CarouselSlot::IndicatorHighlight => "carousel-indicator-highlight",
            CarouselSlot::Title => "carousel-title",
            CarouselSlot::Description => "carousel-description",
        }
    }

    pub fn base_class(self) -> &'static str {
        match self {
            CarouselSlot::Root => "ui-carousel",
            CarouselSlot::Viewport => "ui-carousel__viewport",
            CarouselSlot::Item => "ui-carousel__slide",
            CarouselSlot::Controls => "ui-carousel__controls",
            CarouselSlot::PrevButton => "ui-carousel__button ui-carousel__button--prev",
            CarouselSlot::NextButton => "ui-carousel__button ui-carousel__button--next",
            CarouselSlot::Indicators => "ui-carousel__indicators",
            CarouselSlot::Indicator => "ui-carousel__indicator",
            CarouselSlot::IndicatorDot => "ui-carousel__indicator-dot",
            CarouselSlot::IndicatorHighlight => "ui-active-highlight",
            CarouselSlot::Title => "ui-carousel__title",
            CarouselSlot::Description => "ui-carousel__description",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselPartStateInput {
    pub slot: CarouselSlot,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub has_disabled_items: bool,
    pub orientation: CarouselOrientation,
    pub loop_navigation: bool,
    pub is_controlled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_orientation: bool,
    pub has_custom_loop_navigation: bool,
    pub has_custom_selected_index: bool,
    pub has_custom_default_selected_index: bool,
    pub has_custom_on_selected_index_change: bool,
    pub has_custom_motion: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CarouselPartState {
    pub slot: CarouselSlot,
    pub slot_attr: &'static str,
    pub base_class: &'static str,
    pub state_attr: &'static str,
    pub item_attr: &'static str,
    pub selected_attr: &'static str,
    pub focus_attr: &'static str,
    pub orientation: CarouselOrientation,
    pub orientation_attr: &'static str,
    pub navigation_attr: &'static str,
    pub selection_mode_attr: &'static str,
    pub loop_attr: Option<&'static str>,
    pub bounded_attr: Option<&'static str>,
    pub item_count: usize,
    pub selected_index: Option<usize>,
    pub focused_index: Option<usize>,
    pub is_empty: bool,
    pub has_items: bool,
    pub has_selection: bool,
    pub has_focus: bool,
    pub has_disabled_items: bool,
    pub loop_navigation: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_id_base: bool,
    pub has_custom_aria_label: bool,
    pub has_custom_class_name: bool,
    pub has_custom_orientation: bool,
    pub has_custom_loop_navigation: bool,
    pub has_custom_selected_index: bool,
    pub has_custom_default_selected_index: bool,
    pub has_custom_on_selected_index_change: bool,
    pub has_custom_motion: bool,
    pub id_source_attr: &'static str,
    pub aria_label_source_attr: &'static str,
    pub class_source_attr: &'static str,
    pub orientation_source_attr: &'static str,
    pub loop_navigation_source_attr: &'static str,
    pub selected_index_source_attr: &'static str,
    pub default_selected_index_source_attr: &'static str,
    pub selected_index_change_source_attr: &'static str,
    pub motion_source_attr: &'static str,
}
