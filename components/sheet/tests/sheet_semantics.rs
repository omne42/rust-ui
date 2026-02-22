use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mapped = match rel_path {
        "Cargo.toml" => "../../crates/ui/Cargo.toml".to_string(),
        "src/lib.rs" => "../../crates/ui/src/lib.rs".to_string(),
        "src/css.rs" => "../../crates/ui/src/css.rs".to_string(),
        "src/root.rs" => "../../crates/ui/src/root.rs".to_string(),
        "src/code_block/view.rs" => "../../components/code-block/src/view.rs".to_string(),
        _ if rel_path.starts_with("src/") => {
            format!("src/{}", &rel_path["src/".len()..])
        }
        _ if rel_path.starts_with("../ui-headless/") => {
            format!("../../crates/{}", &rel_path["../".len()..])
        }
        _ if rel_path.starts_with("../ui-motion/") => {
            format!("../../crates/{}", &rel_path["../".len()..])
        }
        _ if rel_path.starts_with("../ui-state-primitives/") => {
            format!("../../crates/{}", &rel_path["../".len()..])
        }
        _ if rel_path.starts_with("../ui-visual-primitive/") => {
            format!("../../crates/{}", &rel_path["../".len()..])
        }
        _ => rel_path.to_string(),
    };
    let path = manifest_dir.join(mapped);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn sheet_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Sheet internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn sheet_is_exported_and_exposes_state_contracts() {
    let module_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let crate_source = load_source("src/lib.rs");

    for needle in [
        "pub use logic::SheetPlacement;",
        "pub use motion::SheetMotion;",
        "pub use view::Sheet;",
    ] {
        assert!(
            module_source.contains(needle),
            "sheet module should include `{needle}` state contracts."
        );
    }

    for needle in [
        "pub(crate) enum SheetSlot",
        "pub(crate) struct SheetPartStateInput",
        "pub(crate) struct SheetPartState",
    ] {
        assert!(
            logic_source.contains(needle),
            "sheet logic should own implementation details `{needle}`."
        );
    }

    for forbidden in [
        "pub enum SheetSlot",
        "pub struct SheetPartStateInput",
        "pub struct SheetPartState",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "sheet mod.rs should stay minimal and avoid implementation details: `{forbidden}`."
        );
    }

    assert!(
        crate_source.contains("pub use sheet::{Sheet, SheetMotion, SheetPlacement};")
            || (crate_source.contains("pub use sheet::Sheet;")
                && crate_source.contains("pub use sheet::SheetMotion;")
                && crate_source.contains("pub use sheet::SheetPlacement;")),
        "crate root should re-export `Sheet`, `SheetPlacement`, and `SheetMotion` contracts."
    );
}

#[test]
fn sheet_logic_exposes_state_helpers() {
    let source = load_source("src/logic.rs");

    for needle in [
        "pub(crate) enum SheetSlot",
        "pub(crate) struct SheetPartStateInput",
        "pub(crate) struct SheetPartState",
        "pub const DEFAULT_DISMISSABLE: bool = true;",
        "pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;",
        "pub fn state_attr_for_open(is_open: bool)",
        "pub fn dismiss_attr(is_dismissable: bool)",
        "pub fn keyboard_dismiss_attr(is_keyboard_dismiss_disabled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn resolve_state(input: SheetPartStateInput)",
        "pub fn compose_class_name(state: SheetPartState)",
        "pub fn should_close_on_escape(",
    ] {
        assert!(
            source.contains(needle),
            "Sheet logic should include `{needle}` for centralized source/state contracts."
        );
    }
}

#[test]
fn sheet_api_naming_and_control_contract_are_consistent_for_component_scope() {
    let view = load_source("src/view.rs");
    let primitives_overlay_trigger = load_source("../ui-state-primitives/src/overlay_trigger.rs");

    for needle in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "#[prop(optional)] placement: SheetPlacement",
        "#[prop(optional, default = logic::DEFAULT_DISMISSABLE)] is_dismissable: bool",
        "#[prop(optional, default = logic::DEFAULT_KEYBOARD_DISMISS_DISABLED)]",
        "is_keyboard_dismiss_disabled: bool",
        "#[prop(optional)] motion: SheetMotion",
    ] {
        assert!(
            view.contains(needle),
            "Sheet API naming should keep stable prop contract via `{needle}`."
        );
    }

    for forbidden in ["className", "onOpenChange", "defaultOpen"] {
        assert!(
            !view.contains(forbidden),
            "Sheet API should avoid alias drift token `{forbidden}`."
        );
    }

    // Sheet is a presentational overlay surface that consumes external open signal.
    // Uncontrolled/default-open behavior lives in shared primitives.
    for needle in [
        "pub struct OverlayTriggerStateOptions",
        "pub default_open: Option<bool>",
        "pub on_open_change: Option<OverlayOnOpenChange>",
    ] {
        assert!(
            primitives_overlay_trigger.contains(needle),
            "Controlled/uncontrolled pairing should remain in state primitives via `{needle}`."
        );
    }
}

#[test]
fn sheet_escape_respects_default_prevented_composition_and_keyboard_flag() {
    let source = load_source("src/view.rs");

    for needle in [
        "default_prevented",
        "is_composing",
        "logic::should_close_on_escape(",
        "is_keyboard_dismiss_disabled",
        "stop_propagation()",
    ] {
        assert!(
            source.contains(needle),
            "Sheet should include `{needle}` for stable Escape-dismiss behavior."
        );
    }
}

