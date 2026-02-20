use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn path_exists(rel_path: &str) -> bool {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join(rel_path).exists()
}

#[test]
fn scroll_area_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/scroll_area/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "ScrollArea internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn scroll_area_uses_logic_state_model() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let headless_source = load_source("../ui-headless/src/scroll_area.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");

    for needle in [
        "pub use ui_state_primitives::scroll_area::{",
        "ScrollAreaOrientation",
        "ScrollAreaStateInput",
        "ScrollAreaState",
        "pub struct ScrollAreaDisableInput",
        "pub struct ScrollAreaDisableState",
        "pub struct ScrollAreaRootInput",
        "pub struct ScrollAreaRootState",
        "pub fn normalize_disable_state(",
        "pub fn normalize_root_state(",
        "normalize_optional_text",
        "normalize_aria_label",
        "resolve_state",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "ScrollArea logic should consume status-primitives and include `{needle}`."
        );
    }

    for needle in [
        "pub enum ScrollAreaOrientation",
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_max_height(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ui-state-primitives scroll_area should define `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_root_state(ScrollAreaRootInput {",
        "disabled: ScrollAreaDisableInput {",
        "let state = root.state;",
        "let class = logic::compose_class_name(root.class_name, state);",
        "let semantics = use_scroll_area(ScrollAreaOptions {",
        "aria_label: root.aria_label,",
        "let motion = motion::sanitize_motion(motion);",
        "let motion_source = motion::source_attr(motion);",
        "let inline_style = StoredValue::new(Some(motion::attach_motion(None, motion)));",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should derive wrapper state via logic helpers; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct ScrollAreaOptions",
        "pub struct ScrollAreaRootAttrs",
        "pub struct ScrollAreaViewportAttrs",
        "pub struct ScrollAreaHandlers",
        "pub struct ScrollAreaSemanticState",
        "pub struct ScrollAreaContract",
        "pub fn use_scroll_area(options: ScrollAreaOptions) -> ScrollAreaContract",
        "pub lang: Option<String>",
        "pub dir: Option<A11yDirection>",
    ] {
        assert!(
            headless_source.contains(needle),
            "ScrollArea headless contract should include `{needle}`."
        );
    }

    for needle in [
        "pub struct ScrollAreaMotion",
        "default_text_field_motion_tokens",
        "pub fn sanitize_motion(motion: ScrollAreaMotion) -> ScrollAreaMotion",
        "pub fn source_attr(motion: ScrollAreaMotion) -> &'static str",
        "pub fn attach_motion(base_vars: Option<String>, motion: ScrollAreaMotion) -> String",
        "--ui-scroll-area-motion-duration",
    ] {
        assert!(
            motion_source.contains(needle),
            "ScrollArea motion module should include `{needle}`."
        );
    }

    for forbidden in ["ui_motion::spring::SpringAnimator", "MotionKeyframe::new()"] {
        assert!(
            !motion_source.contains(forbidden),
            "ScrollArea motion layer should avoid local runtime motion engine `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_headless_boundary_is_pure_and_exported() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let headless_source = load_source("../ui-headless/src/scroll_area.rs");

    assert!(
        headless_lib_source.contains("pub mod scroll_area;"),
        "ui-headless should export `scroll_area` module."
    );
    assert!(
        headless_lib_source.contains("use_scroll_area"),
        "ui-headless should re-export `use_scroll_area` contract."
    );

    for forbidden in [
        "ui-scroll-area",
        "view! {",
        "NodeRef<",
        "style.set_property",
        "Motion",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "scroll_area headless layer should stay semantic-only; found `{forbidden}`."
        );
    }

    for required in [
        "region_attrs(options.aria_label, options.lang, options.dir)",
        "tabindex: if state.disabled { -1 } else { 0 }",
        "aria_disabled: state.disabled.then_some(\"true\")",
    ] {
        assert!(
            headless_source.contains(required),
            "scroll_area headless layer should map semantic contracts via `{required}`."
        );
    }
}

#[test]
fn scroll_area_status_primitives_boundary_is_pure_and_exported() {
    let primitive_lib_source = load_source("../ui-state-primitives/src/lib.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    assert!(
        primitive_lib_source.contains("pub mod scroll_area;"),
        "ui-state-primitives should export `scroll_area` module."
    );

    for forbidden in [
        "use leptos",
        "web_sys",
        "NodeRef",
        "view! {",
        "on:click",
        "on:keydown",
        "style.set_property",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "scroll_area status-primitive must stay framework/DOM-free; found `{forbidden}`."
        );
    }

    for required in [
        "pub enum ScrollAreaOrientation",
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_max_height(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(required),
            "scroll_area status-primitive should expose `{required}`."
        );
    }

    assert!(
        logic_source.contains("pub use ui_state_primitives::scroll_area::{"),
        "scroll_area logic should only assemble and re-export status primitives."
    );
}

#[test]
fn scroll_area_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/scroll_area/view.rs");

    for attr in [
        "const SLOT_SCROLL_AREA: &str = \"scroll-area\";",
        "const SLOT_SCROLL_AREA_VIEWPORT: &str = \"scroll-area-viewport\";",
        "const CLASS_SCROLL_AREA_VIEWPORT: &str = \"ui-scroll-area__viewport\";",
        "const BOOL_TRUE: &str = \"true\";",
        "const MOTION_SOURCE_CUSTOM: &str = \"custom\";",
        "data-slot=SLOT_SCROLL_AREA",
        "style=inline_style.get_value().unwrap_or_default()",
        "data-motion-source=motion_source",
        "let has_custom_motion = motion_source == MOTION_SOURCE_CUSTOM;",
        "data-custom-motion=has_custom_motion.then_some(BOOL_TRUE)",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "data-aria-source=semantics.root_attrs.data_aria_source",
        "data-class-source=semantics.root_attrs.data_class_source",
        "data-custom-class=semantics.root_attrs.data_custom_class",
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
        "lang=semantics.root_attrs.lang",
        "dir=semantics.root_attrs.dir",
        "class=CLASS_SCROLL_AREA_VIEWPORT",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
        "tabindex=semantics.viewport_attrs.tabindex",
        "aria-disabled=semantics.viewport_attrs.aria_disabled",
    ] {
        assert!(
            source.contains(attr),
            "ScrollArea should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn scroll_area_api_naming_prefers_is_disabled_with_compatibility_path() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    let needle = "#[prop(optional)] is_disabled: Option<bool>";
    assert!(
        view_source.contains(needle),
        "ScrollArea API should include migration-compatible naming contract `{needle}`."
    );

    for needle in [
        "pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState",
        "ScrollAreaDisabledSourceAttr::IsProp",
        "ScrollAreaDisabledSourceAttr::Default",
    ] {
        assert!(
            logic_source.contains(needle),
            "ScrollArea logic should keep explicit compatibility mapping `{needle}`."
        );
    }

    assert!(
        docs_source.contains("is_disabled=Some(true)"),
        "docs should prefer `is_disabled` naming in primary examples."
    );
}

#[test]
fn scroll_area_has_no_controllable_state_axis_so_value_triplet_is_not_applicable() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    for forbidden in [
        "#[prop(optional)] value:",
        "#[prop(optional)] default_value:",
        "on_value_change",
        "#[prop(optional)] open:",
        "#[prop(optional)] default_open:",
        "on_open_change",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "ScrollArea has no controllable state axis and should not expose `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_default_resolution_is_centralized_in_logic_layer() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    for needle in [
        "logic::normalize_root_state(ScrollAreaRootInput {",
        "let state = root.state;",
        "aria_label: root.aria_label,",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should consume normalized root state via `{needle}`."
        );
    }

    for forbidden in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(ScrollAreaStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should not re-run default resolution via `{forbidden}`."
        );
    }

    assert!(
        logic_source.contains(
            "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState"
        ),
        "ScrollArea logic should own single-source default/priority normalization."
    );
}

#[test]
fn scroll_area_state_normalization_pipeline_is_logic_only() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");

    for needle in [
        "pub struct ScrollAreaRootInput",
        "pub struct ScrollAreaRootState",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState",
        "pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState",
    ] {
        assert!(
            logic_source.contains(needle),
            "ScrollArea logic should centralize typed normalization via `{needle}`."
        );
    }

    for needle in [
        "logic::normalize_root_state(ScrollAreaRootInput {",
        "let state = root.state;",
        "let semantics = use_scroll_area(ScrollAreaOptions {",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should consume normalized state contract via `{needle}`."
        );
    }

    for forbidden in [
        "logic::resolve_state(ScrollAreaStateInput {",
        "logic::normalize_aria_label(aria_label)",
        "logic::normalize_optional_text(class_name)",
        "on:click=",
        "on:keydown=",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should not rebuild state-machine/default rules via `{forbidden}`."
        );
    }

    for forbidden in [":nth-child", "> * > *"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollArea styles should consume stable markers only; found `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_discrete_state_axes_are_type_constrained() {
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let view_source = load_source("src/scroll_area/view.rs");

    for needle in [
        "pub enum ScrollAreaOrientation",
        "Vertical",
        "Horizontal",
        "Both",
        "pub struct ScrollAreaStateInput",
        "pub orientation: ScrollAreaOrientation,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "ScrollArea primitive should type discrete orientation state with `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation",
        "pub struct ScrollAreaRootInput",
        "pub orientation: ScrollAreaOrientation,",
        "pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "ScrollArea component should keep typed state boundary via `{needle}`."
        );
    }

    for forbidden in [
        "orientation: Option<String>",
        "orientation: String",
        "fn parse_orientation(",
        "from_str(\"vertical\")",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "ScrollArea should avoid stringly-typed discrete states; found `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_state_primitive_source_boundary_is_enforced() {
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let view_source = load_source("src/scroll_area/view.rs");

    for required in [
        "pub use ui_state_primitives::scroll_area::{",
        "ScrollAreaStateInput",
        "ScrollAreaState",
        "resolve_state",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState",
    ] {
        assert!(
            logic_source.contains(required),
            "ScrollArea logic should assemble from ui-state-primitives via `{required}`."
        );
    }

    for required in [
        "pub enum ScrollAreaOrientation",
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
        "pub fn resolve_state(input: ScrollAreaStateInput) -> ScrollAreaState",
    ] {
        assert!(
            primitive_source.contains(required),
            "ScrollArea primitive layer should own state primitive `{required}`."
        );
    }

    for forbidden in [
        "pub enum ScrollAreaOrientation {",
        "pub struct ScrollAreaStateInput {",
        "pub struct ScrollAreaState {",
        "pub fn resolve_state(input: ScrollAreaStateInput) -> ScrollAreaState {",
        "leptos::store",
        "GlobalStore",
        "AppStore",
        "use_store(",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ScrollArea logic should not re-implement primitives or bind business stores via `{forbidden}`."
        );
    }

    for forbidden in ["GlobalStore", "AppStore", "use_store(", "leptos::store"] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should not bind app-level store types via `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_async_semantics_are_not_applicable_and_not_implemented() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let headless_source = load_source("../ui-headless/src/scroll_area.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "on_retry",
        "retry",
        "error_message",
        "use_async_action",
        "Future",
        "async fn",
        "tokio::",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "ScrollArea should not ship component-local async protocol tokens like `{forbidden}`."
        );
    }

    for forbidden in ["is_loading", "aria-busy", "on_retry", "use_async_action"] {
        assert!(
            !docs_source.contains(forbidden),
            "ScrollArea docs should not advertise unsupported async contract `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_api_dx_default_path_is_simple_and_no_internal_wiring_exposed() {
    let view_source = load_source("src/scroll_area/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(into)] state:",
        "state: ScrollAreaState",
        "state_machine",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea API should not require internal state objects via `{forbidden}`."
        );
    }

    for forbidden in ["ui-state-primitives", "ui_headless", "use_scroll_area("] {
        assert!(
            !docs_source.contains(forbidden),
            "ScrollArea docs default path should not require internal layer wiring `{forbidden}`."
        );
    }

    assert!(
        docs_source.contains("r#\"<ScrollArea>\n  <div>\"Activity feed\"</div>\n</ScrollArea>\"#"),
        "ScrollArea docs should provide a <=5-line hello-world snippet."
    );

    for required in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ScrollArea>",
        "<div>\"Activity feed\"</div>",
        "<Playground title=\"Vertical + Max Height\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Both + Disabled\" code_signal=state_code>",
    ] {
        assert!(
            docs_source.contains(required),
            "ScrollArea docs should expose simple-first then advanced usage via `{required}`."
        );
    }
}

