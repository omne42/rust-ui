use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    if path.exists() {
        return fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
    }

    if let Some(component_path) = rel_path.strip_prefix("src/") {
        let mut parts = component_path.splitn(2, '/');
        let component = parts.next().unwrap_or_default();
        let Some(suffix) = parts.next() else {
            return fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"));
        };

        let component_dir = component.replace('_', "-");
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        let migrated = workspace_dir.join(format!("components/{component_dir}/src/{suffix}"));

        if migrated.exists() {
            return fs::read_to_string(&migrated)
                .unwrap_or_else(|e| panic!("read_to_string failed for {migrated:?}: {e}"));
        }
    }

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn pressable_feedback_keeps_internal_modules_private_and_exports_stable_api() {
    let source = load_source("../../components/pressable-feedback/src/mod.rs");

    for needle in ["pub mod logic", "pub mod view", "pub mod motion"] {
        assert!(
            !source.contains(needle),
            "PressableFeedback internals should stay private; found `{needle}`."
        );
    }

    for stable_export in [
        "pub use logic::{DEFAULT_ARIA_LABEL, PressableFeedbackEffect, PressableFeedbackTone};",
        "pub use motion::PressableFeedbackMotion;",
        "pub use view::PressableFeedback;",
    ] {
        assert!(
            source.contains(stable_export),
            "PressableFeedback should expose a stable ui-layer API; missing `{stable_export}`."
        );
    }
}

#[test]
fn pressable_feedback_consumes_state_primitives() {
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");

    for needle in [
        "pub use ui_state_primitives::pressable_feedback::{",
        "DEFAULT_IS_BOUNDED",
        "DEFAULT_IS_DISABLED",
        "normalize_flags",
        "PressableFeedbackStateContractInput",
        "PressableFeedbackStateContract",
        "normalize_state_contract",
        "PressableFeedbackTone",
        "PressableFeedbackEffect",
        "PressableFeedbackStateInput",
        "PressableFeedbackPressedAxis",
        "PressableFeedbackPressedMode",
        "PressableFeedbackDefaultPressedSource",
        "PressableFeedbackPressedChangeSource",
        "normalize_pressed_axis(",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "PressableFeedback logic should bridge state primitives from `ui-state-primitives`; missing `{needle}`."
        );
    }

    for needle in [
        "pub enum PressableFeedbackTone",
        "pub enum PressableFeedbackEffect",
        "pub struct PressableFeedbackStateInput",
        "pub struct PressableFeedbackState",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "PressableFeedback state primitive should define `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "logic::normalize_state_contract(logic::PressableFeedbackStateContractInput {",
        "logic::normalize_pressed_axis(is_pressed, default_pressed, on_pressed_change)",
        "logic::resolve_state(PressableFeedbackStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "use_controllable_state(",
        "pressed_axis.value",
        "Some(pressed_axis.default_value)",
        "pressed_axis.on_value_change",
        "pressed_axis.pressed_mode.as_attr()",
        "pressed_axis.default_pressed_source.as_attr()",
        "pressed_axis.pressed_change_source.as_attr()",
        "use_pressable_feedback_a11y(PressableFeedbackA11yOptions {",
        "motion::attach_motion(root_ref, pressed, motion, has_highlight)",
        "trigger_ripple(ripple_ref, ripple_motion)",
        "role=role_attr",
        "tabindex=tabindex_attr",
        "aria-disabled=aria_disabled_attr",
        "lang=lang_attr.clone()",
        "dir=dir_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "PressableFeedback view should derive state via logic/motion helpers; missing `{needle}`."
        );
    }
}

#[test]
fn pressable_feedback_public_api_uses_is_on_default_naming_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for needle in [
        "#[prop(optional)] is_bounded: Option<bool>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] is_pressed: Option<Signal<bool>>,",
        "#[prop(optional)] default_pressed: Option<bool>,",
        "#[prop(optional)] on_pressed_change: Option<Callback<bool>>,",
        "#[prop(optional)] on_press: Option<OnPress>,",
        "bounded: is_bounded,",
    ] {
        assert!(
            view_source.contains(needle),
            "PressableFeedback public API should follow naming contract (`is_*` / `on_*`); missing `{needle}`."
        );
    }

    for legacy in [
        "#[prop(optional, default = true)] bounded: bool,",
        " bounded=",
    ] {
        assert!(
            !view_source.contains(legacy),
            "PressableFeedback view API should not expose legacy naming `{legacy}`."
        );
    }

    for docs_needle in [
        "is_bounded={}",
        "is_bounded=true",
        "is_bounded=false",
        "on_press=on_workbench_press",
    ] {
        assert!(
            docs_source.contains(docs_needle),
            "PressableFeedback docs should reflect naming migration; missing `{docs_needle}`."
        );
    }

    assert!(
        !docs_source.contains(" bounded="),
        "PressableFeedback docs should not keep legacy `bounded=` examples."
    );
}

#[test]
fn pressable_feedback_default_values_are_normalized_only_in_logic() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");

    for needle in [
        "pub const DEFAULT_IS_BOUNDED: bool = true;",
        "pub const DEFAULT_IS_DISABLED: bool = false;",
        "pub fn normalize_flags(",
        "is_bounded: is_bounded.unwrap_or(DEFAULT_IS_BOUNDED)",
        "is_disabled: is_disabled.unwrap_or(DEFAULT_IS_DISABLED)",
    ] {
        assert!(
            primitive_source.contains(needle),
            "PressableFeedback state primitive should own default normalization; missing `{needle}`."
        );
    }

    assert!(
        view_source.contains(
            "logic::normalize_state_contract(logic::PressableFeedbackStateContractInput {"
        ),
        "PressableFeedback view should consume normalized defaults/contracts through logic bridge."
    );
    assert!(
        !view_source.contains("#[prop(optional, default = true)] is_bounded: bool,"),
        "PressableFeedback view should not keep inline default fallback for `is_bounded`."
    );
    assert!(
        logic_source.contains("pub use ui_state_primitives::pressable_feedback::{"),
        "PressableFeedback logic should bridge normalized defaults from ui-state-primitives."
    );
}

#[test]
fn pressable_feedback_state_normalization_is_centralized_in_logic() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");

    for needle in [
        "pub struct PressableFeedbackStateContractInput",
        "pub struct PressableFeedbackStateContract",
        "pub fn normalize_state_contract(",
        "let flags = normalize_flags(input.is_bounded, input.is_disabled);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let class_name = normalize_optional_text(input.class_name);",
        "has_highlight: input.effect.has_highlight()",
        "has_ripple: input.effect.has_ripple()",
    ] {
        assert!(
            primitive_source.contains(needle),
            "PressableFeedback state primitive should centralize state-input normalization; missing `{needle}`."
        );
    }

    for forbidden in [
        "effect.has_highlight()",
        "effect.has_ripple()",
        "logic::normalize_aria_label(",
        "logic::normalize_optional_text(class_name)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not rebuild state normalization rules; found `{forbidden}`."
        );
    }

    for forbidden in [
        "pub struct PressableFeedbackStateContractInput {",
        "pub struct PressableFeedbackStateContract {",
        "pub fn normalize_state_contract(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not reimplement state kernels; found `{forbidden}`."
        );
    }
}

