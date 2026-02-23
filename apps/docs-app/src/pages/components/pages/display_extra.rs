use crate::pages::components::ComponentPage;
use crate::pages::components::pages::playground_workbench::{bool_word, rust_string_literal};
use crate::playground::Playground;
use leptos::prelude::*;
use ui::color_swatch_picker::ColorSwatchPickerMotion;
use ui::{
    Chart, ChartKind, ChartPoint, ColorSwatch, ColorSwatchPicker, ColorSwatchPickerItem,
    ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize, EmptyState, EmptyStateAlign,
    EmptyStateTone, ErrorView, ErrorViewMotion, ErrorViewTone, FlipCard, FlipCardMotion, Icon,
    IconSize, IconTone, Keyboard, KeyboardTone, LabeledValue, LabeledValueOrientation,
    LabeledValueTone, PressableFeedback, PressableFeedbackEffect, PressableFeedbackMotion,
    PressableFeedbackTone, RippleMotion, SegmentedControl, SegmentedControlSize, Skeleton,
    SkeletonGroup, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant,
    SkeletonVariant, Switch, TextAlign, TextElement, TextTone, TextWeight,
};
use ui_headless::A11yDirection;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ChartWorkbenchState {
    kind_index: usize,
    dataset_index: usize,
    is_disabled: bool,
    is_show_grid: bool,
    custom_class: bool,
    lang: bool,
}

impl Default for ChartWorkbenchState {
    fn default() -> Self {
        Self {
            kind_index: 0,
            dataset_index: 0,
            is_disabled: false,
            is_show_grid: true,
            custom_class: false,
            lang: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ChartWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 6 {
            return None;
        }

        let parse_index = |at: usize, max: usize| {
            parts
                .get(at)?
                .parse::<usize>()
                .ok()
                .map(|value| value.min(max))
        };
        let parse_bool = |at: usize| match *parts.get(at)? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        };

        Some(Self {
            kind_index: parse_index(0, 1)?,
            dataset_index: parse_index(1, 2)?,
            is_disabled: parse_bool(2)?,
            is_show_grid: parse_bool(3)?,
            custom_class: parse_bool(4)?,
            lang: parse_bool(5)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{},{}",
            self.kind_index,
            self.dataset_index,
            bool_digit(self.is_disabled),
            bool_digit(self.is_show_grid),
            bool_digit(self.custom_class),
            bool_digit(self.lang),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const CHART_WORKBENCH_STORAGE_KEY: &str = "docs:chart:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_chart_workbench_state() -> Option<ChartWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(CHART_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    ChartWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_chart_workbench_state() -> Option<ChartWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_chart_workbench_state(state: ChartWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(CHART_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_chart_workbench_state(_state: ChartWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_chart_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(CHART_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_chart_workbench_state() {}

#[path = "display_extra/chart.rs"]
mod chart;
#[path = "display_extra/color_swatch.rs"]
mod color_swatch;
#[path = "display_extra/color_swatch_picker.rs"]
mod color_swatch_picker;
#[path = "display_extra/empty_state.rs"]
mod empty_state;
#[path = "display_extra/error_view.rs"]
mod error_view;
#[path = "display_extra/flip_card.rs"]
mod flip_card;
#[path = "display_extra/icon.rs"]
mod icon;
#[path = "display_extra/keyboard.rs"]
mod keyboard;
#[path = "display_extra/labeled_value.rs"]
mod labeled_value;
#[path = "display_extra/pressable_feedback.rs"]
mod pressable_feedback;
#[path = "display_extra/skeleton_group.rs"]
mod skeleton_group;
#[path = "display_extra/text.rs"]
mod text;

pub(super) use chart::chart;
pub(super) use color_swatch::color_swatch;
pub(super) use color_swatch_picker::color_swatch_picker;
pub(super) use empty_state::empty_state;
pub(super) use error_view::error_view;
pub(super) use flip_card::flip_card;
pub(super) use icon::icon;
pub(super) use keyboard::keyboard;
pub(super) use labeled_value::labeled_value;
pub(super) use pressable_feedback::pressable_feedback;
pub(super) use skeleton_group::skeleton_group;
pub(super) use text::text;