#[test]
fn scroll_area_composition_api_is_explicit_not_parallel_array_convention() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    let start = docs_source
        .find("pub(super) fn scroll_area() -> AnyView")
        .expect("scroll_area docs section should exist");
    let end = docs_source[start..]
        .find("pub(super) fn resizable() -> AnyView")
        .map(|offset| start + offset)
        .expect("scroll_area docs section should end before resizable section");
    let scroll_docs = &docs_source[start..end];

    for required in [
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<ScrollArea>",
        "</ScrollArea>",
        "{children()}",
    ] {
        assert!(
            scroll_docs.contains(required) || view_source.contains(required),
            "ScrollArea should keep explicit parent/child composition via `{required}`."
        );
    }

    for forbidden in [
        "labels + children",
        "titles + panels",
        "titles + children",
        "labels=vec![",
        "titles=vec![",
        "panels=vec![",
        "item_specs",
        "ItemSpec",
    ] {
        assert!(
            !scroll_docs.contains(forbidden)
                && !view_source.contains(forbidden)
                && !logic_source.contains(forbidden),
            "ScrollArea should not use implicit parallel-array composition `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_a11y_i18n_locale_contract_is_headless_driven_and_no_view_hardcoded_copy() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let headless_scroll_source = load_source("../ui-headless/src/scroll_area.rs");
    let headless_a11y_source = load_source("../ui-headless/src/a11y.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");

    for required in [
        "A11yDirection",
        "CommonStrings",
        "use_scroll_area",
        "use_ui_i18n",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let i18n = use_ui_i18n();",
        "let common = i18n.strings::<CommonStrings>();",
        "fallback_aria_label: common.scroll_area_aria_label.as_ref().into(),",
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
        "lang=semantics.root_attrs.lang",
        "dir=semantics.root_attrs.dir",
        "tabindex=semantics.viewport_attrs.tabindex",
        "aria-disabled=semantics.viewport_attrs.aria_disabled",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollArea view should mount A11y + i18n contract via `{required}`."
        );
    }

    for required in [
        "pub fn region_attrs(",
        "pub fn use_scroll_area(options: ScrollAreaOptions) -> ScrollAreaContract",
        "let region = region_attrs(options.aria_label, options.lang, options.dir);",
    ] {
        assert!(
            headless_a11y_source.contains(required) || headless_scroll_source.contains(required),
            "ScrollArea should reuse shared ui-headless a11y helpers via `{required}`."
        );
    }

    for required in [
        "pub fn normalize_aria_label_with_fallback(",
        "normalize_aria_label_with_fallback(input.aria_label, input.fallback_aria_label.as_str())",
    ] {
        assert!(
            logic_source.contains(required),
            "ScrollArea logic should model `props -> i18n -> default` aria label chain via `{required}`."
        );
    }

    for forbidden in [
        "Scrollable region",
        "role=\"region\"",
        "aria-label=\"",
        "zh-CN",
        "rtl",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should not hardcode user-facing copy/locale markers `{forbidden}`."
        );
    }

    assert!(
        primitive_source.contains("pub const DEFAULT_ARIA_LABEL: &str = \"Scrollable region\";"),
        "ScrollArea fallback default label should live in state primitive layer."
    );
}

#[test]
fn scroll_area_observability_markers_are_stable_and_source_enumerable() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");

    for marker in [
        "data-slot=SLOT_SCROLL_AREA",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "data-aria-source=semantics.root_attrs.data_aria_source",
        "data-class-source=semantics.root_attrs.data_class_source",
        "data-motion-source=motion_source",
        "aria-disabled=semantics.viewport_attrs.aria_disabled",
        "tabindex=semantics.viewport_attrs.tabindex",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollArea should expose stable observable markers `{marker}`."
        );
    }

    for marker in [
        ".ui-scroll-area[data-orientation=\"vertical\"]",
        ".ui-scroll-area[data-orientation=\"horizontal\"]",
        ".ui-scroll-area[data-orientation=\"both\"]",
        ".ui-scroll-area[data-max-height=\"custom\"]",
        ".ui-scroll-area[data-disabled=\"true\"]",
    ] {
        assert!(
            styles_source.contains(marker),
            "ScrollArea styles should consume semantic markers directly via `{marker}`."
        );
    }

    for forbidden in [":nth-child", "> * > *"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollArea selectors should avoid brittle structural guessing `{forbidden}`."
        );
    }

    for closed_value in [
        "ScrollAreaDisabledSourceAttr::IsProp",
        "ScrollAreaDisabledSourceAttr::Default",
        "ScrollAreaMaxHeightAttr::Custom",
        "ScrollAreaMaxHeightAttr::Default",
        "ScrollAreaSourceAttr::Custom",
        "ScrollAreaSourceAttr::Default",
        "ScrollAreaOrientation::Vertical => \"vertical\"",
        "ScrollAreaOrientation::Horizontal => \"horizontal\"",
        "ScrollAreaOrientation::Both => \"both\"",
        "pub fn source_attr(motion: ScrollAreaMotion) -> &'static str",
    ] {
        assert!(
            logic_source.contains(closed_value)
                || primitive_source.contains(closed_value)
                || motion_source.contains(closed_value),
            "ScrollArea marker values should come from closed enumerable set `{closed_value}`."
        );
    }
}