#[test]
fn pressable_feedback_discrete_state_axes_use_type_constrained_enums() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");

    for needle in [
        "pub enum PressableFeedbackPressedMode",
        "pub enum PressableFeedbackDefaultPressedSource",
        "pub enum PressableFeedbackPressedChangeSource",
        "pressed_mode: PressableFeedbackPressedMode",
        "default_pressed_source: PressableFeedbackDefaultPressedSource",
        "pressed_change_source: PressableFeedbackPressedChangeSource",
        "PressableFeedbackPressedMode::Controlled",
        "PressableFeedbackPressedMode::Uncontrolled",
    ] {
        assert!(
            primitive_source.contains(needle),
            "PressableFeedback state primitive should model discrete mode/source axes with enums; missing `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] tone: PressableFeedbackTone,",
        "#[prop(optional)] effect: PressableFeedbackEffect,",
        "pressed_axis.pressed_mode.as_attr()",
        "pressed_axis.default_pressed_source.as_attr()",
        "pressed_axis.pressed_change_source.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "PressableFeedback view should consume typed discrete axes/contracts; missing `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] tone: Option<String>",
        "#[prop(optional, into)] effect: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback should not expose stringly-typed discrete state inputs `{forbidden}`."
        );
    }
    assert!(
        logic_source.contains("pub use ui_state_primitives::pressable_feedback::{"),
        "PressableFeedback logic should bridge discrete axis enums from ui-state-primitives."
    );
}

#[test]
fn pressable_feedback_logic_only_bridges_state_primitives_for_state_kernels() {
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");

    assert!(
        logic_source.contains("pub use ui_state_primitives::pressable_feedback::{"),
        "PressableFeedback logic should bridge state primitives through pub use."
    );
    assert!(
        logic_source.contains("resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {"),
        "PressableFeedback logic should map Signal/Callback presence to primitive axis input."
    );

    for forbidden in [
        "pub struct PressableFeedbackFlags {",
        "pub struct PressableFeedbackStateContractInput {",
        "pub enum PressableFeedbackPressedMode {",
        "pub fn normalize_state_contract(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not redefine reusable state primitive `{forbidden}`."
        );
    }

    for needle in [
        "pub struct PressableFeedbackFlags",
        "pub struct PressableFeedbackStateContractInput",
        "pub enum PressableFeedbackPressedMode",
        "pub fn normalize_state_contract(",
        "pub fn resolve_pressed_axis_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "PressableFeedback reusable state kernels should live in ui-state-primitives; missing `{needle}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_async_loading_retry_contracts() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "#[prop(optional)] is_loading",
        "on_retry",
        "aria-busy",
        "use_async_action",
        "data-loading",
        "data-error",
        "data-retry",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback should not expose async-only view contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not embed async-only state contract `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback state primitive should stay async-agnostic; found `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should stay async-agnostic; found `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise unsupported async contract `{forbidden}`."
        );
    }
}

#[test]
fn pressable_feedback_dx_default_api_path_is_minimal_and_internal_complexity_hidden() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(into)] state:",
        "state: PressableFeedbackState",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback base API should not expose internal state wiring `{forbidden}`."
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "use_controllable_state(",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs hello-world should not require internal primitive/headless wiring `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("title=\"Hello World (Default API)\""),
        "PressableFeedback docs should expose a clear default hello-world entry."
    );
    assert!(
        docs_source.contains("title=\"Workbench (Config + Live Actual Config)\""),
        "PressableFeedback docs should keep advanced configuration in a separate path."
    );
    assert!(
        docs_source.contains(
            "code_imports=\"use leptos::prelude::*;\\nuse ui::{PressableFeedback};\".to_string()"
        ),
        "PressableFeedback hello-world docs should import only `PressableFeedback`."
    );

    let showcase_anchor = "let showcase_code = Signal::derive(move || {";
    let showcase_start = docs_source
        .find(showcase_anchor)
        .unwrap_or_else(|| panic!("PressableFeedback docs missing `{showcase_anchor}` anchor."));
    let showcase_tail = &docs_source[showcase_start..];
    let raw_start = showcase_tail.find("r#\"").unwrap_or_else(|| {
        panic!("PressableFeedback showcase code should use a raw-string literal.")
    }) + 3;
    let raw_tail = &showcase_tail[raw_start..];
    let raw_end = raw_tail.find("\"#").unwrap_or_else(|| {
        panic!(
            "PressableFeedback showcase code raw-string literal should have a closing delimiter."
        )
    });
    let showcase_code = &raw_tail[..raw_end];

    let showcase_lines = showcase_code
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        showcase_lines <= 5,
        "PressableFeedback hello-world snippet should stay within 5 lines; got {showcase_lines} lines."
    );

    for forbidden in ["effect=", "tone=", "on_press=", "is_pressed=", "state="] {
        assert!(
            !showcase_code.contains(forbidden),
            "PressableFeedback hello-world snippet should stay on default API path without `{forbidden}`."
        );
    }
}

#[test]
fn pressable_feedback_avoids_parallel_slot_composite_api_contracts() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let protocol_source = load_source("../../components/pressable-feedback/src/protocol.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    assert!(
        view_source.contains("children: Children,"),
        "PressableFeedback should keep explicit single-slot composition via `children: Children`."
    );

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "item_specs",
        "ItemSpec",
        "PressableFeedbackItem",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not expose composite parallel-slot contract `{forbidden}`."
        );
        assert!(
            !protocol_source.contains(forbidden),
            "PressableFeedback protocol should not define composite parallel-slot schema `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not recommend composite parallel-slot usage `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("<PressableFeedback>"),
        "PressableFeedback docs should keep explicit usage as `<PressableFeedback>...</PressableFeedback>`."
    );
}

#[test]
fn pressable_feedback_has_no_dragging_macro_micro_state_machine_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "on:pointermove",
        "on:mousemove",
        "touchmove",
        "is_dragging",
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "drag_state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should stay press-only and not expose drag-loop contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not define drag macro/micro state machine `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "PressableFeedback motion should not encode drag-loop convergence contract `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback state primitive should not carry drag state axis `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose drag-specific semantics `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise unsupported drag contract `{forbidden}`."
        );
    }

    for required_press_handler in [
        "on:pointerdown=on_pointer_down",
        "on:pointerup=on_pointer_up",
        "on:pointercancel=on_pointer_cancel",
    ] {
        assert!(
            view_source.contains(required_press_handler),
            "PressableFeedback should keep press interactions explicit; missing `{required_press_handler}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_two_pass_geometry_measure_rectification_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "getBoundingClientRect",
        "ResizeObserver",
        "IntersectionObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "scrollWidth",
        "scrollHeight",
        "Action::Measure",
        "Action::Rectification",
        "Rectification",
        "measure_pass",
        "anchor_rect",
        "overlay_rect",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not implement geometry two-pass pipeline contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement geometry rectification contract `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "PressableFeedback motion should not encode geometry measurement/rectification `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback state primitive should remain geometry-measurement agnostic; found `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose geometry two-pass API `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise geometry two-pass contract `{forbidden}`."
        );
    }

    for required_press_handler in [
        "on:pointerdown=on_pointer_down",
        "on:pointerup=on_pointer_up",
        "on:pointercancel=on_pointer_cancel",
    ] {
        assert!(
            view_source.contains(required_press_handler),
            "PressableFeedback should remain press-event based; missing `{required_press_handler}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_collection_registration_protocol_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "item_id",
        "collection_index",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not expose collection registration protocol `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement collection registration protocol `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback state primitive should stay collection-agnostic; found `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should stay collection-agnostic; found `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise collection registration protocol `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("children: Children,"),
        "PressableFeedback should keep single-slot composition and not model dynamic child registry."
    );
}

#[test]
fn pressable_feedback_has_no_slot_projection_keepalive_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "KeepAlive",
        "Lazy",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "slot_projection",
        "projection_mode",
        "on_hidden",
        "on_shown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not expose slot projection contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement keepalive projection strategy `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "PressableFeedback motion should not depend on slot projection lifecycle `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback state primitive should not include slot projection state `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not include slot projection lifecycle `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise slot projection API `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("children: Children,"),
        "PressableFeedback should stay a direct single-slot render path."
    );
}

