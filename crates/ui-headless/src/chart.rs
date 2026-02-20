use crate::a11y::{A11yDirection, region_attrs};
use ui_state_primitives::chart::{ChartState, next_index_for_key};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartAttrs {
    pub role: &'static str,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
    pub data_kind: &'static str,
    pub data_state: &'static str,
    pub data_empty: Option<&'static str>,
    pub data_has_points: Option<&'static str>,
    pub data_disabled: Option<&'static str>,
    pub data_enabled: Option<&'static str>,
    pub data_show_grid: Option<&'static str>,
    pub data_controlled: Option<&'static str>,
    pub data_uncontrolled: Option<&'static str>,
    pub data_class_source: &'static str,
    pub data_custom_class: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartKeyAction {
    Noop,
    MoveTo(usize),
    Activate(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ChartHandlers;

impl ChartHandlers {
    pub fn on_key_down(
        self,
        key: &str,
        current_index: usize,
        point_count: usize,
        disabled: bool,
    ) -> ChartKeyAction {
        if disabled {
            return ChartKeyAction::Noop;
        }

        if key == "Enter" || key == " " {
            return ChartKeyAction::Activate(current_index);
        }

        if let Some(next) = next_index_for_key(key, current_index, point_count) {
            return ChartKeyAction::MoveTo(next);
        }

        ChartKeyAction::Noop
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartSemanticState {
    pub kind: &'static str,
    pub state: &'static str,
    pub active_index: usize,
    pub point_count: usize,
    pub has_points: bool,
    pub is_empty: bool,
    pub disabled: bool,
    pub enabled: bool,
    pub show_grid: bool,
    pub is_controlled: bool,
    pub is_uncontrolled: bool,
    pub has_custom_class_name: bool,
    pub class_source: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartContract {
    pub attrs: ChartAttrs,
    pub handlers: ChartHandlers,
    pub state: ChartSemanticState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChartOptions {
    pub state: ChartState,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
}

pub fn use_chart(options: ChartOptions) -> ChartContract {
    let region = region_attrs(options.aria_label, options.lang, options.dir);

    ChartContract {
        attrs: ChartAttrs {
            role: region.role,
            aria_label: region.aria_label,
            lang: region.lang,
            dir: region.dir,
            data_kind: options.state.kind_attr,
            data_state: options.state.state_attr,
            data_empty: options.state.is_empty.then_some("true"),
            data_has_points: options.state.has_points.then_some("true"),
            data_disabled: options.state.disabled.then_some("true"),
            data_enabled: options.state.enabled.then_some("true"),
            data_show_grid: options.state.show_grid.then_some("true"),
            data_controlled: options.state.is_controlled.then_some("true"),
            data_uncontrolled: options.state.is_uncontrolled.then_some("true"),
            data_class_source: options.state.class_source_attr,
            data_custom_class: options.state.has_custom_class_name.then_some("true"),
        },
        handlers: ChartHandlers,
        state: ChartSemanticState {
            kind: options.state.kind_attr,
            state: options.state.state_attr,
            active_index: options.state.active_index,
            point_count: options.state.point_count,
            has_points: options.state.has_points,
            is_empty: options.state.is_empty,
            disabled: options.state.disabled,
            enabled: options.state.enabled,
            show_grid: options.state.show_grid,
            is_controlled: options.state.is_controlled,
            is_uncontrolled: options.state.is_uncontrolled,
            has_custom_class_name: options.state.has_custom_class_name,
            class_source: options.state.class_source_attr,
        },
    }
}

#[cfg(test)]
#[path = "test/chart.rs"]
mod tests;
