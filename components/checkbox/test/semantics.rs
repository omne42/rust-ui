fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "motion" => include_str!("../src/motion.rs"),
        "component_toml" => include_str!("../src/Component.toml"),
        "rbi" => include_str!("../src/checkbox.rbi"),
        "readme" => include_str!("../src/README.md"),
        "docs_forms" => include_str!("../../../apps/docs-app/src/pages/components/pages/forms.rs"),
        "primitive" => include_str!("../../../crates/ui-state-primitives/src/checkbox.rs"),
        "headless_checkbox" => include_str!("../../../crates/ui-headless/src/checkbox.rs"),
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "checkbox_cargo" => include_str!("../Cargo.toml"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "ui_motion_non_wasm_stub_test" => {
            include_str!("../../../crates/ui-motion/tests/non_wasm_stub.rs")
        }
        "ui_motion_spring_test" => include_str!("../../../crates/ui-motion/tests/spring.rs"),
        "ui_components_lib" => include_str!("../../../crates/ui-components/src/lib.rs"),
        "ui_components_cargo" => include_str!("../../../crates/ui-components/Cargo.toml"),
        "ui_components_css" => include_str!("../../../crates/ui-components/src/css.rs"),
        "ui_components_root" => include_str!("../../../crates/ui-components/src/root.rs"),
        "web_demo_cargo" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "docs_app_cargo" => include_str!("../../../apps/docs-app/Cargo.toml"),
        "tree_shaking_script" => {
            include_str!("../../../scripts/check-ui-components-tree-shaking.sh")
        }
        "tree_shaking_budget" => include_str!("../../../scripts/tree_shaking_budget.env"),
        "ci_workflow" => include_str!("../../../.github/workflows/ci.yml"),
        "check_script" => include_str!("../../../scripts/check.sh"),
        "platform_script" => include_str!("../../../scripts/check-ui-components-platforms.sh"),
        "streaming_script" => include_str!("../../../scripts/check-ui-components-streaming.sh"),
        "docs_pages_registry" => {
            include_str!("../../../apps/docs-app/src/pages/components/pages.rs")
        }
        "docs_theme_visual_baseline" => include_str!(
            "../../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs"
        ),
        "e2e_theme_visual_baseline" => {
            include_str!("../../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs")
        }
        "heroui_strategy" => include_str!("../../../docs/spec/heroui-parameter-design-strategy.md"),
        "test_logic" => include_str!("logic.rs"),
        "test_motion" => include_str!("motion.rs"),
        "test_semantics" => include_str!("semantics.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

#[test]
fn checkbox_component_module_boundaries_are_correct() {
    let source = load_source("mod");

    for needle in ["mod logic;", "mod motion;", "pub mod styles;", "mod view;"] {
        assert!(
            source.contains(needle),
            "Checkbox module boundary should include `{needle}`."
        );
    }

    for forbidden in ["pub mod logic", "pub mod motion", "pub mod view"] {
        assert!(
            !source.contains(forbidden),
            "Checkbox internals should stay private; found `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_uses_primitives_headless_motion_and_theme_contracts() {
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let style_source = load_source("styles");
    let primitive_source = load_source("primitive");

    assert!(
        logic_source.contains(
            "pub use ui_state_primitives::checkbox::{CheckboxState, CheckboxStateInput, resolve_state};",
        ),
        "Checkbox logic should consume ui-state-primitives checkbox state contract.",
    );

    for forbidden in [
        "pub struct CheckboxStateInput {",
        "pub struct CheckboxState {",
        "pub fn resolve_state(input: CheckboxStateInput) -> CheckboxState",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Checkbox logic must not reimplement state primitives; found `{forbidden}`."
        );
    }

    for needle in [
        "pub struct CheckboxStateInput",
        "pub struct CheckboxState",
        "pub fn resolve_state(input: CheckboxStateInput) -> CheckboxState",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Checkbox state primitive should define `{needle}` in ui-state-primitives.",
        );
    }

    for needle in [
        "use_checkbox",
        "CheckboxOptions",
        "logic::derive_render_state(logic::CheckboxRenderStateInput",
        "logic::compose_class_name(class_name, variant, size)",
        "motion::attach_root_motion",
        "motion::attach_indicator_motion",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox view should wire `{needle}` through assembly layer.",
        );
    }

    for needle in [
        "var(--ui-checkbox-gap)",
        "var(--ui-checkbox-size-default)",
        "var(--ui-checkbox-radius-default)",
        "var(--ui-text-field-motion-duration)",
        "var(--ui-text-field-motion-easing)",
    ] {
        assert!(
            style_source.contains(needle),
            "Checkbox styles should stay token-first and include `{needle}`.",
        );
    }
}

#[test]
fn checkbox_api_naming_contract_prefers_is_on_default_prefixes() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_toml = load_source("component_toml");
    let rbi = load_source("rbi");
    let readme = load_source("readme");

    for needle in [
        "normalize_checked_signal(",
        "normalize_checked_change_handler(",
        "normalize_is_disabled(",
        "#[prop(optional)] is_checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] on_checked_change: Option<WriteSignal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "let checked_control = logic::resolve_checked_control(",
        "logic::normalize_is_disabled(is_disabled, disabled)",
        "resolve_checked_axis(CheckboxCheckedAxisInput {",
        "name = \"is_checked\"",
        "name = \"on_checked_change\"",
        "name = \"default_checked\"",
        "name = \"is_disabled\"",
        "is_checked: Option<leptos::prelude::ReadSignal<bool>>",
        "on_checked_change: Option<leptos::prelude::WriteSignal<bool>>",
        "default_checked: Option<bool>",
        "is_disabled: Option<bool>",
        "主命名已切到 `is_checked/on_checked_change/default_checked` 与 `is_disabled`。",
        "兼容别名 `checked/set_checked/disabled/on_change` 仍可用",
        "迁移建议：先替换 docs 和业务调用到主命名，再逐步删除别名调用。",
    ] {
        let found = logic_source.contains(needle)
            || view_source.contains(needle)
            || component_toml.contains(needle)
            || rbi.contains(needle)
            || readme.contains(needle);
        assert!(
            found,
            "Checkbox API naming contract should include `{needle}`."
        );
    }
}

#[test]
fn checkbox_controlled_uncontrolled_pair_contract_is_complete() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_toml = load_source("component_toml");
    let rbi = load_source("rbi");
    let readme = load_source("readme");

    for needle in [
        "pub type CheckedControlMode = ui_state_primitives::checkbox::CheckboxControlMode;",
        "pub struct CheckedControl",
        "pub fn resolve_checked_control(",
        "if checked_axis.mode == CheckedControlMode::Controlled {",
        "mode: checked_axis.mode,",
        "let checked_control = logic::resolve_checked_control(",
        "if let Some(on_checked_change) = on_checked_change {",
        "data-state-source=move || render_state.get().state_source_attr",
        "name = \"is_checked\"",
        "name = \"on_checked_change\"",
        "name = \"default_checked\"",
        "is_checked: Option<leptos::prelude::ReadSignal<bool>>",
        "on_checked_change: Option<leptos::prelude::WriteSignal<bool>>",
        "default_checked: Option<bool>",
        "主命名已切到 `is_checked/on_checked_change/default_checked` 与 `is_disabled`。",
    ] {
        let found = logic_source.contains(needle)
            || view_source.contains(needle)
            || component_toml.contains(needle)
            || rbi.contains(needle)
            || readme.contains(needle);
        assert!(
            found,
            "Checkbox control contract should include `{needle}`."
        );
    }

    for forbidden in ["set_fallback_checked", "unwrap_or(set_fallback_checked)"] {
        assert!(
            !view_source.contains(forbidden),
            "Checkbox controlled path must not hide local writes (`{forbidden}`).",
        );
    }
}

#[test]
fn checkbox_default_value_source_is_logic_only() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for needle in [
        "pub fn resolve_checked_control(",
        "resolve_checked_axis(CheckboxCheckedAxisInput {",
        "pub fn compose_class_name(",
        "pub fn normalize_optional_text(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Checkbox logic should own default/value-priority normalization via `{needle}`.",
        );
    }

    for forbidden in [
        "unwrap_or(base_class)",
        ".filter(|value| !value.trim().is_empty())",
        "default_checked.unwrap_or(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Checkbox view should not contain default fallback branch `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_state_normalization_is_centralized_in_logic() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for needle in [
        "pub struct CheckboxRenderStateInput",
        "pub struct CheckboxRenderState",
        "pub fn derive_render_state(input: CheckboxRenderStateInput) -> CheckboxRenderState",
        "let state = resolve_state(CheckboxStateInput {",
        "state_source_attr: input.control_mode.source_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Checkbox logic should centralize state derivation via `{needle}`.",
        );
    }

    for forbidden in [
        "logic::resolve_state(logic::CheckboxStateInput {",
        "data-state-source=if control_mode == logic::CheckedControlMode::Controlled",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Checkbox view should not rebuild state-machine rules (`{forbidden}`).",
        );
    }

    for needle in [
        "let render_state = Memo::new(move |_| {",
        "logic::derive_render_state(logic::CheckboxRenderStateInput {",
        "data-state=move || render_state.get().state.data_state()",
        "data-state-source=move || render_state.get().state_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox view should consume normalized state outputs via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_discrete_states_are_enum_constrained() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for needle in [
        "pub enum CheckboxVariant",
        "pub enum CheckboxSize",
        "pub type CheckedControlMode = ui_state_primitives::checkbox::CheckboxControlMode;",
        "pub enum CheckboxCheckedState",
        "pub struct CheckboxRenderStateInput",
        "pub checked_state: CheckboxCheckedState,",
        "is_checked: input.checked_state.is_checked(),",
        "checked_state: logic::CheckboxCheckedState::from_bool(checked.get()),",
    ] {
        let found = logic_source.contains(needle) || view_source.contains(needle);
        assert!(
            found,
            "Checkbox discrete state constraints should include `{needle}`."
        );
    }

    for forbidden in [
        "variant: String",
        "size: String",
        "mode: String",
        "status: String",
        "status: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "Checkbox should not use free-form string inputs for discrete states (`{forbidden}`).",
        );
    }
}

#[test]
fn checkbox_public_api_does_not_expose_dom_detail_types() {
    let component_toml = load_source("component_toml");
    let rbi = load_source("rbi");
    let view = load_source("view");
    let ui_components_lib = load_source("ui_components_lib");

    for forbidden in ["node_ref", "NodeRef<", "web_sys", "leptos::html::"] {
        assert!(
            !rbi.contains(forbidden),
            "Checkbox public RBI API should not expose DOM detail `{forbidden}`.",
        );
    }

    assert!(
        !component_toml.contains("name = \"node_ref\""),
        "Checkbox Component.toml should not expose node_ref input in public API.",
    );

    assert!(
        !view.contains("#[prop(optional)] node_ref:"),
        "Checkbox component props should not expose a node_ref argument.",
    );

    assert!(
        ui_components_lib.contains(
            "pub use checkbox::{Checkbox, CheckboxMotion, CheckboxSize, CheckboxVariant};"
        ),
        "ui-components exports should re-export CheckboxMotion from checkbox root API instead of internal module paths.",
    );
}

#[test]
fn checkbox_dx_paradox_keeps_default_path_simple_and_docs_show_minimal_example() {
    let component_toml = load_source("component_toml");
    let readme = load_source("readme");
    let docs_forms = load_source("docs_forms");
    let view_source = load_source("view");

    assert!(
        !component_toml.contains("name = \"state\""),
        "Checkbox should not require internal state objects as public mandatory API.",
    );

    assert!(
        readme.contains("view! { <Checkbox>\"Accept terms\"</Checkbox> }"),
        "README should present a <=5-line Hello World default path.",
    );

    for needle in [
        "title=\"Hello World\"",
        "description=\"Minimal default path: no state wiring required.\"",
        "<Checkbox>\"Accept terms\"</Checkbox>",
        "is_checked=interactive_checked",
        "on_checked_change=set_interactive_checked",
    ] {
        assert!(
            docs_forms.contains(needle),
            "docs-app checkbox page should include DX contract marker `{needle}`.",
        );
    }

    for forbidden in ["#[prop(optional)] state:", "#[prop()] state:"] {
        assert!(
            !view_source.contains(forbidden),
            "Checkbox public usage should not force internals or raw contracts (`{forbidden}`).",
        );
    }

    for forbidden in ["use_checkbox(", "ui_state_primitives::", "ui_headless::"] {
        assert!(
            !readme.contains(forbidden) && !docs_forms.contains(forbidden),
            "DX docs should not require users to wire internal contracts directly (`{forbidden}`).",
        );
    }
}