#[test]
fn pressable_feedback_has_no_env_stream_sampling_to_action_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "window().add_event_listener",
        "on:resize",
        "BreakpointChanged",
        "Action::BreakpointChanged",
        "Action::ThemeChanged",
        "Action::IntersectionChanged",
        "debounce(",
        "throttle(",
        "env_stream",
        "env_event",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not implement env-stream sampling contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement env-stream action convergence `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback primitive should not carry env-stream state axis `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose env-stream protocol `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise env-stream API `{forbidden}`."
        );
    }

    for required_press_handler in [
        "on:pointerdown=on_pointer_down",
        "on:pointerup=on_pointer_up",
        "on:pointercancel=on_pointer_cancel",
    ] {
        assert!(
            view_source.contains(required_press_handler),
            "PressableFeedback should remain press-event driven; missing `{required_press_handler}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_event_light_cone_collection_bus_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "ContextBus",
        "context_bus",
        "SelectionState::All",
        "selection_state",
        "batch_select",
        "bulk_select",
        "grid_selection",
        "table_selection",
        "prop_drill",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not expose event-light-cone collection bus contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement collection-bus compressed selection contract `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback primitive should not carry large-collection selection compression state `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose table/grid event-light-cone protocol `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise collection bus selection API `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("children: Children,"),
        "PressableFeedback should keep single-surface composition instead of collection-item bus orchestration."
    );
}

#[test]
fn pressable_feedback_has_no_causality_bus_trace_id_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "TraceId",
        "trace_id",
        "causality_bus",
        "CausalityBus",
        "bus_broadcast",
        "publisher",
        "subscriber",
        "event_envelope",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not expose causality-bus trace contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement causality-bus trace chain `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback primitive should not include trace-id bus state `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose trace-id bus protocol `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise causality-bus trace API `{forbidden}`."
        );
    }

    for required_press_handler in [
        "on:pointerdown=on_pointer_down",
        "on:pointerup=on_pointer_up",
        "on:pointercancel=on_pointer_cancel",
    ] {
        assert!(
            view_source.contains(required_press_handler),
            "PressableFeedback should stay direct input-driven; missing `{required_press_handler}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_overlay_focus_stack_gc_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "FocusStack",
        "focus_stack",
        "FocusManager",
        "focus_manager",
        "FallbackTo",
        "fallback_to",
        "restore_focus",
        "focus_restore",
        "previous_focus",
        "return_focus_ref",
        "focus_target_ref",
        "on_open_change",
        "default_open",
        "is_open",
        "document.body",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not implement overlay focus-stack restore contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not implement overlay focus-stack GC contract `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback primitive should not include overlay focus-stack state axis `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose overlay focus-stack API `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise overlay focus-stack semantics `{forbidden}`."
        );
    }

    for required_local_focus_contract in [
        "let root_ref: NodeRef<html::Div> = NodeRef::new();",
        "let ripple_ref: NodeRef<html::Span> = NodeRef::new();",
        "on:blur=on_blur",
    ] {
        assert!(
            view_source.contains(required_local_focus_contract),
            "PressableFeedback should keep local press-surface focus/ripple wiring instead of overlay focus-stack recovery; missing `{required_local_focus_contract}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_foreign_zone_escape_hatch_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let protocol_source = load_source("../../components/pressable-feedback/src/protocol.rs");
    let mod_source = load_source("../../components/pressable-feedback/src/mod.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "third_party_instance",
        "imperative_instance",
        "foreign_handle",
        "chart_instance",
        "map_instance",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not expose foreign-zone imperative integration contract `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not let foreign imperative instances pollute state normalization `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "PressableFeedback motion should not carry foreign-zone cleanup/proxy contracts `{forbidden}`."
        );
        assert!(
            !protocol_source.contains(forbidden),
            "PressableFeedback protocol should not expose foreign imperative instance schema `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "PressableFeedback public exports should not leak foreign-zone APIs `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback primitive should stay third-party-instance agnostic; found `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not expose foreign imperative bridge `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not advertise foreign-zone escape hatch API `{forbidden}`."
        );
    }

    for required_public_api in [
        "#[prop(optional)] on_press: Option<OnPress>,",
        "pub use view::PressableFeedback;",
        "pub use motion::PressableFeedbackMotion;",
        "resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {",
    ] {
        assert!(
            view_source.contains(required_public_api)
                || mod_source.contains(required_public_api)
                || logic_source.contains(required_public_api),
            "PressableFeedback should keep public API constrained to press semantics, not foreign imperative instances; missing `{required_public_api}`."
        );
    }
}

#[test]
fn pressable_feedback_has_no_hydration_discontinuity_nondeterministic_init_contract() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let protocol_source = load_source("../../components/pressable-feedback/src/protocol.rs");
    let mod_source = load_source("../../components/pressable-feedback/src/mod.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for forbidden in [
        "SystemTime::now(",
        "Instant::now(",
        "Date::now(",
        "js_sys::Date",
        "Math::random",
        "randomUUID",
        "Uuid::new_v4",
        "uuid::Uuid::new_v4",
        "rand::random",
        "thread_rng",
        "fastrand",
        "nanoid",
        "IdProvider",
        "use_id(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view should not initialize hydration-critical ids with nondeterministic source `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "PressableFeedback logic should not use nondeterministic hydration seed `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "PressableFeedback motion should not create nondeterministic hydration ids/seeds `{forbidden}`."
        );
        assert!(
            !protocol_source.contains(forbidden),
            "PressableFeedback protocol should not expose nondeterministic id schema `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "PressableFeedback public exports should not leak nondeterministic id helpers `{forbidden}`."
        );
        assert!(
            !primitive_source.contains(forbidden),
            "PressableFeedback primitive should remain deterministic for SSR/hydration; found `{forbidden}`."
        );
        assert!(
            !headless_source.contains(forbidden),
            "PressableFeedback headless contract should not use nondeterministic id/init source `{forbidden}`."
        );
        assert!(
            !docs_source.contains(forbidden),
            "PressableFeedback docs should not recommend nondeterministic id initialization `{forbidden}`."
        );
    }

    for required_stable_marker in [
        "data-slot=\"pressable-feedback\"",
        "data-slot=\"pressable-feedback-highlight\"",
        "data-slot=\"pressable-feedback-content\"",
    ] {
        assert!(
            view_source.contains(required_stable_marker),
            "PressableFeedback should keep stable semantic markers for SSR/hydration parity; missing `{required_stable_marker}`."
        );
    }
}

