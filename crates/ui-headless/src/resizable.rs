use crate::a11y::{A11yDirection, locale_attrs};
use leptos::prelude::*;
use ui_state_primitives::resizable::{
    ResizableOrientation, ResizableState, ResizableStateInput, SplitBounds, clamp_split,
    resolve_state, split_from_drag, split_step_for_key,
};

#[derive(Clone)]
pub struct ResizableOptions {
    pub orientation: ResizableOrientation,
    pub split_percent: Signal<f64>,
    pub bounds: SplitBounds,
    pub is_disabled: bool,
    pub is_controlled: bool,
    pub with_handle: bool,
    pub has_custom_class_name: bool,
    pub aria_label: String,
    pub lang: Option<String>,
    pub dir: Option<A11yDirection>,
    pub on_split_percent_change: Callback<f64>,
}

#[derive(Clone)]
pub struct ResizableRootAttrs {
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct ResizableHandleAttrs {
    pub role: &'static str,
    pub tabindex: i32,
    pub aria_label: String,
    pub aria_orientation: Memo<&'static str>,
    pub aria_valuemin: Memo<String>,
    pub aria_valuemax: Memo<String>,
    pub aria_valuenow: Memo<String>,
    pub aria_disabled: Memo<Option<&'static str>>,
    pub lang: Option<String>,
    pub dir: Option<&'static str>,
}

#[derive(Clone)]
pub struct ResizableHandlers {
    pub on_handle_pointer_down: Callback<(f64, f64), bool>,
    pub on_pointer_move: Callback<(f64, f64, f64, f64)>,
    pub on_pointer_up: Callback<()>,
    pub on_handle_key_down: Callback<(String, bool), bool>,
}

#[derive(Clone)]
pub struct ResizableContractState {
    pub is_dragging: ReadSignal<bool>,
    pub resolved: Memo<ResizableState>,
}

#[derive(Clone)]
pub struct ResizableAria {
    pub attrs: ResizableRootAttrs,
    pub handle_attrs: ResizableHandleAttrs,
    pub handlers: ResizableHandlers,
    pub state: ResizableContractState,
}

fn position_for_orientation(orientation: ResizableOrientation, x: f64, y: f64) -> f64 {
    match orientation {
        ResizableOrientation::Horizontal => x,
        ResizableOrientation::Vertical => y,
    }
}

fn extent_for_orientation(
    orientation: ResizableOrientation,
    container_width: f64,
    container_height: f64,
) -> f64 {
    match orientation {
        ResizableOrientation::Horizontal => container_width,
        ResizableOrientation::Vertical => container_height,
    }
}

pub fn use_resizable(options: ResizableOptions) -> ResizableAria {
    let ResizableOptions {
        orientation,
        split_percent,
        bounds,
        is_disabled,
        is_controlled,
        with_handle,
        has_custom_class_name,
        aria_label,
        lang,
        dir,
        on_split_percent_change,
    } = options;
    let locale = locale_attrs(lang, dir);

    let (is_dragging, set_dragging) = signal(false);
    let (drag_start_position, set_drag_start_position) = signal(0.0_f64);
    let (drag_start_split_percent, set_drag_start_split_percent) =
        signal(split_percent.get_untracked());

    let resolved = Memo::new(move |_| {
        resolve_state(ResizableStateInput {
            orientation,
            split_percent: split_percent.get(),
            bounds,
            disabled: is_disabled,
            dragging: is_dragging.get(),
            is_controlled,
            with_handle,
            has_custom_class_name,
        })
    });

    let on_handle_pointer_down = Callback::new(move |(x, y): (f64, f64)| -> bool {
        if is_disabled {
            return false;
        }

        set_dragging.set(true);
        set_drag_start_position.set(position_for_orientation(orientation, x, y));
        set_drag_start_split_percent.set(resolved.get_untracked().split_percent);
        true
    });

    let on_pointer_move = Callback::new(move |(x, y, width, height): (f64, f64, f64, f64)| {
        if is_disabled || !is_dragging.get_untracked() {
            return;
        }

        let next = split_from_drag(
            drag_start_split_percent.get_untracked(),
            drag_start_position.get_untracked(),
            position_for_orientation(orientation, x, y),
            extent_for_orientation(orientation, width, height),
            bounds,
        );
        on_split_percent_change.run(next);
    });

    let on_pointer_up = Callback::new(move |_| {
        if is_dragging.get_untracked() {
            set_dragging.set(false);
        }
    });

    let on_handle_key_down = Callback::new(move |(key, accelerated): (String, bool)| -> bool {
        if is_disabled {
            return false;
        }

        let Some(delta) = split_step_for_key(&key, orientation, accelerated) else {
            return false;
        };

        let next = clamp_split(resolved.get_untracked().split_percent + delta, bounds);
        on_split_percent_change.run(next);
        true
    });

    ResizableAria {
        attrs: ResizableRootAttrs {
            lang: locale.lang.clone(),
            dir: locale.dir,
        },
        handle_attrs: ResizableHandleAttrs {
            role: "separator",
            tabindex: if is_disabled { -1 } else { 0 },
            aria_label,
            aria_orientation: Memo::new(move |_| resolved.get().orientation_attr),
            aria_valuemin: Memo::new(move |_| format!("{:.2}", resolved.get().min_split_percent)),
            aria_valuemax: Memo::new(move |_| format!("{:.2}", resolved.get().max_split_percent)),
            aria_valuenow: Memo::new(move |_| format!("{:.2}", resolved.get().split_percent)),
            aria_disabled: Memo::new(move |_| resolved.get().disabled.then_some("true")),
            lang: locale.lang,
            dir: locale.dir,
        },
        handlers: ResizableHandlers {
            on_handle_pointer_down,
            on_pointer_move,
            on_pointer_up,
            on_handle_key_down,
        },
        state: ResizableContractState {
            is_dragging,
            resolved,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resizable_contract_exposes_locale_and_separator_attrs() {
        let (split, _set_split) = signal(48.0_f64);
        let contract = use_resizable(ResizableOptions {
            orientation: ResizableOrientation::Horizontal,
            split_percent: split.into(),
            bounds: SplitBounds {
                min: 20.0,
                max: 80.0,
            },
            is_disabled: false,
            is_controlled: false,
            with_handle: true,
            has_custom_class_name: false,
            aria_label: "Pane splitter".to_string(),
            lang: Some("  zh-CN ".to_string()),
            dir: Some(A11yDirection::Rtl),
            on_split_percent_change: Callback::new(|_| {}),
        });

        assert_eq!(contract.handle_attrs.role, "separator");
        assert_eq!(contract.handle_attrs.tabindex, 0);
        assert_eq!(contract.handle_attrs.aria_label, "Pane splitter");
        assert_eq!(
            contract.handle_attrs.aria_orientation.get_untracked(),
            "horizontal"
        );
        assert_eq!(contract.attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.attrs.dir, Some("rtl"));
        assert_eq!(contract.handle_attrs.lang.as_deref(), Some("zh-CN"));
        assert_eq!(contract.handle_attrs.dir, Some("rtl"));
    }

    #[test]
    fn pointer_and_keyboard_handlers_update_split_value() {
        let (split, set_split) = signal(50.0_f64);
        let contract = use_resizable(ResizableOptions {
            orientation: ResizableOrientation::Horizontal,
            split_percent: split.into(),
            bounds: SplitBounds {
                min: 25.0,
                max: 75.0,
            },
            is_disabled: false,
            is_controlled: true,
            with_handle: true,
            has_custom_class_name: false,
            aria_label: "Splitter".to_string(),
            lang: None,
            dir: None,
            on_split_percent_change: Callback::new(move |next| set_split.set(next)),
        });

        assert!(contract.handlers.on_handle_pointer_down.run((100.0, 0.0)));
        assert!(contract.state.is_dragging.get_untracked());

        contract
            .handlers
            .on_pointer_move
            .run((200.0, 0.0, 200.0, 100.0));
        assert_eq!(split.get_untracked(), 75.0);

        assert!(
            contract
                .handlers
                .on_handle_key_down
                .run(("ArrowLeft".to_string(), false))
        );
        assert_eq!(split.get_untracked(), 73.0);

        contract.handlers.on_pointer_up.run(());
        assert!(!contract.state.is_dragging.get_untracked());
    }

    #[test]
    fn disabled_contract_is_noop_for_interaction_handlers() {
        let (split, set_split) = signal(44.0_f64);
        let contract = use_resizable(ResizableOptions {
            orientation: ResizableOrientation::Vertical,
            split_percent: split.into(),
            bounds: SplitBounds {
                min: 10.0,
                max: 90.0,
            },
            is_disabled: true,
            is_controlled: false,
            with_handle: false,
            has_custom_class_name: false,
            aria_label: "Splitter".to_string(),
            lang: None,
            dir: None,
            on_split_percent_change: Callback::new(move |next| set_split.set(next)),
        });

        assert!(!contract.handlers.on_handle_pointer_down.run((10.0, 10.0)));
        contract
            .handlers
            .on_pointer_move
            .run((10.0, 60.0, 100.0, 100.0));
        assert!(
            !contract
                .handlers
                .on_handle_key_down
                .run(("ArrowDown".to_string(), false))
        );
        assert_eq!(split.get_untracked(), 44.0);
        assert_eq!(contract.handle_attrs.tabindex, -1);
        assert_eq!(
            contract.handle_attrs.aria_disabled.get_untracked(),
            Some("true")
        );
    }
}