#[test]
fn checkbox_state_markers_are_stable_observable_and_enumerated() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let primitive_source = load_source("primitive");

    for needle in [
        "data-state=move || render_state.get().state.data_state()",
        "data-checked=move || render_state.get().state.is_checked.then_some(\"true\")",
        "data-unchecked=move || render_state.get().state.is_unchecked.then_some(\"true\")",
        "data-disabled=move || render_state.get().state.is_disabled.then_some(\"true\")",
        "data-focus-visible=move || render_state.get().state.is_focus_visible.then_some(\"true\")",
        "data-state-source=move || render_state.get().state_source_attr",
        "data-checked-source=checked_source_attr",
        "data-handler-source=handler_source_attr",
        "aria-checked=move || aria.attrs.aria_checked.get()",
        "aria-disabled=aria.attrs.aria_disabled",
        "role=aria.attrs.role",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox should expose stable semantic marker `{needle}`.",
        );
    }

    for needle in [
        "pub enum CheckboxCheckedValueSource",
        "pub enum CheckboxChangeHandlerSource",
        "pub const fn source_attr(self) -> &'static str",
        "Self::IsChecked => \"is-checked\"",
        "Self::CheckedAlias => \"checked-alias\"",
        "Self::DefaultChecked => \"default-checked\"",
        "Self::ImplicitDefault => \"implicit-default\"",
        "Self::OnCheckedChange => \"on-checked-change\"",
        "Self::SetCheckedAlias => \"set-checked-alias\"",
        "Self::Missing => \"missing\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Checkbox marker values should stay in closed enumerations via `{needle}`.",
        );
    }

    for needle in [
        "pub checked_source_attr: &'static str,",
        "pub handler_source_attr: &'static str,",
        "checked_source_attr: checked_axis.source.source_attr(),",
        "handler_source_attr: handler_source.source_attr(),",
    ] {
        assert!(
            logic_source.contains(needle),
            "Checkbox logic should preserve source markers through `{needle}`.",
        );
    }
}

#[test]
fn checkbox_styles_depend_on_explicit_semantic_state_markers() {
    let style_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");

    for needle in [
        ".ui-checkbox[data-motion-source=\"custom\"]",
        ".ui-checkbox[data-state=\"checked\"]",
        ".ui-checkbox[data-state=\"unchecked\"]",
        ".ui-checkbox[data-disabled=\"true\"]",
        ".ui-checkbox[data-enabled=\"true\"]",
        ".ui-checkbox[data-focus-visible=\"true\"]",
    ] {
        assert!(
            style_source.contains(needle),
            "Checkbox style branches should depend on explicit semantic markers via `{needle}`.",
        );
    }

    for forbidden in [
        ":nth-child",
        ".ui-checkbox:disabled",
        ".ui-checkbox:not(:disabled)",
    ] {
        assert!(
            !style_source.contains(forbidden),
            "Checkbox styles should not guess state through fragile selectors (`{forbidden}`).",
        );
    }

    assert!(
        !view_source.contains("style="),
        "Checkbox view should not carry business style logic in inline style attributes.",
    );

    for needle in [
        "style.set_property(\"--ui-checkbox-scale\"",
        "style.set_property(\"--ui-checkbox-indicator\"",
    ] {
        assert!(
            motion_source.contains(needle),
            "Checkbox runtime style writes should be limited to motion CSS variables (`{needle}`).",
        );
    }

    for forbidden in [
        "style.set_property(\"background",
        "style.set_property(\"border",
        "style.set_property(\"color",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "Checkbox motion should not inject business visual styles at runtime (`{forbidden}`).",
        );
    }
}

#[test]
fn checkbox_styles_use_defensive_variable_fallback_chain_locally() {
    let styles_source = load_source("styles");
    let theme_css_source = include_str!("../../../crates/ui-theme/src/css.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "var(--ui-checkbox-gap, var(--ui-fallback-checkbox-gap))",
        "var(--ui-checkbox-disabled-opacity, var(--ui-fallback-checkbox-disabled-opacity))",
        "var(--ui-checkbox-focus-outline-width, var(--ui-fallback-checkbox-focus-outline-width))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-checkbox-focus-outline-offset, var(--ui-fallback-checkbox-focus-outline-offset))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-accent-fg, var(--ui-fallback-accent-fg))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-checkbox-size-default, var(--ui-fallback-checkbox-size-default))",
        "var(--ui-checkbox-size-sm, var(--ui-fallback-checkbox-size-sm))",
        "var(--ui-checkbox-size-lg, var(--ui-fallback-checkbox-size-lg))",
        "var(--ui-checkbox-radius-default, var(--ui-fallback-checkbox-radius-default))",
        "var(--ui-checkbox-radius-sm, var(--ui-fallback-checkbox-radius-sm))",
        "var(--ui-checkbox-radius-lg, var(--ui-fallback-checkbox-radius-lg))",
        "var(--ui-checkbox-indicator-size-default, var(--ui-fallback-checkbox-indicator-size-default))",
        "var(--ui-checkbox-indicator-size-sm, var(--ui-fallback-checkbox-indicator-size-sm))",
        "var(--ui-checkbox-indicator-size-lg, var(--ui-fallback-checkbox-indicator-size-lg))",
    ] {
        assert!(
            styles_source.contains(required),
            "checkbox styles should keep defensive double-fallback token `{required}`.",
        );
    }

    for required in [
        "--ui-fallback-checkbox-gap:",
        "--ui-fallback-checkbox-disabled-opacity:",
        "--ui-fallback-checkbox-focus-outline-width:",
        "--ui-fallback-checkbox-focus-outline-offset:",
        "--ui-fallback-checkbox-size-default:",
        "--ui-fallback-checkbox-size-sm:",
        "--ui-fallback-checkbox-size-lg:",
        "--ui-fallback-checkbox-radius-default:",
        "--ui-fallback-checkbox-radius-sm:",
        "--ui-fallback-checkbox-radius-lg:",
        "--ui-fallback-checkbox-indicator-size-default:",
        "--ui-fallback-checkbox-indicator-size-sm:",
        "--ui-fallback-checkbox-indicator-size-lg:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should stay SSOT for checkbox fallback token `{required}`.",
        );
    }

    for forbidden in [
        "gap: var(--ui-checkbox-gap);",
        "opacity: var(--ui-checkbox-disabled-opacity);",
        "outline: var(--ui-checkbox-focus-outline-width) solid var(--ui-focus-ring);",
        "outline-offset: var(--ui-checkbox-focus-outline-offset);",
        "border-radius: var(--ui-radius-md);",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg);",
        "color: var(--ui-accent-fg);",
        "font-size: var(--ui-font-size-150, 14px);",
        "line-height: var(--ui-line-height-150, 20px);",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "checkbox styles should not keep raw terminal token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_styles_use_defensive_variable_fallback_chain_locally";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "checkbox_styles_use_defensive_variable_fallback_chain_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep defensive-variable governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_cascade_layer_and_runtime_style_contract_is_enforced_locally() {
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");
    let css_source = load_source("ui_components_css");
    let root_source = load_source("ui_components_root");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let motion_source = load_source("motion");

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-checkbox\")]",
        "out.push_str(crate::checkbox::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css aggregation should keep cascade-layer marker `{required}`.",
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "UiRoot should keep components css injection path marker `{required}`.",
        );
    }

    assert!(
        !view_source.contains("style=") && !logic_source.contains("style="),
        "checkbox view/logic should not embed business inline styles.",
    );

    for required in [
        "style.set_property(\"--ui-checkbox-scale\"",
        "style.set_property(\"--ui-checkbox-indicator\"",
    ] {
        assert!(
            motion_source.contains(required),
            "checkbox motion should use CSS variable runtime updates via `{required}`.",
        );
    }

    for forbidden in [
        "style.set_property(\"top\"",
        "style.set_property(\"left\"",
        "style.set_property(\"right\"",
        "style.set_property(\"bottom\"",
        "style.set_property(\"width\"",
        "style.set_property(\"height\"",
        "style.set_property(\"padding\"",
        "style.set_property(\"margin\"",
        "style.set_property(\"background\"",
        "style.set_property(\"border\"",
        "style.set_property(\"color\"",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox runtime style should stay CSS-variable-only; forbid `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_cascade_layer_and_runtime_style_contract_is_enforced_locally";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "checkbox_cascade_layer_and_runtime_style_contract_is_enforced_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep cascade-layer governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution() {
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let logic_test_source = load_source("test_logic");
    let semantics_test_source = load_source("test_semantics");
    let motion_test_source = load_source("test_motion");

    for needle in [
        "fn resolve_checked_control_uncontrolled_uses_default_and_internal_writer()",
        "fn resolve_checked_control_controlled_without_writer_stays_read_only()",
        "fn derive_render_state_centralizes_state_derivation_and_source_marker()",
    ] {
        assert!(
            logic_test_source.contains(needle),
            "Checkbox semantic test matrix should cover controlled/uncontrolled/disabled branches via `{needle}`.",
        );
    }

    for needle in [
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())",
        "on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())",
        "on:pointerenter=move |_| aria.handlers.hover.on_pointer_enter.run(())",
        "on:pointerleave=move |_| aria.handlers.hover.on_pointer_leave.run(())",
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox semantic contract should expose keyboard/pointer interaction path `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_root_motion(",
        "pub fn attach_indicator_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Checkbox should preserve SSR/wasm split behavior via `{needle}`.",
        );
    }

    for needle in [
        "fn checkbox_state_markers_are_stable_observable_and_enumerated()",
        "fn checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution()",
    ] {
        assert!(
            semantics_test_source.contains(needle),
            "Checkbox semantics test suite should contain explicit semantic-contract assertions via `{needle}`.",
        );
    }

    for source in [logic_test_source, semantics_test_source, motion_test_source] {
        for forbidden in [
            "assert_snapshot!",
            "assert_json_snapshot!",
            "assert_debug_snapshot!",
            "insta::",
        ] {
            assert!(
                !source.contains(forbidden),
                "Checkbox tests must not replace semantic assertions with snapshots (`{forbidden}`).",
            );
        }
    }
}

#[test]
fn checkbox_component_files_respect_responsibility_boundaries() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{CheckboxSize, CheckboxVariant};",
        "pub use motion::CheckboxMotion;",
        "pub use view::Checkbox;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal export boundary via `{needle}`.",
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod motion",
        "pub mod view",
        "pub fn resolve_checked_control(",
        "#[component]",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry implementation details (`{forbidden}`).",
        );
    }

    for needle in [
        "pub fn normalize_checked_signal(",
        "pub fn normalize_checked_change_handler(",
        "pub fn normalize_is_disabled(",
        "pub fn derive_render_state(input: CheckboxRenderStateInput) -> CheckboxRenderState",
        "pub struct CheckedControl",
        "pub struct CheckboxRenderState",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should own normalization/derivation/source markers via `{needle}`.",
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef<",
        "web_sys",
        "leptos::html::",
        "style.set_property(",
        "on:click",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not contain DOM/style/event implementation (`{forbidden}`).",
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should keep token-first static CSS in a dedicated constant.",
    );
    for needle in [
        "var(--ui-checkbox-gap)",
        "var(--ui-checkbox-size-default)",
        "var(--ui-checkbox-radius-default)",
        "var(--ui-text-field-motion-duration)",
        "var(--ui-text-field-motion-easing)",
    ] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should consume theme tokens via `{needle}`.",
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "on:click=",
        "Accept terms",
        "aria-label",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not carry component structure/behavior/copy (`{forbidden}`).",
        );
    }

    for needle in [
        "use_checkbox",
        "CheckboxOptions",
        "view! {",
        "logic::resolve_checked_control(",
        "logic::derive_render_state(logic::CheckboxRenderStateInput {",
        "motion::attach_root_motion(",
        "motion::attach_indicator_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should focus on rendering and headless mounting via `{needle}`.",
        );
    }
    for forbidden in [
        "resolve_checked_axis(",
        "resolve_checked_change_handler_source(",
        "SpringAnimator::new(",
        "style.set_property(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not reimplement primitive/motion internals (`{forbidden}`).",
        );
    }

    for needle in [
        "pub struct CheckboxMotion",
        "pub fn sanitize_motion(motion: CheckboxMotion) -> CheckboxMotion",
        "pub fn attach_root_motion(",
        "pub fn attach_indicator_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should map semantic state to ui-motion contract via `{needle}`.",
        );
    }
    for forbidden in [
        "view! {",
        "use_checkbox(",
        "role=",
        "aria-checked",
        "data-state=",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not carry view/headless semantics (`{forbidden}`).",
        );
    }
}