#[test]
fn pressable_feedback_a11y_and_i18n_contracts_are_headless_driven_and_view_has_no_hardcoded_copy() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let headless_source = load_source("../../crates/ui-headless/src/pressable_feedback.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");

    for needle in [
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "use_pressable_feedback_a11y(PressableFeedbackA11yOptions {",
        "lang,",
        "dir,",
        "role=role_attr",
        "tabindex=tabindex_attr",
        "aria-disabled=aria_disabled_attr",
        "lang=lang_attr.clone()",
        "dir=dir_attr",
        "on:keydown=on_key_down",
        "on:keyup=on_key_up",
        "on:blur=on_blur",
    ] {
        assert!(
            view_source.contains(needle),
            "PressableFeedback view should mount headless a11y + locale contract; missing `{needle}`."
        );
    }

    for forbidden in [
        "\"Pressable feedback\"",
        "\"Press me\"",
        "\"Submit\"",
        "\"Confirm\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "PressableFeedback view.rs should not hardcode user-visible copy `{forbidden}`."
        );
    }

    assert!(
        headless_source.contains("use crate::a11y::{A11yDirection, locale_attrs};"),
        "PressableFeedback headless contract should reuse shared a11y utilities from `ui-headless/src/a11y.rs`."
    );
    assert!(
        headless_source.contains("let locale = locale_attrs(options.lang, options.dir);"),
        "PressableFeedback headless contract should derive lang/dir through shared locale helper."
    );
    assert!(
        a11y_source.contains("pub fn locale_attrs("),
        "Shared a11y locale helper should be defined in `ui-headless/src/a11y.rs`."
    );
    assert!(
        primitive_source.contains("pub fn normalize_aria_label("),
        "PressableFeedback primitive should keep aria-label fallback as a typed state contract."
    );
}

#[test]
fn pressable_feedback_emits_baseline_style_state_data_attributes() {
    let source = load_source("../../components/pressable-feedback/src/view.rs");

    for attr in [
        "data-slot=\"pressable-feedback\"",
        "data-slot=\"pressable-feedback-highlight\"",
        "data-slot=\"pressable-feedback-content\"",
        "data-tone=move || state.get().tone_attr",
        "data-effect=move || state.get().effect_attr",
        "data-state=move || state.get().state_attr",
        "data-boundary=move || state.get().boundary_attr",
        "data-bounded=move || state.get().is_bounded.then_some(\"true\")",
        "data-unbounded=move || state.get().is_unbounded.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-highlight=move || state.get().highlight_attr",
        "data-ripple=move || state.get().ripple_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-pressed-mode=pressed_mode_attr",
        "data-default-pressed-source=default_pressed_source_attr",
        "data-pressed-change-source=pressed_change_source_attr",
        "data-pressed-controlled=is_pressed_controlled.then_some(\"true\")",
        "data-pressed-uncontrolled=(!is_pressed_controlled).then_some(\"true\")",
        "data-custom-default-pressed=has_custom_default_pressed.then_some(\"true\")",
        "data-custom-pressed-change=has_custom_on_pressed_change.then_some(\"true\")",
        "lang=lang_attr.clone()",
        "dir=dir_attr",
    ] {
        assert!(
            source.contains(attr),
            "PressableFeedback should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn pressable_feedback_state_markers_are_observable_queryable_and_closed_set() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");

    for marker in [
        "data-slot=\"pressable-feedback\"",
        "data-state=move || state.get().state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-pressed=move || state.get().is_pressed.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-pressed-mode=pressed_mode_attr",
        "data-default-pressed-source=default_pressed_source_attr",
        "data-pressed-change-source=pressed_change_source_attr",
        "data-pressed-controlled=is_pressed_controlled.then_some(\"true\")",
        "data-pressed-uncontrolled=(!is_pressed_controlled).then_some(\"true\")",
        "role=role_attr",
        "aria-disabled=aria_disabled_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "PressableFeedback should expose stable observable/queryable marker `{marker}`."
        );
    }

    for closed_set in [
        "PressableFeedbackTone::Default => \"default\"",
        "PressableFeedbackTone::Neutral => \"neutral\"",
        "PressableFeedbackTone::Accent => \"accent\"",
        "PressableFeedbackEffect::Scale => \"scale\"",
        "PressableFeedbackEffect::Highlight => \"highlight\"",
        "PressableFeedbackEffect::Ripple => \"ripple\"",
        "PressableFeedbackEffect::HighlightRipple => \"highlight-ripple\"",
        "Self::Controlled => \"controlled\"",
        "Self::Uncontrolled => \"uncontrolled\"",
        "Self::Provided => \"provided\"",
        "Self::Default => \"default\"",
        "Self::None => \"none\"",
        "(\"ui-pressable-feedback--state-disabled\", \"disabled\")",
        "(\"ui-pressable-feedback--state-pressed\", \"pressed\")",
        "(\"ui-pressable-feedback--state-idle\", \"idle\")",
    ] {
        assert!(
            primitive_source.contains(closed_set),
            "PressableFeedback primitive should keep marker value sets enumerable/closed; missing `{closed_set}`."
        );
    }
}

#[test]
fn pressable_feedback_semantics_matrix_covers_core_paths_without_snapshot_dependency() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_test_source = load_source("../../components/pressable-feedback/test/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let semantics_source = load_source("../../components/pressable-feedback/test/semantics.rs");

    for required_semantic_marker in [
        "role=role_attr",
        "aria-disabled=aria_disabled_attr",
        "data-state=move || state.get().state_attr",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-pressed-controlled=is_pressed_controlled.then_some(\"true\")",
        "data-pressed-uncontrolled=(!is_pressed_controlled).then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(required_semantic_marker),
            "PressableFeedback semantic contract should expose `{required_semantic_marker}`."
        );
    }

    for required_interaction_path in [
        "on:keydown=on_key_down",
        "on:keyup=on_key_up",
        "on:pointerdown=on_pointer_down",
        "on:pointerup=on_pointer_up",
        "on:pointercancel=on_pointer_cancel",
    ] {
        assert!(
            view_source.contains(required_interaction_path),
            "PressableFeedback interaction path matrix should cover `{required_interaction_path}`."
        );
    }

    for required_state_matrix_case in [
        "fn normalize_pressed_axis_reports_controlled_sources()",
        "fn normalize_pressed_axis_uncontrolled_uses_default_fallback()",
        "fn normalize_flags_prefers_explicit_inputs()",
        "assert!(flags.is_disabled);",
        "fn normalize_state_contract_uses_fallback_markers()",
        "assert!(!contract.flags.is_disabled);",
    ] {
        assert!(
            logic_test_source.contains(required_state_matrix_case),
            "PressableFeedback test matrix should keep controlled/uncontrolled + disabled coverage; missing `{required_state_matrix_case}`."
        );
    }

    for required_platform_branch in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(required_platform_branch),
            "PressableFeedback platform matrix should keep SSR/wasm branch evidence `{required_platform_branch}`."
        );
    }

    for forbidden_snapshot_dependency in [
        concat!("assert_", "snapshot", "!"),
        concat!("in", "sta::"),
        concat!("snapshot", "!"),
        concat!("toMatch", "Snapshot"),
        concat!("to_match_", "snapshot"),
    ] {
        assert!(
            !semantics_source.contains(forbidden_snapshot_dependency),
            "PressableFeedback semantic contract tests should not depend on snapshot-only assertion `{forbidden_snapshot_dependency}`."
        );
        assert!(
            !logic_test_source.contains(forbidden_snapshot_dependency),
            "PressableFeedback logic tests should not depend on snapshot-only assertion `{forbidden_snapshot_dependency}`."
        );
    }
}