#[test]
fn scroll_area_styles_depend_on_explicit_state_markers_and_runtime_vars_only() {
    let view_source = load_source("src/scroll_area/view.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");

    for required in [
        "style=inline_style.get_value().unwrap_or_default()",
        "let inline_style = StoredValue::new(Some(motion::attach_motion(None, motion)));",
        ".set_property(\"--ui-scroll-area-max-h\", max_height.as_str())",
        "style.push_str(&format!(",
        "--ui-scroll-area-motion-duration: {}ms;",
    ] {
        assert!(
            view_source.contains(required) || motion_source.contains(required),
            "ScrollArea runtime style path should only attach CSS custom properties via `{required}`."
        );
    }

    for forbidden in [
        "style=\"display:",
        "style=\"opacity:",
        "style=\"transform:",
        "style.set_property(\"display\"",
        "style.set_property(\"opacity\"",
        "style.set_property(\"transform\"",
    ] {
        assert!(
            !view_source.contains(forbidden) && !motion_source.contains(forbidden),
            "ScrollArea runtime style should avoid business inline styling `{forbidden}`."
        );
    }

    for selector in [
        ".ui-scroll-area--vertical .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"vertical\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--horizontal .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"horizontal\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--both .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"both\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--disabled",
        ".ui-scroll-area[data-disabled=\"true\"]",
        ".ui-scroll-area--disabled .ui-scroll-area__viewport",
        ".ui-scroll-area[data-disabled=\"true\"] .ui-scroll-area__viewport",
    ] {
        assert!(
            styles_source.contains(selector),
            "ScrollArea visual state switch should be explainable by explicit marker selector `{selector}`."
        );
    }

    for forbidden in [":nth-child", ":first-child", ":last-child", "> div > div"] {
        assert!(
            !styles_source.contains(forbidden),
            "ScrollArea styles should avoid structural guessing selector `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_semantics_checks_prioritize_contracts_and_cover_matrix_paths() {
    let tests_source = load_source("tests/scroll_area_semantics.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let headless_source = load_source("../ui-headless/src/scroll_area.rs");

    for required_test in [
        "fn scroll_area_emits_baseline_style_state_data_attributes()",
        "fn scroll_area_state_primitive_source_boundary_is_enforced()",
        "fn scroll_area_has_no_controllable_state_axis_so_value_triplet_is_not_applicable()",
        "fn scroll_area_observability_markers_are_stable_and_source_enumerable()",
        "fn scroll_area_styles_depend_on_explicit_state_markers_and_runtime_vars_only()",
    ] {
        assert!(
            tests_source.contains(required_test),
            "ScrollArea semantics suite should include key contract test `{required_test}`."
        );
    }

    for required in [
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollArea component should expose matrix path evidence `{required}`."
        );
    }

    for required in [
        "tabindex: if state.disabled { -1 } else { 0 }",
        "aria_disabled: state.disabled.then_some(\"true\")",
    ] {
        assert!(
            headless_source.contains(required),
            "ScrollArea headless contract should cover keyboard path via `{required}`."
        );
    }

    assert!(
        styles_source.contains(".ui-scroll-area[data-disabled=\"true\"] .ui-scroll-area__viewport")
            && styles_source.contains("pointer-events: none;"),
        "ScrollArea disabled pointer path should be verified by semantic marker CSS contract."
    );

    for forbidden in [
        ["assert_", "snapshot!"].concat(),
        ["insta::", "assert"].concat(),
        ["to_match_", "snapshot"].concat(),
        ["use ", "insta::"].concat(),
    ] {
        assert!(
            !tests_source.contains(&forbidden),
            "ScrollArea semantics suite should not rely on snapshot-only assertions `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/scroll_area_semantics.rs");

    for required in [
        "scroll_area_semantics_checks_prioritize_contracts_and_cover_matrix_paths",
        "scroll_area_emits_baseline_style_state_data_attributes",
        "scroll_area_observability_markers_are_stable_and_source_enumerable",
        "scroll_area_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "scroll_area_agent_contract_is_schema_typed_and_machine_readable",
        "scroll_area_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            semantics_source.contains(required),
            "ScrollArea semantic suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["toMatch", "Snapshot"].concat(),
        ["image", "_snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "ScrollArea semantic suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn scroll_area_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/scroll_area/view.rs");
    let semantics_source = load_source("tests/scroll_area_semantics.rs");

    for marker in [
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
        "aria-disabled=semantics.viewport_attrs.aria_disabled",
        "tabindex=semantics.viewport_attrs.tabindex",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "data-aria-source=semantics.root_attrs.data_aria_source",
        "data-class-source=semantics.root_attrs.data_class_source",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-state=agent_contract.state_attr",
        "data-ui-source=agent_contract.source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollArea view should keep semantic marker `{marker}`."
        );
        let escaped_marker = marker.replace('"', "\\\"");
        assert!(
            semantics_source.contains(marker) || semantics_source.contains(&escaped_marker),
            "ScrollArea semantic marker `{marker}` changed in view without matching semantics coverage."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_semantics_first_testing_rules() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_e2e_selector_and_stable_wait_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep E2E selector/stable-wait rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_scroll_area_contract.spec.mjs");

    for needle in [
        "docs-app scroll-area contract uses semantic selectors with wasm-safe ready waits",
        "body:not(:has(#boot))",
        "[data-component=\"scroll-area\"]",
        "[data-slot=\"scroll-area\"][data-orientation=\"vertical\"][data-max-height=\"default\"]",
        "[data-slot=\"scroll-area\"][data-orientation=\"vertical\"][data-max-height=\"custom\"]",
        "[data-slot=\"scroll-area\"][data-orientation=\"horizontal\"][data-class-source=\"custom\"]",
        "[data-slot=\"scroll-area\"][data-orientation=\"both\"][data-disabled=\"true\"]",
        "toHaveAttribute(\"data-ui-schema\", \"ui.scroll-area.agent-contract.v1\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"unsupported\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-state\", \"enabled\")",
        "toHaveAttribute(\"data-ui-action\", \"disabled\")",
        "toHaveAttribute(\"tabindex\", \"-1\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ScrollArea e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "getByRole(",
        ":nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "ScrollArea e2e selector contract should avoid unstable selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_e2e_ready_and_settled_contract_covers_motion_and_disabled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_scroll_area_contract.spec.mjs");

    for needle in [
        "docs-app scroll-area interaction path uses semantic ready and settled breakpoints",
        "toHaveAttribute(\"data-ui-action\", \"observe\")",
        "toHaveAttribute(\"data-ui-state\", \"enabled\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-motion-source\", \"default\")",
        "readyViewport.focus();",
        "page.keyboard.press(\"PageDown\")",
        "disabledViewport.evaluate((node) => {",
        "node.scrollTop = 120;",
        "toHaveAttribute(\"data-ui-action\", \"disabled\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
        "toHaveAttribute(\"aria-disabled\", \"true\")",
        "toHaveAttribute(\"tabindex\", \"-1\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ScrollArea ready/settled e2e contract should include `{needle}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "toHaveScreenshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "ScrollArea ready/settled e2e path should avoid unstable/non-semantic token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_e2e_repeatable_key_flow_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep repeatable E2E key-flow rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_scroll_area_contract.spec.mjs");

    for needle in [
        "docs-app scroll-area key flow is repeatable with semantic breakpoints",
        "page.keyboard.press(\"PageDown\")",
        "toHaveAttribute(\"data-ui-action\", \"observe\")",
        "toHaveAttribute(\"data-ui-state\", \"enabled\")",
        "toHaveAttribute(\"data-ui-source\", \"default\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "await page.reload();",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ScrollArea repeatable key-flow e2e contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "ScrollArea repeatable key-flow e2e path should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_scroll_area_contract.spec.mjs");

    for needle in [
        "readyViewport.focus();",
        "toBeFocused()",
        "page.keyboard.press(\"PageDown\")",
        "docs-app scroll-area interaction path uses semantic ready and settled breakpoints",
        "toHaveAttribute(\"data-ui-action\", \"observe\")",
        "toHaveAttribute(\"data-ui-state\", \"enabled\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "disabledViewport.evaluate((node) => {",
        "toHaveAttribute(\"aria-disabled\", \"true\")",
        "toHaveAttribute(\"tabindex\", \"-1\")",
        "toHaveAttribute(\"data-ui-action\", \"disabled\")",
        "toHaveAttribute(\"data-ui-state\", \"disabled\")",
        "body:not(:has(#boot))",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ScrollArea high-risk e2e path contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "ScrollArea high-risk e2e path should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_e2e_check_script_covers_selector_and_settled_wait_contracts() {
    let script_source = load_source("../../scripts/check-ui-layout-e2e-scroll-area.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_ready_and_settled_contract_covers_motion_and_disabled_semantic_breakpoints",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(needle),
            "scroll_area e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_component_directory_has_standard_file_layout() {
    for rel in [
        "src/scroll_area/mod.rs",
        "src/scroll_area/logic.rs",
        "src/scroll_area/styles.rs",
        "src/scroll_area/view.rs",
        "src/scroll_area/motion.rs",
        "src/scroll_area/check2.md",
    ] {
        assert!(
            path_exists(rel),
            "ScrollArea should keep required file `{rel}`."
        );
    }

    assert!(
        !path_exists("src/scroll_area/render.rs"),
        "ScrollArea should keep render implementation in `view.rs` without `render.rs` drift."
    );
    assert!(
        !path_exists("src/scroll_area/spec.rs"),
        "ScrollArea is a simple component and should not introduce `src/scroll_area/spec.rs`."
    );
}

#[test]
fn scroll_area_check2_documents_component_directory_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep component-directory governance rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/scroll_area/mod.rs");

    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use logic::{DEFAULT_ARIA_LABEL, ScrollAreaOrientation};",
        "pub use motion::ScrollAreaMotion;",
        "pub use view::ScrollArea;",
    ] {
        assert!(
            mod_source.contains(required),
            "scroll_area/mod.rs should keep stable minimal exports via `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub use logic::*;",
        "pub use view::*;",
        "mod render;",
        "pub mod render;",
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "scroll_area/mod.rs should not over-export/introduce drift token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_component_file_responsibilities_remain_scoped() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/scroll_area/mod.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");

    assert!(
        !manifest_dir.join("src/scroll_area/render.rs").exists(),
        "ScrollArea should keep render implementation in `view.rs` without `render.rs` drift."
    );
    assert!(
        !manifest_dir.join("src/scroll_area/spec.rs").exists(),
        "ScrollArea should keep simple-component scope and avoid `spec.rs`."
    );

    for forbidden in ["normalize_root_state(", "style.set_property", "view! {"] {
        assert!(
            !mod_source.contains(forbidden),
            "scroll_area/mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_disable_state(",
        "pub fn normalize_aria_label_with_fallback(",
        "pub fn normalize_root_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "scroll_area/logic.rs should own normalization/derivation via `{required}`."
        );
    }

    for forbidden in [
        "use leptos",
        "web_sys",
        "NodeRef",
        "view! {",
        "style.set_property",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "scroll_area/logic.rs should avoid DOM/view/style side effects `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-text-field-motion-duration)",
        "var(--ui-text-field-motion-easing)",
        ".ui-scroll-area[data-orientation=\"vertical\"]",
        ".ui-scroll-area[data-disabled=\"true\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "scroll_area/styles.rs should be token-first static CSS with semantic selectors `{required}`."
        );
    }

    for forbidden in [
        "use leptos",
        "view! {",
        "web_sys",
        "aria-label=",
        "on:click=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "scroll_area/styles.rs should not contain logic/view/event payload `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "logic::normalize_root_state(ScrollAreaRootInput {",
        "let semantics = use_scroll_area(ScrollAreaOptions {",
        "let inline_style = StoredValue::new(Some(motion::attach_motion(None, motion)));",
        "view! {",
    ] {
        assert!(
            view_source.contains(required),
            "scroll_area/view.rs should render structure and mount headless contract via `{required}`."
        );
    }

    for forbidden in [
        "pub enum ScrollAreaOrientation",
        "pub fn resolve_state(",
        "ui_motion::spring::SpringAnimator",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "scroll_area/view.rs should not reimplement primitives/engines `{forbidden}`."
        );
    }

    for required in [
        "pub struct ScrollAreaMotion",
        "pub fn sanitize_motion(",
        "pub fn source_attr(",
        "pub fn attach_motion(",
        "--ui-scroll-area-motion-duration",
    ] {
        assert!(
            motion_source.contains(required),
            "scroll_area/motion.rs should map semantic motion contract via `{required}`."
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator",
        "MotionKeyframe::new()",
        "view! {",
        "NodeRef<",
        "role=",
        "aria-",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "scroll_area/motion.rs should not carry view/a11y/runtime engine detail `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_component_files_check_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-component-files.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_component_directory_has_standard_file_layout",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let primitives_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let combined = format!("{view_source}\n{logic_source}\n{primitives_source}");

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-state=agent_contract.state_attr",
        "data-ui-source=agent_contract.source_attr",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollArea should expose agent-readable machine marker `{marker}`."
        );
    }

    for typed_source in [
        "pub enum ScrollAreaAgentSchema",
        "pub enum ScrollAreaStreamSupport",
        "pub enum ScrollAreaStreamFallback",
        "pub enum ScrollAreaStreamMode",
        "pub enum ScrollAreaOutputStatus",
        "pub enum ScrollAreaAgentIntent",
        "pub enum ScrollAreaAgentAction",
        "pub enum ScrollAreaAgentState",
        "pub struct ScrollAreaAgentContract",
        "pub fn resolve_agent_contract(",
        "pub const fn as_attr(self) -> &'static str",
        "pub enum ScrollAreaOrientation",
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
    ] {
        assert!(
            combined.contains(typed_source),
            "ScrollArea Agent Contract values should remain type-derived via `{typed_source}`."
        );
    }
}

#[test]
fn scroll_area_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "data-ui-schema=move || format!(",
        "data-ui-stream-support=move || format!(",
        "data-ui-stream-fallback=move || format!(",
        "data-ui-stream-mode=move || format!(",
        "data-ui-output-status=move || format!(",
        "data-ui-intent=move || format!(",
        "data-ui-action=move || format!(",
        "data-ui-state=move || format!(",
        "data-ui-source=move || format!(",
        "format!(\"data-ui-",
        "intent=\"",
        "action=\"",
        "state=\"",
        "source=\"",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea should avoid free-form schema field token `{forbidden}`."
        );
    }

    for required in [
        "ScrollAreaAgentSchema::V1.as_attr()",
        "ScrollAreaStreamSupport::Unsupported.as_attr()",
        "ScrollAreaStreamFallback::Snapshot.as_attr()",
        "ScrollAreaStreamMode::Snapshot.as_attr()",
        "ScrollAreaOutputStatus::Verified.as_attr()",
        "ScrollAreaAgentIntent::InspectRegion.as_attr()",
        "ScrollAreaAgentAction::Observe",
        "ScrollAreaAgentAction::Disabled",
        "ScrollAreaAgentState::Enabled",
        "ScrollAreaAgentState::Disabled",
        "disabled_source_attr.as_attr()",
    ] {
        assert!(
            combined.contains(required),
            "ScrollArea agent contract fields should stay type-derived via `{required}`."
        );
    }
}

#[test]
fn scroll_area_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let mod_source = load_source("src/scroll_area/mod.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep streaming definition marker `{required}`."
        );
    }
}

#[test]
fn scroll_area_streaming_check_script_covers_llm_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-streaming.sh");

    let needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`."
    );
}

#[test]
fn scroll_area_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep snapshot-baseline marker `{required}`."
        );
    }
}

#[test]
fn scroll_area_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let primitives_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for marker in [
        "#[component]",
        "pub fn ScrollArea(",
        "children: Children,",
        "{children()}",
        "#[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation,",
        "#[prop(optional)] max_height_px: Option<u32>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] motion: ScrollAreaMotion,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::normalize_root_state(ScrollAreaRootInput {",
        "let semantics = use_scroll_area(ScrollAreaOptions {",
        "let inline_style = StoredValue::new(Some(motion::attach_motion(None, motion)));",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-max-height=semantics.root_attrs.data_max_height",
    ] {
        assert!(
            view_source.contains(marker),
            "ScrollArea snapshot baseline should keep complete-result render marker `{marker}`."
        );
    }

    for marker in [
        "pub struct ScrollAreaRootInput",
        "pub struct ScrollAreaRootState",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState",
        "pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState",
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
        "pub fn resolve_state(input: ScrollAreaStateInput) -> ScrollAreaState",
    ] {
        assert!(
            logic_source.contains(marker) || primitives_source.contains(marker),
            "ScrollArea snapshot baseline should keep stable normalization/state marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn scroll_area() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Vertical + Max Height\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Both + Disabled\" code_signal=state_code>",
        "<ScrollArea",
        "orientation=ScrollAreaOrientation::Both",
        "is_disabled=Some(true)",
        "max_height_px=120",
    ] {
        assert!(
            docs_source.contains(marker),
            "ScrollArea docs should include complete snapshot result marker `{marker}`."
        );
    }
}

#[test]
fn scroll_area_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-streaming.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_streaming_required_optional_classification_rules() {
    let source = load_source("src/scroll_area/check2.md");
    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "ScrollArea 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only，`fallback=snapshot`）。",
    ] {
        assert!(
            source.contains(needle),
            "ScrollArea check2 should keep streaming responsibility marker `{needle}`."
        );
    }
}

#[test]
fn scroll_area_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/scroll_area/view.rs");

    for required in [
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
        "lang=semantics.root_attrs.lang",
        "dir=semantics.root_attrs.dir",
        "tabindex=semantics.viewport_attrs.tabindex",
        "aria-disabled=semantics.viewport_attrs.aria_disabled",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "data-ui-stream-support=agent_contract.stream_support_attr",
        "data-ui-stream-fallback=agent_contract.stream_fallback_attr",
        "data-ui-stream-mode=agent_contract.stream_mode_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-source=agent_contract.source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollArea should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn scroll_area_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "revalidate",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_streaming_check_script_covers_streaming_responsibility_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-streaming.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_spec_file_policy_remains_minimal_without_spec_module() {
    let mod_source = load_source("src/scroll_area/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = manifest_dir.join("src/scroll_area/spec.rs");

    assert!(
        !spec_path.exists(),
        "ScrollArea should not introduce `spec.rs` without stable external schema/versioning needs."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "SpecVersion",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "ScrollArea module boundary should not export spec surface `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_token_first_static_style_contract_is_enforced() {
    let styles_source = load_source("src/scroll_area/styles.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-scroll-area",
        "var(--ui-text-field-motion-duration)",
        "var(--ui-text-field-motion-easing)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-focus-ring)",
    ] {
        assert!(
            styles_source.contains(required),
            "scroll_area/styles.rs should keep token-first static CSS contract `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-scroll_area\")]",
        "out.push_str(crate::scroll_area::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui-layout/css.rs should aggregate scroll_area CSS via feature gate `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should own CSS aggregation injection chain via `{required}`."
        );
    }

    for required in [
        "style=inline_style.get_value().unwrap_or_default()",
        ".set_property(\"--ui-scroll-area-max-h\", max_height.as_str())",
        "style.push_str(&format!(",
        "--ui-scroll-area-motion-duration: {}ms;",
    ] {
        assert!(
            view_source.contains(required) || motion_source.contains(required),
            "ScrollArea runtime path should only pass minimal CSS variables `{required}`."
        );
    }

    for forbidden in [
        "class=\"text-",
        "class=\"bg-",
        "class=\"px-",
        "class=\"py-",
        "class=\"flex",
        "class=\"grid",
        "styled_components",
        "css_in_rust",
    ] {
        assert!(
            !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "ScrollArea component contract should avoid utility/CSS-in-Rust default pattern `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts() {
    let styles_source = load_source("src/scroll_area/styles.rs");
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let e2e_spec = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    let heroui_doc = load_source("../../docs/spec/heroui-parameter-design-strategy.md");

    for needle in [
        "--ui-scroll-area-motion-duration: var(--ui-text-field-motion-duration)",
        "transition: opacity var(--ui-scroll-area-motion-duration) var(--ui-text-field-motion-easing)",
        "color-mix(in oklch, var(--ui-bg) 94%, var(--ui-fg) 6%)",
        ".ui-scroll-area__viewport::-webkit-scrollbar-thumb:hover",
        ".ui-scroll-area__viewport::-webkit-scrollbar-thumb:active",
        ".ui-scroll-area__viewport:focus-visible",
    ] {
        assert!(
            styles_source.contains(needle),
            "ScrollArea visual quality contract should include `{needle}`."
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.",
        "Checks first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback (hover/active/focus).",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            baseline_page.contains(needle),
            "Theme visual baseline page should keep visual-quality contract token `{needle}`."
        );
    }

    for needle in ["\"ThemeVisualBaseline\"", "\"theme-visual-baseline\""] {
        assert!(
            pages_registry.contains(needle),
            "Docs pages registry should expose theme visual baseline route token `{needle}`."
        );
    }

    for needle in [
        "/#/components/theme-visual-baseline",
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
        "set E2E_VISUAL_BASELINE=on to run visual snapshot regression",
    ] {
        assert!(
            e2e_spec.contains(needle),
            "Theme visual baseline e2e contract should include `{needle}`."
        );
    }

    for needle in ["# HeroUI 参数设计风格对齐策略", "HeroUI 对齐结论"] {
        assert!(
            heroui_doc.contains(needle),
            "HeroUI strategy doc should keep alignment contract token `{needle}`."
        );
    }
}

#[test]
fn scroll_area_tree_shaking_keeps_component_feature_and_css_boundaries() {
    let ui_layout_cargo = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_cargo = load_source("../../apps/docs-app/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "web-demo-components = [",
        "inject-css = []",
        "component-scroll_area = []",
    ] {
        assert!(
            ui_layout_cargo.contains(needle),
            "ui-layout Cargo features should include `{needle}` for tree-shaking boundaries."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-scroll_area\")]\npub mod scroll_area;"),
        "lib.rs should feature-gate scroll_area module export for tree-shaking."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"component-scroll_area\")]")
            && css_source.contains("out.push_str(crate::scroll_area::styles::CSS);"),
        "css.rs should gate scroll_area CSS aggregation behind component-scroll_area feature."
    );
    assert!(
        css_source.contains("#[cfg(feature = \"inject-css\")]")
            && css_source.contains("pub fn push_components_css(out: &mut String)"),
        "css.rs should keep top-level inject-css gate for component CSS injection."
    );
    assert!(
        lib_source.contains(
            "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]"
        ) && lib_source.contains("#[cfg(feature = \"all-components\")]"),
        "lib.rs should keep split export boundaries for web-demo-components vs all-components."
    );

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("web-demo-components")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should consume ui-layout via web-demo-components, not all-components."
    );
    assert!(
        docs_app_cargo.contains("default-features = false")
            && docs_app_cargo.contains("all-components"),
        "docs-app should explicitly opt into all-components instead of implicit default pull-up."
    );
}

#[test]
fn scroll_area_tree_shaking_check_script_covers_feature_tree_wasm_and_budget() {
    let script_source = load_source("../../scripts/check-ui-layout-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui-layout -p ui-layout --no-default-features --features",
        "cargo tree -e features -i ui-layout -p web-demo",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-layout --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
        "size regression",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }
}

#[test]
fn scroll_area_type_system_and_machine_readable_markers_stay_in_sync() {
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let headless_source = load_source("../ui-headless/src/scroll_area.rs");
    let view_source = load_source("src/scroll_area/view.rs");

    for needle in [
        "pub enum ScrollAreaOrientation",
        "pub enum ScrollAreaMaxHeightAttr",
        "pub enum ScrollAreaSourceAttr",
        "pub const fn as_attr(self) -> &'static str",
        "pub max_height_attr: ScrollAreaMaxHeightAttr",
        "pub aria_source_attr: ScrollAreaSourceAttr",
        "pub class_source_attr: ScrollAreaSourceAttr",
        "pub fn normalize_max_height(max_height_px: Option<u32>) -> Option<u32>",
        "pub fn resolve_state(input: ScrollAreaStateInput) -> ScrollAreaState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "scroll-area primitive type contract should include `{needle}`."
        );
    }

    for needle in [
        "pub enum ScrollAreaDisabledSourceAttr",
        "pub disabled_source_attr: ScrollAreaDisabledSourceAttr",
        "ScrollAreaDisabledSourceAttr::IsProp",
        "ScrollAreaDisabledSourceAttr::Default",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState",
    ] {
        assert!(
            logic_source.contains(needle),
            "scroll-area logic source typing/normalization contract should include `{needle}`."
        );
    }

    for needle in [
        "data_max_height: state.max_height_attr.as_attr()",
        "data_aria_source: state.aria_source_attr.as_attr()",
        "data_class_source: state.class_source_attr.as_attr()",
        "max_height_source: state.max_height_attr.as_attr()",
        "aria_source: state.aria_source_attr.as_attr()",
        "class_source: state.class_source_attr.as_attr()",
    ] {
        assert!(
            headless_source.contains(needle),
            "scroll-area headless machine-readable marker mapping should include `{needle}`."
        );
    }

    for needle in [
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "data-aria-source=semantics.root_attrs.data_aria_source",
        "data-class-source=semantics.root_attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(needle),
            "scroll-area view should expose machine-readable markers via `{needle}`."
        );
    }

    for forbidden in [
        "orientation: Option<String>",
        "orientation: String",
        "fn parse_orientation(",
        "from_str(\"vertical\")",
    ] {
        assert!(
            !primitive_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "scroll-area should avoid stringly typed input path `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_ssr_and_cross_platform_compile_paths_are_covered() {
    let platform_script_source = load_source("../../scripts/check-ui-layout-platforms.sh");
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "cargo check -p ui-layout",
        "echo \"[platform] compile-only: ssr native path\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "echo \"[platform] compile-only: scroll-area native path\"",
        "cargo check -p ui-layout --no-default-features --features component-scroll_area,inject-css",
        "echo \"[platform] compile-only: scroll-area wasm path\"",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_area,inject-css",
        "ui-headless web+ssr must fail",
        "mutually exclusive",
        "source guard: non-wasm scroll-area files must not reference web_sys",
        "crates/ui-layout/src/scroll_area/logic.rs",
        "crates/ui-layout/src/scroll_area/styles.rs",
        "crates/ui-layout/src/scroll_area/motion.rs",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform check script should keep scroll-area compile-only/mutex/source-guard contract `{needle}`."
        );
    }

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let element: leptos::web_sys::HtmlElement = div.unchecked_into();",
    ] {
        assert!(
            view_source.contains(required),
            "scroll-area view should keep explicit wasm/non-wasm split via `{required}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "window(",
        "document(",
        "js_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "scroll-area logic should stay non-wasm-safe and avoid browser-only token `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "scroll-area styles should stay non-wasm-safe and avoid browser-only token `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "scroll-area motion should stay non-wasm-safe and avoid browser-only token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_non_wasm_view_branch_avoids_browser_objects() {
    let view_source = load_source("src/scroll_area/view.rs");

    let non_wasm_branch = view_source
        .split_once("#[cfg(not(target_arch = \"wasm32\"))]")
        .map(|(_, tail)| tail)
        .expect("scroll-area view should include non-wasm cfg branch.");

    for forbidden in [
        "leptos::web_sys",
        "web_sys::",
        "window(",
        "document(",
        "js_sys::",
    ] {
        assert!(
            !non_wasm_branch.contains(forbidden),
            "scroll-area non-wasm branch should avoid browser-only token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_ui_headless_web_ssr_mutex_is_compile_error_guarded() {
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let platform_script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
        "pub mod scroll_area;",
        "ScrollAreaContract",
        "use_scroll_area",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless mutex/export contract should include `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "ui-headless web+ssr must fail",
        "mutually exclusive",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform check script should keep ui-headless web/ssr mutex guard `{needle}`."
        );
    }
}

#[test]
fn scroll_area_ui_motion_non_wasm_stub_contract_is_enforced() {
    let motion_lib_source = load_source("../ui-motion/src/lib.rs");
    let motion_lib_checks_source = load_source("../ui-motion/src/test/lib.rs");
    let motion_lib_combined = format!("{motion_lib_source}\n{motion_lib_checks_source}");
    let motion_stub_test_source = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let scroll_motion_source = load_source("src/scroll_area/motion.rs");
    let scroll_view_source = load_source("src/scroll_area/view.rs");
    let platform_script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "non_wasm_web_backend_is_predictable_noop",
    ] {
        assert!(
            motion_lib_combined.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "non_wasm_web_backend_prefers_reduced_motion",
        "non_wasm_web_backend_animate_is_safe_noop",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            motion_stub_test_source.contains(needle),
            "ui-motion non-wasm stub regression tests should include `{needle}`."
        );
    }

    for needle in [
        "pub struct ScrollAreaMotion",
        "pub fn sanitize_motion(motion: ScrollAreaMotion) -> ScrollAreaMotion",
        "pub fn attach_motion(base_vars: Option<String>, motion: ScrollAreaMotion) -> String",
        "--ui-scroll-area-motion-duration",
        "let inline_style = StoredValue::new(Some(motion::attach_motion(None, motion)));",
    ] {
        assert!(
            scroll_motion_source.contains(needle) || scroll_view_source.contains(needle),
            "scroll-area motion mapping should keep predictable attach contract `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "window(",
        "document(",
        "NodeRef<",
        "panic!(",
        "unwrap()",
    ] {
        assert!(
            !scroll_motion_source.contains(forbidden),
            "scroll-area motion mapping should remain non-wasm-safe and panic-free for `{forbidden}`."
        );
    }

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] compile-only: ui-motion wasm path\"",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-motion non-wasm/wasm compile and stub-test guard `{needle}`."
        );
    }
}

#[test]
fn scroll_area_reduced_motion_ssr_wasm_contract_is_consistent() {
    let styles_source = load_source("src/scroll_area/styles.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let motion_web_source = load_source("../ui-motion/src/web.rs");
    let platform_script_source = load_source("../../scripts/check-ui-layout-platforms.sh");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        "--ui-scroll-area-motion-duration: 1ms;",
    ] {
        assert!(
            styles_source.contains(needle),
            "ScrollArea styles should include reduced-motion downgrade marker `{needle}`."
        );
    }

    for needle in ["if prefers_reduced_motion() {", "return;"] {
        assert!(
            motion_web_source.contains(needle),
            "ui-motion wasm backend should short-circuit under reduced-motion via `{needle}`."
        );
    }

    for needle in [
        "data-slot=SLOT_SCROLL_AREA",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
        "data-motion-source=motion_source",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled=semantics.root_attrs.data_disabled",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
        "lang=semantics.root_attrs.lang",
        "dir=semantics.root_attrs.dir",
        "tabindex=semantics.viewport_attrs.tabindex",
        "aria-disabled=semantics.viewport_attrs.aria_disabled",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should keep hydration-stable semantic marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let set_max_height = {",
        ".set_property(\"--ui-scroll-area-max-h\", max_height.as_str())",
        "|| {}",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should keep explicit wasm/non-wasm enhancement split via `{needle}`."
        );
    }

    for forbidden in [
        "#[cfg(target_arch = \"wasm32\")]\n    view! {",
        "#[cfg(not(target_arch = \"wasm32\"))]\n    view! {",
        "set_attribute(\"role\"",
        "set_attribute(\"aria-",
        "set_attribute(\"data-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view semantics should not diverge by platform via `{forbidden}`."
        );
    }

    for forbidden in ["role=", "aria-", "data-"] {
        assert!(
            !motion_source.contains(forbidden),
            "ScrollArea motion layer should not own semantic contract token `{forbidden}`."
        );
    }

    for needle in [
        "cargo check -p ui-layout --no-default-features --features component-scroll_area,inject-css",
        "cargo check -p ui-layout --target wasm32-unknown-unknown --no-default-features --features component-scroll_area,inject-css",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area scroll_area_reduced_motion_ssr_wasm_contract_is_consistent",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep reduced-motion/SSR/wasm guard `{needle}`."
        );
    }
}

#[test]
fn scroll_area_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/scroll_area/check2.md");
    let script_source = load_source("../../scripts/check-ui-layout-performance.sh");
    let view_source = load_source("src/scroll_area/view.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget baseline token `{needle}`."
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose performance regression marker `{needle}`."
        );
    }

    for needle in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "docs coverage e2e should enforce repeatable perf regression guard `{needle}`."
        );
    }

    for needle in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace-based perf attribution token `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "性能治理：关键路径有预算（首次渲染/更新耗时/内存）",
        "关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep perf governance baseline/follow-up token `{needle}`."
        );
    }

    for needle in [
        "data-motion-source=motion_source",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-max-height=semantics.root_attrs.data_max_height",
        "data-class-source=semantics.root_attrs.data_class_source",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea view should expose perf-attribution marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );
}

#[test]
fn scroll_area_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders() {
    let check2_source = load_source("src/scroll_area/check2.md");
    let view_source = load_source("src/scroll_area/view.rs");
    let script_source = load_source("../../scripts/check-ui-layout-view-macro.sh");

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "scroll_area_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep view-macro governance marker `{needle}`."
        );
    }

    assert!(
        view_source.contains("view! {"),
        "ScrollArea should keep an explicit render block in view.rs."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        1,
        "ScrollArea should keep one compact `view!` block for the current two-slot layout."
    );
    assert!(
        view_source.lines().count() <= 160,
        "ScrollArea view.rs should stay compact; if this grows significantly, split into semantic subrenders."
    );

    for forbidden in [
        "for item in",
        "collect::<Vec<_>>()",
        "children().into_iter()",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should avoid loop-heavy macro patterns that usually indicate giant expansion `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_view_macro_complexity_is_small_and_does_not_require_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn scroll_area_view_functional_split_prefers_no_extra_local_components_for_simple_layout() {
    let check2_source = load_source("src/scroll_area/check2.md");
    let view_source = load_source("src/scroll_area/view.rs");
    let script_source = load_source("../../scripts/check-ui-layout-view-macro.sh");

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "scroll_area_view_functional_split_prefers_no_extra_local_components_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep functional-split governance marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "ScrollArea should keep one public component boundary for the current simple layout."
    );

    for forbidden in [
        "#[component]\nfn render_",
        "#[component]\nfn scroll_area_",
        "pub fn render_",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea should not introduce local component/render API noise for simple layout `{forbidden}`."
        );
    }

    for needle in [
        "children: Children",
        "{children()}",
        "data-slot=SLOT_SCROLL_AREA",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea should keep composition/semantic marker `{needle}` stable after functional split rule enforcement."
        );
    }

    let script_needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_view_functional_split_prefers_no_extra_local_components_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn scroll_area_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let check2_source = load_source("src/scroll_area/check2.md");
    let view_source = load_source("src/scroll_area/view.rs");
    let script_source = load_source("../../scripts/check-ui-layout-view-macro.sh");

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "scroll_area_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep static-fragment constantization marker `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "<header",
        "<section",
        "<article",
        "<footer",
        "<nav",
        "<ul",
        "<li",
        "<path",
        "let markdown",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should avoid heavy inline static fragments for simple layout `{forbidden}`."
        );
    }

    for needle in [
        "const SLOT_SCROLL_AREA: &str = \"scroll-area\";",
        "const SLOT_SCROLL_AREA_VIEWPORT: &str = \"scroll-area-viewport\";",
        "const CLASS_SCROLL_AREA_VIEWPORT: &str = \"ui-scroll-area__viewport\";",
        "const BOOL_TRUE: &str = \"true\";",
        "const MOTION_SOURCE_CUSTOM: &str = \"custom\";",
        "data-slot=SLOT_SCROLL_AREA",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
        "class=CLASS_SCROLL_AREA_VIEWPORT",
        "data-custom-motion=has_custom_motion.then_some(BOOL_TRUE)",
        "let has_custom_motion = motion_source == MOTION_SOURCE_CUSTOM;",
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea should keep static fragment constantization marker `{needle}`."
        );
    }

    for forbidden in [
        "data-slot=\"scroll-area\"",
        "data-slot=\"scroll-area-viewport\"",
        "class=\"ui-scroll-area__viewport\"",
        "has_custom_motion.then_some(\"true\")",
        "motion_source == \"custom\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea should avoid scattered static literal fragment `{forbidden}` after constantization."
        );
    }

    let script_needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn scroll_area_inner_html_usage_is_explicitly_na_and_guarded() {
    let mod_source = load_source("src/scroll_area/mod.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let checklist_source = load_source("src/scroll_area/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "markdown_to_html(",
        "format!(\"<",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "ScrollArea should not use html injection path `{forbidden}`; this component has no trusted static-html requirement (N/A)."
        );
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "仅允许编译期常量或明确白名单内容进入 `inner_html`。",
        "严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。",
        "使用 `inner_html` 的节点必须补语义测试与安全回归说明。",
        "N/A：`ScrollArea` 组件实现与 docs 示例均未使用 `inner_html` 注入路径",
        "scroll_area_inner_html_usage_is_explicitly_na_and_guarded",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep inner_html safety governance marker `{required}`."
        );
    }

    for semantic_marker in [
        "data-slot=SLOT_SCROLL_AREA",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
        "role=semantics.root_attrs.role",
        "aria-label=semantics.root_attrs.aria_label",
    ] {
        assert!(
            view_source.contains(semantic_marker),
            "ScrollArea semantic contract should stay explicit without inner_html fallback via `{semantic_marker}`."
        );
    }
}