#[test]
fn checkbox_component_directory_standard_files_follow_contract_and_na_paths_locally() {
    let check2_source = include_str!("../check2.md");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-components-component-files.sh");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "checkbox component directory should include `{required_file}`.",
        );
    }
    for absent_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(absent_file).exists(),
            "checkbox component directory should keep `{absent_file}` absent.",
        );
    }

    for required in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{CheckboxSize, CheckboxVariant};",
        "pub use motion::CheckboxMotion;",
        "pub use view::Checkbox;",
    ] {
        assert!(
            mod_source.contains(required),
            "checkbox mod.rs should keep minimal stable export marker `{required}`.",
        );
    }
    for forbidden in [
        "pub mod logic",
        "pub mod motion",
        "pub mod view",
        "mod spec;",
        "mod render;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "checkbox mod.rs should not over-export or drift to `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CheckedControl",
        "pub fn resolve_checked_control(",
        "pub struct CheckboxRenderStateInput",
        "pub struct CheckboxRenderState",
        "pub fn derive_render_state(input: CheckboxRenderStateInput) -> CheckboxRenderState",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox logic.rs should keep normalized state derivation marker `{required}`.",
        );
    }
    for forbidden in ["view! {", "NodeRef<", "web_sys::", "window()", "document()"] {
        assert!(
            !logic_source.contains(forbidden),
            "checkbox logic.rs should stay free of DOM/platform token `{forbidden}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-",
        ".ui-checkbox[data-state=\"checked\"]",
    ] {
        assert!(
            styles_source.contains(required),
            "checkbox styles.rs should keep token-first CSS marker `{required}`.",
        );
    }
    for forbidden in ["#[component]", "use ui_headless", "use leptos", "on:click="] {
        assert!(
            !styles_source.contains(forbidden),
            "checkbox styles.rs should avoid render/headless concern `{forbidden}`.",
        );
    }

    for required in [
        "#[component]",
        "let aria = use_checkbox(CheckboxOptions {",
        "view! {",
        "data-state=move || render_state.get().state.data_state()",
        "lang=aria.attrs.lang",
        "dir=aria.attrs.dir",
        "motion::attach_root_motion(",
        "motion::attach_indicator_motion(",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox view.rs should keep render + headless mount marker `{required}`.",
        );
    }
    for forbidden in [
        "resolve_checked_axis(",
        "resolve_checked_change_handler_source(",
        "SpringAnimator::new(",
        "style.set_property(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "checkbox view.rs should avoid hidden primitive/motion internals `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CheckboxMotion",
        "pub fn sanitize_motion(motion: CheckboxMotion) -> CheckboxMotion",
        "pub fn attach_root_motion(",
        "pub fn attach_indicator_motion(",
        "ui_motion::spring::SpringAnimator::new(",
    ] {
        assert!(
            motion_source.contains(required),
            "checkbox motion.rs should keep semantic->motion mapping marker `{required}`.",
        );
    }
    for forbidden in [
        "view! {",
        "use_checkbox(",
        "role=",
        "aria-checked",
        "data-state=",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "checkbox motion.rs should avoid view/headless concern `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_component_directory_standard_files_follow_contract_and_na_paths_locally";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "checkbox_component_directory_standard_files_follow_contract_and_na_paths_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep component-directory governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_file_placement_discipline_contract_is_explicit_for_interactive_component_scope_locally()
{
    let check2_source = include_str!("../check2.md");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let script_source = include_str!("../../../scripts/check-ui-components-component-files.sh");

    assert!(
        check2_source.contains("文件落点纪律"),
        "checkbox check2 should explicitly track file-placement discipline gate.",
    );

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required_file).exists(),
            "file-placement discipline requires `{required_file}` in checkbox source directory.",
        );
    }
    for forbidden_file in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden_file).exists(),
            "checkbox should keep `{forbidden_file}` absent in current scope.",
        );
    }

    assert!(
        mod_source.contains("mod logic;")
            && mod_source.contains("mod motion;")
            && mod_source.contains("pub mod styles;")
            && mod_source.contains("mod view;"),
        "mod.rs should keep canonical module boundary for file-placement discipline.",
    );

    assert!(
        logic_source.contains(
            "pub fn derive_render_state(input: CheckboxRenderStateInput) -> CheckboxRenderState"
        ) && styles_source.contains("pub const CSS: &str =")
            && view_source.contains("#[component]")
            && motion_source.contains("pub struct CheckboxMotion"),
        "logic/styles/view/motion should keep canonical responsibility anchors.",
    );

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_file_placement_discipline_contract_is_explicit_for_interactive_component_scope_locally";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "checkbox_file_placement_discipline_contract_is_explicit_for_interactive_component_scope_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 should keep file-placement-discipline marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component_locally() {
    let check2_source = include_str!("../check2.md");
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");
    let readme_source = load_source("readme");
    let component_toml = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let script_source = include_str!("../../../scripts/check-ui-components-component-files.sh");

    assert!(
        check2_source.contains("Hyper-Structure Builder（`spec.rs`）"),
        "checkbox checklist should explicitly track hyper-structure builder gate.",
    );

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "checkbox is not a complex schema-driven component; spec.rs should remain N/A.",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "CheckboxSpec",
        "Spec::new(",
        ".render()",
        "schema_version",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !component_toml.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "checkbox should not expose hyper-structure builder artifact `{forbidden}` in current scope.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component_locally";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "checkbox_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 should keep hyper-structure builder marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_context_compression_manifest_and_rbi_are_present_and_consistent_locally() {
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-component-files.sh");
    let component_manifest = load_source("component_toml");
    let component_rbi = load_source("rbi");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required_file in ["Component.toml", "checkbox.rbi"] {
        assert!(
            src_dir.join(required_file).exists(),
            "checkbox context-compression file should exist: `{required_file}`.",
        );
    }

    for required in [
        "schema_version = \"1\"",
        "name = \"Checkbox\"",
        "crate = \"ui-checkbox\"",
        "name = \"is_checked\"",
        "name = \"on_checked_change\"",
        "name = \"default_checked\"",
        "name = \"is_disabled\"",
        "name = \"variant\"",
        "name = \"size\"",
        "name = \"motion\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "checkbox Component.toml should keep context-compression marker `{required}`.",
        );
    }

    for required in [
        "pub enum CheckboxVariant",
        "pub enum CheckboxSize",
        "pub struct CheckboxMotion",
        "pub fn Checkbox(",
        "is_checked: Option<leptos::prelude::ReadSignal<bool>>",
        "on_checked_change: Option<leptos::prelude::WriteSignal<bool>>",
        "default_checked: Option<bool>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            component_rbi.contains(required),
            "checkbox.rbi should keep signature-projection marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_context_compression_manifest_and_rbi_are_present_and_consistent_locally";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "checkbox_context_compression_manifest_and_rbi_are_present_and_consistent_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 should keep context-compression marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_token_first_static_styles_are_aggregated_via_ui_root_without_utility_pollution() {
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let ui_components_css_source = load_source("ui_components_css");
    let ui_components_root_source = load_source("ui_components_root");

    for needle in [
        "pub const CSS: &str",
        "var(--ui-checkbox-gap)",
        "var(--ui-checkbox-size-default)",
        "var(--ui-checkbox-radius-default)",
        "var(--ui-bg)",
        "var(--ui-border)",
        "var(--ui-accent)",
        "var(--ui-focus-ring)",
    ] {
        assert!(
            styles_source.contains(needle),
            "Checkbox styles should remain token-first static css via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-checkbox\")]",
        "out.push_str(crate::checkbox::styles::CSS);",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "Checkbox styles should be feature-gated in ui-components css aggregation via `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] inject_components_css: bool",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root_source.contains(needle),
            "UiRoot should remain the centralized style injection boundary via `{needle}`.",
        );
    }

    for source in [styles_source, view_source, motion_source] {
        for forbidden in ["@apply", "tailwind", "tw-", "styled(", "css!(", "stylex"] {
            assert!(
                !source.contains(forbidden),
                "Checkbox component layer should not adopt utility-first/CSS-in-Rust default marker `{forbidden}`.",
            );
        }
    }

    assert!(
        !view_source.contains("style="),
        "Checkbox view should not inject business inline styles.",
    );
}