#[test]
fn pressable_feedback_platform_matrix_keeps_cfg_boundaries_and_non_wasm_browser_free() {
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let checklist_source = load_source("../../components/pressable-feedback/check2.md");

    for required_platform_branch in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "use leptos::wasm_bindgen::JsCast;",
        "let element: leptos::web_sys::HtmlElement = node.unchecked_into();",
    ] {
        assert!(
            motion_source.contains(required_platform_branch),
            "PressableFeedback motion should keep explicit platform branch contract `{required_platform_branch}`."
        );
    }

    let (_, non_wasm_branch) = motion_source
        .split_once("#[cfg(not(target_arch = \"wasm32\"))]")
        .expect("PressableFeedback motion should define explicit non-wasm branch.");

    for forbidden_non_wasm_browser_ref in ["web_sys::", "wasm_bindgen", "window()", "document()"] {
        assert!(
            !non_wasm_branch.contains(forbidden_non_wasm_browser_ref),
            "PressableFeedback non-wasm motion branch should not touch browser APIs `{forbidden_non_wasm_browser_ref}`."
        );
    }

    for forbidden_component_browser_ref in ["web_sys::", "js_sys::"] {
        assert!(
            !view_source.contains(forbidden_component_browser_ref),
            "PressableFeedback view should not directly depend on browser-only API `{forbidden_component_browser_ref}`."
        );
        assert!(
            !logic_source.contains(forbidden_component_browser_ref),
            "PressableFeedback logic should not directly depend on browser-only API `{forbidden_component_browser_ref}`."
        );
    }

    for required_compile_only_evidence in [
        "cargo test --workspace",
        "cargo check -p ui --target wasm32-unknown-unknown",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-<your_component>,inject-css",
    ] {
        assert!(
            checklist_source.contains(required_compile_only_evidence),
            "PressableFeedback checklist should keep compile-only evidence command `{required_compile_only_evidence}`."
        );
    }
}

#[test]
fn pressable_feedback_preserves_ui_headless_web_ssr_feature_mutex_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let component_cargo_source = load_source("../../components/pressable-feedback/Cargo.toml");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let checklist_source = load_source("../../components/pressable-feedback/check2.md");
    let src_checklist_source = load_source("../../components/pressable-feedback/src/check2.md");

    for required_headless_mutex_guard in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib_source.contains(required_headless_mutex_guard),
            "ui-headless should keep web/ssr mutex guard `{required_headless_mutex_guard}`."
        );
    }

    for required_headless_feature in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo_source.contains(required_headless_feature),
            "ui-headless Cargo feature surface should keep `{required_headless_feature}`."
        );
    }

    assert!(
        component_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "PressableFeedback should depend on ui-headless without forcing conflicting feature flags."
    );
    assert!(
        !component_cargo_source.contains(
            "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"] }"
        ),
        "PressableFeedback must not force-enable both ui-headless `web` and `ssr` features."
    );

    assert!(
        view_source.contains("use_pressable_feedback_a11y(PressableFeedbackA11yOptions {"),
        "PressableFeedback should keep headless contract mount while respecting ui-headless feature mutex."
    );

    for required_compile_proof in [
        "cargo check -p ui-headless --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
    ] {
        assert!(
            checklist_source.contains(required_compile_proof)
                || src_checklist_source.contains(required_compile_proof),
            "Checklist should keep ui-headless feature-path proof command `{required_compile_proof}`."
        );
    }
}

#[test]
fn pressable_feedback_motion_contract_keeps_non_wasm_noop_stub_predictable() {
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let component_motion_source = load_source("../../components/pressable-feedback/src/motion.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let checklist_source = load_source("../../components/pressable-feedback/check2.md");
    let src_checklist_source = load_source("../../components/pressable-feedback/src/check2.md");

    for required_ui_motion_stub in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "#[cfg(all(test, not(target_arch = \"wasm32\")))]",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(required_ui_motion_stub),
            "ui-motion should keep non-wasm predictable no-op backend contract `{required_ui_motion_stub}`."
        );
    }

    for required_component_noop in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_pressed: leptos::prelude::Signal<bool>",
        "_motion: PressableFeedbackMotion",
        "_has_highlight: bool",
        ") {",
        "}",
    ] {
        assert!(
            component_motion_source.contains(required_component_noop),
            "PressableFeedback motion should keep non-wasm attach no-op contract `{required_component_noop}`."
        );
    }

    for forbidden_non_wasm_behavior in ["panic!(", ".unwrap()", ".expect("] {
        let (_, non_wasm_branch) = component_motion_source
            .split_once("#[cfg(not(target_arch = \"wasm32\"))]")
            .expect("PressableFeedback motion should define explicit non-wasm no-op branch.");
        assert!(
            !non_wasm_branch.contains(forbidden_non_wasm_behavior),
            "PressableFeedback non-wasm motion branch should be safe and predictable; found `{forbidden_non_wasm_behavior}`."
        );
    }

    assert!(
        view_source.contains("motion::attach_motion(root_ref, pressed, motion, has_highlight);"),
        "PressableFeedback view should call motion attach uniformly and rely on non-wasm no-op fallback."
    );

    for required_compile_proof in [
        "cargo check -p ui --target wasm32-unknown-unknown",
        "cargo test --workspace",
    ] {
        assert!(
            checklist_source.contains(required_compile_proof)
                || src_checklist_source.contains(required_compile_proof),
            "Checklist should keep toolchain compile proof command `{required_compile_proof}`."
        );
    }
}

