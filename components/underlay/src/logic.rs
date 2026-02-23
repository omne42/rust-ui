use leptos::prelude::*;
use ui_state_primitives::underlay::{
    UnderlayOpenState as PrimitiveUnderlayOpenState,
    UnderlayOpenStateInput as PrimitiveUnderlayOpenStateInput,
    UnderlayViewStateInput as PrimitiveUnderlayViewStateInput,
    resolve_open_state as resolve_primitive_open_state,
    resolve_view_state as resolve_primitive_view_state,
};

pub use ui_state_primitives::underlay::{
    UnderlayFlags, UnderlayFlagsInput, UnderlayOpenMode, UnderlayPartState, UnderlaySlot,
    UnderlayViewState, resolve_agent_contract, resolve_flags,
};
#[cfg(test)]
pub use ui_state_primitives::underlay::{UnderlayPartStateInput, resolve_state};

pub struct UnderlayOpenStateInput {
    pub is_open: Option<Signal<bool>>,
    pub open: Option<Signal<bool>>,
    pub default_open: Option<bool>,
    pub on_open_change: Option<Callback<bool>>,
}

#[derive(Clone)]
pub struct UnderlayOpenState {
    pub open: Option<Signal<bool>>,
    pub default_open: bool,
    pub on_open_change: Option<Callback<bool>>,
    pub mode: UnderlayOpenMode,
    pub has_default_open: bool,
    pub has_open_change_handler: bool,
    pub open_prop_source_attr: &'static str,
    pub open_mode_attr: &'static str,
    pub open_source_attr: &'static str,
    pub open_change_source_attr: &'static str,
    pub primitive: PrimitiveUnderlayOpenState,
}

pub fn normalize_open_state(input: UnderlayOpenStateInput) -> UnderlayOpenState {
    let open = input.is_open.or(input.open);
    let primitive = resolve_primitive_open_state(PrimitiveUnderlayOpenStateInput {
        has_is_open_prop: input.is_open.is_some(),
        has_open_prop: input.open.is_some(),
        default_open: input.default_open,
        has_on_open_change: input.on_open_change.is_some(),
    });

    UnderlayOpenState {
        open,
        default_open: primitive.default_open,
        on_open_change: input.on_open_change,
        mode: primitive.mode,
        has_default_open: primitive.has_default_open,
        has_open_change_handler: primitive.has_open_change_handler,
        open_prop_source_attr: primitive.open_prop_source_attr,
        open_mode_attr: primitive.open_mode_attr,
        open_source_attr: primitive.open_source_attr,
        open_change_source_attr: primitive.open_change_source_attr,
        primitive,
    }
}

pub fn normalize_flags(input: UnderlayFlagsInput) -> UnderlayFlags {
    resolve_flags(input)
}

#[derive(Clone)]
pub struct UnderlayViewStateInput {
    pub slot: UnderlaySlot,
    pub open: bool,
    pub transparent: bool,
    pub disabled: bool,
    pub has_on_close: bool,
    pub has_custom_class_name: bool,
    pub open_state: UnderlayOpenState,
    pub flags: UnderlayFlags,
}

pub fn resolve_view_state(input: UnderlayViewStateInput) -> UnderlayViewState {
    let flags = UnderlayFlags {
        transparent: input.transparent,
        disabled: input.disabled,
        ..input.flags
    };
    resolve_primitive_view_state(PrimitiveUnderlayViewStateInput {
        slot: input.slot,
        open: input.open,
        has_on_close: input.has_on_close,
        has_custom_class_name: input.has_custom_class_name,
        open_state: input.open_state.primitive,
        flags,
    })
}

pub fn normalize_optional_text(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.into())
    })
}

pub fn compose_class_name(base_class_name: Option<String>, state: UnderlayPartState) -> String {
    let mut classes = vec![state.base_class.into()];

    if state.is_open {
        classes.push("ui-underlay--open".to_string());
    }

    if state.is_transparent {
        classes.push("ui-underlay--transparent".to_string());
    }

    if state.is_disabled {
        classes.push("ui-underlay--disabled".to_string());
    }

    if state.is_interactive {
        classes.push("ui-underlay--interactive".to_string());
    }

    if state.has_custom_transparent {
        classes.push("ui-underlay--custom-transparent".to_string());
    }

    if state.has_custom_disabled {
        classes.push("ui-underlay--custom-disabled".to_string());
    }

    if state.has_custom_close_handler {
        classes.push("ui-underlay--custom-close".to_string());
    }

    if state.has_custom_class_name {
        classes.push("ui-underlay--custom-class".to_string());
        if let Some(base_class_name) = base_class_name {
            classes.push(base_class_name);
        }
    }

    classes.join(" ")
}

#[cfg(test)]
#[path = "../test/logic.rs"]
mod tests;