#[test]
fn sheet_view_uses_logic_state_contracts() {
    let source = load_source("src/view.rs");

    for needle in [
        "A11yDirection",
        "OverlayDialogA11yAttrs",
        "overlay_dialog_attrs(",
        "logic::normalize_optional_text(aria_labelledby)",
        "logic::normalize_optional_text(aria_describedby)",
        "logic::normalize_optional_text(lang)",
        "struct SheetStateInputs",
        "fn resolve_part_state(",
        "resolve_part_state(logic::SheetSlot::Root, open.get_untracked(), state_inputs)",
        "resolve_part_state(logic::SheetSlot::Backdrop, false, state_inputs)",
        "resolve_part_state(logic::SheetSlot::Panel, false, state_inputs)",
        "fn render_backdrop(",
        "fn render_panel(",
        "{render_backdrop(backdrop_state, backdrop_class, is_dismissable, on_close_for_backdrop)}",
        "{render_panel(",
        "logic::compose_class_name(root_state)",
        "data-slot=root_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-placement=root_state.placement_attr",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
        "data-custom-aria-labelledby=root_state.has_custom_aria_labelledby.then_some(\"true\")",
        "data-custom-aria-describedby=root_state.has_custom_aria_describedby.then_some(\"true\")",
        "data-exit-source=root_state.exit_source_attr",
        "data-custom-exit=root_state.has_on_exit_complete.then_some(\"true\")",
        "data-slot=backdrop_state.slot_attr",
        "data-slot=panel_state.slot_attr",
        "data-state=panel_state.state_attr",
        "lang=move || panel_lang.with_value(|value| value.clone())",
        "dir=panel_dir",
    ] {
        assert!(
            source.contains(needle),
            "Sheet view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn sheet_state_markers_are_observable_queryable_and_closed_set() {
    let view = load_source("src/view.rs");
    let logic = load_source("src/logic.rs");

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-placement=root_state.placement_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        assert!(
            view.contains(needle),
            "Sheet should expose stable observable markers via `{needle}`."
        );
    }

    for needle in [
        "pub fn state_attr_for_open(is_open: bool) -> &'static str",
        "\"dismissable\"",
        "\"locked\"",
        "\"enabled\"",
        "\"disabled\"",
        "\"bottom\"",
        "\"left\"",
        "\"right\"",
        "\"custom\"",
        "\"default\"",
    ] {
        assert!(
            logic.contains(needle),
            "Sheet state marker values should remain a closed set with `{needle}`."
        );
    }
}

#[test]
fn sheet_a11y_i18n_locale_contract_uses_headless_overlay_attrs() {
    let view = load_source("src/view.rs");
    let headless_a11y = load_source("../ui-headless/src/a11y.rs");

    for needle in [
        "use ui_headless::{",
        "A11yDirection",
        "OverlayDialogA11yAttrs",
        "overlay_dialog_attrs",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let panel_a11y: OverlayDialogA11yAttrs = overlay_dialog_attrs(",
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "lang=move || panel_lang.with_value(|value| value.clone())",
        "dir=panel_dir",
    ] {
        assert!(
            view.contains(needle),
            "Sheet should keep a11y/i18n/l10n integration point via `{needle}`."
        );
    }

    for needle in [
        "pub fn overlay_dialog_attrs(",
        "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>)",
        "pub struct OverlayDialogA11yAttrs",
    ] {
        assert!(
            headless_a11y.contains(needle),
            "ui-headless a11y shared utility should provide `{needle}`."
        );
    }

    for forbidden in [
        "\"Open sheet\"",
        "\"Close\"",
        "\"Sheet content\"",
        "\"Backdrop clicks and Escape are disabled.\"",
    ] {
        assert!(
            !view.contains(forbidden),
            "Sheet view should not hardcode user-facing copy: `{forbidden}`."
        );
    }
}

#[test]
fn sheet_styles_include_state_and_source_marker_selectors() {
    let source = load_source("src/styles.rs");

    for needle in [
        ".ui-sheet[data-motion-source=\"custom\"]",
        ".ui-sheet[data-custom-motion=\"true\"]",
        ".ui-sheet[data-placement-source=\"custom\"]",
        ".ui-sheet[data-dismiss-source=\"custom\"]",
        ".ui-sheet[data-keyboard-dismiss-source=\"custom\"]",
        ".ui-sheet--custom-aria-labelledby",
        ".ui-sheet[data-custom-aria-labelledby=\"true\"]",
        ".ui-sheet[data-aria-labelledby-source=\"custom\"]",
        ".ui-sheet--custom-aria-describedby",
        ".ui-sheet[data-custom-aria-describedby=\"true\"]",
        ".ui-sheet[data-aria-describedby-source=\"custom\"]",
        ".ui-sheet[data-exit-source=\"custom\"]",
        ".ui-sheet[data-custom-exit=\"true\"]",
        ".ui-sheet[data-dismissable=\"true\"] .ui-sheet__backdrop",
        ".ui-sheet[data-keyboard-dismiss-disabled=\"true\"] .ui-sheet__panel",
        ".ui-sheet__backdrop[data-state=\"backdrop\"]",
        ".ui-sheet__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(needle),
            "Sheet styles should include `{needle}` for deterministic marker behavior."
        );
    }

    for needle in [
        ".ui-sheet[data-state=\"open\"]",
        ".ui-sheet[data-open=\"true\"]",
        ".ui-sheet[data-state=\"closed\"]",
        ".ui-sheet[data-closed=\"true\"]",
        "pointer-events: none;",
    ] {
        assert!(
            source.contains(needle),
            "Sheet styles should include `{needle}` so closed sheets cannot intercept page clicks."
        );
    }
}

#[test]
fn sheet_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let source = load_source("src/styles.rs");

    for needle in [
        ".ui-sheet[data-state=\"open\"]",
        ".ui-sheet[data-state=\"closed\"]",
        ".ui-sheet[data-dismiss-source=\"custom\"]",
        ".ui-sheet[data-keyboard-dismiss-source=\"custom\"]",
        ".ui-sheet[data-aria-labelledby-source=\"custom\"]",
        ".ui-sheet[data-aria-describedby-source=\"custom\"]",
        ".ui-sheet[data-exit-source=\"custom\"]",
    ] {
        assert!(
            source.contains(needle),
            "Sheet styles should key off explicit semantic markers via `{needle}`."
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type("] {
        assert!(
            !source.contains(forbidden),
            "Sheet styles should avoid brittle DOM-guessing selectors: `{forbidden}`."
        );
    }
}

#[test]
fn sheet_styles_consume_ui_theme_overlay_tokens() {
    let styles = load_source("src/styles.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let theme_tokens = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_map = load_source("../../crates/ui-theme/src/theme.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    for needle in [
        "z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));",
        "left: var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset));",
        "right: var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset));",
        "bottom: var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset));",
        "100vh",
        "100vw",
        "var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg)) * 8",
    ] {
        assert!(
            styles.contains(needle),
            "Sheet styles should consume ui-theme overlay tokens via `{needle}`."
        );
    }

    for needle in [
        "--ui-overlay-z-index:",
        "--ui-overlay-panel-min-width:",
        "--ui-overlay-viewport-inset:",
        "pub struct OverlayLayoutTokens",
        "panel_min_width_px",
        "viewport_inset_px",
        "pub fn overlay_layout_tokens(ctx: ThemeContext) -> OverlayLayoutTokens",
        "--ui-overlay-panel-min-width",
        "--ui-overlay-viewport-inset",
    ] {
        assert!(
            theme_css.contains(needle)
                || theme_tokens.contains(needle)
                || theme_map.contains(needle)
                || styling_spec.contains(needle),
            "ui-theme contract should expose `{needle}` for sheet token-first styling."
        );
    }

    for forbidden in [
        "z-index: 1000;",
        "max-height: 90vh;",
        "width: min(420px, 92vw);",
    ] {
        assert!(
            !styles.contains(forbidden),
            "Sheet styles should not keep hardcoded visual constants once ui-theme tokens exist: `{forbidden}`."
        );
    }
}

#[test]
fn sheet_semantics_contract_checks_prioritize_semantics_over_snapshots() {
    let source = load_source("tests/sheet_semantics.rs");

    for needle in [
        "fn sheet_view_uses_logic_state_contracts()",
        "fn sheet_state_markers_are_observable_queryable_and_closed_set()",
        "fn sheet_styles_include_state_and_source_marker_selectors()",
        "fn sheet_styles_depend_on_explicit_state_markers_not_dom_guessing()",
    ] {
        assert!(
            source.contains(needle),
            "sheet semantics suite should include contract test `{needle}`."
        );
    }

    let snapshot_macro = format!("{}{}", "insta::assert_", "snapshot!(");
    let assert_snapshot_macro = format!("{}{}", "assert_", "snapshot!(");
    let assert_debug_snapshot_macro = format!("{}{}{}", "assert_", "debug_", "snapshot!(");
    for forbidden in [
        snapshot_macro.as_str(),
        assert_snapshot_macro.as_str(),
        assert_debug_snapshot_macro.as_str(),
    ] {
        assert!(
            !source.contains(forbidden),
            "sheet semantics should not rely on snapshot-only assertions: `{forbidden}`."
        );
    }
}

#[test]
fn sheet_component_files_respect_layered_responsibilities() {
    let module = load_source("src/mod.rs");
    let logic = load_source("src/logic.rs");
    let styles = load_source("src/styles.rs");
    let view = load_source("src/view.rs");
    let motion = load_source("src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Sheet;",
    ] {
        assert!(
            module.contains(needle),
            "sheet mod.rs should keep export boundary via `{needle}`."
        );
    }

    for forbidden in ["#[component]", "view! {", "web_sys::", "NodeRef<"] {
        assert!(
            !logic.contains(forbidden),
            "sheet logic.rs should avoid rendering/DOM concerns: `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles.contains(needle),
            "sheet styles.rs should stay token-first static CSS via `{needle}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "set_property(\"background",
        "set_property(\"color",
    ] {
        assert!(
            !view.contains(forbidden),
            "sheet view.rs should avoid inline style business branching: `{forbidden}`."
        );
    }

    for forbidden in ["raf::", "request_animation_frame("] {
        assert!(
            !motion.contains(forbidden),
            "sheet motion.rs should not reimplement generic motion engine: `{forbidden}`."
        );
    }
}

#[test]
fn sheet_does_not_define_spec_module_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let module_source = load_source("src/mod.rs");
    let sheet_spec = manifest_dir.join("src/spec.rs");
    let button_spec = manifest_dir.join("../button/src/spec.rs");

    assert!(
        !sheet_spec.exists(),
        "Sheet is a simple component and should not define `src/spec.rs` without explicit complex schema-contract need."
    );
    assert!(
        !module_source.contains("mod spec;") && !module_source.contains("pub mod spec;"),
        "sheet module should not wire a `spec` module for simple component scope."
    );
    assert!(
        button_spec.exists(),
        "button should remain the canonical complex component that keeps `spec.rs` boundary."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn sheet_motion_contract_exposes_default_custom_and_direction_checks() {
    let source = load_source("src/motion.rs");

    for needle in [
        "pub struct SheetMotion",
        "fn default_motion_uses_slide_spring_contract()",
        "fn placement_offset_maps_to_sheet_direction_contract()",
        "fn supports_custom_motion_contract()",
    ] {
        assert!(
            source.contains(needle),
            "Sheet motion module should include `{needle}` for baseline-style regression coverage."
        );
    }
}

#[test]
fn sheet_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::sheet::styles::CSS);"),
        "ui css aggregator should include sheet styles."
    );
}

#[test]
fn sheet_token_first_static_css_contract_is_wired_through_ui_root() {
    let styles = load_source("src/styles.rs");
    let css_aggregator = load_source("src/css.rs");
    let root = load_source("src/root.rs");
    let view = load_source("src/view.rs");

    for needle in [
        "pub const CSS: &str = r#\"",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "border: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "box-shadow: var(--ui-shadow-lg, var(--ui-fallback-shadow-sm));",
        "padding: var(--ui-space-lg, var(--ui-fallback-space-lg));",
        "border-top-left-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "border-top-right-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
    ] {
        assert!(
            styles.contains(needle),
            "sheet styles should stay token-first static CSS via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-sheet\")]",
        "out.push_str(crate::sheet::styles::CSS);",
    ] {
        assert!(
            css_aggregator.contains(needle),
            "ui css aggregator should gate and include sheet css via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root.contains(needle),
            "UiRoot should inject aggregated component CSS via `{needle}`."
        );
    }

    for forbidden in ["@apply ", "styled!(", "Style::new("] {
        assert!(
            !styles.contains(forbidden),
            "sheet styles should not default to utility-first/CSS-in-Rust patterns: `{forbidden}`."
        );
    }

    for forbidden in ["style=", "set_property("] {
        assert!(
            !view.contains(forbidden),
            "sheet view should not carry inline runtime style business logic: `{forbidden}`."
        );
    }
}

#[test]
fn sheet_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn sheet() -> AnyView",
        "title=\"Sheet\"",
        "slug=\"sheet\"",
        "State + Source Markers",
        "data-placement-source",
        "<Sheet",
    ] {
        assert!(
            source.contains(needle),
            "sheet docs page should contain `{needle}`."
        );
    }
}

#[test]
fn sheet_source_first_docs_are_copy_paste_ready_and_traceable() {
    let docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground = load_source("../../apps/docs-app/src/playground.rs");
    let code_block = load_source("src/code_block/view.rs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "data-slot=\"sheet-source-first\"",
        "\"Source-first / Copy-Paste Ready\"",
        "Snippet",
        "label=\"Copy starter\".to_string()",
        "copyable=true",
        "use leptos::prelude::*;\\nuse ui::*;",
        "data-slot=\"sheet-source-paths\"",
        "components/sheet/src/mod.rs",
        "components/sheet/src/logic.rs",
        "components/sheet/src/view.rs",
        "components/sheet/src/styles.rs",
        "components/sheet/src/motion.rs",
        "data-slot=\"sheet-source-prerequisites\"",
        "\"component-sheet\"",
        "\"inject-css\"",
    ] {
        assert!(
            docs.contains(needle),
            "Sheet source-first docs should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "Show code",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground.contains(needle),
            "Playground copy-ready pipeline should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"code-block\"",
        "data-copyable=state.copyable.then_some(\"true\")",
        "class_name=\"ui-code-block__copy-button\".to_string()",
        "on_press=copy_logic.copy",
    ] {
        assert!(
            code_block.contains(needle),
            "CodeBlock one-click copy markers should include `{needle}`."
        );
    }

    for needle in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should keep source-first copy-ready governance marker `{needle}`."
        );
    }
}

#[test]
fn sheet_heroui_strategy_and_component_docs_stay_synced() {
    let strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_index = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "### Sheet 同步记录（2026-02-18）",
        "`Sheet` 继续保持 overlay primitive 定位",
        "`open/on_close/placement/is_dismissable/is_keyboard_dismiss_disabled/motion/aria_labelledby/aria_describedby/on_exit_complete`",
        "component_doc!(\"Sheet\", \"sheet\", \"Overlays\", overlays::sheet)",
        "#/components/sheet",
        "Source-first / Copy-Paste Ready",
        "HeroUI 对齐结论：保持“默认路径零门槛、进阶参数按需开启”的体验目标",
    ] {
        assert!(
            strategy.contains(needle),
            "Sheet HeroUI strategy sync should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Sheet\", \"sheet\", \"Overlays\", overlays::sheet)",
        "pub(super) fn sheet() -> AnyView",
        "title=\"Sheet\"",
        "slug=\"sheet\"",
        "data-slot=\"sheet-source-first\"",
    ] {
        assert!(
            docs_index.contains(needle) || docs_page.contains(needle),
            "Sheet docs entry/index should include `{needle}`."
        );
    }

    assert!(
        check2.contains("HeroUI 对标文档与组件文档同步"),
        "Sheet checklist should keep HeroUI/doc sync governance item."
    );
}

#[test]
fn sheet_visual_desire_baseline_is_documented_with_overlay_button_input() {
    let baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let docs_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let heroui_strategy = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let heroui_study = load_source("../../docs/research/spectrum-heroui-style-interface-study.md");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "description=\"Default theme visual baseline for hierarchy, contrast, and interaction cues. Includes Button/Input/Overlay for visual regression snapshots.\"",
        "title=\"Default Theme Visual Baseline\"",
        "interactive feedback (hover/active/focus)",
        "Default theme should feel trustworthy at first glance",
        "<Button variant=ButtonVariant::Accent>\"Primary Action\"</Button>",
        "<Input",
        "<Overlay",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            baseline_page.contains(needle),
            "theme visual baseline page should include `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            docs_registry.contains(needle),
            "docs registry should expose theme visual baseline via `{needle}`."
        );
    }

    for needle in [
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "不追求 1:1 复刻 React Spectrum 或 HeroUI 的全部 API。",
    ] {
        assert!(
            heroui_strategy.contains(needle) || heroui_study.contains(needle),
            "HeroUI alignment docs should keep non-1:1 API-copy stance via `{needle}`."
        );
    }
}