#[test]
fn pressable_feedback_component_files_respect_layered_responsibilities() {
    let mod_source = load_source("../../components/pressable-feedback/src/mod.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let styles_source = load_source("../../components/pressable-feedback/src/styles.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let motion_source = load_source("../../components/pressable-feedback/src/motion.rs");

    for required_mod_export in [
        "pub use logic::{DEFAULT_ARIA_LABEL, PressableFeedbackEffect, PressableFeedbackTone};",
        "pub use motion::PressableFeedbackMotion;",
        "pub use view::PressableFeedback;",
    ] {
        assert!(
            mod_source.contains(required_mod_export),
            "PressableFeedback mod.rs should keep only stable export boundary; missing `{required_mod_export}`."
        );
    }
    for forbidden_mod_impl in [
        "pub mod logic",
        "pub mod view",
        "pub mod motion",
        "fn normalize_pressed_axis(",
        "pub fn attach_motion(",
        "pub const CSS",
    ] {
        assert!(
            !mod_source.contains(forbidden_mod_impl),
            "PressableFeedback mod.rs should not carry implementation detail `{forbidden_mod_impl}`."
        );
    }

    for required_logic_responsibility in [
        "resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {",
        "pub struct PressableFeedbackPressedAxis {",
        "pub fn normalize_pressed_axis(",
        "pub use ui_state_primitives::pressable_feedback::{",
    ] {
        assert!(
            logic_source.contains(required_logic_responsibility),
            "PressableFeedback logic.rs should focus on normalization/derivation/source markers; missing `{required_logic_responsibility}`."
        );
    }
    for forbidden_logic_detail in [
        "web_sys",
        "NodeRef",
        "on:pointer",
        "data-slot=",
        "pub const CSS",
    ] {
        assert!(
            !logic_source.contains(forbidden_logic_detail),
            "PressableFeedback logic.rs should not include DOM/style details `{forbidden_logic_detail}`."
        );
    }

    for required_styles_contract in [
        "pub const CSS: &str = r#\"",
        "var(--ui-pressable-feedback-fg",
        ".ui-pressable-feedback[data-state=\"pressed\"]",
        ".ui-pressable-feedback[data-effect=\"highlight-ripple\"]",
    ] {
        assert!(
            styles_source.contains(required_styles_contract),
            "PressableFeedback styles.rs should keep token-first static css contract; missing `{required_styles_contract}`."
        );
    }
    for forbidden_styles_impl in [
        "fn ",
        "Signal<",
        "on:pointerdown",
        "aria-label",
        "Pressable feedback",
    ] {
        assert!(
            !styles_source.contains(forbidden_styles_impl),
            "PressableFeedback styles.rs should not include runtime logic/a11y copy `{forbidden_styles_impl}`."
        );
    }

    for required_view_boundary in [
        "logic::normalize_state_contract(logic::PressableFeedbackStateContractInput {",
        "logic::normalize_pressed_axis(is_pressed, default_pressed, on_pressed_change)",
        "use_pressable_feedback_a11y(PressableFeedbackA11yOptions {",
        "motion::attach_motion(root_ref, pressed, motion, has_highlight)",
        "view! {",
        "role=role_attr",
        "aria-disabled=aria_disabled_attr",
    ] {
        assert!(
            view_source.contains(required_view_boundary),
            "PressableFeedback view.rs should focus on structure render + headless mount; missing `{required_view_boundary}`."
        );
    }
    for forbidden_view_hidden_decision in [
        "unwrap_or(DEFAULT_IS_BOUNDED)",
        "unwrap_or(DEFAULT_IS_DISABLED)",
        "pub const CSS",
        "SpringAnimator::new(",
    ] {
        assert!(
            !view_source.contains(forbidden_view_hidden_decision),
            "PressableFeedback view.rs should not hide kernel/motion engine decisions `{forbidden_view_hidden_decision}`."
        );
    }

    for required_motion_contract in [
        "pub struct PressableFeedbackMotion",
        "pub fn sanitize_motion(",
        "ui_motion::spring::sanitize_config",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(required_motion_contract),
            "PressableFeedback motion.rs should keep semantic-to-motion contract attach; missing `{required_motion_contract}`."
        );
    }
    for forbidden_motion_semantics in [
        "on:keydown",
        "on:pointerdown",
        "aria-disabled",
        "data-state=",
        "labels + children",
    ] {
        assert!(
            !motion_source.contains(forbidden_motion_semantics),
            "PressableFeedback motion.rs should not carry interaction/a11y/view semantics `{forbidden_motion_semantics}`."
        );
    }
}

#[test]
fn pressable_feedback_keeps_spec_rs_opt_in_for_complex_components_only() {
    let mod_source = load_source("../../components/pressable-feedback/src/mod.rs");
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );
    let protocol_source = load_source("../../components/pressable-feedback/src/protocol.rs");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("../../components/pressable-feedback/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "PressableFeedback should not introduce `spec.rs` for a simple press container component."
    );

    for forbidden_spec_surface in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "PressableFeedbackSpec",
        "::spec::",
    ] {
        assert!(
            !mod_source.contains(forbidden_spec_surface),
            "PressableFeedback mod.rs should not expose speculative spec API `{forbidden_spec_surface}`."
        );
        assert!(
            !docs_source.contains(forbidden_spec_surface),
            "PressableFeedback docs should stay on direct component usage and not require spec builder `{forbidden_spec_surface}`."
        );
    }

    assert!(
        protocol_source.contains("pub struct PressableFeedbackProtocol"),
        "PressableFeedback should keep lightweight protocol schema in `protocol.rs` without escalating to `spec.rs`."
    );
}

#[test]
fn pressable_feedback_obeys_token_first_static_style_contract_and_uiroot_injection_pipeline() {
    let styles_source = load_source("../../components/pressable-feedback/src/styles.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let ui_css_source = load_source("../../crates/ui/src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");

    for required_token_contract in [
        "pub const CSS: &str = r#\"",
        "--ui-pressable-feedback-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-pressable-feedback-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "--ui-pressable-feedback-accent: var(--ui-accent, var(--ui-fallback-accent));",
        "--ui-pressable-feedback-disabled-opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity, 0.62));",
        "--ui-pressable-feedback-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width, 1px));",
    ] {
        assert!(
            styles_source.contains(required_token_contract),
            "PressableFeedback styles.rs should keep token-first static style contract; missing `{required_token_contract}`."
        );
    }

    for required_css_aggregation in [
        "#[cfg(feature = \"component-pressable_feedback\")]",
        "out.push_str(crate::pressable_feedback::styles::CSS);",
        "out.push_str(\"\\n@layer ui {\\n\");",
    ] {
        assert!(
            ui_css_source.contains(required_css_aggregation),
            "PressableFeedback styles should be aggregated by crates/ui/src/css.rs; missing `{required_css_aggregation}`."
        );
    }

    for required_uiroot_injection in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root_source.contains(required_uiroot_injection),
            "UiRoot should inject aggregated component CSS through the centralized pipeline; missing `{required_uiroot_injection}`."
        );
    }

    for forbidden_component_pollution in [
        "class=\"flex",
        "class=\"grid",
        "class=\"p-",
        "class=\"m-",
        "style! {",
        "style!{",
    ] {
        assert!(
            !view_source.contains(forbidden_component_pollution),
            "PressableFeedback component layer should not adopt Utility-First/CSS-in-Rust defaults `{forbidden_component_pollution}`."
        );
        assert!(
            !styles_source.contains(forbidden_component_pollution),
            "PressableFeedback styles.rs should stay static token CSS and avoid Utility-First/CSS-in-Rust contamination `{forbidden_component_pollution}`."
        );
    }
}