#[test]
fn checkbox_visual_desire_gate_reuses_theme_baseline_and_heroui_alignment_contracts() {
    let docs_pages_registry = load_source("docs_pages_registry");
    let theme_visual_baseline = load_source("docs_theme_visual_baseline");
    let e2e_theme_visual_baseline = load_source("e2e_theme_visual_baseline");
    let heroui_strategy = load_source("heroui_strategy");

    for needle in [
        "component_doc!(\"Checkbox\", \"checkbox\", \"Forms\", forms::checkbox)",
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            docs_pages_registry.contains(needle),
            "Checkbox visual desire gate should keep docs registry baseline marker `{needle}`.",
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            theme_visual_baseline.contains(needle),
            "Theme visual baseline page should keep visual quality contract token `{needle}`.",
        );
    }

    for needle in [
        "E2E_VISUAL_BASELINE",
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            e2e_theme_visual_baseline.contains(needle),
            "E2E baseline should keep screenshot regression marker `{needle}`.",
        );
    }

    for needle in [
        "# HeroUI 参数设计风格对齐策略",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
        "HeroUI 对齐结论",
    ] {
        assert!(
            heroui_strategy.contains(needle),
            "HeroUI strategy should keep quality-alignment marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_tree_shaking_contract_is_feature_gated_and_ci_enforced() {
    let ui_components_cargo = load_source("ui_components_cargo");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let web_demo_cargo = load_source("web_demo_cargo");
    let docs_app_cargo = load_source("docs_app_cargo");
    let tree_shaking_script = load_source("tree_shaking_script");
    let tree_shaking_budget = load_source("tree_shaking_budget");
    let ci_workflow = load_source("ci_workflow");

    for needle in [
        "component-checkbox = [\"dep:ui-checkbox\"]",
        "ui-checkbox = { path = \"../../components/checkbox\", optional = true }",
        "all-components = [",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components Cargo feature graph should include `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-checkbox\")]",
        "pub use ui_checkbox as checkbox;",
        "#[cfg(feature = \"all-components\")]",
        "pub use all_components::*;",
        "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]",
        "pub use web_demo_components::*;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components lib export boundary should stay feature-gated via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(feature = \"component-checkbox\")]",
        "out.push_str(crate::checkbox::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "Checkbox CSS aggregation should stay feature-gated via `{needle}`.",
        );
    }

    for needle in [
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "default-features = false",
        "web-demo-components",
    ] {
        assert!(
            web_demo_cargo.contains(needle),
            "web-demo should consume feature-pruned ui-components contract via `{needle}`.",
        );
    }
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo must not pull all-components in its dependency contract.",
    );

    assert!(
        docs_app_cargo.contains(
            "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"all-components\"] }"
        ),
        "docs-app should explicitly opt into all-components as acceptance surface.",
    );

    for needle in [
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "Tree-shaking CI script should enforce `{needle}`.",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(needle),
            "Tree-shaking size budget contract should include `{needle}`.",
        );
    }

    assert!(
        ci_workflow.contains("run: ./scripts/check-ui-components-tree-shaking.sh"),
        "CI workflow should run the tree-shaking gate script.",
    );
}

#[test]
fn checkbox_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = include_str!("../check2.md");
    let tree_shaking_script = load_source("tree_shaking_script");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-checkbox = [\"dep:ui-checkbox\"]",
        "#[cfg(feature = \"component-checkbox\")]",
        "out.push_str(crate::checkbox::styles::CSS);",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-checkbox,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep tree-shaking evidence marker `{required}`.",
        );
    }

    for command in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_tree_shaking_contract_is_feature_gated_and_ci_enforced",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$CHECKBOX_MIN_FEATURES\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$CHECKBOX_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(command),
            "tree-shaking gate script should keep checkbox command `{command}`.",
        );
    }
}

#[test]
fn checkbox_ssr_hydration_discontinuity_contract_avoids_time_random_and_implicit_ids() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let headless_checkbox = load_source("headless_checkbox");
    let component_toml = load_source("component_toml");
    let rbi = load_source("rbi");

    for source in [logic_source, view_source, motion_source, headless_checkbox] {
        for forbidden in [
            "now(",
            "Date::now",
            "SystemTime::now",
            "Instant::now",
            "Uuid::new_v4",
            "uuid::",
            "rand::",
            "thread_rng",
            "random(",
            "Math::random",
        ] {
            assert!(
                !source.contains(forbidden),
                "Checkbox SSR contract should not depend on non-deterministic init `{forbidden}`.",
            );
        }
    }

    for source in [view_source, component_toml, rbi] {
        for forbidden in ["use_id", "IdProvider", "id_provider", "name = \"id\""] {
            assert!(
                !source.contains(forbidden),
                "Checkbox API should not carry hidden/random id generation surface `{forbidden}`.",
            );
        }
    }
}

#[test]
fn checkbox_platform_contract_covers_web_ssr_wasm_and_non_wasm_boundaries() {
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let check_script = load_source("check_script");
    let platform_script = load_source("platform_script");

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_root_motion(",
        "pub fn attach_indicator_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Checkbox motion should keep explicit wasm/non-wasm branches via `{needle}`.",
        );
    }

    for forbidden in ["web_sys", "js_sys", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "Non-wasm-facing checkbox files should not reference browser-only APIs (`{forbidden}`).",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --no-default-features --features inject-css,dev-all-components",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components",
    ] {
        assert!(
            check_script.contains(needle),
            "Workspace gate should keep compile-only evidence command `{needle}`.",
        );
    }

    for needle in [
        "echo \"[platform] compile-only: default native path\"",
        "echo \"[platform] compile-only: ssr native path\"",
        "echo \"[platform] compile-only: web wasm path (ui-headless)\"",
        "echo \"[platform] compile-only: web wasm path\"",
        "echo \"[platform] compile guard: ui-headless web+ssr must fail\"",
    ] {
        assert!(
            platform_script.contains(needle),
            "Platform script should preserve cross-platform compile/guard evidence `{needle}`.",
        );
    }
}

#[test]
fn checkbox_headless_web_ssr_mutex_contract_is_preserved() {
    let ui_headless_lib = load_source("ui_headless_lib");
    let checkbox_cargo = load_source("checkbox_cargo");
    let check_script = load_source("check_script");
    let platform_script = load_source("platform_script");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            ui_headless_lib.contains(needle),
            "ui-headless must keep web/ssr compile-time mutex guard `{needle}`.",
        );
    }

    assert!(
        checkbox_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "Checkbox should depend on shared ui-headless contract through workspace path dependency.",
    );
    assert!(
        !checkbox_cargo.contains("features = [\"web\", \"ssr\"]"),
        "Checkbox must not force-enable both ui-headless web+ssr features together.",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            check_script.contains(needle),
            "Workspace check gate should keep split web/ssr compile path `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "rg -n \"mutually exclusive\"",
    ] {
        assert!(
            platform_script.contains(needle),
            "Platform guard should enforce ui-headless web/ssr mutex via `{needle}`.",
        );
    }
}

#[test]
fn checkbox_motion_non_wasm_stub_contract_is_predictable_and_safe() {
    let motion_source = load_source("motion");
    let ui_motion_lib = load_source("ui_motion_lib");
    let ui_motion_non_wasm_stub_test = load_source("ui_motion_non_wasm_stub_test");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should expose non-wasm no-op backend contract `{needle}`.",
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
        "web::animate(&(), &keyframes, MotionOptions::default());",
    ] {
        assert!(
            ui_motion_non_wasm_stub_test.contains(needle),
            "ui-motion should keep non-wasm stub regression coverage `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_root_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_indicator_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Checkbox non-wasm motion path should be explicit no-op via `{needle}`.",
        );
    }

    for forbidden in ["panic!(", ".expect(", ".unwrap("] {
        assert!(
            !motion_source.contains(forbidden),
            "Checkbox motion should not panic in non-wasm fallback paths (`{forbidden}`).",
        );
    }
}

#[test]
fn checkbox_reduced_motion_ssr_wasm_branches_keep_semantics_consistent() {
    let styles_source = load_source("styles");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let ui_motion_spring_test = load_source("ui_motion_spring_test");
    let check_script = load_source("check_script");

    for needle in [
        "@media (prefers-reduced-motion: reduce)",
        ".ui-checkbox__box {",
        "transition: none;",
    ] {
        assert!(
            styles_source.contains(needle),
            "Checkbox reduced-motion style fallback should include `{needle}`.",
        );
    }

    for needle in [
        "fn reduced_motion_set_target_applies_immediately()",
        "fn reduced_motion_set_target_triggers_on_rest_synchronously()",
        "fn reduced_motion_clear_on_rest_stops_triggering()",
    ] {
        assert!(
            ui_motion_spring_test.contains(needle),
            "ui-motion spring regression suite should cover reduced-motion behavior via `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]\npub fn attach_root_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_root_motion(",
        "#[cfg(target_arch = \"wasm32\")]\npub fn attach_indicator_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_indicator_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "Checkbox motion should keep explicit wasm/ssr(non-wasm) branches via `{needle}`.",
        );
    }

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features inject-css,dev-all-components",
    ] {
        assert!(
            check_script.contains(needle),
            "Workspace compile-only gate should keep SSR/wasm branch coverage command `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || render_state.get().state.data_state()",
        "data-checked=move || render_state.get().state.is_checked.then_some(\"true\")",
        "data-focus-visible=move || render_state.get().state.is_focus_visible.then_some(\"true\")",
        "aria-checked=move || aria.attrs.aria_checked.get()",
        "aria-disabled=aria.attrs.aria_disabled",
        "role=aria.attrs.role",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox view should keep stable semantic contract across SSR/wasm via `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("#[cfg("),
        "Checkbox view semantics should not diverge via target-specific cfg branching.",
    );
}

#[test]
fn checkbox_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally() {
    let motion_source = load_source("motion");
    let motion_unit_test_source = load_source("test_motion");
    let ui_motion_source = load_source("ui_motion_lib");
    let platform_script = load_source("platform_script");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "pub struct CheckboxMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub indicator_spring: ui_motion::spring::SpringConfig,",
        "stiffness: 260.0",
        "damping: 16.0",
        "stiffness: 340.0",
        "damping: 22.0",
        "fn sanitize_spring(",
        "fn sanitize_indicator_spring(",
        "pub fn sanitize_motion(",
        "pub fn attach_root_motion(",
        "pub fn attach_indicator_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "checkbox motion contract should keep marker `{needle}`.",
        );
    }

    for needle in [
        "fn default_motion_has_reasonable_params()",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "fn sanitize_motion_clamps_scale_values_and_keeps_valid_springs()",
    ] {
        assert!(
            motion_unit_test_source.contains(needle),
            "checkbox motion unit coverage should keep `{needle}`.",
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion non-wasm/reduced-motion no-op contract should keep marker `{needle}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally";
    assert!(
        platform_script.contains(script_needle),
        "platform gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "checkbox_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep motion contractualization evidence `{required}`.",
        );
    }
}