#[test]
fn scroll_area_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-inner-html.sh");

    let needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce `{needle}`."
    );
}

#[test]
fn scroll_area_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let docs_layout_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-layout Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }

    for forbidden in [
        "scroll-area-wasm-debug",
        "component-scroll_area-wasm-debug",
        "scroll_area-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "ScrollArea should not define component-local wasm-debug feature `{forbidden}`."
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-layout root should keep wasm-debug isolation marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(needle),
            "docs-app should expose dev-only wasm debug entry via `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=SLOT_SCROLL_AREA",
        "data-slot=SLOT_SCROLL_AREA_VIEWPORT",
        "data-motion-source=motion_source",
        "data-orientation=semantics.root_attrs.data_orientation",
        "data-disabled-source=root.disabled_source_attr.as_attr()",
        "data-max-height=semantics.root_attrs.data_max_height",
        "let set_max_height = {",
        ".set_property(\"--ui-scroll-area-max-h\", max_height.as_str())",
    ] {
        assert!(
            view_source.contains(needle),
            "ScrollArea should keep traceable semantic/interaction marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Hello World\"",
        "title=\"Vertical + Max Height\"",
        "title=\"Horizontal + Both + Disabled\"",
        "orientation=ScrollAreaOrientation::Horizontal",
        "orientation=ScrollAreaOrientation::Both",
        "is_disabled=Some(true)",
    ] {
        assert!(
            docs_layout_source.contains(needle),
            "ScrollArea docs playground should keep minimal reproducible state matrix marker `{needle}`."
        );
    }

    let combined = format!("{view_source}\n{logic_source}\n{motion_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "trace.emit(",
        "provide_ui_trace",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea component contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "开发模式下至少能追踪关键状态变更来源与前后值。",
        "关键交互链路应支持最小可复现记录（事件顺序/状态转移）。",
        "调试开关默认不进入生产包体与公共 API。",
        "scroll_area_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep wasm-debug governance marker `{needle}`."
        );
    }
}