#[test]
fn pressable_feedback_styles_include_effect_boundary_and_markers() {
    let source = load_source("../../components/pressable-feedback/src/styles.rs");

    for selector in [
        ".ui-pressable-feedback--tone-default",
        ".ui-pressable-feedback[data-tone=\"default\"]",
        ".ui-pressable-feedback--state-pressed",
        ".ui-pressable-feedback[data-state=\"pressed\"]",
        ".ui-pressable-feedback--effect-highlight-ripple",
        ".ui-pressable-feedback[data-effect=\"highlight-ripple\"]",
        ".ui-pressable-feedback--boundary-bounded",
        ".ui-pressable-feedback[data-boundary=\"bounded\"]",
        ".ui-pressable-feedback--highlight-enabled",
        ".ui-pressable-feedback[data-highlight=\"enabled\"]",
        ".ui-pressable-feedback--ripple-enabled",
        ".ui-pressable-feedback[data-ripple=\"enabled\"]",
        ".ui-pressable-feedback--custom-class",
        ".ui-pressable-feedback[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "PressableFeedback styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn pressable_feedback_styles_are_driven_by_explicit_markers_not_dom_guesswork() {
    let styles_source = load_source("../../components/pressable-feedback/src/styles.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");

    for required_selector in [
        ".ui-pressable-feedback[data-state=\"pressed\"]",
        ".ui-pressable-feedback[data-state=\"disabled\"]",
        ".ui-pressable-feedback[data-highlight=\"enabled\"] .ui-pressable-feedback__highlight",
        ".ui-pressable-feedback[data-highlight=\"none\"] .ui-pressable-feedback__highlight",
        ".ui-pressable-feedback[data-ripple=\"enabled\"] .ui-pressable-feedback__ripple",
        ".ui-pressable-feedback[data-ripple=\"none\"] .ui-pressable-feedback__ripple",
    ] {
        assert!(
            styles_source.contains(required_selector),
            "PressableFeedback styles should branch from explicit semantic markers; missing `{required_selector}`."
        );
    }

    for forbidden_selector in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !styles_source.contains(forbidden_selector),
            "PressableFeedback styles should not rely on brittle DOM-structure selector `{forbidden_selector}`."
        );
    }

    for forbidden_inline_style in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden_inline_style),
            "PressableFeedback view should not encode business styling via inline styles `{forbidden_inline_style}`."
        );
    }

    for forbidden_conditional_mount in [
        "<Show when=move || state.get().has_highlight>",
        "<Show when=move || state.get().has_ripple>",
    ] {
        assert!(
            !view_source.contains(forbidden_conditional_mount),
            "PressableFeedback visual state switching should not depend on conditional node mounting `{forbidden_conditional_mount}`."
        );
    }

    for required_always_mounted_node in [
        "<span class=\"ui-pressable-feedback__highlight\" data-slot=\"pressable-feedback-highlight\" aria-hidden=\"true\"></span>",
        "<MotionRipple",
        "class_name=\"ui-pressable-feedback__ripple\".to_string()",
    ] {
        assert!(
            view_source.contains(required_always_mounted_node),
            "PressableFeedback should keep visual nodes mounted and let semantic markers drive visibility; missing `{required_always_mounted_node}`."
        );
    }
}

#[test]
fn pressable_feedback_styles_are_token_first_theme_driven() {
    let source = load_source("../../components/pressable-feedback/src/styles.rs");

    for needle in [
        "--ui-pressable-feedback-fg: var(--ui-fg, var(--ui-fallback-fg));",
        "--ui-pressable-feedback-fg-muted: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "--ui-pressable-feedback-accent: var(--ui-accent, var(--ui-fallback-accent));",
        "--ui-pressable-feedback-disabled-opacity: var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity, 0.62));",
        "--ui-pressable-feedback-highlight-mix: var(--ui-command-option-focus-mix, var(--ui-fallback-command-option-focus-mix, 16%));",
        "--ui-pressable-feedback-outline-mix: var(--ui-command-group-border-mix, var(--ui-fallback-command-group-border-mix, 24%));",
        "--ui-pressable-feedback-outline-width: var(--ui-button-focus-outline-width, var(--ui-fallback-button-focus-outline-width, 1px));",
        "--ui-pressable-feedback-outline-offset: var(--ui-button-focus-outline-offset, var(--ui-fallback-button-focus-outline-offset, 2px));",
        "--ui-pressable-feedback-ripple-duration-ms: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration, 180ms));",
        "opacity: var(--ui-pressable-feedback-disabled-opacity);",
        "--ui-ripple-duration-ms: var(--ui-pressable-feedback-ripple-duration-ms);",
    ] {
        assert!(
            source.contains(needle),
            "PressableFeedback styles should consume ui-theme variables via token-first mapping; missing `{needle}`."
        );
    }

    for legacy_hardcoded in ["opacity: 0.62;", "--ui-ripple-duration-ms: 420;"] {
        assert!(
            !source.contains(legacy_hardcoded),
            "PressableFeedback styles should not keep legacy hardcoded theme values `{legacy_hardcoded}`."
        );
    }
}

#[test]
fn pressable_feedback_motion_contract_is_present() {
    let source = load_source("../../components/pressable-feedback/src/motion.rs");

    for needle in [
        "pub struct PressableFeedbackMotion",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "--ui-pressable-feedback-scale",
        "--ui-pressable-feedback-highlight-opacity",
    ] {
        assert!(
            source.contains(needle),
            "PressableFeedback motion should expose `{needle}` for spring-driven press feedback transitions."
        );
    }
}

#[test]
fn pressable_feedback_docs_page_covers_primary_playgrounds() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for needle in [
        "pub(crate) fn pressable_feedback() -> AnyView",
        "title=\"PressableFeedback\"",
        "slug=\"pressable-feedback\"",
        "description=\"baseline-style press feedback container with centralized effect/tone/boundary/source contracts, spring-driven scale/highlight motion, and optional ripple composition.\"",
        "title=\"Hello World (Default API)\"",
        "title=\"Workbench (Config + Live Actual Config)\"",
        "title=\"State Matrix (Effect / Tone / Disabled Comparison)\"",
        "<PressableFeedback",
    ] {
        assert!(
            source.contains(needle),
            "display_extra/pressable_feedback docs should include `{needle}` for pressable-feedback primary playground coverage.",
        );
    }
}

#[test]
fn pressable_feedback_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );

    for needle in [
        "title=\"Hello World (Default API)\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui::{PressableFeedback};\".to_string()",
        "let showcase_code = Signal::derive(move || {",
        "r#\"<PressableFeedback>",
        "<div class=\"docs-ripple-surface\">\"Hello feedback\"</div>",
        "</PressableFeedback>\"#",
        "\"press_count: \" {move || press_count.get()}",
        "title=\"State Matrix (Effect / Tone / Disabled Comparison)\"",
        "on_press=on_workbench_press",
        "effect=PressableFeedbackEffect::HighlightRipple",
        "tone=PressableFeedbackTone::Neutral",
        "is_bounded=false",
        "motion=PressableFeedbackMotion {",
        "pressed_scale: 0.94",
        "highlight_opacity: 0.2",
        "duration_ms: 720",
        "class_name=\"docs-pressable-feedback-custom\".to_string()",
        "\"Disabled custom\"",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "display_extra/pressable_feedback docs playgrounds should contain `{needle}` for pressable-feedback contracts.",
        );
    }

    assert!(
        !source.contains(
            "<PressableFeedback effect=PressableFeedbackEffect::Highlight tone=PressableFeedbackTone::Accent on_press=on_press>"
        ),
        "PressableFeedback hello-world docs should not require non-default advanced props."
    );
}