#[test]
fn sheet_tree_shaking_contract_is_feature_gated_and_budgeted() {
    let cargo = load_source("Cargo.toml");
    let lib = load_source("src/lib.rs");
    let css = load_source("src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");
    let script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let budget = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "component-sheet = [\"dep:ui-sheet\"]",
        "web-demo-components = [",
        "all-components = [",
    ] {
        assert!(
            cargo.contains(needle),
            "ui feature surface should include `{needle}` for tree-shaking contracts."
        );
    }

    assert!(
        lib.contains("#[cfg(feature = \"component-sheet\")]\npub use ui_sheet as sheet;"),
        "sheet module bridge should stay feature-gated in lib.rs."
    );
    assert!(
        css.contains("#[cfg(feature = \"component-sheet\")]")
            && css.contains("out.push_str(crate::sheet::styles::CSS);"),
        "sheet css aggregation should stay feature-gated in css.rs."
    );

    for needle in [
        "MIN_FEATURES=\"component-accordion,inject-css\"",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components'",
        "if ! grep -q 'web-demo-components'",
    ] {
        assert!(
            script.contains(needle),
            "tree-shaking gate script should keep `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget.contains(needle),
            "tree-shaking budget file should define `{needle}`."
        );
    }

    for needle in [
        "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "web-demo-components",
    ] {
        assert!(
            web_demo_cargo.contains(needle),
            "web-demo should consume ui via feature-bundled source mode contract `{needle}`."
        );
    }

    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo should not pull ui all-components feature."
    );
}