#[test]
fn checkbox_performance_governance_budget_is_defined_and_blocking_locally() {
    let shell_source = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = include_str!("../../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let script_source = include_str!("../../../scripts/check-ui-components-performance.sh");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let check2_source = include_str!("../check2.md");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let styles_source = load_source("styles");

    for needle in [
        "\"checkbox\" => UiPerfBudget {",
        "max_mount_ms: 22.0,",
        "max_update_ms: Some(7.0),",
        "max_heap_kb: Some(320.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should define checkbox performance budget via `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(\"Checkbox\", \"checkbox\", \"Forms\", forms::checkbox)",
        "\"Checkbox\"",
        "\"checkbox\"",
        "forms::checkbox",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs catalog should keep checkbox coverage marker `{needle}`.",
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
            "UiPerfProbe should keep perf observability marker `{needle}`.",
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
            "docs coverage e2e should keep blocking perf regression assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep render_count marker `{needle}`.",
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "Button",
        "Input",
    ] {
        assert!(
            check2_source.contains(needle),
            "checkbox checklist should keep performance-governance marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_performance_governance_budget_is_defined_and_blocking_locally",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "let render_state = Memo::new(move |_| {",
        "logic::derive_render_state(logic::CheckboxRenderStateInput {",
        "data-state=move || render_state.get().state.data_state()",
        "data-state-source=move || render_state.get().state_source_attr",
        "data-motion-source=if motion == CheckboxMotion::default()",
        "@media (prefers-reduced-motion: reduce)",
    ] {
        assert!(
            view_source.contains(needle)
                || logic_source.contains(needle)
                || styles_source.contains(needle),
            "checkbox render/state/style contract should keep perf attribution marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let view_source = load_source("view");
    let semantics_source = load_source("test_semantics");
    let performance_script = include_str!("../../../scripts/check-ui-components-performance.sh");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "role=aria.attrs.role",
        "aria-checked=move || aria.attrs.aria_checked.get()",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=move || render_state.get().state.data_state()",
        "data-focus-visible=move || render_state.get().state.is_focus_visible.then_some(\"true\")",
        "data-state-source=move || render_state.get().state_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "checkbox semantic regression contract should keep aria/data/focus marker `{needle}`.",
        );
    }

    for needle in [
        "fn checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution()",
        "fn checkbox_performance_governance_budget_is_defined_and_blocking_locally()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "checkbox semantics suite should keep prerequisite regression test `{needle}`.",
        );
    }

    for forbidden in ["assert_snapshot!", "assert_json_snapshot!", "insta::assert"] {
        assert!(
            !semantics_source.contains(forbidden),
            "checkbox semantic/perf contract must not rely on snapshot-only assertion `{forbidden}`.",
        );
    }

    assert!(
        performance_script.contains(
            "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        ),
        "performance gate script should include checkbox semantic/perf matrix test command.",
    );

    for needle in [
        "render_count",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            todo_source.contains(needle),
            "render_count follow-up tracking contract should keep `{needle}`.",
        );
    }

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution",
        "checkbox_performance_governance_budget_is_defined_and_blocking_locally",
        "checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "render_count",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep semantic/performance regression marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view_source = load_source("view");
    let semantics_source = load_source("test_semantics");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-performance.sh");

    for required in [
        "role=aria.attrs.role",
        "aria-checked=move || aria.attrs.aria_checked.get()",
        "aria-disabled=aria.attrs.aria_disabled",
        "data-state=move || render_state.get().state.data_state()",
        "data-state-source=move || render_state.get().state_source_attr",
        "data-checked-source=checked_source_attr",
        "data-handler-source=handler_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox view should expose semantic contract marker `{required}`."
        );
    }

    for required in [
        "fn checkbox_state_markers_are_stable_observable_and_enumerated()",
        "fn checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution()",
        "fn checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            semantics_source.contains(required),
            "checkbox semantic suite should keep contract-first regression `{required}`."
        );
    }

    for forbidden in ["assert_snapshot!", "assert_json_snapshot!", "insta::assert"] {
        assert!(
            !semantics_source.contains(forbidden),
            "semantic-priority contract should not rely on snapshot-only check `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
        "components/checkbox/test/semantics.rs::checkbox_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep semantic-test-priority marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_view_macro_complexity_is_split_into_semantic_subrenders_locally() {
    let view_source = load_source("view");
    let script_source = include_str!("../../../scripts/check-ui-components-view-macro.sh");
    let check2_source = include_str!("../check2.md");

    assert!(
        view_source.contains("view! {"),
        "Checkbox view should keep explicit leptos render blocks.",
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        3,
        "Checkbox should bound macro expansion to root + indicator + icon semantic blocks.",
    );
    assert!(
        view_source.lines().count() <= 220,
        "Checkbox view.rs should stay compact after semantic split.",
    );
    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Checkbox should keep a single public component boundary.",
    );

    for needle in [
        "fn render_checkbox_indicator_icon() -> impl IntoView",
        "fn render_checkbox_indicator(indicator_ref: NodeRef<html::Span>) -> impl IntoView",
        "{render_checkbox_indicator(indicator_ref)}",
        "const SLOT_CHECKBOX_BOX: &str = \"checkbox-box\";",
        "const SLOT_CHECKBOX_INDICATOR: &str = \"checkbox-indicator\";",
        "const SLOT_CHECKBOX_LABEL: &str = \"checkbox-label\";",
        "data-slot=SLOT_CHECKBOX_BOX",
        "data-slot=SLOT_CHECKBOX_INDICATOR",
        "data-slot=SLOT_CHECKBOX_LABEL",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox view macro split should keep semantic marker `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_checkbox_indicator(",
        "#[component]\nfn render_checkbox_indicator_icon(",
        "for item in",
        "collect::<Vec<_>>()",
        "match children",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Checkbox view should avoid macro-heavy expansion token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_view_macro_complexity_is_split_into_semantic_subrenders_locally";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控",
        "checkbox_view_macro_complexity_is_split_into_semantic_subrenders_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "Checkbox checklist should keep view-macro governance marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_view_functional_split_prefers_plain_functions_over_local_components_locally() {
    let view_source = load_source("view");
    let script_source = include_str!("../../../scripts/check-ui-components-view-macro.sh");
    let check2_source = include_str!("../check2.md");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "Checkbox should keep one public component boundary and avoid local component noise.",
    );

    for needle in [
        "fn render_checkbox_indicator_icon() -> impl IntoView",
        "fn render_checkbox_indicator(indicator_ref: NodeRef<html::Span>) -> impl IntoView",
        "{render_checkbox_indicator(indicator_ref)}",
        "{render_checkbox_indicator_icon()}",
        "const SLOT_CHECKBOX: &str = \"checkbox\";",
        "const SLOT_CHECKBOX_BOX: &str = \"checkbox-box\";",
        "const SLOT_CHECKBOX_INDICATOR: &str = \"checkbox-indicator\";",
        "const SLOT_CHECKBOX_LABEL: &str = \"checkbox-label\";",
        "data-slot=SLOT_CHECKBOX",
        "data-slot=SLOT_CHECKBOX_BOX",
        "data-slot=SLOT_CHECKBOX_INDICATOR",
        "data-slot=SLOT_CHECKBOX_LABEL",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox function-first split should keep marker `{needle}`.",
        );
    }

    for forbidden in [
        "#[component]\nfn render_checkbox_indicator(",
        "#[component]\nfn render_checkbox_indicator_icon(",
        "#[component]\r\nfn render_checkbox_indicator(",
        "#[component]\r\nfn render_checkbox_indicator_icon(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Checkbox local render fragments should remain plain functions (`{forbidden}`).",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_view_functional_split_prefers_plain_functions_over_local_components_locally";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 函数式拆分优先",
        "checkbox_view_functional_split_prefers_plain_functions_over_local_components_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "Checkbox checklist should keep function-first governance marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_static_fragments_are_constantized_with_stable_semantics_locally() {
    let view_source = load_source("view");
    let script_source = include_str!("../../../scripts/check-ui-components-view-macro.sh");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "const SLOT_CHECKBOX: &str = \"checkbox\";",
        "const SLOT_CHECKBOX_BOX: &str = \"checkbox-box\";",
        "const SLOT_CHECKBOX_INDICATOR: &str = \"checkbox-indicator\";",
        "const SLOT_CHECKBOX_LABEL: &str = \"checkbox-label\";",
        "const CHECK_ICON_VIEW_BOX: &str = \"0 0 24 24\";",
        "const CHECK_ICON_STROKE_WIDTH: &str = \"3.5\";",
        "const CHECK_ICON_STROKE_LINECAP: &str = \"round\";",
        "const CHECK_ICON_STROKE_LINEJOIN: &str = \"round\";",
        "const CHECK_ICON_PATH: &str = \"M4.5 12.75l6 6 9-13.5\";",
        "viewBox=CHECK_ICON_VIEW_BOX",
        "stroke_width=CHECK_ICON_STROKE_WIDTH",
        "stroke_linecap=CHECK_ICON_STROKE_LINECAP",
        "stroke_linejoin=CHECK_ICON_STROKE_LINEJOIN",
        "d=CHECK_ICON_PATH",
        "data-slot=SLOT_CHECKBOX",
        "data-slot=SLOT_CHECKBOX_BOX",
        "data-slot=SLOT_CHECKBOX_INDICATOR",
        "data-slot=SLOT_CHECKBOX_LABEL",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox static fragment constantization should include `{needle}`.",
        );
    }

    for needle in [
        "aria-hidden=\"true\"",
        "focusable=\"false\"",
        "role=aria.attrs.role",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox static fragment should preserve stable a11y semantics via `{needle}`.",
        );
    }

    for literal in [
        "\"checkbox\"",
        "\"checkbox-box\"",
        "\"checkbox-indicator\"",
        "\"checkbox-label\"",
        "\"M4.5 12.75l6 6 9-13.5\"",
    ] {
        let count = view_source.matches(literal).count();
        assert_eq!(
            count, 1,
            "Checkbox static literal `{literal}` should be centralized to one constant source.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_static_fragments_are_constantized_with_stable_semantics_locally";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] 静态片段常量化",
        "checkbox_static_fragments_are_constantized_with_stable_semantics_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "Checkbox checklist should keep static fragment governance marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally() {
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-inner-html.sh");
    let docs_source = include_str!("../../../apps/docs-app/src/pages/components/pages/forms.rs");

    for (rel_path, source) in [
        ("mod", load_source("mod")),
        ("logic", load_source("logic")),
        ("styles", load_source("styles")),
        ("motion", load_source("motion")),
        ("view", load_source("view")),
    ] {
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "Checkbox `{rel_path}` should not contain raw-html injection token `{forbidden}`.",
            );
        }
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "Checkbox docs examples should not contain raw-html injection token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] `inner_html` 使用约束",
        "checkbox_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "Checkbox checklist should keep inner-html governance marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally() {
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-wasm-debug.sh");
    let docs_app_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");

    let checkbox_cargo = load_source("checkbox_cargo");
    let ui_components_cargo = load_source("ui_components_cargo");
    let ui_components_lib = load_source("ui_components_lib");
    let component_toml = load_source("component_toml");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let readme_source = load_source("readme");
    let rbi_source = load_source("rbi");

    for needle in ["[features]", "default = []"] {
        assert!(
            checkbox_cargo.contains(needle),
            "checkbox crate feature boundary should include `{needle}`.",
        );
    }

    for forbidden in [
        "wasm-debug",
        "checkbox-wasm-debug",
        "checkbox_wasm_debug",
        "component-checkbox-wasm-debug",
    ] {
        assert!(
            !checkbox_cargo.contains(forbidden),
            "checkbox crate should not expose production-facing wasm debug feature `{forbidden}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui-components should keep shared wasm-debug feature marker `{needle}`.",
        );
    }

    for forbidden in [
        "checkbox-wasm-debug =",
        "checkbox_wasm_debug =",
        "component-checkbox-wasm-debug",
        "component-checkbox\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components feature graph should not leak checkbox-specific debug toggle `{forbidden}`.",
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
        "#[cfg(target_arch = \"wasm32\")]\nmod observability;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui-components root should keep shared wasm-debug isolation marker `{needle}`.",
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
            "docs app should keep wasm-debug visual entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle) || trace_source.contains(needle),
            "global trace/debug overlay should keep marker `{needle}`.",
        );
    }

    for needle in [
        "data-state=move || render_state.get().state.data_state()",
        "data-state-source=move || render_state.get().state_source_attr",
        "data-checked-source=checked_source_attr",
        "data-handler-source=handler_source_attr",
        "data-motion-source=if motion == CheckboxMotion::default()",
    ] {
        assert!(
            view_source.contains(needle),
            "Checkbox should keep stable marker `{needle}` for equivalent debug traceability.",
        );
    }

    for forbidden in [
        "UiTrace",
        "use_ui_trace",
        "provide_ui_trace",
        "trace.emit(",
        "wasm_debug_proxy!",
        "observability::",
        "debug_overlay",
        "request_replay",
        "replay",
        "timeline",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !readme_source.contains(forbidden)
                && !component_toml.contains(forbidden)
                && !rbi_source.contains(forbidden),
            "Checkbox runtime/public contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`.",
    );

    for needle in [
        "- [x] WASM 调试要求",
        "checkbox_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally",
    ] {
        assert!(
            check2_source.contains(needle),
            "Checkbox checklist should keep wasm-debug governance marker `{needle}`.",
        );
    }
}

#[test]
fn checkbox_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na_locally()
 {
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("docs_forms");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-dx.sh");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-style/isolated-canvas marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn checkbox() -> AnyView",
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: edit checkbox props and inspect actual state contracts.\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "let (interactive_checked, set_interactive_checked) = signal(true);",
        "is_checked=interactive_checked",
        "on_checked_change=set_interactive_checked",
        "\"checked: \" {move || interactive_checked.get()}",
        "slug=\"checkbox\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "checkbox docs should keep interactive workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "CHECKBOX_WORKBENCH_STORAGE_KEY",
        "load_checkbox_workbench_state(",
        "save_checkbox_workbench_state(",
        "clear_checkbox_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "checkbox keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent.",
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "checkbox_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep DX governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na_locally";
    assert!(
        script_source.contains(script_needle),
        "DX gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = load_source("docs_forms");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");
    let dx_script_source = include_str!("../../../scripts/check-ui-components-dx.sh");
    let check2_source = include_str!("../check2.md");

    for required in [
        "title=\"Hello World\"",
        "title=\"Variant + Disabled matrix\"",
        "title=\"Controlled vs Uncontrolled (Comparison)\"",
        "data-slot=\"checkbox-streaming-policy\"",
        "data-slot=\"checkbox-streaming-modes\"",
        "Streaming Optional; fallback=snapshot.",
        "Snapshot mode renders verified full output for checkbox semantics.",
        "data-slot=\"checkbox-copy-ready\"",
        "data-slot=\"checkbox-source-paths\"",
        "data-slot=\"checkbox-source-prerequisites\"",
        "code_imports=\"use leptos::prelude::*;\\nuse ui_components::*;\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox docs should keep copy-paste-ready marker `{required}`.",
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "compose_copy_ready_code",
        "data-slot=\"playground-code\"",
        "data-slot=\"code-block\"",
    ] {
        assert!(
            playground_source.contains(required),
            "playground runtime should keep copy-ready import marker `{required}`.",
        );
    }

    for required in [
        "docs-app checkbox playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui_components::*;",
        "Streaming Optional; fallback=snapshot.",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox e2e docs contract should keep copy-ready marker `{required}`.",
        );
    }

    let dx_script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(dx_script_needle),
        "dx gate script should include `{dx_script_needle}`.",
    );

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "components/checkbox/test/semantics.rs::checkbox_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "e2e/tests/docs_app_checkbox_contract.spec.mjs::docs-app checkbox playground source is copy-paste ready",
        "bash scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep docs-product marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 should keep docs-sync/state-matrix rule `{required}`.",
        );
    }
}