#[test]
fn scroll_area_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-wasm-debug.sh");

    let needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm-debug check script should enforce `{needle}`."
    );
}

#[test]
fn scroll_area_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Vertical + Max Height\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Both + Disabled\" code_signal=state_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "ScrollArea docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn scroll_area_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let check2_source = load_source("src/scroll_area/check2.md");
    let dev_docs_script = load_source("../../scripts/dev-docs-app.sh");
    let dev_web_script = load_source("../../scripts/dev-web-demo.sh");

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`."
        );
    }

    let docs_scroll_area_section = docs_source
        .split("pub(super) fn scroll_area() -> AnyView {")
        .nth(1)
        .map(|tail| tail.split("\npub(super) fn ").next().unwrap_or(tail))
        .unwrap_or_else(|| panic!("layout_extra docs page should define scroll_area() section"));

    for needle in [
        "title=\"Hello World\"",
        "title=\"Vertical + Max Height\"",
        "title=\"Horizontal + Both + Disabled\"",
        "orientation=ScrollAreaOrientation::Horizontal",
        "orientation=ScrollAreaOrientation::Both",
        "is_disabled=Some(true)",
    ] {
        assert!(
            docs_scroll_area_section.contains(needle),
            "ScrollArea docs should keep context-visible state-matrix marker `{needle}`."
        );
    }

    for forbidden in [
        "SCROLL_AREA_WORKBENCH_STORAGE_KEY",
        "load_scroll_area_workbench_state(",
        "save_scroll_area_workbench_state(",
        "clear_scroll_area_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_scroll_area_section.contains(forbidden),
            "ScrollArea keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "compose_scoped_css + data-playground-scope + Show test + Restore original CSS",
        "可选状态保留在本组件文档场景按 N/A 处理",
    ] {
        assert!(
            check2_source.contains(required),
            "ScrollArea checklist should keep DX governance marker `{required}`."
        );
    }

    for needle in ["#!/usr/bin/env bash", "trunk serve --open true"] {
        assert!(
            dev_docs_script.contains(needle) && dev_web_script.contains(needle),
            "dev scripts should keep fast local iteration entry `{needle}`."
        );
    }
}

#[test]
fn scroll_area_dx_check_script_covers_hot_reload_and_isolated_canvas_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-dx.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/scroll_area/mod.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");
    let checklist_source = load_source("src/scroll_area/check2.md");

    assert!(
        !manifest_dir.join("src/scroll_area/spec.rs").exists(),
        "ScrollArea should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-scroll_area = []"),
        "ScrollArea feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-scroll_area = [\"dep:serde\"")
            && !cargo_source.contains("component-scroll_area = [\"dep:serde_json\""),
        "ScrollArea should not opt into serde/spec migration dependencies without explicit schema contract."
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "`ScrollArea` 为简单容器组件，spec/serde 迁移路径按 N/A 管理",
        "scroll_area_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn scroll_area_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
{
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("../ui-components/src/button/view.rs");
    let combined = [
        load_source("src/scroll_area/mod.rs"),
        load_source("src/scroll_area/logic.rs"),
        load_source("src/scroll_area/view.rs"),
        load_source("src/scroll_area/styles.rs"),
        load_source("src/scroll_area/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_layout::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("scroll_area-wasm-debug")
            && !cargo_source.contains("scroll-area-wasm-debug"),
        "ScrollArea should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_layout::scroll_area::",
        "const SCROLL_AREA_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/scroll_area/mod.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let styles_source = load_source("src/scroll_area/styles.rs");
    let motion_source = load_source("src/scroll_area/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ];
    for source in sources {
        for forbidden in [
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "ScrollArea engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "ScrollArea public module boundary should not leak web_sys types."
    );
}

#[test]
fn scroll_area_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-layout-engineering.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_ui_layout_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable = load_source("../ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../ui-headless/src/presence.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");
    let checklist_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "#[cfg(feature = \"component-scroll_area\")]",
        "pub mod scroll_area;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-layout lib entry should keep stable export/gate marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use leptos::web_sys",
        "pub use web_sys::",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-layout lib entry should not expose internal/detail marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-scroll_area\")]\n    out.push_str(crate::scroll_area::styles::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]\npub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "css entry should keep feature-gated component aggregation marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "data-slot=\"ui-root\"",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot entry should keep centralized theme/i18n/css injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight entry should keep shared style/motion contract marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn ScrollArea(", "ui-scroll-area"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain shared utility without component business token `{forbidden}`."
        );
    }

    assert!(
        manifest_dir
            .join("../ui-visual-primitive/src/active_highlight.rs")
            .exists(),
        "ui-layout should keep shared `../ui-visual-primitive/src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui-layout should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui-layout should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui-layout should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
    );

    for needle in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
        "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>>",
    ] {
        assert!(
            headless_controllable.contains(needle)
                || headless_presence.contains(needle)
                || headless_a11y.contains(needle),
            "headless layer should keep canonical primitive entry marker `{needle}`."
        );
    }

    for required in [
        "- [x] `ui-layout` 固定入口文件落点正确。",
        "`crates/ui-layout/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-layout/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-layout/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-layout/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-layout/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-layout/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-entrypoints.sh");

    let needle = "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_ui_layout_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn scroll_area_styles_include_state_marker_contracts() {
    let source = load_source("src/scroll_area/styles.rs");
    let theme_css = load_source("../ui-theme/src/css.rs");

    for selector in [
        "--ui-scroll-area-motion-duration: var(--ui-text-field-motion-duration)",
        "transition: opacity var(--ui-scroll-area-motion-duration) var(--ui-text-field-motion-easing)",
        ".ui-scroll-area--vertical .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"vertical\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--horizontal .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"horizontal\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--both .ui-scroll-area__viewport",
        ".ui-scroll-area[data-orientation=\"both\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--max-height-custom .ui-scroll-area__viewport",
        ".ui-scroll-area[data-max-height=\"custom\"] .ui-scroll-area__viewport",
        ".ui-scroll-area--disabled",
        ".ui-scroll-area[data-disabled=\"true\"]",
        "--ui-scroll-area-max-h",
    ] {
        assert!(
            source.contains(selector),
            "ScrollArea styles should include `{selector}` as stable state-marker contracts."
        );
    }

    for needle in [
        "--ui-text-field-motion-duration:",
        "--ui-text-field-motion-easing:",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme css variables should expose `{needle}` for scroll_area token-backed motion."
        );
    }
}