#[test]
fn sheet_type_system_and_semantic_markers_define_machine_readable_contracts() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let tests = load_source("tests/sheet_semantics.rs");

    for needle in [
        "pub enum SheetPlacement",
        "Bottom,",
        "Left,",
        "Right,",
        "pub(crate) enum SheetSlot",
        "pub(crate) struct SheetPartStateInput",
        "placement: SheetPlacement,",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn resolve_state(input: SheetPartStateInput) -> SheetPartState",
    ] {
        assert!(
            logic.contains(needle),
            "Sheet logic should model discrete state via typed contracts `{needle}`."
        );
    }

    for forbidden in ["placement: String", "placement: Option<String>"] {
        assert!(
            !logic.contains(forbidden),
            "Sheet discrete placement axis should avoid string protocol `{forbidden}`."
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-placement=root_state.placement_attr",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        assert!(
            view.contains(needle),
            "Sheet view should expose machine-readable semantic markers via `{needle}`."
        );
    }

    for needle in [
        "fn state_dismiss_and_keyboard_attrs_follow_contract()",
        "fn normalize_optional_text_trims_and_filters_blank_values()",
        "fn resolve_state_tracks_source_markers()",
        "fn sheet_state_markers_are_observable_queryable_and_closed_set()",
    ] {
        assert!(
            logic.contains(needle) || tests.contains(needle),
            "Type/semantic contract breakpoints should remain directly test-locatable via `{needle}`."
        );
    }
}

#[test]
fn sheet_cross_platform_compile_contract_has_explicit_cfg_and_no_non_wasm_web_sys_usage() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Sheet motion should keep explicit wasm/non-wasm cfg split via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let is_composing = false;",
        "let default_prevented = false;",
    ] {
        assert!(
            view_source.contains(needle),
            "Sheet view should keep explicit platform fallbacks via `{needle}`."
        );
    }

    for source in [&mod_source, &logic_source, &styles_source, &view_source] {
        for forbidden in ["web_sys", "wasm_bindgen", "js_sys", "window.", "document."] {
            assert!(
                !source.contains(forbidden),
                "Non-wasm sheet module files should not reference browser-only API `{forbidden}`."
            );
        }
    }
}