#[test]
fn checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = load_source("docs_forms");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let readme_source = load_source("readme");
    let component_toml = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub fn normalize_is_disabled(is_disabled: Option<bool>, disabled: bool) -> bool",
        "is_disabled.unwrap_or(disabled)",
        "pub fn resolve_checked_control(",
        "default_checked: Option<bool>",
        "resolve_checked_axis(CheckboxCheckedAxisInput {",
        "#[prop(optional)] is_checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] on_checked_change: Option<WriteSignal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] disabled: bool",
    ] {
        assert!(
            logic_source.contains(required) || view_source.contains(required),
            "checkbox API/default contract should keep marker `{required}` for docs sync.",
        );
    }

    for required in [
        "name = \"is_checked\"",
        "name = \"on_checked_change\"",
        "name = \"default_checked\"",
        "name = \"is_disabled\"",
        "default = \"None\"",
        "default = \"false\"",
        "is_checked: Option<leptos::prelude::ReadSignal<bool>>",
        "on_checked_change: Option<leptos::prelude::WriteSignal<bool>>",
        "default_checked: Option<bool>",
        "is_disabled: Option<bool>",
        "disabled: bool",
        "`is_checked`",
        "`on_checked_change`",
        "`default_checked`",
        "`is_disabled`",
        "`disabled`（兼容别名）",
    ] {
        assert!(
            component_toml.contains(required)
                || rbi_source.contains(required)
                || readme_source.contains(required),
            "checkbox schema/readme signature should keep docs-sync marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn checkbox() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Variant + Disabled matrix\"",
        "title=\"Controlled vs Uncontrolled (Comparison)\"",
        "description=\"受控路径展示外部单一事实来源；非受控路径由 default_checked 初始化后内部管理。\"",
        "is_checked=checked",
        "on_checked_change=set_checked",
        "is_checked=marketing",
        "on_checked_change=set_marketing",
        "is_disabled=true",
        "is_checked=comparison_controlled",
        "on_checked_change=set_comparison_controlled",
        "default_checked=Some(true)",
        "let (interactive_variant_index, set_interactive_variant_index) = signal(Some(0_usize));",
        "let (interactive_size_index, set_interactive_size_index) = signal(Some(0_usize));",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox docs should keep synced example/matrix/default marker `{required}`.",
        );
    }

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox",
        "checkbox_check2_documents_docs_sync_and_state_matrix_rules",
        "checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "components/checkbox/check2.md should keep docs-sync evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include docs-sync/state-matrix marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let source = include_str!("../check2.md");

    assert!(
        source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "checkbox check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox",
        "title=\"Variant + Disabled matrix\"",
        "title=\"Controlled vs Uncontrolled (Comparison)\"",
        "is_checked/on_checked_change/default_checked/is_disabled",
        "components/checkbox/src/logic.rs",
        "components/checkbox/src/view.rs",
        "components/checkbox/src/Component.toml",
        "checkbox_check2_documents_docs_sync_and_state_matrix_rules",
        "checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "checkbox_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(required),
            "checkbox check2 docs-sync/state-matrix section should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_documentation_as_product_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn checkbox_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("readme");
    let pages_source = load_source("docs_pages_registry");
    let docs_source = load_source("docs_forms");

    for required in [
        "# Checkbox",
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：先用 `<Checkbox>\"Accept terms\"</Checkbox>` 完成交互。",
        "常见受控：在需要外部状态单一事实来源时使用 `is_checked + on_checked_change`。",
        "### Controlled（高级入口）",
    ] {
        assert!(
            readme_source.contains(required),
            "checkbox README should include beginner-first marker `{required}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("checkbox README should include hello-world section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("checkbox README should include common-usage section");
    let readme_progressive = readme_source
        .find("## 先用起来，再进阶")
        .expect("checkbox README should include beginner-first progression section");
    let readme_advanced = readme_source
        .find("### Controlled（高级入口）")
        .expect("checkbox README should include advanced controlled section");
    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_advanced,
        "checkbox README should keep beginner-first progression order (hello -> common -> progression -> advanced).",
    );

    for required in [
        "component_doc!(\"Checkbox\", \"checkbox\", \"Forms\", forms::checkbox)",
        "pub(super) fn checkbox() -> AnyView",
        "title=\"Hello World\"",
        "title=\"Variant + Disabled matrix\"",
        "title=\"Controlled vs Uncontrolled (Comparison)\"",
    ] {
        assert!(
            pages_source.contains(required) || docs_source.contains(required),
            "checkbox docs entry should include `{required}`.",
        );
    }

    let docs_hello = docs_source
        .find("title=\"Hello World\"")
        .expect("checkbox docs should include hello-world playground");
    let docs_matrix = docs_source
        .find("title=\"Variant + Disabled matrix\"")
        .expect("checkbox docs should include matrix playground");
    let docs_advanced = docs_source
        .find("title=\"Controlled vs Uncontrolled (Comparison)\"")
        .expect("checkbox docs should include advanced comparison playground");
    assert!(
        docs_hello < docs_matrix && docs_matrix < docs_advanced,
        "checkbox docs should keep beginner-first order before advanced controls.",
    );
}

#[test]
fn checkbox_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include documentation-as-product marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_documentation_as_product_item_complete() {
    let source = include_str!("../check2.md");

    assert!(
        source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "checkbox check2 should mark documentation-as-product item complete.",
    );

    for required in [
        "components/checkbox/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 先用起来，再进阶",
        "checkbox_check2_documents_documentation_as_product_rules",
        "checkbox_documentation_entry_exists_with_beginner_first_progression",
        "checkbox_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(required),
            "checkbox check2 documentation-as-product section should retain marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_interactive_playground_rules() {
    let check2 = include_str!("../check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2.contains(required),
            "checkbox check2 interactive-playground section should include `{required}`."
        );
    }
}

#[test]
fn checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = load_source("docs_forms");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let mod_source = load_source("mod");
    let check2_source = include_str!("../check2.md");

    for required in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test: edit checkbox props and inspect actual state contracts.\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "controls=move || view!",
        "id_base=\"docs-checkbox-variant\".to_string()",
        "id_base=\"docs-checkbox-size\".to_string()",
        "Switch checked=interactive_checked set_checked=set_interactive_checked",
        "Switch checked=interactive_disabled set_checked=set_interactive_disabled",
        "checked=interactive_custom_class",
        "set_checked=set_interactive_custom_class",
        "let (interactive_checked, set_interactive_checked) = signal(true);",
        "let (interactive_disabled, set_interactive_disabled) = signal(false);",
        "let (interactive_custom_class, set_interactive_custom_class) = signal(false);",
        "is_checked=interactive_checked",
        "on_checked_change=set_interactive_checked",
        "variant=interactive_variant.get()",
        "size=interactive_size.get()",
        "is_disabled=interactive_disabled.get()",
        "\"checked: \" {move || interactive_checked.get()}",
        "\" · disabled: \" {move || interactive_disabled.get()}",
        "data-slot=\"checkbox-e2e-interactive-surface\"",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox docs should provide interactive playground marker `{required}`."
        );
    }

    for required in [
        "let section_class = \"docs-card playground\";",
        "<div class=\"playground__preview\" data-playground-scope=scope_id.clone()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<aside class=\"playground__panel playground__controls\" data-slot=\"playground-controls\">",
    ] {
        assert!(
            playground_source.contains(required),
            "docs-app Playground should keep interactive preview marker `{required}`."
        );
    }

    assert!(
        !docs_source.contains("Spec::new(") && !mod_source.contains("mod spec;"),
        "checkbox interactive playground should keep AI Spec linkage as N/A for non-spec component scope."
    );
    assert!(
        check2_source.contains("AI Spec 联动条款对该组件按 N/A 处理"),
        "checkbox check2 should document AI Spec clause as N/A for non-spec component."
    );
}

#[test]
fn checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");

    for required in [
        "docs-app checkbox key flow is repeatable and failures map to semantic breakpoints",
        "await page.goto(CHECKBOX_PAGE);",
        "await waitForWasmReady(page);",
        "resolveControlledFlow(page)",
        "await checkbox.focus();",
        "await expect(checkbox).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "await expectCheckboxSettled(checkbox, {",
        "await page.reload();",
        "await expect(checkbox).toHaveAttribute(\"data-state-source\", \"controlled\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox interactive playground should keep repeatable e2e marker `{required}`."
        );
    }
}

#[test]
fn checkbox_dx_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include interactive-playground marker `{required}`."
        );
    }
}

#[test]
fn checkbox_check2_marks_interactive_playground_item_complete() {
    let check2 = include_str!("../check2.md");

    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "checkbox check2 should mark interactive-playground item complete."
    );

    for required in [
        "title=\"Interactive Playground\"",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox",
        "e2e/tests/docs_app_checkbox_contract.spec.mjs::docs-app checkbox key flow is repeatable and failures map to semantic breakpoints",
        "AI Spec 联动条款对该组件按 N/A 处理",
        "components/checkbox/test/semantics.rs::checkbox_check2_documents_interactive_playground_rules",
        "components/checkbox/test/semantics.rs::checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/checkbox/test/semantics.rs::checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "crates/ui-components/tests/checkbox_semantics.rs::checkbox_check2_documents_interactive_playground_rules",
        "crates/ui-components/tests/checkbox_semantics.rs::checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "crates/ui-components/tests/checkbox_semantics.rs::checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "crates/ui-components/tests/checkbox_semantics.rs::checkbox_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "checkbox check2 interactive-playground section should reference `{required}`."
        );
    }
}

#[test]
fn checkbox_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 source-first section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = load_source("docs_forms");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");
    let logic_source = load_source("logic");
    let view_source = load_source("view");

    for required in [
        "data-slot=\"checkbox-source-first\"",
        "data-slot=\"checkbox-copy-ready\"",
        "data-slot=\"checkbox-source-paths\"",
        "data-slot=\"checkbox-source-prerequisites\"",
        "<code>\"apps/docs-app/src/playground.rs::compose_copy_ready_code\"</code>",
        "<code>\"components/checkbox/src/view.rs\"</code>",
        "<code>\"components/checkbox/src/logic.rs\"</code>",
        "<code>\"components/checkbox/src/styles.rs\"</code>",
        "<code>\"apps/docs-app/src/pages/components/pages/forms.rs\"</code>",
        "<code>\"component-checkbox\"</code>",
        "<code>\"inject-css\"</code>",
        "code_imports=\"use leptos::prelude::*;\\nuse ui_components::*;\".to_string()",
    ] {
        assert!(
            docs_source.contains(required),
            "checkbox source-first docs should include `{required}`.",
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str = \"use leptos::prelude::*;\\nuse ui_components::*;\";",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "data-slot=\"playground-toggle-code\"",
        "data-slot=\"playground-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "playground copy-ready pipeline should include `{required}`.",
        );
    }

    for required in [
        "docs-app checkbox playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui_components::*;",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox e2e source-first contract should include `{required}`.",
        );
    }

    for required in [
        "#[prop(optional)] is_checked: Option<ReadSignal<bool>>",
        "#[prop(optional)] on_checked_change: Option<WriteSignal<bool>>",
        "#[prop(optional)] default_checked: Option<bool>",
        "is_checked=checked",
        "on_checked_change=set_checked",
        "default_checked=Some(true)",
    ] {
        assert!(
            logic_source.contains(required)
                || view_source.contains(required)
                || docs_source.contains(required),
            "source-first docs/code sync contract should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include source-first marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = include_str!("../check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "checkbox check2 should mark source-first copy-paste-ready item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "e2e/tests/docs_app_checkbox_contract.spec.mjs::docs-app checkbox playground source is copy-paste ready",
        "checkbox_check2_documents_source_first_copy_paste_ready_rules",
        "checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "checkbox_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 source-first section should retain marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 heroui-benchmark docs-sync section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("heroui_strategy");
    let pages_source = load_source("docs_pages_registry");
    let docs_source = load_source("docs_forms");
    let readme_source = load_source("readme");

    for required in [
        "### Checkbox 同步记录（2026-02-20）",
        "参数模型同步：`Checkbox` 参数主轴保持 `is_checked/default_checked/on_checked_change`",
        "component_doc!(\"Checkbox\", \"checkbox\", \"Forms\", forms::checkbox)",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox()",
        "title=\"Checkbox\"",
        "slug=\"checkbox\"",
        "`components/checkbox/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include checkbox synchronization marker `{required}`.",
        );
    }

    for required in [
        "component_doc!(",
        "\"Checkbox\"",
        "\"checkbox\"",
        "forms::checkbox",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose checkbox entry marker `{required}`.",
        );
    }

    for required in [
        "pub(super) fn checkbox() -> AnyView {",
        "title=\"Checkbox\"",
        "slug=\"checkbox\"",
        "data-slot=\"checkbox-source-first\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app checkbox page should stay indexable via marker `{required}`.",
        );
    }

    for required in [
        "# Checkbox",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
    ] {
        assert!(
            readme_source.contains(required),
            "checkbox README should stay as equivalent component docs entry via `{required}`.",
        );
    }
}