#[test]
fn pressable_feedback_visual_desire_baseline_covers_docs_and_interaction_cues() {
    let docs_source = load_source(
        "../../apps/docs-app/src/pages/components/pages/display_extra/pressable_feedback.rs",
    );
    let styles_source = load_source("../../components/pressable-feedback/src/styles.rs");

    for docs_needle in [
        "title=\"Default Theme Visual Baseline (Visual Desire)\"",
        "description=\"First-impression baseline for hierarchy, contrast layers, and hover/active/focus cues. Use this section as the screenshot regression anchor.\"",
        "data-visual-baseline=\"pressable-feedback-default-theme\"",
        "data-slot=\"pressable-feedback-visual-baseline-screenshot\"",
        "\"HeroUI-quality visual direction baseline for PressableFeedback under default theme.\"",
        "\"Screenshot baseline anchor: compare hover/active/focus feedback and disabled contrast.\"",
        "\"Primary Surface\"",
        "\"Accent Surface\"",
        "\"Disabled Surface\"",
    ] {
        assert!(
            docs_source.contains(docs_needle),
            "PressableFeedback docs should keep Visual Desire baseline evidence; missing `{docs_needle}`."
        );
    }

    for style_needle in [
        "--ui-pressable-feedback-hover-highlight-opacity: var(--ui-button-hover-overlay-opacity, var(--ui-fallback-button-hover-overlay-opacity, 0.08));",
        "--ui-pressable-feedback-active-highlight-opacity: var(--ui-button-active-overlay-opacity, var(--ui-fallback-button-active-overlay-opacity, 0.16));",
        ".ui-pressable-feedback:not([data-disabled=\"true\"]):hover {",
        ".ui-pressable-feedback:not([data-disabled=\"true\"]):active,",
        ".ui-pressable-feedback:not([data-disabled=\"true\"]):focus-visible {",
        ".ui-pressable-feedback:not([data-disabled=\"true\"]):hover .ui-pressable-feedback__content {",
        ".ui-pressable-feedback:not([data-disabled=\"true\"]):focus-visible .ui-pressable-feedback__content {",
    ] {
        assert!(
            styles_source.contains(style_needle),
            "PressableFeedback styles should expose hover/active/focus visual baseline cues; missing `{style_needle}`."
        );
    }
}

#[test]
fn pressable_feedback_tree_shaking_feature_gates_remain_component_scoped() {
    let ui_cargo_source = load_source("../../crates/ui/Cargo.toml");
    let ui_lib_source = load_source("../../crates/ui/src/lib.rs");
    let ui_css_source = load_source("../../crates/ui/src/css.rs");
    let web_demo_cargo_source = load_source("../../apps/web-demo/Cargo.toml");

    for required_feature_contract in [
        "component-pressable_feedback = [\"component-ripple\", \"dep:ui-pressable-feedback\"]",
        "\"component-pressable_feedback\",",
    ] {
        assert!(
            ui_cargo_source.contains(required_feature_contract),
            "ui/Cargo.toml should keep component-scoped feature gate for PressableFeedback; missing `{required_feature_contract}`."
        );
    }

    for required_lib_gate in [
        "#[cfg(feature = \"component-pressable_feedback\")]\npub use ui_pressable_feedback as pressable_feedback;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "pub use web_demo_components::*;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
    ] {
        assert!(
            ui_lib_source.contains(required_lib_gate),
            "ui/lib.rs should keep feature-gated export boundaries; missing `{required_lib_gate}`."
        );
    }

    for required_css_gate in [
        "#[cfg(feature = \"inject-css\")]",
        "#[cfg(feature = \"component-pressable_feedback\")]",
        "out.push_str(crate::pressable_feedback::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_css_source.contains(required_css_gate),
            "ui/css.rs should keep feature-gated style aggregation for tree-shaking; missing `{required_css_gate}`."
        );
    }

    assert!(
        web_demo_cargo_source.contains(
            "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"
        ),
        "web-demo should consume ui with default-features disabled to avoid implicit all-components pull-up."
    );

    assert!(
        !web_demo_cargo_source.contains("\"all-components\""),
        "web-demo should not enable `all-components` implicitly."
    );
}

#[test]
fn pressable_feedback_type_system_and_semantic_markers_form_machine_readable_state_contract() {
    let primitive_source =
        load_source("../../crates/ui-state-primitives/src/pressable_feedback.rs");
    let primitive_test_source =
        load_source("../../crates/ui-state-primitives/src/test/pressable_feedback.rs");
    let logic_source = load_source("../../components/pressable-feedback/src/logic.rs");
    let logic_test_source = load_source("../../components/pressable-feedback/test/logic.rs");
    let view_source = load_source("../../components/pressable-feedback/src/view.rs");
    let semantics_source = load_source("../../components/pressable-feedback/test/semantics.rs");

    for typed_axis in [
        "pub enum PressableFeedbackTone",
        "pub enum PressableFeedbackEffect",
        "pub enum PressableFeedbackPressedMode",
        "pub enum PressableFeedbackDefaultPressedSource",
        "pub enum PressableFeedbackPressedChangeSource",
        "pub struct PressableFeedbackStateContract",
        "pub fn resolve_pressed_axis_state(",
        "pub fn normalize_state_contract(",
    ] {
        assert!(
            primitive_source.contains(typed_axis),
            "PressableFeedback should keep typed state axes instead of string/bool protocols; missing `{typed_axis}`."
        );
    }

    for normalized_logic in [
        "pub use ui_state_primitives::pressable_feedback::{",
        "resolve_pressed_axis_state(PressableFeedbackPressedAxisInput {",
        "pub fn normalize_pressed_axis(",
    ] {
        assert!(
            logic_source.contains(normalized_logic),
            "PressableFeedback logic.rs should normalize through typed primitives; missing `{normalized_logic}`."
        );
    }

    for state_marker in [
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-effect=move || state.get().effect_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-pressed-mode=pressed_mode_attr",
        "data-default-pressed-source=default_pressed_source_attr",
        "data-pressed-change-source=pressed_change_source_attr",
    ] {
        assert!(
            view_source.contains(state_marker),
            "PressableFeedback should expose machine-readable semantic markers; missing `{state_marker}`."
        );
    }

    for regression_locator in [
        "fn resolve_pressed_axis_state_maps_control_and_source_markers()",
        "fn normalize_state_contract_aggregates_sources_and_effect_flags()",
        "fn normalize_state_contract_uses_fallback_markers()",
        "fn normalize_pressed_axis_reports_controlled_sources()",
        "fn normalize_pressed_axis_uncontrolled_uses_default_fallback()",
        "fn pressable_feedback_discrete_state_axes_use_type_constrained_enums()",
        "fn pressable_feedback_state_markers_are_observable_queryable_and_closed_set()",
    ] {
        assert!(
            primitive_test_source.contains(regression_locator)
                || logic_test_source.contains(regression_locator)
                || semantics_source.contains(regression_locator),
            "State-contract regressions should be directly locatable through explicit tests; missing `{regression_locator}`."
        );
    }
}