#[test]
fn sheet_headless_web_ssr_mutex_guard_is_preserved() {
    let sheet_view = load_source("src/view.rs");
    let headless_lib = load_source("../ui-headless/src/lib.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "use ui_headless::{",
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
    ] {
        assert!(
            sheet_view.contains(needle),
            "Sheet should keep explicit ui-headless integration contract `{needle}`."
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib.contains(needle),
            "ui-headless should preserve web/ssr mutex compile_error guard `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "rg -n \"mutually exclusive\" \"$MUTEX_LOG\"",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform verification script should keep ui-headless mutex checks via `{needle}`."
        );
    }
}

#[test]
fn sheet_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe() {
    let ui_motion_lib = load_source("../ui-motion/src/lib.rs");
    let ui_motion_non_wasm_test = load_source("../ui-motion/tests/non_wasm_stub.rs");
    let sheet_motion = load_source("src/motion.rs");
    let platform_script = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should preserve non-wasm predictable no-op stub via `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_test.contains(needle),
            "ui-motion non-wasm regression tests should include `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "drop(sanitize_motion(motion));",
        "if !is_open.get() {",
        "finish_exit.run(());",
    ] {
        assert!(
            sheet_motion.contains(needle),
            "Sheet motion non-wasm branch should degrade safely via `{needle}`."
        );
    }

    for needle in [
        "echo \"[platform] compile-only: ui-motion native path\"",
        "cargo check -p ui-motion",
        "echo \"[platform] ui-motion non-wasm stub tests\"",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script.contains(needle),
            "platform/toolchain script should keep ui-motion non-wasm checks via `{needle}`."
        );
    }
}

#[test]
fn sheet_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split() {
    let sheet_motion = load_source("src/motion.rs");
    let ui_motion_spring = load_source("../ui-motion/src/spring.rs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());",
        "if prefers_reduced_motion {",
        "if reduced_motion {",
        "finish_exit.run(());",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            sheet_motion.contains(needle),
            "Sheet motion should include reduced-motion/ssr/wasm branch marker `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring should keep reduced-motion fast-path token `{needle}`."
        );
    }

    for needle in [
        "组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "reduced-motion",
        "SSR 输出必须与客户端 hydration 兼容",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain reduced-motion/SSR/wasm governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let check2_source = load_source("src/check2.md");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep perf budget baseline token `{needle}`."
        );
    }

    assert!(
        pages_source
            .contains("component_doc!(\"Sheet\", \"sheet\", \"Overlays\", overlays::sheet),"),
        "Sheet docs route should stay in coverage traversal for perf probe evidence."
    );

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_source.contains(needle),
            "UiPerfProbe should expose machine-readable perf marker `{needle}`."
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
            "docs coverage e2e should keep blocking perf assertion `{needle}`."
        );
    }

    for needle in [
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance check script should include blocking command `{needle}`."
        );
    }

    assert!(
        todo_source.contains(
            "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据"
        ),
        "Performance governance should keep render_count follow-up marker in TODO plan."
    );

    for needle in [
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Sheet should expose attribution marker `{needle}` for perf triage."
        );
    }

    let view_effect_count = view_source.matches("Effect::new(").count();
    assert_eq!(
        view_effect_count, 0,
        "Sheet view should avoid direct effect loops; found {view_effect_count}.",
    );

    let motion_effect_count = motion_source.matches("Effect::new(").count();
    assert!(
        motion_effect_count <= 3,
        "Sheet motion should keep bounded effect loops (<=3), found {motion_effect_count}.",
    );

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sheet checklist should keep perf governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_view_macro_complexity_is_bounded_for_current_scope() {
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("src/check2.md");

    assert!(
        view_source.contains("view! {"),
        "Sheet should keep explicit render block in view.rs."
    );
    assert!(
        view_source.matches("view! {").count() <= 3,
        "Sheet should keep a bounded number of render blocks (<=3) after semantic extraction."
    );
    assert!(
        view_source.lines().count() <= 260,
        "Sheet view.rs should stay compact; split semantic sub-blocks when this grows."
    );

    for needle in [
        "let root_state = resolve_part_state(logic::SheetSlot::Root, open.get_untracked(), state_inputs);",
        "let backdrop_state = resolve_part_state(logic::SheetSlot::Backdrop, false, state_inputs);",
        "let panel_state = resolve_part_state(logic::SheetSlot::Panel, false, state_inputs);",
        "fn render_backdrop(",
        "fn render_panel(",
    ] {
        assert!(
            view_source.contains(needle),
            "Sheet should keep state derivation outside macro-heavy render block via `{needle}`."
        );
    }

    for forbidden in ["for item in", ".map(|", "collect::<Vec<_>>()", "match ("] {
        assert!(
            !view_source.contains(forbidden),
            "Sheet view should avoid loop-heavy or branch-heavy macro patterns `{forbidden}`."
        );
    }

    for needle in [
        "`view!` 宏复杂度受控",
        "复杂结构按语义子块拆分",
        "编译时间/产物体积异常增长时，优先排查宏展开体量",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sheet checklist should retain macro-complexity governance marker `{needle}`."
        );
    }
}