#[test]
fn checkbox_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-dx.sh");

    for required in [
        "echo \"[dx] contract: checkbox heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should include heroui-benchmark marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = include_str!("../check2.md");

    assert!(
        check2_source.contains(
            "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"
        ),
        "checkbox check2 should mark heroui-benchmark docs-sync item complete."
    );

    for required in [
        "docs/spec/heroui-parameter-design-strategy.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "apps/docs-app/src/pages/components/pages/forms.rs::checkbox",
        "components/checkbox/src/README.md",
        "checkbox_check2_documents_heroui_benchmark_docs_sync_rules",
        "checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "checkbox_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 heroui-benchmark section should retain marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_status_primitives_layer_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "所有状态原语必须从 `status-primitives`（`ui-state-primitives`）获取，组件层只能消费，不得自造。",
        "桥接规范：`ui-state-primitives` 结构体必须是 POJO（Plain Old Rust Object），不持有 Leptos `Signal` 或框架绑定状态容器。",
        "消费规范：`ui-headless` 或组件 `logic.rs` 负责解包 `Signal` 当前值传入 primitive 方法，并将结果显式写回 `Signal`。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 status-primitives section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let primitive_source = load_source("primitive");
    let check2_source = include_str!("../check2.md");

    for required in [
        "pub use ui_state_primitives::checkbox::{CheckboxState, CheckboxStateInput, resolve_state};",
        "resolve_checked_axis(CheckboxCheckedAxisInput {",
        "resolve_checked_control(",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox logic should consume status-primitives marker `{required}`.",
        );
    }

    for forbidden in [
        "pub struct CheckboxStateInput {",
        "pub struct CheckboxState {",
        "pub fn resolve_state(input: CheckboxStateInput) -> CheckboxState {",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "checkbox component layer should not reimplement primitive state machine `{forbidden}`.",
        );
    }

    for required in [
        "pub struct CheckboxStateInput",
        "pub struct CheckboxState",
        "pub fn resolve_state(input: CheckboxStateInput) -> CheckboxState",
    ] {
        assert!(
            primitive_source.contains(required),
            "ui-state-primitives checkbox primitive should define `{required}`.",
        );
    }

    for forbidden in ["leptos", "web_sys", "wasm_bindgen"] {
        assert!(
            !primitive_source.contains(forbidden),
            "ui-state-primitives checkbox primitive should remain framework-agnostic; found `{forbidden}`.",
        );
    }

    assert!(
        view_source.contains("logic::resolve_state(logic::CheckboxStateInput {"),
        "checkbox view should consume normalized primitive output from logic.",
    );

    for required in [
        "components/checkbox/src/logic.rs",
        "ui_state_primitives::checkbox",
        "crates/ui-state-primitives/src/checkbox.rs",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 status-primitives evidence should reference `{required}`.",
        );
    }
}