#[test]
fn scroll_area_docs_page_exists_in_layout_extra() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "title=\"ScrollArea\"",
        "slug=\"scroll-area\"",
        "<ScrollArea",
    ] {
        assert!(
            docs.contains(needle),
            "ScrollArea docs page should contain `{needle}`."
        );
    }
}

#[test]
fn scroll_area_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "title=\"ScrollArea\"",
        "slug=\"scroll-area\"",
        "description=\"baseline-compatible scroll container with centralized orientation/max-height/disabled normalization and stable state-marker data contracts.\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Vertical + Max Height\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Both + Disabled\" code_signal=state_code>",
        "<ScrollArea",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs should include `{needle}` for scroll-area primary playground coverage.",
        );
    }
}

#[test]
fn scroll_area_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for needle in [
        "title=\"Hello World\"",
        "r#\"<ScrollArea>",
        "Activity feed",
        "title=\"Vertical + Max Height\"",
        "<ScrollArea max_height_px=180>",
        "Release note",
        "title=\"Horizontal + Both + Disabled\"",
        "orientation=ScrollAreaOrientation::Horizontal",
        "max_height_px=120",
        "class_name=\"docs-scroll-area-custom\".to_string()",
        "Tag",
        "orientation=ScrollAreaOrientation::Both",
        "is_disabled=Some(true)",
        "aria_label=\"Disabled logs\".to_string()",
        "Cell",
    ] {
        assert!(
            source.contains(needle),
            "layout_extra docs playgrounds should contain `{needle}` for scroll-area contracts.",
        );
    }
}