#[test]
fn sheet_functional_fragment_split_prefers_plain_functions_over_extra_components() {
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("src/check2.md");

    for needle in [
        "fn render_backdrop(",
        "fn render_panel(",
        ") -> impl IntoView {",
        "{render_backdrop(backdrop_state, backdrop_class, is_dismissable, on_close_for_backdrop)}",
        "{render_panel(",
    ] {
        assert!(
            view_source.contains(needle),
            "Sheet view should extract lightweight fragments into plain Rust functions via `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn RenderBackdrop",
        "#[component]\nfn RenderPanel",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sheet should not add local fragment components for lightweight blocks: `{forbidden}`."
        );
    }

    for needle in [
        "函数式拆分优先",
        "纯静态或轻逻辑片段优先函数化",
        "禁止把所有局部片段都升格为 `#[component]`",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sheet checklist should keep functional split governance token `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn sheet_motion_sanitizes_custom_contract_values() {
    let source = load_source("src/motion.rs");

    for needle in [
        "pub fn sanitize_motion(motion: SheetMotion) -> SheetMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "initial_offset_px",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "drop(sanitize_motion(motion));",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_offset_range()",
    ] {
        assert!(
            source.contains(needle),
            "Sheet motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }
}

#[test]
fn sheet_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let custom_motion = SheetMotion {",
        "initial_offset_px: 56.0",
        "title=\"State + Source Markers\"",
        "placement=SheetPlacement::Right",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=custom_motion",
        "on_exit_complete=finish_exit",
        "on_exit_complete=on_marker_exit_complete",
        "Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "sheet docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn sheet_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn sheet() -> AnyView",
        "title=\"Sheet\"",
        "slug=\"sheet\"",
        "description=\"Sheet overlay (mobile-friendly) with placement, spring enter/exit, and dismiss control flags.\"",
        "<Playground title=\"Bottom sheet\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "code_signal=marker_code",
        "<Sheet",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs should include `{needle}` for sheet primary playground coverage.",
        );
    }
}

#[test]
fn sheet_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Bottom sheet\"",
        "<Button on_press=open_sheet>\"Open sheet\"</Button>",
        "open=open",
        "placement=SheetPlacement::Bottom",
        "on_close=on_close",
        "on_exit_complete=on_exit_complete",
        "\"Esc/backdrop closes. Focus trap enabled.\"",
        "title=\"State + Source Markers\"",
        "description=\"Inspect `data-state`, `data-placement-source`, `data-dismiss-source`, `data-keyboard-dismiss-source`, `data-motion-source`, and `data-exit-source` contracts.\"",
        "<Button on_press=open_marker>\"Open marker sheet\"</Button>",
        "open=marker_open",
        "placement=SheetPlacement::Right",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=custom_motion",
        "on_exit_complete=on_marker_exit_complete",
        "initial_offset_px: 56.0",
        "Inspect data-placement-source / data-dismiss-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for sheet contracts.",
        );
    }
}

#[test]
fn sheet_static_fragments_are_constantized_for_docs_examples() {
    let overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "const SHEET_PLAYGROUND_CODE: &str = r#\"",
        "const SHEET_MARKER_PLAYGROUND_CODE: &str = r#\"",
        "let code = Signal::derive(move || SHEET_PLAYGROUND_CODE.to_string());",
        "let marker_code = Signal::derive(move || SHEET_MARKER_PLAYGROUND_CODE.to_string());",
    ] {
        assert!(
            overlays.contains(needle),
            "Sheet docs should keep static code snippets constantized via `{needle}`."
        );
    }

    for needle in [
        "静态片段常量化",
        "可判定为纯静态的片段应避免重复动态构造",
        "静态资源变更路径要清晰",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain static-fragment governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_inner_html_is_not_used_in_component_or_docs_paths() {
    let view = load_source("src/view.rs");
    let styles = load_source("src/styles.rs");
    let logic = load_source("src/logic.rs");
    let overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let check2 = load_source("src/check2.md");

    for source in [view, styles, logic, overlays] {
        assert!(
            !source.contains("inner_html="),
            "Sheet path should not use `inner_html` for untrusted content injection."
        );
    }

    for needle in [
        "`inner_html` 使用约束",
        "仅允许编译期常量",
        "严禁直接或间接注入用户输入",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain inner_html safety token `{needle}`."
        );
    }
}

#[test]
fn sheet_wasm_debug_contract_has_trace_entry_and_feature_isolation() {
    let docs_app = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_e2e = load_source("../../e2e/tests/docs_app_debug_overlay.spec.mjs");
    let cargo_toml = load_source("Cargo.toml");
    let check2 = load_source("src/check2.md");

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app.contains(needle),
            "docs-app should keep wasm debug visual entry via `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"ui-debug-overlay-event\"",
        "data-kind=\"open-change\"",
        "debug overlay captures traced open/close events",
    ] {
        assert!(
            debug_overlay_e2e.contains(needle),
            "debug overlay e2e should keep trace regression contract `{needle}`."
        );
    }

    for needle in [
        "sheet-wasm-debug = [\"component-sheet\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_toml.contains(needle),
            "ui features should keep debug isolation via `{needle}`."
        );
    }

    for needle in [
        "WASM 调试要求",
        "关键状态可追踪",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain wasm-debug governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_dx_contract_has_workbench_and_semantic_e2e_waits() {
    let dev_script = load_source("../../scripts/dev-docs-app.sh");
    let overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let nav_sheet_e2e = load_source("../../e2e/tests/docs_app_nav_sheet.spec.mjs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "exec trunk serve --open true",
        "PATH=\"$HOME/.cargo/bin:$PATH\"",
    ] {
        assert!(
            dev_script.contains(needle),
            "docs dev loop should keep fast feedback script contract `{needle}`."
        );
    }

    for needle in [
        "<Playground title=\"Bottom sheet\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "<Button on_press=open_sheet>\"Open sheet\"</Button>",
    ] {
        assert!(
            overlays.contains(needle),
            "Sheet docs should keep interactive workbench contract via `{needle}`."
        );
    }

    for needle in [
        "body:not(:has(#boot))",
        "[data-slot=\"sheet\"][data-state=\"open\"][data-placement=\"left\"]",
        "[data-slot=\"sheet-panel\"][role=\"dialog\"]",
        "await page.keyboard.press(\"Escape\");",
    ] {
        assert!(
            nav_sheet_e2e.contains(needle),
            "Sheet e2e should keep semantic selectors and settled wait strategy via `{needle}`."
        );
    }
    assert!(
        !nav_sheet_e2e.contains(".docs-mobile-nav"),
        "Sheet e2e should avoid brittle class-based selector contracts."
    );

    for needle in [
        "DX 要求",
        "有 Workbench 隔离画布",
        "常见样式调整应走快速反馈路径",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain DX governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_engineering_and_entry_boundaries_stay_consistent() {
    let lib = load_source("src/lib.rs");
    let css = load_source("src/css.rs");
    let root = load_source("src/root.rs");
    let active_highlight = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let sheet_mod = load_source("src/mod.rs");
    let cargo = load_source("Cargo.toml");
    let check2 = load_source("src/check2.md");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "#[cfg(feature = \"component-sheet\")]",
        "pub use ui_sheet as sheet;",
    ] {
        assert!(
            lib.contains(needle),
            "ui lib entry should keep feature-gated sheet boundary via `{needle}`."
        );
    }
    assert!(
        lib.contains("pub use sheet::{Sheet, SheetMotion, SheetPlacement};")
            || (lib.contains("pub use sheet::Sheet;")
                && lib.contains("pub use sheet::SheetMotion;")
                && lib.contains("pub use sheet::SheetPlacement;")),
        "ui lib entry should re-export Sheet contracts without leaking internals."
    );

    for needle in [
        "#[cfg(feature = \"component-sheet\")]",
        "out.push_str(crate::sheet::styles::CSS);",
    ] {
        assert!(
            css.contains(needle),
            "ui css entry should keep feature-gated sheet css via `{needle}`."
        );
    }

    for needle in [
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
    ] {
        assert!(
            root.contains(needle),
            "UiRoot should keep centralized theme/css/i18n boundary via `{needle}`."
        );
    }

    assert!(
        active_highlight.contains("pub const CSS: &str = r#\"")
            || active_highlight.contains("pub fn attach_active_highlight"),
        "active_highlight entry should stay dedicated to shared highlight style/motion capability."
    );

    for missing in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !manifest_dir.join(missing).exists(),
            "ui should not define deprecated shared primitive file `{missing}`."
        );
    }

    for needle in [
        "tracing = { version = \"0.1\", optional = true }",
        "serde = { version = \"1.0\", features = [\"derive\"], optional = true }",
    ] {
        assert!(
            cargo.contains(needle),
            "engineering dependency surface should keep structured contract via `{needle}`."
        );
    }

    for forbidden in ["tokio::", "async_std::"] {
        assert!(
            !sheet_mod.contains(forbidden),
            "Sheet public boundary should not leak runtime-specific async types: `{forbidden}`."
        );
    }

    for needle in [
        "工程能力统一",
        "`ui` 固定入口文件落点正确",
        "组件目录标准文件落点正确",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain boundary governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_agent_contract_and_streaming_snapshot_markers_are_explicit() {
    let logic = load_source("src/logic.rs");
    let view = load_source("src/view.rs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "pub struct SheetAgentContract",
        "pub fn agent_contract() -> SheetAgentContract",
        "schema_attr: \"sheet.v1\"",
        "render_mode_attr: \"snapshot\"",
        "streaming_attr: \"optional\"",
        "fallback_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            logic.contains(needle),
            "Sheet logic should keep typed agent contract schema via `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-ui-render-mode=agent_contract.render_mode_attr",
        "data-ui-streaming=agent_contract.streaming_attr",
        "data-ui-fallback=agent_contract.fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            view.contains(needle),
            "Sheet view should expose explicit agent-consumable markers via `{needle}`."
        );
    }

    for needle in [
        "语义标记统一升级为 Agent Contract",
        "流式在这里仅指 LLM 输出渲染",
        "`Snapshot` 是所有组件的基础能力",
        "`Streaming` 是否强制，按组件职责判断",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain AI contract token `{needle}`."
        );
    }
}