#[test]
fn checkbox_two_pass_geometry_rendering_is_na_and_measurement_free() {
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let motion_source = load_source("motion");
    let component_toml = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "getBoundingClientRect(",
        "getClientRects(",
        "clientWidth",
        "clientHeight",
        "offsetWidth",
        "offsetHeight",
        "measure_pass",
        "rectification_pass",
        "layout_rect",
        "geometry_rect",
    ] {
        let found = logic_source.contains(forbidden)
            || view_source.contains(forbidden)
            || motion_source.contains(forbidden)
            || component_toml.contains(forbidden)
            || rbi_source.contains(forbidden);
        assert!(
            !found,
            "checkbox should remain two-pass-geometry free and avoid `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "N/A 理由（checkbox）：该组件为单体勾选控件，无 tooltip/popover/menu 几何定位语义，不读取 DOM 尺寸/位置，不存在 `Intent -> Measure -> Rectification` 回流链路。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 two-pass section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_engineering_script_covers_status_primitives_and_two_pass_geometry_contracts() {
    let script_source = include_str!("../../../scripts/check-ui-components-engineering.sh");

    for required in [
        "echo \"[engineering] contract: checkbox status-primitives sourcing + two-pass geometry NA\"",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_status_primitives_layer_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_two_pass_geometry_rendering_is_na_and_measurement_free",
    ] {
        assert!(
            script_source.contains(required),
            "engineering check script should include marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_status_primitives_and_two_pass_geometry_items_complete() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。",
        "checkbox_check2_documents_status_primitives_layer_rules",
        "checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine",
        "checkbox_two_pass_geometry_rendering_is_na_and_measurement_free",
        "checkbox_engineering_script_covers_status_primitives_and_two_pass_geometry_contracts",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 should retain status-primitives/two-pass completion evidence `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep e2e selector/stable-wait governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");
    let docs_source = load_source("docs_forms");

    for required in [
        "const CHECKBOX_PAGE = \"/#/components/checkbox\";",
        "body:not(:has(#boot))",
        "async function waitForWasmReady(page)",
        "async function expectCheckboxReady(surface, checkbox)",
        "data-slot=\"checkbox-e2e-interactive-surface\"",
        "data-slot=\"checkbox-e2e-controlled-surface\"",
        "data-slot=\"checkbox-e2e-controlled-row\"",
        "data-slot=\"checkbox-e2e-controlled-target\"",
        "data-slot=\"checkbox-e2e-controlled-checked\"",
        "data-slot=\"checkbox-e2e-controlled-last-change\"",
        "data-slot=\"checkbox-e2e-matrix-surface\"",
        "data-slot=\"checkbox-e2e-disabled-on\"",
        "data-slot=\"checkbox-e2e-disabled-off\"",
        "[data-slot=\"checkbox-e2e-controlled-target\"] [data-slot=\"checkbox\"][role=\"checkbox\"]",
        "[data-slot=\"checkbox-e2e-disabled-on\"] [data-slot=\"checkbox\"][role=\"checkbox\"]",
        "[data-slot=\"checkbox-e2e-disabled-off\"] [data-slot=\"checkbox\"][role=\"checkbox\"]",
    ] {
        assert!(
            e2e_source.contains(required) || docs_source.contains(required),
            "checkbox e2e selector/stable-wait contract should include `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "hasText:",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox e2e contract should avoid brittle selector/wait token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");

    for required in [
        "async function expectCheckboxReady(surface, checkbox)",
        "async function expectCheckboxSettled(checkbox, expected)",
        "toHaveAttribute(\"data-e2e-ready\", \"true\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"snapshot\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
        "toHaveAttribute(\"data-ui-action\", \"press.toggle\")",
        "toHaveAttribute(\"data-ui-source\", \"state-primitives\")",
        "await expectCheckboxSettled(checkbox, {",
        "ariaChecked: \"false\"",
        "dataState: \"unchecked\"",
        "uiState: \"unchecked\"",
        "ariaChecked: \"true\"",
        "dataState: \"checked\"",
        "uiState: \"checked\"",
        "await expectCheckboxSettled(disabledOn, {",
        "await expectCheckboxSettled(disabledOff, {",
        "uiState: \"disabled\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox e2e ready/settled contract should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_e2e_check_script_covers_selector_and_settled_wait_contract() {
    let script_source = include_str!("../../../scripts/check-ui-components-e2e-checkbox.sh");

    for required in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths",
    ] {
        assert!(
            script_source.contains(required),
            "checkbox e2e check script should enforce `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "components/checkbox/test/semantics.rs::checkbox_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/checkbox/test/semantics.rs::checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "components/checkbox/test/semantics.rs::checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths",
        "components/checkbox/test/semantics.rs::checkbox_e2e_check_script_covers_selector_and_settled_wait_contract",
        "crates/ui-components/tests/checkbox_semantics.rs::checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "scripts/check-ui-components-e2e-checkbox.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 e2e selector stability section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_repeatable_e2e_regression_collection() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep repeatable e2e regression governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");
    let script_source = include_str!("../../../scripts/check-ui-components-e2e-checkbox.sh");

    for required in [
        "docs-app checkbox key flow is repeatable and failures map to semantic breakpoints",
        "await page.reload();",
        "await expect(checkbox).toHaveAttribute(\"data-state-source\", \"controlled\");",
        "await expect(checkbox).toHaveAttribute(\"data-checked-source\", \"is_checked\");",
        "await expect(checkbox).toHaveAttribute(\"data-handler-source\", \"on_checked_change\");",
        "await checkbox.focus();",
        "await expect(checkbox).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "await expectCheckboxSettled(checkbox, {",
        "await expect(checkedState).toContainText(\"true\");",
        "await expect(changeState).toContainText(\"true\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox repeatable key-flow contract should include semantic breakpoint `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "locator(\"text=",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "checkbox repeatable key-flow should avoid brittle/non-semantic token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "checkbox e2e script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_checkbox_contract.spec.mjs");
    let script_source = include_str!("../../../scripts/check-ui-components-e2e-checkbox.sh");

    for required in [
        "docs-app checkbox high-risk paths keep focus keyboard and disabled branches semantically explicit",
        "await checkbox.focus();",
        "await expect(checkbox).toBeFocused();",
        "await page.keyboard.press(\"Space\");",
        "const disabledOn = matrix",
        "const disabledOff = matrix",
        "await expect(disabledOn).toBeDisabled();",
        "await expect(disabledOn).toHaveAttribute(\"aria-disabled\", \"true\");",
        "await expect(disabledOff).toBeDisabled();",
        "await expect(disabledOff).toHaveAttribute(\"aria-disabled\", \"true\");",
        "await disabledOn.click({ force: true });",
        "await disabledOff.click({ force: true });",
        "uiState: \"disabled\"",
    ] {
        assert!(
            e2e_source.contains(required),
            "checkbox high-risk e2e flow should include semantic marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "checkbox e2e script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = include_str!("../../../scripts/check-ui-components-e2e-checkbox.sh");

    for required in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_repeatable_e2e_regression_collection",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(required),
            "checkbox e2e key-flow check script should enforce `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_e2e_repeatable_key_flow_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "components/checkbox/test/semantics.rs::checkbox_check2_documents_repeatable_e2e_regression_collection",
        "components/checkbox/test/semantics.rs::checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/checkbox/test/semantics.rs::checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/checkbox/test/semantics.rs::checkbox_e2e_check_script_covers_selector_and_key_flow_contracts",
        "crates/ui-components/tests/checkbox_semantics.rs::checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "scripts/check-ui-components-e2e-checkbox.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox check2 repeatable key-flow section should include `{required}`.",
        );
    }
}

#[test]
fn checkbox_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope_locally() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");
    let readme_source = load_source("readme");
    let component_toml = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let check2_source = include_str!("../check2.md");

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/checkbox/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "checkbox should keep spec/schema serialization path as N/A for simple component scope."
    );

    let combined = [
        mod_source,
        logic_source,
        view_source,
        styles_source,
        motion_source,
        readme_source,
        component_toml,
        rbi_source,
    ]
    .join("\n");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "mod spec;",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep engineering governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events_locally()
 {
    let ui_components_cargo = load_source("ui_components_cargo");
    let button_view_source = include_str!("../../../components/button/src/view.rs");
    let combined = [
        load_source("mod"),
        load_source("logic"),
        load_source("view"),
        load_source("styles"),
        load_source("motion"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui_components::button::state_change\"",
    ] {
        assert!(
            ui_components_cargo.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`.",
        );
    }

    for forbidden_feature in [
        "checkbox-wasm-debug =",
        "checkbox_wasm_debug =",
        "component-checkbox\", \"dep:tracing",
        "component-checkbox-wasm-debug",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden_feature),
            "checkbox should not define component-local tracing feature `{forbidden_feature}` when no local debug event/replay contract exists.",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::checkbox::",
        "const CHECKBOX_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "checkbox should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_engineering_contract_avoids_runtime_leaks_in_public_api_surface_locally() {
    let mod_source = load_source("mod");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let styles_source = load_source("styles");
    let motion_source = load_source("motion");

    for source in [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ] {
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
                "checkbox engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "checkbox public module boundary should not leak web_sys types."
    );
}

#[test]
fn checkbox_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally()
 {
    let logic_source = load_source("logic");
    let component_toml = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let readme_source = load_source("readme");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-components-engineering.sh");

    for required in [
        "pub enum CheckboxAgentSchemaVersion",
        "Self::V1 => \"v1\"",
        "schema_version = \"1\"",
        "schema = \"ui.checkbox.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            logic_source.contains(required) || component_toml.contains(required),
            "checkbox version contract should keep v1 marker `{required}`.",
        );
    }

    let combined = [logic_source, component_toml, rbi_source, readme_source].join("\n");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !combined.contains(forbidden),
            "without major breaking upgrade, checkbox should not introduce migration marker `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally";
    assert!(
        script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Checkbox` 改动未引入跨大版本 API 破坏升级",
        "CheckboxAgentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.checkbox.agent-contract.v1",
        "checkbox_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "check2 should include checkbox version-deprecation migration marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_engineering_check_script_covers_serde_tracing_and_runtime_boundaries_locally() {
    let script_source = include_str!("../../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope_locally",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events_locally",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_engineering_contract_avoids_runtime_leaks_in_public_api_surface_locally",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn checkbox_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let motion = load_source("motion");
    let combined = format!("{module}\n{logic}\n{styles}\n{view}\n{motion}");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "checkbox non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn checkbox_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![",
        "Cow::Borrowed(\"ui-checkbox\")",
        "Cow::Borrowed(variant.class_name())",
        "Cow::Borrowed(size.class_name())",
        "Cow::Owned(custom_class_name)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic.contains(required),
            "checkbox logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "trimmed.to_string()",
        "format!(\"ui-checkbox {} {}\"",
        "\"ui-checkbox\".to_string()",
        "String::from(\"ui-checkbox\")",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "checkbox string hotspot contract should avoid `{forbidden}`.",
        );
    }
}

#[test]
fn checkbox_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = include_str!("../../../scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering gate script should include checkbox rust-hygiene command `{needle}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "components/checkbox/test/semantics.rs::checkbox_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/checkbox/test/semantics.rs::checkbox_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/checkbox/test/semantics.rs::checkbox_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-components-engineering.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep rust-hygiene evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_ui_components_fixed_entry_files_follow_layered_boundaries_locally() {
    let check2_source = include_str!("../check2.md");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_components_root = load_source("ui_components_root");
    let active_highlight =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state = include_str!("../../../crates/ui-headless/src/controllable_state.rs");
    let presence = include_str!("../../../crates/ui-headless/src/presence.rs");
    let a11y = include_str!("../../../crates/ui-headless/src/a11y.rs");
    let script_source = include_str!("../../../scripts/check-ui-components-entrypoints.sh");

    for required in [
        "#[cfg(feature = \"component-checkbox\")]",
        "pub use ui_checkbox as checkbox;",
        "pub mod root;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib.rs should keep fixed entry marker `{required}`.",
        );
    }

    for forbidden in ["pub use web_sys", "web_sys::", "NodeRef<", "JsValue"] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components lib.rs should not leak platform detail `{forbidden}`.",
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-checkbox\")]",
        "out.push_str(crate::checkbox::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css.rs should keep fixed entry marker `{required}`.",
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "ui-components root.rs should keep centralized injection marker `{required}`.",
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should contain `{required}`.",
        );
    }

    for forbidden in ["Checkbox", "aria-", "data-state"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic and avoid component semantic token `{forbidden}`.",
        );
    }

    for required in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state.contains(required),
            "ui-headless controllable_state canonical path should contain `{required}`.",
        );
    }

    for required in [
        "pub struct Presence",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
    ] {
        assert!(
            presence.contains(required),
            "ui-headless presence canonical path should contain `{required}`.",
        );
    }

    for required in ["pub fn locale_attrs(", "pub fn aria_controls_when_open("] {
        assert!(
            a11y.contains(required),
            "ui-headless a11y canonical path should contain `{required}`.",
        );
    }

    let ui_components_src_dir =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-components/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui-components/src/{forbidden_file} should be absent by fixed-entrypoint contract.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_ui_components_fixed_entry_files_follow_layered_boundaries_locally";
    assert!(
        script_source.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "checkbox_ui_components_fixed_entry_files_follow_layered_boundaries_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep fixed-entrypoint governance marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_agent_contract_is_schema_typed_and_machine_readable_locally() {
    let check2_source = include_str!("../check2.md");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_toml = load_source("component_toml");

    for typed_source in [
        "pub const CHECKBOX_AGENT_SCHEMA: &str = \"ui.checkbox.agent-contract\";",
        "pub enum CheckboxAgentSchemaVersion",
        "pub enum CheckboxAgentIntent",
        "pub enum CheckboxAgentAction",
        "pub enum CheckboxAgentState",
        "pub enum CheckboxAgentSource",
        "pub struct CheckboxAgentContract",
        "pub struct CheckboxAgentContractInput",
        "fn resolve_agent_state(render_state: CheckboxRenderState) -> CheckboxAgentState",
        "pub fn resolve_agent_contract(input: CheckboxAgentContractInput) -> CheckboxAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "Checkbox Agent Contract should stay type-derived via `{typed_source}`.",
        );
    }

    for marker in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source",
        "data-ui-checked-source=move || agent_contract.get().checked_source",
        "data-ui-handler-source=move || agent_contract.get().handler_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "Checkbox view should mount Agent Contract marker `{marker}`.",
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.checkbox.agent-contract.v1\"",
        "intent = \"selection.toggle\"",
        "action = \"press.toggle\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-output-status\"",
    ] {
        assert!(
            component_toml.contains(required),
            "Checkbox manifest should keep Agent Contract marker `{required}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "format!(\"data-ui-schema",
        "format!(\"data-ui-intent",
        "format!(\"data-ui-state",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Checkbox Agent Contract should avoid free-form schema token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "checkbox_agent_contract_is_schema_typed_and_machine_readable_locally",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep Agent Contract evidence `{required}`.",
        );
    }
}

#[test]
fn checkbox_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally() {
    let check2_source = include_str!("../check2.md");
    let logic_source = load_source("logic");
    let view_source = load_source("view");
    let component_toml = load_source("component_toml");
    let script_source = include_str!("../../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [\"children()\", \"render_checkbox_indicator()\", \"render_checkbox_indicator_icon()\"]",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            component_toml.contains(required),
            "Checkbox manifest should keep whitelist-safe render path marker `{required}`.",
        );
    }

    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Checkbox Agent Contract render path should forbid `{forbidden}`.",
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_agent_contract_is_schema_typed_and_machine_readable_locally",
        "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene gate script should include `{script_needle}`.",
        );
    }

    for required in [
        "checkbox_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally",
        "白名单能力边界",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep Agent Contract whitelist evidence `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = include_str!("../check2.md");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let docs_forms_source = load_source("docs_forms");
    let streaming_script_source = load_source("streaming_script");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "checkbox 不是 LLM 正文阅读面，本组件保持 snapshot-only 渲染路径；仅暴露治理型 `data-ui-stream-support/fallback/output-status` 标记，不引入 token 增量传输协议。",
        "components/checkbox/test/semantics.rs::checkbox_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "use_ai_space_state",
        "AiSpace",
        "data-ui-stream-mode",
        "data-ui-output-state",
        "stream_chunk",
        "token_delta",
        "incremental_patch",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !docs_forms_source.contains(forbidden),
            "checkbox should not carry LLM-streaming protocol token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = include_str!("../check2.md");
    let streaming_script_source = load_source("streaming_script");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "components/checkbox/test/semantics.rs::checkbox_check2_documents_snapshot_as_default_baseline_capability",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep snapshot-baseline marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_snapshot_as_default_baseline_capability";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}

#[test]
fn checkbox_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let check2_source = include_str!("../check2.md");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let component_toml = load_source("component_toml");
    let rbi_source = load_source("rbi");
    let readme_source = load_source("readme");
    let streaming_script_source = load_source("streaming_script");

    for required in [
        "children: Children,",
        "let class = logic::compose_class_name(class_name, variant, size);",
        "let render_state = Memo::new(move |_| {",
        "data-state=move || render_state.get().state.data_state()",
        "data-state-source=move || render_state.get().state_source_attr",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox snapshot baseline should keep complete-render marker `{required}`.",
        );
    }

    for required in [
        "pub fn resolve_checked_control(",
        "pub fn derive_render_state(input: CheckboxRenderStateInput) -> CheckboxRenderState",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox logic should keep stable complete-input normalization marker `{required}`.",
        );
    }

    for required in [
        "name = \"snapshot_rendering\"",
        "enabled = true",
        "name = \"is_checked\"",
        "name = \"default_checked\"",
        "name = \"variant\"",
        "name = \"size\"",
        "name = \"motion\"",
    ] {
        assert!(
            component_toml.contains(required),
            "checkbox manifest should keep snapshot baseline capability/input marker `{required}`.",
        );
    }

    for required in [
        "pub fn Checkbox(",
        "children: leptos::children::Children,",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(required),
            "checkbox RBI should project complete snapshot-render signature marker `{required}`.",
        );
    }

    assert!(
        readme_source.contains("view! { <Checkbox>\"Accept terms\"</Checkbox> }"),
        "checkbox docs should keep complete snapshot render hello-world path."
    );

    for forbidden in [
        "stream_chunk",
        "token_delta",
        "partial_payload",
        "incremental_patch",
        "data-ui-stream-mode",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "checkbox snapshot baseline should not depend on streaming-only token `{forbidden}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );

    {
        let required = "components/checkbox/test/semantics.rs::checkbox_snapshot_baseline_consumes_complete_result_and_renders_stably";
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep snapshot-stability evidence marker `{required}`.",
        );
    }
}

#[test]
fn checkbox_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let check2_source = include_str!("../check2.md");
    let view_source = load_source("view");
    let logic_source = load_source("logic");
    let component_toml = load_source("component_toml");
    let streaming_script_source = load_source("streaming_script");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "checkbox 不是正文阅读面，归类为 `Streaming Optional`。",
        "`fallback=snapshot`",
        "`data-ui-output-status`",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "components/checkbox/test/semantics.rs::checkbox_check2_marks_streaming_scope_as_optional_with_snapshot_fallback",
    ] {
        assert!(
            check2_source.contains(required),
            "checkbox checklist should keep streaming required/optional marker `{required}`.",
        );
    }

    for required in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            view_source.contains(required),
            "checkbox view should keep streaming-governance + role/aria continuity marker `{required}`.",
        );
    }

    for required in [
        "pub enum CheckboxAgentStreamSupport",
        "pub enum CheckboxAgentStreamFallback",
        "pub enum CheckboxAgentOutputStatus",
        "stream_support: CheckboxAgentStreamSupport::Optional",
        "stream_fallback: CheckboxAgentStreamFallback::Snapshot",
        "output_status: CheckboxAgentOutputStatus::Verified",
    ] {
        assert!(
            logic_source.contains(required),
            "checkbox logic should keep typed streaming-governance marker `{required}`.",
        );
    }

    for required in [
        "name = \"stream_support\"",
        "values = [\"optional\"]",
        "name = \"stream_fallback\"",
        "values = [\"snapshot\"]",
        "name = \"output_status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_toml.contains(required),
            "checkbox manifest should keep streaming-governance marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_marks_streaming_scope_as_optional_with_snapshot_fallback";
    assert!(
        streaming_script_source.contains(script_needle),
        "streaming gate script should include `{script_needle}`.",
    );
}