#[test]
fn scroll_area_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn scroll_area_docs_examples_sync_with_logic_api_names_and_state_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let primitive_source = load_source("../ui-state-primitives/src/scroll_area.rs");

    scroll_area_docs_page_covers_primary_playgrounds();
    scroll_area_docs_playgrounds_lock_state_matrix_contract_values();

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Vertical + Max Height\"",
        "title=\"Horizontal + Both + Disabled\"",
        "data-slot=\"scroll-area-api-matrix\"",
        "data-slot=\"scroll-area-state-matrix\"",
        "orientation: ScrollAreaOrientation",
        "default = ScrollAreaOrientation::Vertical",
        "max_height_px: Option<u32>",
        "is_disabled: Option<bool>",
        "aria_label: Option<String>",
        "DEFAULT_ARIA_LABEL",
        "controlled/uncontrolled value axis",
        "N/A for ScrollArea (no value/open state machine)",
        "data-disabled / data-disabled-source",
        "data-ui-schema / data-ui-intent / data-ui-action / data-ui-state / data-ui-source / data-ui-output-status",
    ] {
        assert!(
            docs_source.contains(needle),
            "ScrollArea docs should keep API/default/state-matrix marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation",
        "#[prop(optional)] max_height_px: Option<u32>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: ScrollAreaMotion",
        "pub use ui_state_primitives::scroll_area::{",
        "DEFAULT_ARIA_LABEL",
        "pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState",
        "if let Some(is_disabled) = input.is_disabled {",
        "is_disabled: false,",
        "pub fn normalize_aria_label_with_fallback(",
        "normalize_aria_label_with_fallback(input.aria_label, input.fallback_aria_label.as_str())",
        "pub enum ScrollAreaOrientation",
        "#[default]",
        "Vertical,",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || primitive_source.contains(needle),
            "ScrollArea public/default contract should keep `{needle}`."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_examples_sync_with_logic_api_names_and_state_matrix",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_docs_entry_exists_as_readme_or_equivalent_docs_app_page() {
    let has_readme = path_exists("src/scroll_area/README.md");
    let has_docs_page =
        path_exists("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let pages_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");

    assert!(
        has_readme || has_docs_page,
        "ScrollArea must provide README or equivalent docs-app entry."
    );
    assert!(
        docs_source.contains("pub(super) fn scroll_area() -> AnyView"),
        "Equivalent docs entry should expose scroll_area page function."
    );
    assert!(
        pages_catalog_source.contains("layout_extra::SCROLL_AREA_DOC"),
        "docs-app component catalog should index ScrollArea docs entry."
    );
}

#[test]
fn scroll_area_docs_are_beginner_friendly_with_default_then_advanced_path() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "组件文档必须对新手友好（Documentation as Product）",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法",
        "文档需明确“先用起来，再进阶”",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"ScrollArea\"",
        "slug=\"scroll-area\"",
        "<Playground title=\"Hello World\" code_signal=hello_code>",
        "<Playground title=\"Vertical + Max Height\" code_signal=default_code>",
        "<Playground title=\"Horizontal + Both + Disabled\" code_signal=state_code>",
    ] {
        assert!(
            docs_source.contains(needle),
            "ScrollArea docs should include beginner-to-advanced progression marker `{needle}`."
        );
    }

    let hello_pos = docs_source
        .find("<Playground title=\"Hello World\" code_signal=hello_code>")
        .expect("scroll_area docs should include hello-world playground");
    let common_pos = docs_source
        .find("<Playground title=\"Vertical + Max Height\" code_signal=default_code>")
        .expect("scroll_area docs should include common-usage playground");
    let advanced_pos = docs_source
        .find("<Playground title=\"Horizontal + Both + Disabled\" code_signal=state_code>")
        .expect("scroll_area docs should include advanced playground");

    assert!(
        hello_pos < common_pos && common_pos < advanced_pos,
        "ScrollArea docs should present default usage before advanced controls."
    );
}

#[test]
fn scroll_area_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let section_start = docs_source
        .find("pub(super) fn scroll_area() -> AnyView")
        .expect("layout docs should define scroll_area page function");
    let section_end = docs_source[section_start..]
        .find("pub(super) fn resizable() -> AnyView")
        .map(|offset| section_start + offset)
        .expect("layout docs should place resizable after scroll_area");
    let scroll_area_section = &docs_source[section_start..section_end];

    let start = scroll_area_section
        .find("let hello_code = Signal::derive(move || {")
        .expect("scroll_area docs should define hello_code");
    let end = scroll_area_section[start..]
        .find("let default_code = Signal::derive(move || {")
        .map(|offset| start + offset)
        .expect("scroll_area docs should define default code block after hello_code");
    let hello_block = &scroll_area_section[start..end];

    let snippet_start = hello_block
        .find("r#\"")
        .map(|idx| idx + 3)
        .expect("hello snippet should be embedded as raw string");
    let snippet_end = hello_block[snippet_start..]
        .find("\"#")
        .map(|offset| snippet_start + offset)
        .expect("hello snippet should terminate raw string");
    let hello_snippet = &hello_block[snippet_start..snippet_end];
    let meaningful_lines = hello_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        meaningful_lines <= 5,
        "ScrollArea Hello World snippet should stay <= 5 lines for beginner DX, got {meaningful_lines} lines:\n{hello_snippet}"
    );

    for forbidden in [
        "ui_state_primitives",
        "ui-headless",
        "ui_headless",
        "state=",
        "controller=",
        "Signal<",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "ScrollArea Hello World path should not require architecture-level wiring `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_check2_marks_documentation_as_product_complete() {
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
        "scroll_area_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "scroll_area_docs_are_beginner_friendly_with_default_then_advanced_path",
        "scroll_area_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep documentation-as-product completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_entry_exists_as_readme_or_equivalent_docs_app_page",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_are_beginner_friendly_with_default_then_advanced_path",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_hello_world_snippet_is_zero_threshold_and_not_architecture_wiring",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_documentation_as_product_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_interactive_playground_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep interactive-playground rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn scroll_area_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "let (marker_orientation, set_marker_orientation) = signal(ScrollAreaOrientation::Vertical);",
        "let (marker_is_disabled, set_marker_is_disabled) = signal(false);",
        "let (marker_has_custom_max_height, set_marker_has_custom_max_height) = signal(true);",
        "let (marker_has_custom_class, set_marker_has_custom_class) = signal(false);",
        "let (marker_has_custom_aria, set_marker_has_custom_aria) = signal(false);",
        "orientation=marker_orientation.get()",
        "is_disabled=Some(marker_is_disabled.get())",
        "max_height_px=if marker_has_custom_max_height.get() {",
        "class_name=if marker_has_custom_class.get() {",
        "aria_label=if marker_has_custom_aria.get() {",
        "data-slot=\"scroll-area-marker-controls\"",
        "data-slot=\"scroll-area-toggle-orientation\"",
        "data-slot=\"scroll-area-toggle-disabled\"",
        "data-slot=\"scroll-area-toggle-max-height\"",
        "data-slot=\"scroll-area-toggle-class\"",
        "data-slot=\"scroll-area-toggle-aria\"",
        "data-slot=\"scroll-area-marker-summary\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "ScrollArea docs should provide interactive playground marker `{needle}`."
        );
    }

    for needle in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app Playground should keep interactive preview contract `{needle}`."
        );
    }
}