#[test]
fn sheet_docs_and_semantic_regression_loop_are_explicitly_covered() {
    let this_suite = load_source("tests/sheet_semantics.rs");
    let overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let nav_sheet_e2e = load_source("../../e2e/tests/docs_app_nav_sheet.spec.mjs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "fn sheet_semantics_contract_checks_prioritize_semantics_over_snapshots()",
        "fn sheet_view_uses_logic_state_contracts()",
        "fn sheet_state_markers_are_observable_queryable_and_closed_set()",
    ] {
        assert!(
            this_suite.contains(needle),
            "Sheet regression suite should keep semantics-first tests via `{needle}`."
        );
    }

    for needle in [
        "title=\"Sheet\"",
        "<Playground title=\"Bottom sheet\" code_signal=code>",
        "title=\"State + Source Markers\"",
    ] {
        assert!(
            overlays.contains(needle),
            "Sheet docs should keep beginner-facing entry + advanced path via `{needle}`."
        );
    }

    for needle in [
        "docs-app mobile nav sheet opens and closes",
        "await page.keyboard.press(\"Escape\");",
        "[data-slot=\"sheet\"]",
    ] {
        assert!(
            nav_sheet_e2e.contains(needle),
            "Sheet e2e should keep repeatable key flow contract via `{needle}`."
        );
    }

    for needle in [
        "语义测试优先",
        "E2E 选择器稳定",
        "关键流程纳入可重复回归集合",
        "docs-app 文档、示例、参数矩阵、状态矩阵同步更新",
        "组件文档必须对新手友好",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain docs/e2e governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_docs_app_interactive_playground_is_live_and_reproducible() {
    let overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let nav_sheet_e2e = load_source("../../e2e/tests/docs_app_nav_sheet.spec.mjs");
    let check2 = load_source("src/check2.md");

    for needle in [
        "<Playground title=\"Bottom sheet\" code_signal=code>",
        "<Button on_press=open_sheet>\"Open sheet\"</Button>",
        "<Show when=move || present.get()>",
        "title=\"State + Source Markers\"",
        "<Button on_press=open_marker>\"Open marker sheet\"</Button>",
        "\"open: \" {move || marker_open_raw.get().to_string()}",
    ] {
        assert!(
            overlays.contains(needle),
            "Sheet docs should keep interactive playground signal wiring via `{needle}`."
        );
    }

    for needle in [
        "docs-app mobile nav sheet opens and closes",
        "body:not(:has(#boot))",
        "[data-slot=\"sheet\"][data-state=\"open\"][data-placement=\"left\"]",
        "await page.keyboard.press(\"Escape\");",
    ] {
        assert!(
            nav_sheet_e2e.contains(needle),
            "Sheet e2e should keep reproducible interactive flow checkpoint `{needle}`."
        );
    }

    for needle in [
        "`apps/docs-app` 必须提供 Interactive Playground",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例",
        "Sheet 非 AI Spec 输入组件，该条按 N/A 记录",
    ] {
        assert!(
            check2.contains(needle),
            "Sheet checklist should retain interactive-playground governance token `{needle}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_status_primitives_remains_dom_and_style_free() {
    let overlay_trigger_source = load_source("../ui-state-primitives/src/overlay_trigger.rs");
    let controlled_source = load_source("../ui-state-primitives/src/controlled.rs");
    let primitives = format!("{overlay_trigger_source}\n{controlled_source}");

    for forbidden in [
        "use leptos",
        "leptos::",
        "web_sys::",
        "wasm_bindgen",
        "view! {",
        "NodeRef<",
        "on:click",
        "style=",
    ] {
        assert!(
            !primitives.contains(forbidden),
            "ui-state-primitives sheet-related contracts should avoid DOM/style runtime dependency `{forbidden}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_ui_headless_remains_visual_and_motion_free() {
    let modal_source = load_source("../ui-headless/src/modal.rs");
    let focus_trap_source = load_source("../ui-headless/src/focus_trap.rs");
    let overlay_stack_source = load_source("../ui-headless/src/overlay_stack.rs");
    let headless_lib_source = load_source("../ui-headless/src/lib.rs");
    let headless = format!("{modal_source}\n{focus_trap_source}\n{overlay_stack_source}");

    for forbidden in [
        ".ui-",
        "class_name:",
        "class=",
        "var(--ui-",
        "Spring",
        "keyframe",
        "animate(",
        "request_animation_frame",
    ] {
        assert!(
            !headless.contains(forbidden),
            "ui-headless sheet-related contracts should avoid visual/motion orchestration token `{forbidden}`."
        );
    }

    for required in [
        "pub struct ModalOptions",
        "pub fn use_modal(options: ModalOptions)",
        "pub struct FocusTrapOptions",
        "pub struct FocusTrapHandlers",
        "pub fn use_focus_trap(options: FocusTrapOptions) -> FocusTrapHandlers",
        "pub struct OverlayRegistration",
        "pub fn use_overlay_stack_registration() -> OverlayRegistration",
        "pub mod modal;",
        "pub mod overlay_stack;",
    ] {
        assert!(
            modal_source.contains(required)
                || focus_trap_source.contains(required)
                || overlay_stack_source.contains(required)
                || headless_lib_source.contains(required),
            "ui-headless sheet contracts should keep typed semantic boundary marker `{required}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_view_keeps_decisions_in_logic_layer() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for required in [
        "resolve_part_state(logic::SheetSlot::Root, open.get_untracked(), state_inputs)",
        "resolve_part_state(logic::SheetSlot::Backdrop, false, state_inputs)",
        "resolve_part_state(logic::SheetSlot::Panel, false, state_inputs)",
        "let root_class = logic::compose_class_name(root_state);",
        "if logic::should_close_on_escape(",
        "logic::resolve_state(logic::SheetPartStateInput {",
    ] {
        assert!(
            view_source.contains(required),
            "Sheet view should consume centralized logic output via `{required}`."
        );
    }

    for forbidden in [
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "pub fn should_close_on_escape(",
        "pub(crate) struct SheetPartStateInput",
        "pub(crate) enum SheetSlot",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Sheet view should not hide key state-decision rule `{forbidden}`."
        );
    }

    for required in [
        "pub(crate) struct SheetPartStateInput",
        "pub fn resolve_state(input: SheetPartStateInput) -> SheetPartState",
        "pub fn compose_class_name(state: SheetPartState) -> String",
        "pub fn should_close_on_escape(",
    ] {
        assert!(
            logic_source.contains(required),
            "Sheet key decision rule should stay centralized in logic layer `{required}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let semantics_test_source = load_source("tests/sheet_semantics.rs");

    for required in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "#[prop(optional)] placement: SheetPlacement",
        "#[prop(optional)] aria_labelledby: Option<String>",
        "#[prop(optional)] aria_describedby: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional, default = logic::DEFAULT_DISMISSABLE)] is_dismissable: bool",
        "#[prop(optional, default = logic::DEFAULT_KEYBOARD_DISMISS_DISABLED)]",
        "is_keyboard_dismiss_disabled: bool",
        "#[prop(optional)] motion: SheetMotion",
        "#[prop(optional)]",
        "on_exit_complete: Option<Callback<()>>",
    ] {
        assert!(
            view_source.contains(required),
            "Sheet public parameter naming/default contract should include `{required}`."
        );
    }

    for required in [
        "pub enum SheetPlacement",
        "pub const DEFAULT_DISMISSABLE: bool = true;",
        "pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;",
        "pub fn normalize_optional_text(value: Option<String>) -> Option<String>",
        "pub fn resolve_state(input: SheetPartStateInput) -> SheetPartState",
    ] {
        assert!(
            logic_source.contains(required),
            "Sheet parameter contract should keep naming/type/default normalization marker `{required}`."
        );
    }

    for required in [
        "fn sheet_view_uses_logic_state_contracts()",
        "fn sheet_state_markers_are_observable_queryable_and_closed_set()",
        "fn sheet_docs_playgrounds_lock_state_matrix_contract_values()",
        "fn sheet_semantics_contract_checks_prioritize_semantics_over_snapshots()",
    ] {
        assert!(
            semantics_test_source.contains(required),
            "Sheet semantics suite should keep parameter-contract regression guard `{required}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_parallel_array_api_is_absent_for_sheet_scope() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let view_source = load_source("src/view.rs");

    for forbidden in [
        "labels + children",
        "titles + panels",
        "labels:",
        "titles:",
        "panels:",
        "ItemSpec",
    ] {
        assert!(
            !docs_source.contains(forbidden) && !view_source.contains(forbidden),
            "Sheet scope should avoid parallel-array/implicit semantic token `{forbidden}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_public_api_does_not_leak_platform_or_runtime_types() {
    let mod_source = load_source("src/mod.rs");

    for forbidden in [
        "web_sys::",
        "leptos::web_sys",
        "wasm_bindgen",
        "tokio::",
        "async_std::",
        "runtime::Handle",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Sheet public API boundary should avoid leaking platform/runtime token `{forbidden}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_no_temporary_patch_contract_drift_tokens_in_sheet_scope() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");

    for forbidden in [
        "TODO temporary",
        "TEMP FIX",
        "HACK",
        "workaround",
        "quick fix",
        "remove later",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Sheet should avoid temporary patch contract-drift marker `{forbidden}`."
        );
    }
}

#[test]
fn sheet_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let primitives_source = load_source("../ui-state-primitives/src/overlay_trigger.rs");
    let headless_source = load_source("../ui-headless/src/lib.rs");

    for required in [
        "pub struct OverlayTriggerStateOptions",
        "pub fn use_overlay_trigger_state(options: OverlayTriggerStateOptions) -> OverlayTriggerState",
        "pub on_open_change: Option<OverlayOnOpenChange>",
        "pub mod controllable_state;",
        "pub use controllable_state::{",
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
    ] {
        assert!(
            primitives_source.contains(required)
                || headless_source.contains(required)
                || view_source.contains(required),
            "Sheet reusable state invariant should stay sunk to primitive/headless marker `{required}`."
        );
    }

    for forbidden in ["pub enum LocalSheetState", "pub enum SheetMachine"] {
        assert!(
            !logic_source.contains(forbidden),
            "Sheet logic should not keep reusable generic state machine locally `{forbidden}`."
        );
    }
}

#[test]
fn sheet_check2_marks_forbidden_anti_patterns_complete() {
    let check2_source = load_source("src/check2.md");

    for needle in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "sheet_anti_pattern_status_primitives_remains_dom_and_style_free",
        "sheet_anti_pattern_ui_headless_remains_visual_and_motion_free",
        "sheet_anti_pattern_view_keeps_decisions_in_logic_layer",
        "sheet_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract",
        "sheet_anti_pattern_parallel_array_api_is_absent_for_sheet_scope",
        "sheet_anti_pattern_public_api_does_not_leak_platform_or_runtime_types",
        "sheet_anti_pattern_no_temporary_patch_contract_drift_tokens_in_sheet_scope",
        "sheet_anti_pattern_reusable_state_invariants_are_sunk_to_primitives_or_headless",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sheet checklist should keep forbidden anti-pattern completion evidence `{needle}`."
        );
    }
}

#[test]
fn sheet_check2_marks_architecture_and_api_foundations_complete() {
    let check2_source = load_source("src/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "sheet_a11y_i18n_locale_contract_uses_headless_overlay_attrs",
        "sheet_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sheet checklist should keep completed foundation evidence marker `{needle}`."
        );
    }
}

#[test]
fn sheet_check2_marks_final_merge_gates_complete_including_full_gate() {
    let check2_source = load_source("src/check2.md");

    for needle in [
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
        "sheet_component_files_respect_layered_responsibilities",
        "sheet_state_markers_are_observable_queryable_and_closed_set",
        "sheet_visual_desire_baseline_is_documented_with_overlay_button_input",
        "sheet_semantics_contract_checks_prioritize_semantics_over_snapshots",
        "sheet_anti_pattern_new_params_follow_naming_type_default_and_semantic_contract",
        "sheet_agent_contract_and_streaming_snapshot_markers_are_explicit",
        "sheet_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe",
        "sheet_source_first_docs_are_copy_paste_ready_and_traceable",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Sheet checklist should keep final-merge-gate evidence marker `{needle}`."
        );
    }

    assert!(
        !check2_source.contains("- [ ] "),
        "Sheet checklist should not leave unchecked items after full completion."
    );
}