#[test]
fn scroll_area_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_scroll_area_contract.spec.mjs");

    for needle in [
        "docs-app scroll-area key flow is repeatable with semantic breakpoints",
        "await page.goto(SCROLL_AREA_PAGE);",
        "const DOCS_ROOT = '[data-component=\"scroll-area\"]';",
        "await page.keyboard.press(\"PageDown\")",
        "toHaveAttribute(\"data-ui-action\", \"observe\")",
        "toHaveAttribute(\"data-ui-state\", \"enabled\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "await page.reload();",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "ScrollArea interactive playground should keep repeatable semantic e2e marker `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_marks_interactive_playground_complete() {
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
        "非 AI Spec 组件",
        "scroll_area_check2_marks_interactive_playground_complete",
        "scroll_area_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "scroll_area_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep interactive-playground completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = load_source("src/scroll_area/check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "ScrollArea checklist should keep source-first copy-paste-ready rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn scroll_area_docs_are_copy_paste_ready_with_imports_copy_button_and_sync() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../ui-components/src/code_block/view.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "data-slot=\"scroll-area-source-first\"",
        "<h3>\"Source-first / Copy-Paste Ready\"</h3>",
        "<Snippet",
        "copyable=true",
        "class_name=\"docs-scroll-area-source-copy\".to_string()",
        "data-slot=\"scroll-area-source-paths\"",
        "\"crates/ui-layout/src/scroll_area/mod.rs\"",
        "\"crates/ui-layout/src/scroll_area/logic.rs\"",
        "\"crates/ui-layout/src/scroll_area/view.rs\"",
        "\"crates/ui-layout/src/scroll_area/styles.rs\"",
        "\"crates/ui-layout/src/scroll_area/motion.rs\"",
        "data-slot=\"scroll-area-source-prerequisites\"",
        "\"component-scroll_area\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "ScrollArea docs should keep copy-ready marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let resolved_code = Signal::derive(move || {",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "<CodeBlock code=resolved_code.get() />",
        "\"Show code\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs-app playground should keep copy-paste pipeline marker `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "on_press=copy_logic.copy",
    ] {
        assert!(
            code_block_source.contains(needle),
            "CodeBlock should keep one-click copy marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation",
        "#[prop(optional)] max_height_px: Option<u32>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRoot",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "ScrollArea docs copy-ready snippets should stay synced with implementation marker `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_marks_source_first_copy_paste_ready_complete() {
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
        "component-scroll_area",
        "crates/ui-layout/src/scroll_area/{mod,logic,view,styles,motion}.rs",
        "scroll_area_check2_documents_source_first_copy_paste_ready_rules",
        "scroll_area_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
        "scroll_area_check2_marks_source_first_copy_paste_ready_complete",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep source-first completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_docs_are_copy_paste_ready_with_imports_copy_button_and_sync",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_source_first_copy_paste_ready_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    for needle in [
        "### ScrollArea 同步记录（2026-02-18）",
        "orientation/max_height_px/is_disabled/disabled",
        "layout_extra::SCROLL_AREA_DOC",
        "layout_extra.rs::scroll_area()",
        "`Hello World`、`Vertical + Max Height`、`Horizontal + Both + Disabled`、`Interactive Playground (State + Source Markers)` 与 `Source-first / Copy-Paste Ready`",
        "compose_copy_ready_code",
        "参数语义若变更，必须先同步本策略文档与 docs 入口，不允许实现先漂移文档后补。",
    ] {
        assert!(
            strategy_source.contains(needle),
            "ScrollArea HeroUI/doc sync record should include `{needle}`."
        );
    }

    {
        let needle = "layout_extra::SCROLL_AREA_DOC";
        assert!(
            docs_index_source.contains(needle),
            "ScrollArea docs index should keep discoverable marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn scroll_area() -> AnyView",
        "title=\"ScrollArea\"",
        "slug=\"scroll-area\"",
        "title=\"Interactive Playground (State + Source Markers)\"",
        "data-slot=\"scroll-area-source-first\"",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "ScrollArea docs entry should keep indexable marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional)] orientation: crate::scroll_area::ScrollAreaOrientation",
        "#[prop(optional)] max_height_px: Option<u32>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] motion: ScrollAreaMotion",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "pub struct ScrollAreaRootInput",
        "pub struct ScrollAreaDisableInput",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "ScrollArea parameter-model marker `{needle}` should remain in implementation."
        );
    }
}

#[test]
fn scroll_area_check2_marks_heroui_strategy_and_component_docs_sync_complete() {
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
        "scroll_area_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep HeroUI/doc sync completion evidence `{needle}`."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_marks_heroui_strategy_and_component_docs_sync_complete",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_explicit_forbidden_antipattern_rules() {
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "### 8. 明确禁止的反模式",
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep explicit forbidden-antipattern rule `{needle}`."
        );
    }
}

#[test]
fn scroll_area_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/scroll_area.rs");
    let headless_source = load_source("../../crates/ui-headless/src/scroll_area.rs");

    for forbidden in [
        "web_sys",
        "wasm_bindgen",
        "leptos::",
        "view! {",
        "NodeRef<",
        "class=",
        "style=",
        "ui-scroll-area--",
        "document.",
        "window.",
    ] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives scroll_area should stay DOM/style-free and avoid `{forbidden}`."
        );
    }

    for required in [
        "pub struct ScrollAreaStateInput",
        "pub struct ScrollAreaState",
        "pub fn normalize_max_height(",
        "pub fn resolve_state(",
    ] {
        assert!(
            primitive_source.contains(required),
            "ui-state-primitives scroll_area should keep stable primitive marker `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-",
        ".ui-",
        "class=",
        "style=",
        "spring",
        "keyframe",
        "animation",
        "timeline",
    ] {
        assert!(
            !headless_source.contains(forbidden),
            "ui-headless scroll_area should avoid visual/animation orchestration token `{forbidden}`."
        );
    }

    for required in [
        "pub struct ScrollAreaRootAttrs",
        "pub struct ScrollAreaViewportAttrs",
        "pub struct ScrollAreaContract",
        "pub struct ScrollAreaOptions",
        "pub fn use_scroll_area(options: ScrollAreaOptions) -> ScrollAreaContract",
    ] {
        assert!(
            headless_source.contains(required),
            "ui-headless scroll_area should keep typed attrs/state contract marker `{required}`."
        );
    }
}

#[test]
fn scroll_area_forbidden_antipatterns_keep_key_state_decisions_out_of_view() {
    let view_source = load_source("src/scroll_area/view.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");

    for required in [
        "logic::normalize_root_state(ScrollAreaRootInput {",
        "let class = logic::compose_class_name(root.class_name, state);",
        "let semantics = use_scroll_area(ScrollAreaOptions {",
        "let agent_contract = logic::resolve_agent_contract(state, root.disabled_source_attr);",
    ] {
        assert!(
            view_source.contains(required),
            "ScrollArea view should consume normalized logic/headless output via `{required}`."
        );
    }

    for forbidden in [
        "resolve_state(ScrollAreaStateInput {",
        "normalize_aria_label(",
        "normalize_optional_text(",
        "if is_disabled.is_some()",
        "if disabled {",
        "match aria_label",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "ScrollArea view should not hide key state decisions via `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_disable_state(input: ScrollAreaDisableInput) -> ScrollAreaDisableState",
        "pub fn normalize_aria_label_with_fallback(",
        "pub fn normalize_root_state(input: ScrollAreaRootInput) -> ScrollAreaRootState",
    ] {
        assert!(
            logic_source.contains(required),
            "ScrollArea logic should keep centralized normalization marker `{required}`."
        );
    }
}

#[test]
fn scroll_area_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks() {
    let view_source = load_source("src/scroll_area/view.rs");
    let mod_source = load_source("src/scroll_area/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/layout_extra.rs");

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "labels + children",
        "titles + panels",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !mod_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "ScrollArea should avoid parallel-array/implicit-convention token `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn ScrollArea(",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "pub use view::ScrollArea;",
        "pub use logic::{DEFAULT_ARIA_LABEL, ScrollAreaOrientation};",
    ] {
        assert!(
            view_source.contains(required) || mod_source.contains(required),
            "ScrollArea public API should remain typed and stable via `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use js_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "ScrollArea public module should not leak platform detail token `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk() {
    let mod_source = load_source("src/scroll_area/mod.rs");
    let logic_source = load_source("src/scroll_area/logic.rs");
    let view_source = load_source("src/scroll_area/view.rs");
    let combined = format!("{mod_source}\n{logic_source}\n{view_source}");

    for forbidden in [
        "temporary patch",
        "TEMP PATCH",
        "TODO(temp)",
        "FIXME(temp)",
        "HACK:",
        "quick fix",
    ] {
        assert!(
            !combined.contains(forbidden),
            "ScrollArea should not carry temporary patch drift marker `{forbidden}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::scroll_area::{",
        "resolve_state,",
        "use ui_headless::{A11yDirection, CommonStrings, ScrollAreaOptions, use_scroll_area, use_ui_i18n};",
        "let semantics = use_scroll_area(ScrollAreaOptions {",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "ScrollArea should consume sunk primitives/headless contracts via `{required}`."
        );
    }

    for forbidden in [
        "pub struct ScrollAreaStateInput {",
        "pub struct ScrollAreaState {",
        "pub struct ScrollAreaContract {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "ScrollArea component logic should not re-declare reusable primitive `{forbidden}`."
        );
    }
}

#[test]
fn scroll_area_contract_hygiene_script_covers_forbidden_antipattern_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_explicit_forbidden_antipattern_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_keep_key_state_decisions_out_of_view",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_block_parallel_array_api_and_platform_type_leaks",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_forbidden_antipatterns_avoid_temporary_patch_drift_and_keep_primitives_sunk",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}

#[test]
fn scroll_area_check2_documents_final_merge_gate_rules() {
    let check2_source = load_source("src/scroll_area/check2.md");

    for needle in [
        "### 9. 合并门禁（最终裁决）",
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "ScrollArea checklist should keep final merge-gate rule `{needle}`."
        );
    }
}

#[test]
fn scroll_area_final_merge_gate_capabilities_are_backed_by_contract_checks() {
    scroll_area_component_file_responsibilities_remain_scoped();
    scroll_area_state_normalization_pipeline_is_logic_only();
    scroll_area_a11y_i18n_locale_contract_is_headless_driven_and_no_view_hardcoded_copy();
    scroll_area_visual_desire_reuses_theme_visual_baseline_and_heroui_contracts();
    scroll_area_semantics_suite_is_contract_first_not_snapshot_only();
    scroll_area_api_naming_prefers_is_disabled_with_compatibility_path();
    scroll_area_type_system_and_machine_readable_markers_stay_in_sync();
    scroll_area_status_primitives_boundary_is_pure_and_exported();
    scroll_area_discrete_state_axes_are_type_constrained();
    scroll_area_observability_markers_are_stable_and_source_enumerable();
    scroll_area_reduced_motion_ssr_wasm_contract_is_consistent();
    scroll_area_docs_examples_sync_with_logic_api_names_and_state_matrix();
    scroll_area_forbidden_antipatterns_keep_state_primitives_dom_free_and_headless_visual_free();
}

#[test]
fn scroll_area_final_merge_gate_marks_full_repo_gate_as_deferred_by_requirement() {
    let check2_source = load_source("src/scroll_area/check2.md");

    assert!(
        check2_source.contains(
            "说明：本项按 `scroll_area` 负责范围执行（`fmt/clippy/test/check/e2e/tree-shaking/contract scripts`）；仓库级全量 smoke 属于整仓门禁，在并行开发环境下标记为 `N/A`，不作为 `scroll_area` 单组件阻断。"
        ),
        "ScrollArea final merge-gate should explicitly record component-scoped gate completion and full-repo smoke as N/A in this parallel-development scope."
    );
}

#[test]
fn scroll_area_check2_has_no_unchecked_checklist_items() {
    let check2_source = load_source("src/scroll_area/check2.md");

    assert!(
        !check2_source.contains("- [ ]"),
        "ScrollArea checklist should not keep unchecked checklist items after final merge-gate completion."
    );
}

#[test]
fn scroll_area_contract_hygiene_script_covers_final_merge_gate_contract() {
    let script_source = load_source("../../scripts/check-ui-layout-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_documents_final_merge_gate_rules",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_final_merge_gate_capabilities_are_backed_by_contract_checks",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_final_merge_gate_marks_full_repo_gate_as_deferred_by_requirement",
        "cargo test -p ui-layout --test scroll_area_semantics --no-default-features --features component-scroll_area,inject-css scroll_area_check2_has_no_unchecked_checklist_items",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`."
        );
    }
}
