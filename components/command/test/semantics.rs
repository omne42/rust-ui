use ui_test_support::source_contract;

static DOCS_COLLECTIONS_COMMAND_SOURCE: std::sync::LazyLock<&'static str> =
    std::sync::LazyLock::new(|| {
        let parent = source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/collections_command.rs",
        );
        let child = source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/collections_command/command.rs",
        );
        let child_compat = child.replace(
            "pub(crate) fn command() -> AnyView {",
            "pub(super) fn command() -> AnyView {",
        );
        Box::leak(format!("{parent}\n{child_compat}").into_boxed_str())
    });

fn docs_command_source() -> &'static str {
    *DOCS_COLLECTIONS_COMMAND_SOURCE
}

#[test]
fn command_view_mounts_headless_semantics_contracts() {
    let source = include_str!("../src/view.rs");

    for needle in [
        "use_listbox(ListBoxOptions",
        "command_input_attrs(lang, dir)",
        "resolve_command_input_key_down(&key, logic::has_query_text(query_value.as_str()))",
        "command_option_a11y_attrs(CommandOptionA11yInput",
        "role=input_a11y.role",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "logic::resolve_root_state(logic::CommandRootStateInput",
    ] {
        assert!(
            source.contains(needle),
            "command view should keep semantic contract marker `{needle}`."
        );
    }

    assert!(
        !source.contains("has_query: !query.get().trim().is_empty()"),
        "query presence should be derived in logic.rs, not in view.rs."
    );
}

#[test]
fn command_a11y_i18n_l10n_contract_is_headless_backed() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");

    for needle in [
        "use_ui_i18n",
        "CommonStrings",
        "command_input_attrs(lang, dir)",
        "command_option_a11y_attrs(CommandOptionA11yInput",
        "common_strings.command_placeholder.as_ref()",
        "common_strings.command_empty_label.as_ref()",
        "common_strings.command_aria_label.as_ref()",
        "lang=input_lang.clone()",
        "dir=input_dir",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "command A11y/i18n contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CommandSourceAttr",
        "I18n",
        "CommandSourceAttr::I18n => \"i18n\"",
    ] {
        assert!(
            mod_source.contains(needle),
            "command source marker should expose i18n source marker `{needle}`."
        );
    }

    for needle in [
        "pub const DEFAULT_PLACEHOLDER",
        "pub const DEFAULT_EMPTY_LABEL",
        "pub const DEFAULT_ARIA_LABEL",
    ] {
        assert!(
            logic_source.contains(needle),
            "command logic should retain default fallback source marker `{needle}`."
        );
    }

    for forbidden in [
        "Type a command or search...",
        "No results found.",
        "Command menu",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs must not hardcode user-visible text marker `{forbidden}`."
        );
    }
}

#[test]
fn command_state_observability_contract_uses_stable_data_and_aria_markers() {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");

    for needle in [
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-id-source=move || root_state.get().id_source_attr.as_attr()",
        "data-placeholder-source=move || root_state.get().placeholder_source_attr.as_attr()",
        "data-empty-label-source=move || root_state.get().empty_label_source_attr.as_attr()",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "aria-selected=move || option_attrs().aria_selected",
        "aria-disabled=move || option_attrs().aria_disabled",
        "data-focused=move || option_attrs().data_focused",
        "data-selected=move || option_attrs().data_selected",
    ] {
        assert!(
            view_source.contains(needle),
            "command observability contract should keep marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CommandRootStateAttr",
        "pub enum CommandQueryAttr",
        "pub enum CommandDisabledAttr",
        "pub enum CommandSourceAttr",
        "pub enum CommandQueryControlAttr",
        "pub enum CommandQueryDefaultSourceAttr",
        "pub enum CommandQueryChangeSourceAttr",
        "CommandQueryControlAttr::Controlled => \"controlled\"",
        "CommandQueryControlAttr::Uncontrolled => \"uncontrolled\"",
        "CommandQueryDefaultSourceAttr::Provided => \"provided\"",
        "CommandQueryDefaultSourceAttr::Empty => \"empty\"",
        "CommandQueryChangeSourceAttr::Provided => \"provided\"",
        "CommandQueryChangeSourceAttr::None => \"none\"",
    ] {
        assert!(
            mod_source.contains(needle),
            "command observability marker set should be closed via enum mapping `{needle}`."
        );
    }
}

#[test]
fn command_styles_depend_on_semantic_state_markers_not_fragile_dom_shape() {
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");

    for needle in [
        ".ui-command[data-state=\"empty\"]",
        ".ui-command[data-query=\"present\"]",
        ".ui-command[data-disabled=\"disabled\"]",
        ".ui-command__option[data-focused=\"true\"]",
        ".ui-command__option[data-selected=\"true\"] .ui-command__item-label",
        ".ui-command__option[data-disabled=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "command styles should keep semantic selector marker `{needle}`."
        );
    }

    for forbidden in [":nth-child(", ":nth-of-type(", "[class*=", "[class^="] {
        assert!(
            !styles_source.contains(forbidden),
            "command styles should not depend on fragile selector marker `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not inject runtime inline style marker `{forbidden}`."
        );
    }
}

#[test]
fn command_token_first_static_style_contract_is_enforced() {
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let ui_components_css_source = include_str!("../../../crates/ui/src/css.rs");
    let ui_root_source = include_str!("../../../crates/ui/src/root.rs");

    for needle in [
        "pub const CSS: &str",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-command-option-padding-y, var(--ui-fallback-command-option-padding-y))",
    ] {
        assert!(
            styles_source.contains(needle),
            "command styles should keep token-first marker `{needle}`."
        );
    }

    assert!(
        ui_components_css_source.contains("#[cfg(feature = \"component-command\")]"),
        "ui css aggregation should keep command feature gate."
    );
    assert!(
        ui_components_css_source.contains("out.push_str(crate::command::styles::CSS);"),
        "ui css aggregation should include command styles constant."
    );
    assert!(
        ui_root_source.contains("if inject_components_css.get_value() {"),
        "UiRoot should gate component CSS injection behind inject_components_css."
    );
    assert!(
        ui_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject aggregated ui CSS through css.rs."
    );

    for forbidden in ["style=", "style:", "tw-", "tailwind", "styled-components"] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not default to utility-first/CSS-in-Rust marker `{forbidden}`."
        );
    }
}

#[test]
fn command_styles_use_defensive_variable_fallback_chain() {
    let styles_source = include_str!("../src/styles.rs");
    let theme_css_source =
        source_contract::source_from_file_relative(file!(), "../../../crates/ui-theme/src/css.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-command-panel-max-width, var(--ui-fallback-command-panel-max-width))",
        "var(--ui-command-input-wrap-padding, var(--ui-fallback-command-input-wrap-padding))",
        "var(--ui-command-option-padding-y, var(--ui-fallback-command-option-padding-y))",
        "var(--ui-command-option-padding-x, var(--ui-fallback-command-option-padding-x))",
        "var(--ui-command-shortcut-padding-x, var(--ui-fallback-command-shortcut-padding-x))",
        "var(--ui-command-shortcut-padding-y, var(--ui-fallback-command-shortcut-padding-y))",
    ] {
        assert!(
            styles_source.contains(needle),
            "command styles should keep defensive fallback marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-command-panel-max-width",
        "--ui-fallback-command-input-wrap-padding",
        "--ui-fallback-command-option-padding-y",
        "--ui-fallback-command-option-padding-x",
        "--ui-fallback-command-shortcut-padding-x",
        "--ui-fallback-command-shortcut-padding-y",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme fallback SSOT should keep command fallback token `{needle}`."
        );
    }

    for forbidden in [
        "rgb(", "rgba(", "hsl(", "hsla(", ", 12px)", ", 16px)", ", 20px)", ", 8px)", ", 4px)",
        ", 2px)",
    ] {
        assert!(
            !styles_source.to_ascii_lowercase().contains(forbidden),
            "command styles should avoid hardcoded literal terminal values marker `{forbidden}`."
        );
    }

    let script_needle =
        "cargo test -p ui-command --lib command_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(script_needle),
        "contract hygiene script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "command_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep defensive-variable evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_cascade_layer_and_runtime_style_contract_is_enforced() {
    let ui_components_css_source = include_str!("../../../crates/ui/src/css.rs");
    let ui_root_source = include_str!("../../../crates/ui/src/root.rs");
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-command\")]",
        "out.push_str(crate::command::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui css aggregation should keep @layer-ui command marker `{needle}`."
        );
    }

    assert!(
        ui_root_source.contains("if inject_components_css.get_value() {"),
        "UiRoot should keep component CSS injection behind inject_components_css gate."
    );
    assert!(
        ui_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should inject aggregated component CSS through css.rs."
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "command should forbid plain inline runtime style marker `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not use runtime inline style path marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract hygiene script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "command_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep cascade-layer evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_tree_shaking_contract_is_feature_gated() {
    let ui_components_cargo = include_str!("../../../crates/ui/Cargo.toml");
    let ui_components_lib = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css = include_str!("../../../crates/ui/src/css.rs");
    let web_demo_cargo = include_str!("../../../apps/web-demo/Cargo.toml");
    let command_cargo = include_str!("../Cargo.toml");

    for needle in [
        "component-command = [\"dep:ui-command\"]",
        "ui-command = { path = \"../../components/command\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(needle),
            "ui should keep component-command feature gate marker `{needle}`."
        );
    }

    assert!(
        ui_components_lib
            .contains("#[cfg(feature = \"component-command\")]\npub use ui_command as command;"),
        "ui lib should gate command module export behind component-command feature."
    );
    assert!(
        ui_components_css.contains("#[cfg(feature = \"component-command\")]"),
        "ui css aggregation should gate command styles behind component-command feature."
    );
    assert!(
        ui_components_css.contains("out.push_str(crate::command::styles::CSS);"),
        "ui css aggregation should include command CSS only through the gated path."
    );

    for needle in [
        "#[cfg(feature = \"all-components\")]\nmod all_components {",
        "#[cfg(feature = \"all-components\")]\npub use all_components::*;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "all-components registry should remain feature-gated marker `{needle}`."
        );
    }

    assert!(
        web_demo_cargo.contains(
            "ui = { path = \"../../crates/ui\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"
        ),
        "web-demo should consume ui with default-features disabled and explicit feature set."
    );
    assert!(
        command_cargo.contains("[features]\ndefault = []"),
        "ui-command crate should keep empty default feature set for source-mode minimal consumption."
    );
}

#[test]
fn command_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let script_source = include_str!("../../../scripts/check-ui-tree-shaking.sh");

    for required in [
        "COMMAND_MIN_FEATURES=\"component-command,inject-css\"",
        "cargo test -p ui-command --lib command_tree_shaking_contract_is_feature_gated",
        "cargo test -p ui-command --lib command_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo test -p ui-command --lib command_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "COMMAND_TREE_OUTPUT=\"$(cargo tree -e features -i ui -p ui --no-default-features --features \"$COMMAND_MIN_FEATURES\")\"",
        "feature \"component-command\" (command-line)",
        "feature \"inject-css\" (command-line)",
        "command minimal feature tree should not pull all-components",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$COMMAND_MIN_FEATURES\"",
    ] {
        assert!(
            script_source.contains(required),
            "tree-shaking gate script should enforce `{required}`.",
        );
    }
}

#[test]
fn command_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-command",
        "crates/ui/src/lib.rs",
        "crates/ui/src/css.rs",
        "command_tree_shaking_contract_is_feature_gated",
        "command_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "scripts/check-ui-tree-shaking.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn command_default_visual_quality_contract_has_hierarchy_and_feedback() {
    let styles_source = include_str!("../src/styles.rs");
    let docs_page_source = docs_command_source();

    for needle in [
        ".ui-command {",
        "box-shadow: var(--ui-shadow-sm, var(--ui-fallback-shadow-sm));",
        ".ui-command__group-heading {",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "font-weight: 600;",
        ".ui-command__group-items {",
        "gap: var(--ui-command-group-items-gap, var(--ui-fallback-command-group-items-gap));",
        ".ui-command__input:focus-visible {",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring));",
        ".ui-command__option[data-focused=\"true\"]",
        ".ui-command__option[data-selected=\"true\"] .ui-command__item-label",
    ] {
        assert!(
            styles_source.contains(needle),
            "command default theme quality should keep marker `{needle}`."
        );
    }

    assert!(
        docs_page_source.contains("description=\"baseline-compatible command palette"),
        "docs-app should keep a default baseline-compatible command presentation."
    );
}

#[test]
fn command_semantics_tests_cover_behavior_matrix_not_visual_snapshots() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let motion_source = include_str!("../src/motion.rs");
    let semantics_test_source = include_str!("./semantics.rs");
    let logic_test_source = include_str!("./logic.rs");

    for needle in [
        "#[prop(optional)] query: Option<Signal<String>>",
        "#[prop(optional, into)] default_query: Option<String>",
        "#[prop(optional)] on_query_change: Option<Callback<String>>",
        "let query_state = use_controllable_state(query, Some(default_query), on_query_change);",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "#[prop(optional)] is_disabled: bool",
        "disabled=is_disabled",
        "aria-disabled=listbox.attrs.aria_disabled",
        "resolve_command_input_key_down(&key, logic::has_query_text(query_value.as_str()))",
        "on:keydown=on_input_key_down",
        "on:pointermove=move |_| on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "on_option_click.run(index);",
    ] {
        assert!(
            view_source.contains(needle),
            "command semantics matrix should include marker `{needle}`."
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(feature = \"web\")",
        "cfg(feature = \"ssr\")",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should avoid platform-split semantic contract marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic should avoid platform-split semantic contract marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion should avoid platform-split semantic contract marker `{forbidden}`."
        );
    }

    let forbidden = [
        ["assert", "_snapshot!"].concat(),
        ["insta::assert", "_snapshot!"].concat(),
        ["to_match", "_snapshot"].concat(),
        ["snapshot", "_assertions"].concat(),
    ];
    for forbidden in forbidden {
        assert!(
            !semantics_test_source.contains(forbidden.as_str()),
            "command semantics tests should not rely on visual snapshot marker `{forbidden}`."
        );
        assert!(
            !logic_test_source.contains(forbidden.as_str()),
            "command logic tests should not rely on visual snapshot marker `{forbidden}`."
        );
    }
}

#[test]
fn command_component_files_follow_responsibility_boundaries() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Command;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep export boundary marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "NodeRef", "on:click", "web_sys", "wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not carry implementation detail marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn resolve_root_state",
        "pub fn resolve_state",
        "pub fn filter_groups",
        "pub fn compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "NodeRef",
        "event_target_value",
        "on:click",
        "style=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not contain render/DOM marker `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "styles.rs should keep token-first static style marker `{needle}`."
        );
    }
    for forbidden in ["#[component]", "view! {", "NodeRef", "on:click"] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not contain runtime/render marker `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "view! {",
        "logic::resolve_root_state(logic::CommandRootStateInput",
        "command_input_attrs(lang, dir)",
        "command_option_a11y_attrs(CommandOptionA11yInput",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep structure/headless mount marker `{needle}`."
        );
    }
    for forbidden in ["pub const CSS: &str", "attach_active_highlight_motion("] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not contain style engine detail marker `{forbidden}`."
        );
    }

    for needle in [
        "attach_active_highlight_motion(",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep motion contract mapping marker `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "command_input_attrs(",
        "on:click",
        "requestAnimationFrame",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement interaction/render/engine marker `{forbidden}`."
        );
    }
}

#[test]
fn command_discrete_state_contracts_are_type_backed_enums() {
    let source = include_str!("../src/mod.rs");

    for needle in [
        "pub enum CommandRootStateAttr",
        "pub enum CommandCollectionAttr",
        "pub enum CommandQueryAttr",
        "pub enum CommandDisabledAttr",
        "pub enum CommandSourceAttr",
        "pub state_attr: CommandRootStateAttr",
        "pub query_attr: CommandQueryAttr",
        "pub disabled_attr: CommandDisabledAttr",
    ] {
        assert!(
            source.contains(needle),
            "command discrete state contract should be enum-based marker `{needle}`."
        );
    }
}

#[test]
fn command_public_api_does_not_expose_platform_dom_types() {
    let source = include_str!("../src/mod.rs");

    for forbidden in ["web_sys", "wasm_bindgen"] {
        assert!(
            !source.contains(forbidden),
            "command public API should not expose platform DOM detail `{forbidden}`."
        );
    }
}

#[test]
fn command_component_boolean_prop_uses_is_prefix_contract() {
    let source = include_str!("../src/view.rs");

    assert!(
        source.contains("#[prop(optional)] is_disabled: bool"),
        "command component should expose the boolean prop as `is_disabled`."
    );
    assert!(
        !source.contains("#[prop(optional)] disabled: bool"),
        "command component should not expose legacy boolean prop name `disabled`."
    );
}

#[test]
fn command_query_axis_supports_controlled_and_uncontrolled_contract() {
    let source = include_str!("../src/view.rs");

    for needle in [
        "#[prop(optional)] query: Option<Signal<String>>",
        "#[prop(optional, into)] default_query: Option<String>",
        "#[prop(optional)] on_query_change: Option<Callback<String>>",
        "let default_query = logic::resolve_default_query(default_query);",
        "let query_state = use_controllable_state(query, Some(default_query), on_query_change);",
        "let query = query_state.value;",
        "let request_query_change = query_state.request_change;",
    ] {
        assert!(
            source.contains(needle),
            "command query state axis should keep controlled/uncontrolled contract marker `{needle}`."
        );
    }

    assert!(
        !source.contains("default_query.unwrap_or_default()"),
        "query default fallback should be normalized in logic.rs, not inside view.rs."
    );
}

#[test]
fn command_state_primitives_source_contract_is_stable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");

    for needle in [
        "use ui_state_primitives::command as command_primitives;",
        "command_primitives::normalize_selected_index(selected_index, item_count)",
        "command_primitives::filter_groups(groups, query)",
    ] {
        assert!(
            logic_source.contains(needle),
            "command logic should source reusable state primitive from ui-state-primitives marker `{needle}`."
        );
    }

    assert!(
        !view_source.contains("store"),
        "command component should not bind business store directly in view layer."
    );
}

#[test]
fn command_async_contract_is_not_applicable_and_unintroduced() {
    let view_source = include_str!("../src/view.rs");

    for forbidden in ["is_loading", "aria-busy", "retry", "use_async_action"] {
        assert!(
            !view_source.contains(forbidden),
            "command currently has no async interaction contract; found unexpected marker `{forbidden}`."
        );
    }
}

#[test]
fn command_dx_default_path_is_simple_and_state_not_required() {
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let docs_page_source = docs_command_source();

    assert!(
        view_source.contains("#[prop(into)] groups: Arc<[CommandGroup]>"),
        "command default path should require data input only via `groups`."
    );
    assert!(
        !view_source.contains("#[prop(optional)] state:")
            && !view_source.contains("#[prop(into)] state:")
            && !view_source.contains(" state: Option<Signal<"),
        "command should not expose internal state object as component API prop."
    );
    assert!(
        readme_source
            .contains("view! { <Command id_base=\"main-cmd\".to_string() groups=groups /> }"),
        "command README should keep one-glance default call path."
    );
    assert!(
        docs_page_source.contains("title=\"Hello World (Default API)\""),
        "docs-app should provide a minimal hello-world playground for command."
    );
    assert!(
        docs_page_source.contains("id_base=\"docs-command-hello\".to_string()"),
        "docs hello-world playground should use direct default command invocation."
    );
}

#[test]
fn command_composite_api_uses_typed_item_tree_not_parallel_arrays() {
    let mod_source = include_str!("../src/mod.rs");
    let view_source = include_str!("../src/view.rs");
    let readme_source = include_str!("../src/README.md");
    let docs_page_source = docs_command_source();

    assert!(
        mod_source.contains("CommandGroup, CommandItem"),
        "command should expose typed item tree primitives (`CommandGroup` + `CommandItem`)."
    );
    assert!(
        view_source.contains("#[prop(into)] groups: Arc<[CommandGroup]>"),
        "command main API should consume grouped typed items instead of parallel arrays."
    );
    for forbidden in [
        "#[prop(into)] labels:",
        "#[prop(into)] titles:",
        "#[prop(into)] panels:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command API should not expose parallel-array prop marker `{forbidden}`."
        );
    }
    assert!(
        readme_source.contains("CommandGroup { heading, items }"),
        "command README should document item title+content in one typed item-tree dimension."
    );
    assert!(
        docs_page_source.contains("groups=Arc::from(vec![CommandGroup::new("),
        "docs should show typed group/item tree as default composition path."
    );
}

#[test]
fn command_macro_micro_dragging_contract_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:drag",
        "on:dragstart",
        "on:dragend",
        "requestAnimationFrame",
        "raf_loop",
        "set_pointer_capture",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement drag macro/micro state machine marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement drag macro/micro state machine marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion should not implement drag macro/micro state machine marker `{forbidden}`."
        );
    }
}

#[test]
fn command_two_pass_geometry_contract_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for forbidden in [
        "TwoPassIntent",
        "Rectification",
        "Measure(",
        "getBoundingClientRect",
        "get_bounding_client_rect",
        "ResizeObserver",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement two-pass geometry marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement two-pass geometry marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion should not implement two-pass geometry marker `{forbidden}`."
        );
    }
}

#[test]
fn command_registration_protocol_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let mod_source = include_str!("../src/mod.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not depend on registration protocol marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not depend on registration protocol marker `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "command public surface should not expose registration protocol marker `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("render_options_content(state: &CommandFilterState")
            && view_source.contains("render_group_section(group, &state.items, &ctx.option)"),
        "command should render from typed group collection order."
    );
    assert!(
        view_source.contains(".item_indices"),
        "command should use ordered group item indices from primitive output."
    );
}

#[test]
fn command_slot_projection_contract_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "keep_alive",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement slot projection marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement slot projection marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion should not implement slot projection marker `{forbidden}`."
        );
    }

    for runtime_forbidden in ["set_interval", "set_timeout"] {
        assert!(
            !view_source.contains(runtime_forbidden),
            "command should not run hidden keep-alive side effects marker `{runtime_forbidden}`."
        );
    }
}

#[test]
fn command_env_stream_contract_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "on:resize",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement env stream marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement env stream marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion should not implement env stream marker `{forbidden}`."
        );
    }
}

#[test]
fn command_event_light_cone_contract_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let mod_source = include_str!("../src/mod.rs");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "bulk_select",
        "select_all",
        "deselect_all",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement event light cone marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement event light cone marker `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "command public API should not expose event light cone marker `{forbidden}`."
        );
    }
}

#[test]
fn command_causality_bus_contract_is_not_applicable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let mod_source = include_str!("../src/mod.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality bus",
        "broadcast",
        "subscriber",
        "publish",
        "subscribe",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement causality bus marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement causality bus marker `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "command public API should not expose causality bus marker `{forbidden}`."
        );
    }
}

#[test]
fn command_focus_stack_overlay_contract_is_not_applicable() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let mod_source = include_str!("../src/mod.rs");

    for forbidden in [
        "FocusManager",
        "FallbackTo",
        "restore_focus",
        "focus_restore",
        "last_focused",
        "document.body",
        "active_element",
        "OverlayStack",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement overlay focus-stack marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement overlay focus-stack marker `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "command public API should not expose overlay focus-stack marker `{forbidden}`."
        );
    }

    for needle in [
        "let options_ref: NodeRef<html::Div> = NodeRef::new();",
        "let highlight_ref: NodeRef<html::Div> = NodeRef::new();",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "command NodeRef usage should stay limited to motion attachment marker `{needle}`."
        );
    }
}

#[test]
fn command_escape_hatch_foreign_zone_contract_is_not_applicable() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let mod_source = include_str!("../src/mod.rs");

    for forbidden in [
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "third_party_instance",
        "external_instance",
        "imperative_instance",
        "Box<dyn",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not implement foreign-zone escape hatch marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not implement foreign-zone escape hatch marker `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "command public API should not expose foreign-zone escape hatch marker `{forbidden}`."
        );
    }

    for forbidden in ["web_sys", "js_sys", "wasm_bindgen"] {
        assert!(
            !mod_source.contains(forbidden),
            "command public API must not leak imperative third-party runtime detail `{forbidden}`."
        );
    }
}

#[test]
fn command_hydration_discontinuity_contract_uses_deterministic_ids() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let mod_source = include_str!("../src/mod.rs");
    let ui_root_source = include_str!("../../../crates/ui/src/root.rs");

    for needle in [
        "pub fn Command(\n    id_base: String,",
        "let id_base = logic::normalize_id_base(id_base);",
        "id_base: format!(\"{}-command\", id_base.get_value()),",
        "let listbox_id = StoredValue::new(format!(\"{}-listbox\", id_base.get_value()));",
        "pub const DEFAULT_ID_BASE: &str = \"command\";",
        "provide_ui_id_provider(id_seed);",
    ] {
        let found = view_source.contains(needle)
            || logic_source.contains(needle)
            || mod_source.contains(needle)
            || ui_root_source.contains(needle);
        assert!(
            found,
            "deterministic SSR/hydration id path should keep marker `{needle}`."
        );
    }

    for forbidden in [
        "now(",
        "SystemTime",
        "UNIX_EPOCH",
        "rand::",
        "random(",
        "Uuid",
        "uuid::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not use nondeterministic hydration marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic should not use nondeterministic hydration marker `{forbidden}`."
        );
        assert!(
            !mod_source.contains(forbidden),
            "command public API should not expose nondeterministic hydration marker `{forbidden}`."
        );
    }
}

#[test]
fn command_ssr_and_cross_platform_contract_avoids_browser_only_dependencies() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let styles_source = include_str!("../src/styles.rs");

    for forbidden in [
        "web_sys",
        "js_sys",
        "wasm_bindgen",
        "window.",
        "document.",
        "document.body",
        "HtmlElement",
        "BrowserOnly",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "command mod.rs should avoid browser-only marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic.rs should avoid browser-only marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view.rs should avoid browser-only marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion.rs should avoid browser-only marker `{forbidden}`."
        );
        assert!(
            !protocol_source.contains(forbidden),
            "command protocol.rs should avoid browser-only marker `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "command styles.rs should avoid browser-only marker `{forbidden}`."
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(feature = \"web\")",
        "cfg(feature = \"ssr\")",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "command mod.rs should not split semantic path by platform marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic.rs should not split semantic path by platform marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view.rs should not split semantic path by platform marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion.rs should not split semantic path by platform marker `{forbidden}`."
        );
    }
}

#[test]
fn command_ui_headless_web_ssr_mutual_exclusion_contract_is_preserved() {
    let headless_lib_source = include_str!("../../../crates/ui-headless/src/lib.rs");
    let headless_cargo_source = include_str!("../../../crates/ui-headless/Cargo.toml");
    let command_cargo_source = include_str!("../Cargo.toml");
    let view_source = include_str!("../src/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        let found = headless_lib_source.contains(needle) || headless_cargo_source.contains(needle);
        assert!(
            found,
            "ui-headless feature mutual exclusion contract should keep marker `{needle}`."
        );
    }

    assert!(
        command_cargo_source.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "ui-command should consume ui-headless through default feature contract without overriding mutually-exclusive web/ssr pair."
    );

    for forbidden in [
        "ui-headless = { path = \"../../crates/ui-headless\", features = [\"web\", \"ssr\"]",
        "feature = \"ssr\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command runtime path should not override ui-headless mutual-exclusion marker `{forbidden}`."
        );
    }
}

#[test]
fn command_ui_motion_non_wasm_noop_contract_is_preserved() {
    let ui_motion_lib_source = include_str!("../../../crates/ui-motion/src/lib.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let motion_source = include_str!("../src/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(needle),
            "ui-motion non-wasm backend should keep noop marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
        "_container_ref: NodeRef<html::Div>",
        "_highlight_ref: NodeRef<html::Div>",
        "_active_index: ReadSignal<usize>",
        "_option_id: Callback<usize, String>",
        "_motion: ActiveHighlightMotion",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active-highlight primitive should keep non-wasm noop attach marker `{needle}`."
        );
    }

    for needle in [
        "use ui_visual_primitive::active_highlight::attach_active_highlight_motion;",
        "attach_active_highlight_motion(",
        "sanitize_motion(motion)",
    ] {
        assert!(
            motion_source.contains(needle),
            "command motion bridge should keep safe mapping marker `{needle}`."
        );
    }

    for forbidden in [
        "requestAnimationFrame",
        "ui_motion::web::animate(",
        "web_sys",
        "js_sys",
        "window.",
        "document.",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "command motion bridge should avoid browser-engine assumption marker `{forbidden}`."
        );
    }
}

#[test]
fn command_reduced_motion_ssr_wasm_contract_is_preserved() {
    let spring_source = include_str!("../../../crates/ui-motion/src/spring.rs");
    let web_motion_source = include_str!("../../../crates/ui-motion/src/web.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let logic_source = include_str!("../src/logic.rs");

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
        "if let Some(on_rest) = self.inner.on_rest.borrow_mut().as_mut() {",
    ] {
        assert!(
            spring_source.contains(needle),
            "spring animator should keep reduced-motion downgrade marker `{needle}`."
        );
    }

    for needle in [
        "match_media(\"(prefers-reduced-motion: reduce)\")",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            web_motion_source.contains(needle),
            "wasm web motion backend should keep reduced-motion short-circuit marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ResizeObserver",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active-highlight should keep wasm enhancement + non-wasm downgrade marker `{needle}`."
        );
    }

    for forbidden in [
        "cfg(target_arch = \"wasm32\")",
        "cfg(feature = \"web\")",
        "cfg(feature = \"ssr\")",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view semantic contract should not split by platform marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion semantic contract should not split by platform marker `{forbidden}`."
        );
    }

    for needle in [
        "let id_base = logic::normalize_id_base(id_base);",
        "let listbox_id = StoredValue::new(format!(\"{}-listbox\", id_base.get_value()));",
        "pub const DEFAULT_ID_BASE: &str = \"command\";",
    ] {
        let found = view_source.contains(needle) || logic_source.contains(needle);
        assert!(
            found,
            "command SSR/hydration path should keep deterministic-id marker `{needle}`."
        );
    }
}

#[test]
fn command_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = include_str!("../src/motion.rs");
    let view_source = include_str!("../src/view.rs");
    let ui_motion_spring_source = include_str!("../../../crates/ui-motion/src/spring.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-platforms.sh");

    for needle in [
        "pub type CommandMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "pub fn sanitize_motion(motion: CommandMotion) -> CommandMotion {",
        "spring.stiffness",
        "spring.damping",
        "spring.mass",
        "spring.precision",
        "pub fn attach_motion(",
        "attach_active_highlight_motion(",
        "sanitize_motion(motion)",
    ] {
        assert!(
            motion_source.contains(needle),
            "command motion contract should keep component-scoped mapping marker `{needle}`."
        );
    }

    assert!(
        view_source.contains("crate::motion::attach_motion("),
        "command view should mount motion through motion.rs attach contract."
    );

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "SpringState::new(target)",
    ] {
        assert!(
            ui_motion_spring_source.contains(needle),
            "ui-motion spring should keep reduced-motion guard marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_active_highlight_motion(",
        "_motion: ActiveHighlightMotion",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active-highlight primitive should keep wasm/non-wasm safe attach marker `{needle}`."
        );
    }

    for forbidden in [
        "requestAnimationFrame",
        "web_sys",
        "js_sys",
        "window.",
        "document.",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "command motion bridge should not hard-bind browser runtime marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(script_needle),
        "platform check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "command_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep motion-contract evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_performance_governance_budget_is_mount_only_traceable_and_blocking() {
    let check2_source = include_str!("../check2.md");
    let shell_source = include_str!("../../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_page_source = docs_command_source();
    let perf_probe_source = include_str!("../../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = include_str!("../../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");
    let view_source = include_str!("../src/view.rs");

    for needle in [
        "component_doc!(",
        "\"Command\"",
        "\"command\"",
        "collections_command::command",
    ] {
        assert!(
            pages_source.contains(needle),
            "Command docs page should stay in component coverage traversal via `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn command() -> AnyView",
        "title=\"Command\"",
        "slug=\"command\"",
        "<ComponentPage",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "Command docs page should mount through ComponentPage contract `{needle}`."
        );
    }

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep perf budget/probe wiring via `{needle}`."
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
        "\"mount-only\"",
        "\"mount-plus-budget\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose repeatable perf marker `{needle}`."
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
            "docs coverage e2e should enforce perf guard `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_performance_governance_budget_is_mount_only_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );

    for needle in [
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-motion-source=move || root_state.get().motion_source_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "Command view should expose perf triage marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "等价证据",
        "command_performance_governance_budget_is_mount_only_traceable_and_blocking",
    ] {
        assert!(
            check2_source.contains(needle),
            "Command check2 should include performance governance evidence token `{needle}`."
        );
    }
}

#[test]
fn command_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
{
    let view_source = include_str!("../src/view.rs");
    let check2_source = include_str!("../check2.md");
    let todo_source = include_str!("../../../docs/plan/TODO.md");
    let script_source = include_str!("../../../scripts/check-ui-performance.sh");

    for marker in [
        "role=input_a11y.role",
        "aria-autocomplete=input_a11y.aria_autocomplete",
        "aria-expanded=input_a11y.aria_expanded",
        "aria-controls=listbox_id.get_value()",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
        "role=listbox.attrs.role",
        "tabindex=listbox.attrs.tabindex",
        "aria-selected=move || option_attrs().aria_selected",
        "aria-disabled=move || option_attrs().aria_disabled",
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "data-focused=move || option_attrs().data_focused",
        "data-selected=move || option_attrs().data_selected",
        "resolve_command_input_key_down(&key, logic::has_query_text(query_value.as_str()))",
        "on:keydown=on_input_key_down",
        "on:pointermove=move |_| on_option_pointer_move.run(index)",
    ] {
        assert!(
            view_source.contains(marker),
            "command semantics/perf matrix should keep aria/data/focus marker `{marker}`."
        );
    }

    for marker in [
        "cargo test -p ui-command --lib command_performance_governance_budget_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-command --lib command_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should enforce command semantics/perf gate marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "TODO should keep render_count follow-up marker `{marker}`."
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "command_state_observability_contract_uses_stable_data_and_aria_markers",
        "command_semantics_tests_cover_behavior_matrix_not_visual_snapshots",
        "command_performance_governance_budget_is_mount_only_traceable_and_blocking",
        "command_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "scripts/check-ui-performance.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "command check2 semantics/perf section should reference `{marker}`."
        );
    }
}

#[test]
fn command_view_macro_complexity_is_partitioned_into_render_helpers() {
    let view_source = include_str!("../src/view.rs");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "fn render_empty_state(",
        "fn render_option_item(",
        "fn render_group_section(",
        "fn render_options_content(",
        "const COMMAND_VIEW_SLOTS: CommandViewSlots = CommandViewSlots {",
        "render_options_content(",
    ] {
        assert!(
            view_source.contains(needle),
            "command view should keep macro-partition helper marker `{needle}`."
        );
    }

    for forbidden in [
        "item_indices.clone()",
        "Some(view! {",
        "<section class=group_slot.base_class() data-slot=group_slot.as_attr()>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should avoid pre-refactor giant-inline marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "render_options_content/render_group_section/render_option_item/render_empty_state",
        "command_view_macro_complexity_is_partitioned_into_render_helpers",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep macro-complexity evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_functional_split_prefers_plain_rust_functions_over_local_components() {
    let view_source = include_str!("../src/view.rs");
    let check2_source = include_str!("../check2.md");

    for needle in [
        "fn render_empty_state(",
        "fn render_option_item(",
        "fn render_group_section(",
        "fn render_options_content(",
        "-> AnyView",
    ] {
        assert!(
            view_source.contains(needle),
            "command view should keep functional-split helper marker `{needle}`."
        );
    }

    let component_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_count, 1,
        "command view should only expose a single top-level `#[component]`."
    );
    assert!(
        view_source.contains("#[component]\npub fn Command("),
        "command root component declaration should remain explicit."
    );

    for forbidden in [
        "#[component]\nfn render_empty_state(",
        "#[component]\nfn render_option_item(",
        "#[component]\nfn render_group_section(",
        "#[component]\nfn render_options_content(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should not promote local helper into component marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "render_empty_state/render_option_item/render_group_section/render_options_content",
        "command_functional_split_prefers_plain_rust_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep functional split evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_static_fragments_are_constantized_or_absent() {
    let view_source = include_str!("../src/view.rs");
    let check2_source = include_str!("../check2.md");

    {
        let needle = "const COMMAND_VIEW_SLOTS: CommandViewSlots = CommandViewSlots {";
        assert!(
            view_source.contains(needle),
            "command view should keep static fragment constantization marker `{needle}`."
        );
    }

    for forbidden in [
        "let slots = CommandViewSlots {",
        "<svg",
        "<footer",
        "inner_html",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command view should avoid scattered static fragment marker `{forbidden}`."
        );
    }

    for needle in [
        "aria-label=aria_label.get_value()",
        "role=input_a11y.role",
        "role=listbox.attrs.role",
    ] {
        assert!(
            view_source.contains(needle),
            "command view should preserve accessibility semantics marker `{needle}`."
        );
    }

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "const COMMAND_VIEW_SLOTS",
        "command_static_fragments_are_constantized_or_absent",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep static-fragment evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_inner_html_contract_disallows_dynamic_html_injection() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let styles_source = include_str!("../src/styles.rs");
    let motion_source = include_str!("../src/motion.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let check2_source = include_str!("../check2.md");

    for forbidden in [
        "inner_html",
        "set_inner_html",
        ".set_inner_html(",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "command mod.rs should not expose html injection marker `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic.rs should not expose html injection marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view.rs should not expose html injection marker `{forbidden}`."
        );
        assert!(
            !styles_source.contains(forbidden),
            "command styles.rs should not expose html injection marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "command motion.rs should not expose html injection marker `{forbidden}`."
        );
        assert!(
            !protocol_source.contains(forbidden),
            "command protocol.rs should not expose html injection marker `{forbidden}`."
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "N/A：`command` 组件当前未使用 `inner_html`/`set_inner_html` 路径。",
        "command_inner_html_contract_disallows_dynamic_html_injection",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep inner_html safety evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated() {
    let check2_source = include_str!("../check2.md");
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let command_cargo_source = include_str!("../Cargo.toml");
    let ui_components_cargo_source = include_str!("../../../crates/ui/Cargo.toml");
    let docs_lib_source = include_str!("../../../apps/docs-app/src/lib.rs");
    let docs_command_page_source = docs_command_source();
    let debug_overlay_source = include_str!("../../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = include_str!("../../../crates/ui-headless/src/trace.rs");

    for needle in [
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "on:keydown=on_input_key_down",
        "on:pointermove=move |_| on_option_pointer_move.run(index)",
        "on:click=move |_| {",
        "on_option_click.run(index);",
    ] {
        assert!(
            view_source.contains(needle),
            "command wasm debug contract should keep local traceability/replay marker `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs app should keep shared wasm debug bootstrap marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "fn render_event(event: ui_headless::UiTraceEvent) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "ui_headless::UiTraceEventKind::Inspect { tag, data_slot }",
        "ui_headless::UiTraceEventKind::Note { message }",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep shared timeline/inspection marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub kind: UiTraceEventKind,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace primitive should keep timestamped event marker `{needle}`."
        );
    }

    for needle in [
        "title=\"Grouped Search + Keyboard Action\"",
        "title=\"Interactive Playground\"",
        "on_action=on_action",
        "\"last action: \"",
    ] {
        assert!(
            docs_command_page_source.contains(needle),
            "command docs page should keep minimal replay path marker `{needle}`."
        );
    }

    for forbidden in [
        "UiTrace",
        "provide_ui_trace",
        "UiDebugOverlay",
        "wasm_debug",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "command mod.rs should keep wasm debug API surface clean from `{forbidden}`."
        );
        assert!(
            !logic_source.contains(forbidden),
            "command logic.rs should keep wasm debug API surface clean from `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "command view.rs should keep wasm debug API surface clean from `{forbidden}`."
        );
    }

    for forbidden in ["command-wasm-debug", "command_wasm_debug"] {
        assert!(
            !command_cargo_source.contains(forbidden),
            "ui-command Cargo features should remain free of command-specific wasm debug flag `{forbidden}`."
        );
        assert!(
            !ui_components_cargo_source.contains(forbidden),
            "ui Cargo features should remain free of command-specific wasm debug flag `{forbidden}`."
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "provide_ui_trace(debug_overlay_enabled)",
        "UiTraceEvent { ts_ms, component, kind }",
        "command_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep wasm debug evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let check2_source = include_str!("../check2.md");
    let docs_page_source = docs_command_source();
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let command_cargo_source = include_str!("../Cargo.toml");
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "<Playground",
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test workbench for command state/source contract tuning.\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"command-workbench-controls\"",
        "id_base=\"docs-command-workbench-scenario\".to_string()",
        "selected_index=workbench_index",
        "set_selected_index=set_workbench_index",
        "let (last_workbench_action, set_last_workbench_action) = signal(\"none\".to_string());",
        "on_action=on_workbench_action",
        "\"last action: \"",
        "move || last_workbench_action.get()",
    ] {
        assert!(
            docs_page_source.contains(needle),
            "command docs workbench should keep DX marker `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, into)] test_css_source: Option<Signal<String>>",
        "#[prop(optional, into)] test_config_signal: Option<Signal<String>>",
        "let scope_selector = format!(\"[data-playground-scope=\\\"{scope_id}\\\"]\");",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "<div data-playground-scope=scope_id.clone()>",
        "let on_reset_test_css: OnPress =",
        "set_test_css.set(default_test_css.get_untracked())",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground infra should keep hot-reload + isolated-canvas DX marker `{needle}`."
        );
    }

    for forbidden in [
        "localStorage",
        "sessionStorage",
        "indexedDB",
        "persist_state",
    ] {
        assert!(
            !docs_page_source.contains(forbidden),
            "command workbench should keep optional state persistence explicitly N/A without hidden storage marker `{forbidden}`."
        );
    }

    assert!(
        command_cargo_source.contains("[features]\ndefault = []"),
        "ui-command crate should keep production surface clean with no default debug/dx persistence feature."
    );
    for forbidden in ["dx-persist", "command-dx", "command_dx"] {
        assert!(
            !command_cargo_source.contains(forbidden),
            "ui-command crate should not expose command-specific DX persistence feature marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        script_source.contains(script_needle),
        "DX check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "test_css_source=workbench_test_css_source",
        "data-playground-scope",
        "optional persist N/A",
        "command_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep DX evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source = docs_command_source();
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    for needle in [
        "const COMMAND_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse std::sync::Arc;\\nuse ui::{Command, CommandGroup, CommandItem};",
        "code_imports=COMMAND_DOC_IMPORTS.to_string()",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "Streaming is optional; fallback stays snapshot.",
        "requested mode:",
        "requested output status:",
        "effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified",
    ] {
        assert!(
            docs_source.contains(needle),
            "command docs should keep copy-ready + streaming/snapshot contract marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should keep import completion marker `{needle}`."
        );
    }
}

#[test]
fn command_dx_check_script_covers_docs_copy_paste_ready_and_workbench_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-command --lib command_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-command --lib command_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

#[test]
fn command_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let source = include_str!("../check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World (Default API)",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
        "COMMAND_DOC_IMPORTS",
        "compose_copy_ready_code",
        "command_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "command_dx_check_script_covers_docs_copy_paste_ready_and_workbench_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            source.contains(needle),
            "command check2 docs-product section should reference `{needle}`."
        );
    }
}

#[test]
fn command_check2_documents_docs_sync_and_state_matrix_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep docs-sync/state-matrix rule `{required}`.",
        );
    }

    for marker in [
        "command_check2_documents_docs_sync_and_state_matrix_rules",
        "command_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "command_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            checklist_source.contains(marker),
            "command docs-sync checklist should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn command_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source = docs_command_source();
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");

    for needle in [
        "pub(super) fn command() -> AnyView",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "id_base=\"docs-command-state-matrix\".to_string()",
        "default_query=state_matrix_default_query.get()",
        "is_disabled=state_matrix_disabled.get()",
        "id_base=\"docs-command-controlled\".to_string()",
        "query=controlled_query",
        "on_query_change=on_controlled_query_change",
        "id_base=\"docs-command-uncontrolled\".to_string()",
        "default_query=\"cal\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "command docs examples should keep state-matrix/API sync marker `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] query: Option<Signal<String>>",
        "#[prop(optional, into)] default_query: Option<String>",
        "#[prop(optional)] on_query_change: Option<Callback<String>>",
        "#[prop(optional)] on_action: Option<Callback<String>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] motion: CommandMotion",
        "#[prop(optional, into)] placeholder: Option<String>",
        "#[prop(optional, into)] empty_label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "command view public API should keep `{needle}` for docs/runtime sync.",
        );
    }

    for needle in [
        "pub const DEFAULT_DISABLED: bool = false;",
        "pub const DEFAULT_QUERY: &str = \"\";",
        "pub const DEFAULT_PLACEHOLDER: &str = \"Type a command or search...\";",
        "pub const DEFAULT_EMPTY_LABEL: &str = \"No results found.\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Command menu\";",
        "pub fn resolve_default_query(value: Option<String>) -> String",
    ] {
        assert!(
            logic_source.contains(needle),
            "command logic defaults should keep `{needle}` for docs consistency.",
        );
    }

    for forbidden in ["is_query=", "default_is_query", "on_query="] {
        assert!(
            !docs_source.contains(forbidden),
            "command docs should avoid stale/aliased API token `{forbidden}`.",
        );
    }
}

#[test]
fn command_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-command --lib command_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce docs-sync/state-matrix contract `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_interactive_playground_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep interactive-playground rule `{required}`.",
        );
    }

    for marker in [
        "command_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "command_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "command_dx_check_script_covers_interactive_playground_contract",
        "command_e2e_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "components/command/scripts/check-ui-e2e-command.sh",
    ] {
        assert!(
            checklist_source.contains(marker),
            "command interactive-playground checklist should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn command_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source = docs_command_source();

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test workbench for command state/source contract tuning.\"",
        "test_css_source=workbench_test_css_source",
        "test_config_signal=workbench_actual_config",
        "let workbench_options = vec![",
        "let (workbench_index, set_workbench_index) = signal(Some(0_usize));",
        "let workbench_disabled = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);",
        "let workbench_custom_text = Signal::derive(move || workbench_index.get().unwrap_or(0) >= 1);",
        "let workbench_custom_motion = Signal::derive(move || workbench_index.get().unwrap_or(0) == 2);",
        "data-slot=\"command-workbench-controls\"",
        "selected_index=workbench_index",
        "set_selected_index=set_workbench_index",
        "data-slot=\"command-workbench\"",
        "id_base=\"docs-command-workbench\".to_string()",
        "on_action=on_workbench_action",
        "is_disabled=workbench_disabled.get()",
        "motion=workbench_motion.get()",
        "placeholder=if workbench_custom_text.get() {",
        "empty_label=if workbench_custom_text.get() {",
        "aria_label=if workbench_custom_text.get() {",
        "class_name=if workbench_custom_text.get() {",
        "data-slot=\"command-workbench-last-action\"",
        "data-last-action=move || last_workbench_action.get()",
    ] {
        assert!(
            docs_source.contains(needle),
            "command docs interactive playground should keep marker `{needle}`.",
        );
    }
}

#[test]
fn command_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_command.spec.mjs");

    for needle in [
        "docs-app command interactive playground key flow is repeatable with semantic breakpoints",
        "const workbenchControls = page.locator('[data-slot=\"command-workbench-controls\"]').first();",
        "workbenchScope = page.locator('[data-slot=\"command-workbench\"]').first();",
        "[data-slot=\"command-workbench-last-action\"]",
        "toHaveAttribute(\"data-last-action\", \"calendar\")",
        "toHaveAttribute(\"data-state\", \"query-results\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(needle),
            "command interactive playground e2e flow should keep marker `{needle}`.",
        );
    }
}

#[test]
fn command_dx_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_interactive_playground_rules",
        "cargo test -p ui-command --lib command_docs_app_provides_interactive_playground_for_props_state_and_preview",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce interactive-playground contract `{needle}`.",
        );
    }
}

#[test]
fn command_e2e_check_script_covers_interactive_playground_contract() {
    let script_source = include_str!("../../../components/command/scripts/check-ui-e2e-command.sh");

    for needle in [
        "cargo test -p ui-command --lib command_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "cargo test -p ui-command --lib command_e2e_check_script_covers_interactive_playground_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "e2e check script should enforce interactive-playground contract `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_source_first_copy_paste_ready_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep source-first copy-paste-ready rule `{required}`.",
        );
    }

    for marker in [
        "command_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "command_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            checklist_source.contains(marker),
            "command source-first checklist should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn command_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source = docs_command_source();
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let code_block_view_source = include_str!("../../../components/code-block/src/view.rs");
    let readme_source = include_str!("../src/README.md");

    for needle in [
        "data-slot=\"command-source-first\"",
        "<h3>\"Source-first Copy-Paste\"</h3>",
        "<code>\"Show code\"</code>",
        "<code>\"Copy\"</code>",
        "COMMAND_DOC_IMPORTS",
        "compose_copy_ready_code",
        "code_imports=COMMAND_DOC_IMPORTS.to_string()",
        "Dependency prerequisites",
        "component-command",
        "inject-css",
        "data-slot=\"command-source-paths\"",
        "components/command/src/mod.rs",
        "components/command/src/logic.rs",
        "components/command/src/view.rs",
        "components/command/src/styles.rs",
        "components/command/src/motion.rs",
        "test_source_path=\"/root/autodl-tmp/zjj/p/rust-ui/components/command/src/styles.rs\".to_string()",
    ] {
        assert!(
            docs_source.contains(needle),
            "command source-first docs should contain `{needle}`.",
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "code_imports: Option<String>",
        "<CodeBlock code=resolved_code.get() />",
        "missing_import_lines(&raw, &imports)",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy-ready pipeline should contain `{needle}`.",
        );
    }

    for needle in [
        "class_name=\"ui-code-block__copy-button\"",
        "copy_to_clipboard_aria_label",
    ] {
        assert!(
            code_block_view_source.contains(needle),
            "CodeBlock should keep one-click copy affordance token `{needle}`.",
        );
    }

    for needle in [
        "## Source-first",
        "组件源码：`components/command/src/{mod,logic,view,styles,motion}.rs`",
        "package feature：`component-command`（可选叠加 `inject-css`）",
    ] {
        assert!(
            readme_source.contains(needle),
            "command README should document source-first dependency/path marker `{needle}`.",
        );
    }
}

#[test]
fn command_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-command --lib command_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce source-first copy-paste-ready contract `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_documentation_as_product_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep documentation-as-product rule `{required}`.",
        );
    }
}

#[test]
fn command_documentation_entry_exists_with_beginner_first_progression() {
    let readme = include_str!("../src/README.md");
    let docs_source = docs_command_source();

    for needle in [
        "# Command",
        "## Hello World",
        "## 常见用法",
        "## 新手路径（先用起来，再进阶）",
        "view! { <Command id_base=\"main-cmd\".to_string() groups=groups /> }",
        "default_query=Some(\"cal\".to_string())",
        "query + on_query_change",
        "placeholder/empty_label/aria_label/class_name/motion",
    ] {
        assert!(
            readme.contains(needle),
            "command README should include beginner-friendly marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn command() -> AnyView",
        "title=\"Command\"",
        "slug=\"command\"",
        "title=\"Hello World (Default API)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Interactive Playground\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "command docs entry should include `{needle}`.",
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("Command README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("Command README should include common usage section");
    let readme_progressive = readme
        .find("## 新手路径（先用起来，再进阶）")
        .expect("Command README should include beginner-to-advanced section");
    let readme_api = readme
        .find("## API 约定")
        .expect("Command README should include API section");

    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_api,
        "Command README should keep default path before advanced guidance.",
    );

    let docs_hello = docs_source
        .find("title=\"Hello World (Default API)\"")
        .expect("Command docs should include Hello World playground");
    let docs_matrix = docs_source
        .find("title=\"State Matrix\"")
        .expect("Command docs should include state matrix playground");
    let docs_controlled = docs_source
        .find("title=\"Controlled vs Uncontrolled\"")
        .expect("Command docs should include controlled/uncontrolled playground");
    let docs_workbench = docs_source
        .find("title=\"Interactive Playground\"")
        .expect("Command docs should include workbench playground");

    assert!(
        docs_hello < docs_matrix
            && docs_matrix < docs_controlled
            && docs_controlled < docs_workbench,
        "Command docs should keep beginner-first order before advanced controls.",
    );
}

#[test]
fn command_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-command --lib command_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce documentation-as-product contract `{needle}`.",
        );
    }
}

#[test]
fn command_check2_marks_documentation_as_product_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "command_check2_documents_documentation_as_product_rules",
        "command_documentation_entry_exists_with_beginner_first_progression",
        "command_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep documentation-as-product evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_heroui_benchmark_docs_sync_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep heroui-benchmark docs-sync rule `{required}`.",
        );
    }
}

#[test]
fn command_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = docs_command_source();
    let readme_source = include_str!("../src/README.md");

    for needle in [
        "### Command 同步记录（2026-02-21）",
        "参数模型同步：`Command` 参数主轴保持 `id_base/groups` 必填 + `query/on_query_change/default_query` 受控/非受控成对轴",
        "component_doc!(\"Command\", \"command\", \"Collections\", collections_command::command)",
        "#/components/command",
        "`components/command/src/README.md` 提供等价组件文档入口",
        "collections_command.rs::command()",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "heroui strategy doc should include command synchronization marker `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"Command\"",
        "\"command\"",
        "collections_command::command",
    ] {
        assert!(
            pages_source.contains(needle),
            "component docs index should expose command entry marker `{needle}`.",
        );
    }

    for needle in [
        "pub(super) fn command() -> AnyView",
        "title=\"Command\"",
        "slug=\"command\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "docs-app command page should stay indexable via marker `{needle}`.",
        );
    }

    assert!(
        readme_source.contains("# Command"),
        "command README should remain an equivalent component doc entry."
    );
}

#[test]
fn command_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-command --lib command_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce heroui-benchmark docs-sync contract `{needle}`.",
        );
    }
}

#[test]
fn command_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for needle in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "command_check2_documents_heroui_benchmark_docs_sync_rules",
        "command_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "command_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep heroui-benchmark docs-sync evidence marker `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_semantics_first_testing_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep semantics-first testing rule `{required}`.",
        );
    }

    for marker in [
        "command_semantics_suite_is_contract_first_not_snapshot_only",
        "command_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(marker),
            "command checklist semantics-first section should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn command_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = include_str!("./semantics.rs");
    let logic_test_source = include_str!("./logic.rs");
    let mod_source = include_str!("../src/mod.rs");

    for required in [
        "command_view_mounts_headless_semantics_contracts",
        "command_state_observability_contract_uses_stable_data_and_aria_markers",
        "command_semantics_tests_cover_behavior_matrix_not_visual_snapshots",
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "role=input_a11y.role",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
        "aria-selected=move || option_attrs().aria_selected",
        "aria-disabled=move || option_attrs().aria_disabled",
    ] {
        assert!(
            semantics_source.contains(required),
            "command semantic suite should keep contract-first assertion marker `{required}`.",
        );
    }

    assert!(
        mod_source
            .contains("#[cfg(test)]\n#[path = \"../test/semantics.rs\"]\nmod semantics_tests;"),
        "command component should keep local *_semantics.rs test entry wired in src/mod.rs."
    );

    let forbidden = [
        ["assert", "_snapshot!"].concat(),
        ["insta::assert", "_snapshot!"].concat(),
        ["to_match", "_snapshot"].concat(),
        ["image", "_snapshot"].concat(),
        ["toHave", "Screenshot"].concat(),
        ["toMatch", "Snapshot"].concat(),
    ];

    for forbidden in forbidden {
        assert!(
            !semantics_source.contains(forbidden.as_str())
                && !logic_test_source.contains(forbidden.as_str()),
            "command semantics should not rely on snapshot-only assertion `{forbidden}` as primary signal.",
        );
    }
}

#[test]
fn command_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = include_str!("../src/view.rs");
    let semantics_source = include_str!("./semantics.rs");

    for marker in [
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-id-source=move || root_state.get().id_source_attr.as_attr()",
        "data-placeholder-source=move || root_state.get().placeholder_source_attr.as_attr()",
        "data-empty-label-source=move || root_state.get().empty_label_source_attr.as_attr()",
        "data-aria-label-source=move || root_state.get().aria_label_source_attr.as_attr()",
        "data-class-source=move || root_state.get().class_source_attr.as_attr()",
        "data-action-source=move || root_state.get().action_source_attr.as_attr()",
        "data-motion-source=move || root_state.get().motion_source_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "role=input_a11y.role",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
        "aria-selected=move || option_attrs().aria_selected",
        "aria-disabled=move || option_attrs().aria_disabled",
        "data-focused=move || option_attrs().data_focused",
        "data-selected=move || option_attrs().data_selected",
    ] {
        assert!(
            view_source.contains(marker),
            "command view should expose semantic marker `{marker}`.",
        );
        assert!(
            semantics_source.contains(marker),
            "command semantic marker `{marker}` changed without matching semantics assertion update.",
        );
    }
}

#[test]
fn command_contract_hygiene_script_covers_semantics_first_testing_guards() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-command --lib command_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-command --lib command_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(required),
            "command checklist should keep e2e-selector/stable-wait rule `{required}`.",
        );
    }

    for marker in [
        "command_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "command_e2e_check_script_covers_selector_contract",
        "components/command/scripts/check-ui-e2e-command.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "command checklist e2e section should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn command_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_command.spec.mjs");
    let docs_source = docs_command_source();

    for required in [
        "[data-slot=\"command-e2e-default\"]",
        "[data-slot=\"command\"][data-ui-schema=\"ui.command.agent-contract\"]",
        "data-query-control",
        "data-query-default-source",
        "data-action-source",
        "[data-slot=\"command-input\"]",
        "[data-slot=\"command-list\"]",
        "[data-slot=\"command-item\"][data-focused=\"true\"]",
        "[data-slot=\"command-last-action\"][data-scenario=\"default\"]",
        "[data-slot=\"command-e2e-markers\"]",
        "[data-slot=\"command-last-action\"][data-scenario=\"markers\"]",
        "toHaveAttribute(\"data-query\", \"present\")",
        "toHaveAttribute(\"data-state\", \"query-results\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-last-action\", \"calendar\")",
        "toHaveAttribute(\"data-last-action\", \"open-recent\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "command e2e spec should keep semantic selector/contract marker `{required}`.",
        );
    }

    for forbidden in [
        "waitForTimeout",
        "setTimeout",
        "sleep(",
        "getByText(",
        "locator(\".docs-",
        ":nth-child(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "command e2e should avoid fragile/wait marker `{forbidden}`.",
        );
    }

    for required in [
        "data-slot=\"command-e2e-default\"",
        "data-slot=\"command-last-action\"",
        "data-scenario=\"default\"",
        "data-slot=\"command-e2e-markers\"",
        "data-scenario=\"markers\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs command page should keep e2e semantic anchor `{required}`.",
        );
    }
}

#[test]
fn command_e2e_check_script_covers_selector_contract() {
    let script_source = include_str!("../../../components/command/scripts/check-ui-e2e-command.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-command --lib command_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-command --lib command_e2e_check_script_covers_selector_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "command e2e check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = include_str!("../check2.md");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(required),
            "command checklist should keep repeatable e2e key-flow governance marker `{required}`.",
        );
    }

    for marker in [
        "command_check2_documents_e2e_repeatable_key_flow_rules",
        "command_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "command_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "components/command/scripts/check-ui-e2e-command.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "command checklist repeatable e2e section should keep evidence marker `{marker}`.",
        );
    }
}

#[test]
fn command_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_command.spec.mjs");
    let script_source = include_str!("../../../components/command/scripts/check-ui-e2e-command.sh");

    for required in [
        "key flow is repeatable and failures map to semantic breakpoints",
        "runDefaultSubmitFlow",
        "toHaveAttribute(\"data-query\", \"present\")",
        "toHaveAttribute(\"data-state\", \"query-results\")",
        "toHaveAttribute(\"data-query-control\", \"uncontrolled\")",
        "toHaveAttribute(\"data-last-action\", \"calendar\")",
        "await page.reload();",
    ] {
        assert!(
            e2e_source.contains(required),
            "command repeatable e2e key flow should include semantic breakpoint marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_e2e_key_flow_is_repeatable_and_failure_points_are_semantic";
    assert!(
        script_source.contains(script_needle),
        "e2e-command gate script should include `{script_needle}`.",
    );
}

#[test]
fn command_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_command.spec.mjs");
    let script_source = include_str!("../../../components/command/scripts/check-ui-e2e-command.sh");

    for required in [
        "high-risk paths keep focus keyboard and settled semantic breakpoints",
        "input.focus()",
        "toBeFocused()",
        "keyboard.press(\"ArrowDown\")",
        "[data-slot=\"command-item\"][data-focused=\"true\"]",
        "toHaveAttribute(\"data-focused\", \"true\")",
        "toHaveAttribute(\"role\", \"option\")",
        "toHaveAttribute(\"data-last-action\", \"calendar\")",
        "toHaveAttribute(\"data-state\", \"query-results\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "command high-risk e2e path should include semantic breakpoint marker `{required}`.",
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints";
    assert!(
        script_source.contains(script_needle),
        "e2e-command gate script should include `{script_needle}`.",
    );
}

#[test]
fn command_e2e_check_script_covers_repeatable_key_flow_contract() {
    let script_source = include_str!("../../../components/command/scripts/check-ui-e2e-command.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-command --lib command_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-command --lib command_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "cargo test -p ui-command --lib command_e2e_check_script_covers_repeatable_key_flow_contract",
    ] {
        assert!(
            script_source.contains(needle),
            "command e2e check script should enforce repeatable key-flow contract `{needle}`.",
        );
    }
}

#[test]
fn command_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let check2_source = include_str!("../check2.md");
    let protocol_source = include_str!("../src/protocol.rs");
    let command_cargo_source = include_str!("../Cargo.toml");
    let ui_components_cargo_source = include_str!("../../../crates/ui/Cargo.toml");
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let readme_source = include_str!("../src/README.md");
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for needle in [
        "use serde::{Deserialize, Serialize};",
        "#[serde(rename_all = \"snake_case\")]",
        "pub enum CommandComponentSchemaVersion",
        "pub struct CommandComponentSpec",
        "#[serde(default)]",
        "pub schema_version: CommandComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(needle),
            "command protocol should keep structured serde contract marker `{needle}`."
        );
    }

    assert!(
        command_cargo_source.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "ui-command manifest should keep serde derive dependency for structured protocol serialization."
    );

    for forbidden in [
        "tokio",
        "async-std",
        "async_std",
        "smol",
        "Runtime",
        "JoinHandle",
    ] {
        assert!(
            !command_cargo_source.contains(forbidden),
            "ui-command manifest should not bind to concrete async runtime marker `{forbidden}`."
        );
    }

    assert!(
        ui_components_cargo_source.contains("tracing = { version = \"0.1\", optional = true }"),
        "ui should keep tracing as optional shared dependency for unified semantics."
    );

    for forbidden in [
        "tracing::",
        "span!(",
        "event!(",
        "#[instrument]",
        "tokio::",
        "async_std::",
        "async-std",
        "Runtime",
        "JoinHandle",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !readme_source.contains(forbidden),
            "command component should keep tracing/runtime details out of component API/flow marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        script_source.contains(script_needle),
        "engineering check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "CommandComponentSpec + schema_version",
        "command_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep engineering evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/command.rbi");
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let styles_source = include_str!("../src/styles.rs");
    let motion_source = include_str!("../src/motion.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Command\"",
        "crate = \"ui-command\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "command manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub enum CommandComponentSchemaVersion",
        "V1",
        "pub struct CommandComponentSpec",
    ] {
        assert!(
            protocol_source.contains(needle),
            "command protocol should keep stable v1 spec marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Command(",
        "query: Option<leptos::prelude::Signal<String>>",
        "default_query: Option<String>",
        "on_query_change: Option<leptos::prelude::Callback<String>>",
        "is_disabled: bool",
    ] {
        assert!(
            rbi_source.contains(needle),
            "command RBI should keep stable public API marker `{needle}`."
        );
    }

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "deprecation_window",
        "deprecated_since",
        "schema_version = \"2\"",
        "contract.v2",
        "SchemaRegistry",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !protocol_source.contains(forbidden)
                && !combined.contains(forbidden),
            "command should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(script_needle),
        "engineering check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Command` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "command_version_deprecation_migration_is_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep version-migration marker `{needle}`."
        );
    }
}

#[test]
fn command_spec_rs_is_not_introduced_without_schema_contract() {
    let mod_source = include_str!("../src/mod.rs");
    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/spec.rs");

    assert!(
        !spec_path.exists(),
        "command should not add src/spec.rs unless a stable schema/spec contract exists."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "command public surface should not expose spec module marker `{forbidden}`."
        );
    }
}

#[test]
fn command_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let ui_components_lib_source = include_str!("../../../crates/ui/src/lib.rs");
    let ui_components_css_source = include_str!("../../../crates/ui/src/css.rs");
    let ui_root_source = include_str!("../../../crates/ui/src/root.rs");
    let active_highlight_source =
        include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-entrypoints.sh");

    for needle in [
        "#[cfg(feature = \"component-command\")]",
        "pub use ui_command as command;",
    ] {
        assert!(
            ui_components_lib_source.contains(needle),
            "ui lib.rs should keep command feature-gated public entry marker `{needle}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !ui_components_lib_source.contains(forbidden),
            "ui lib.rs should not leak platform detail marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String)",
        "#[cfg(feature = \"component-command\")]",
        "out.push_str(crate::command::styles::CSS);",
    ] {
        assert!(
            ui_components_css_source.contains(needle),
            "ui css.rs should keep feature-gated command aggregation marker `{needle}`."
        );
    }

    for needle in [
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "ui root.rs should keep centralized root injection marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight primitive should keep shared style/motion marker `{needle}`."
        );
    }

    for forbidden in ["ui-command", "command::", "ui-tabs", "accordion::"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight primitive should avoid component business semantic marker `{forbidden}`."
        );
    }

    let ui_components_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui/src");
    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src.join(forbidden).exists(),
            "ui fixed entry discipline should keep forbidden file absent `{forbidden}`."
        );
    }

    let ui_headless_src =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/ui-headless/src");
    for expected in ["controllable_state.rs", "presence.rs", "a11y.rs"] {
        assert!(
            ui_headless_src.join(expected).exists(),
            "ui-headless should keep canonical shared entry file `{expected}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        script_source.contains(script_needle),
        "entrypoints check script should include `{script_needle}`."
    );

    for needle in [
        "- [x] `ui` 固定入口文件落点正确。",
        "command_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep fixed-entrypoint evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_component_directory_standard_files_follow_contract_and_na_spec() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "command component should keep required standard file `{required}`."
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "command component should keep non-required file absent `{forbidden}`."
        );
    }

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Command;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable export boundary marker `{needle}`."
        );
    }
    for forbidden in ["pub mod logic;", "pub mod motion;", "pub mod view;"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not over-export implementation module marker `{forbidden}`."
        );
    }

    for needle in [
        "use ui_state_primitives::command as command_primitives;",
        "pub fn resolve_root_state",
        "pub fn resolve_state",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "NodeRef", "on:click", "web_sys", "wasm_bindgen"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not contain render/platform detail marker `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should keep static CSS contract constant."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should keep token-first var(--ui-*) consumption contract."
    );
    for forbidden in ["#[component]", "view! {", "NodeRef", "#ff", "rgb(", "hsl("] {
        assert!(
            !styles_source.to_ascii_lowercase().contains(forbidden),
            "styles.rs should avoid runtime/render or hardcoded theme literal marker `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "view! {",
        "command_input_attrs(lang, dir)",
        "command_option_a11y_attrs(CommandOptionA11yInput",
        "logic::resolve_root_state(logic::CommandRootStateInput",
        "crate::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep Leptos structure + headless mount marker `{needle}`."
        );
    }
    for forbidden in [
        "pub const CSS: &str",
        "attach_active_highlight_motion(",
        "mod render",
        "render.rs",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid style-engine/render-module drift marker `{forbidden}`."
        );
    }

    for needle in [
        "pub type CommandMotion = ui_visual_primitive::active_highlight::ActiveHighlightMotion;",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep semantic-to-motion mapping marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "command_input_attrs(", "on:click"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not include render or interaction semantics marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_component_directory_standard_files_follow_contract_and_na_spec";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 组件目录标准文件落点正确。",
        "command_component_directory_standard_files_follow_contract_and_na_spec",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 should keep component-directory evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_file_placement_discipline_is_strict_for_component_scope() {
    let mod_source = include_str!("../src/mod.rs");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "file-placement discipline requires standard component file `{required}`."
        );
    }

    assert!(
        !src_dir.join("render.rs").exists(),
        "file-placement discipline forbids render.rs drift in command component."
    );

    assert!(
        !src_dir.join("spec.rs").exists(),
        "file-placement discipline keeps spec.rs optional and absent for simple command component."
    );

    for forbidden in ["pub mod logic;", "pub mod motion;", "pub mod view;"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should keep export-only boundary and avoid over-export marker `{forbidden}`."
        );
    }
    for required in ["mod logic;", "mod motion;", "pub mod styles;", "mod view;"] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep standard file placement boundary marker `{required}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "command_file_placement_discipline_is_strict_for_component_scope",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep file-placement discipline evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let mod_source = include_str!("../src/mod.rs");
    let readme_source = include_str!("../src/README.md");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");
    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");

    assert!(
        !src_dir.join("spec.rs").exists(),
        "simple command component should not introduce Hyper-Structure Builder spec.rs."
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "CommandSpec",
        "Spec::new(",
        ".render(",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !readme_source.contains(forbidden),
            "simple command component should not expose hyper-structure builder marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for needle in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`command` 为简单组件，当前无稳定外部 Schema/Builder 契约需求，不引入 `spec.rs` 与 `*Spec::new()...render()` 链路）",
        "command_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep hyper-structure builder N/A evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let component_manifest_source = include_str!("../src/Component.toml");
    let rbi_source = include_str!("../src/command.rbi");
    let check2_source = include_str!("../check2.md");
    let script_source = include_str!("../../../scripts/check-ui-component-files.sh");

    let src_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert!(
        src_dir.join("Component.toml").exists(),
        "command context-compression contract requires src/Component.toml."
    );
    assert!(
        src_dir.join("command.rbi").exists(),
        "command context-compression contract requires src/command.rbi."
    );

    for needle in [
        "schema_version = \"1\"",
        "name = \"Command\"",
        "crate = \"ui-command\"",
        "name = \"id_base\"",
        "name = \"groups\"",
        "name = \"query\"",
        "name = \"default_query\"",
        "name = \"on_query_change\"",
        "name = \"on_action\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            component_manifest_source.contains(needle),
            "Component.toml should keep manifest marker `{needle}`."
        );
    }

    for needle in [
        "pub use crate::CommandMotion;",
        "pub use ui_state_primitives::command::{",
        "pub const DEFAULT_ID_BASE: &str;",
        "pub enum CommandSlot",
        "pub fn Command(",
        "groups: std::sync::Arc<[crate::CommandGroup]>",
        "query: Option<leptos::prelude::Signal<String>>",
        "dir: Option<ui_headless::A11yDirection>",
    ] {
        assert!(
            rbi_source.contains(needle),
            "command.rbi should keep signature projection marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "command_context_compression_manifest_and_rbi_projection_are_present_and_current",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 should keep context-compression evidence marker `{needle}`."
        );
    }
}

#[test]
fn command_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "command_agent_contract_is_schema_typed_and_machine_readable",
        "command_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "command_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep Agent Contract governance rule `{required}`.",
        );
    }
}

#[test]
fn command_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");
    let component_manifest_source = include_str!("../src/Component.toml");

    for needle in [
        "pub const COMMAND_AGENT_SCHEMA: &str = \"ui.command.agent-contract\";",
        "pub enum CommandAgentSchemaVersion",
        "pub enum CommandAgentIntent",
        "pub enum CommandAgentAction",
        "pub enum CommandAgentState",
        "pub enum CommandAgentSource",
        "pub enum CommandAgentConfigPolicy",
        "pub struct CommandAgentContractInput",
        "pub struct CommandAgentContract",
        "pub fn resolve_agent_contract(input: CommandAgentContractInput) -> CommandAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "command logic should keep typed agent contract marker `{needle}`.",
        );
    }

    for needle in [
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_agent_contract(logic::CommandAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-ui-state-source=move || root_state.get().query_control_attr.as_attr()",
        "data-ui-action-source=move || root_state.get().action_source_attr.as_attr()",
        "data-ui-motion-source=move || root_state.get().motion_source_attr.as_attr()",
    ] {
        assert!(
            view_source.contains(needle),
            "command view should mount schemaized agent marker `{needle}`.",
        );
    }

    for needle in [
        "name = \"agent_contract_schema_markers\"",
        "schema = \"ui.command.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
    ] {
        assert!(
            component_manifest_source.contains(needle),
            "command Component.toml should keep schemaized marker declaration `{needle}`.",
        );
    }
}

#[test]
fn command_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");

    for typed_source in [
        "schema_version: CommandAgentSchemaVersion::V1",
        "intent: CommandAgentIntent::CommandDiscovery",
        "action: CommandAgentAction::FilterNavigateSelect",
        "CommandAgentState::Disabled",
        "CommandAgentState::QueryResults",
        "CommandAgentState::QueryEmpty",
        "CommandAgentState::Empty",
        "CommandAgentState::Idle",
        "CommandAgentSource::Controlled",
        "CommandAgentSource::Uncontrolled",
        "config_policy: CommandAgentConfigPolicy::Whitelist",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "command agent fields should stay type-derived via `{typed_source}`.",
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "schema_name: format!(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "command agent contract should avoid free-form schema splicing `{forbidden}`.",
        );
    }
}

#[test]
fn command_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let mod_source = include_str!("../src/mod.rs");
    let motion_source = include_str!("../src/motion.rs");
    let manifest_source = include_str!("../src/Component.toml");
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
            "command render path should stay whitelist-safe without `{forbidden}`.",
        );
    }

    for needle in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "allowed = [",
        "\"render_options_content\"",
        "\"render_group_section\"",
        "\"render_option_item\"",
        "\"render_empty_state\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(needle),
            "command manifest should keep whitelist boundary marker `{needle}`.",
        );
    }
}

#[test]
fn command_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = include_str!("../../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-command --lib command_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-command --lib command_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-command --lib command_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should keep command agent-contract guard `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = include_str!("../check2.md");
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let mod_source = include_str!("../src/mod.rs");
    let motion_source = include_str!("../src/motion.rs");
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`command` 组件不直接渲染 LLM 正文输出",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep streaming-definition marker `{required}`.",
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-stream-mode",
        "data-stream-fallback",
        "data-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "command should avoid unrelated streaming protocol marker `{forbidden}` in component runtime path.",
        );
    }

    let script_needle = "cargo test -p ui-command --lib command_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`."
    );
}

#[test]
fn command_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`command` 组件不直接渲染 LLM 正文输出",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep snapshot-baseline marker `{required}`.",
        );
    }
}

#[test]
fn command_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let docs_source = docs_command_source();

    for marker in [
        "pub fn Command(",
        "#[prop(into)] groups: Arc<[CommandGroup]>",
        "#[prop(optional)] query: Option<Signal<String>>",
        "#[prop(optional, into)] default_query: Option<String>",
        "#[prop(optional)] on_query_change: Option<Callback<String>>",
        "#[prop(optional)] on_action: Option<Callback<String>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] motion: CommandMotion",
        "#[prop(optional, into)] placeholder: Option<String>",
        "#[prop(optional, into)] empty_label: Option<String>",
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional, into)] class_name: Option<String>",
        "let default_query = logic::resolve_default_query(default_query);",
        "let query_state = use_controllable_state(query, Some(default_query), on_query_change);",
        "logic::resolve_root_state(logic::CommandRootStateInput {",
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-query-control=move || root_state.get().query_control_attr.as_attr()",
        "data-query-default-source=move || root_state.get().query_default_source_attr.as_attr()",
        "data-query-change-source=move || root_state.get().query_change_source_attr.as_attr()",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "command snapshot baseline should keep complete-result render marker `{marker}`.",
        );
    }

    for marker in [
        "pub struct CommandRootStateInput<'a>",
        "pub fn resolve_default_query(value: Option<String>) -> String",
        "pub fn resolve_root_state(input: CommandRootStateInput<'_>) -> CommandPartState",
        "pub fn resolve_agent_contract(input: CommandAgentContractInput) -> CommandAgentContract",
    ] {
        assert!(
            logic_source.contains(marker),
            "command logic should keep snapshot-baseline normalization marker `{marker}`.",
        );
    }

    for marker in [
        "title=\"Command\"",
        "slug=\"command\"",
        "title=\"Hello World (Default API)\"",
        "title=\"State + Source Markers\"",
        "title=\"Interactive Playground\"",
        "id_base=\"docs-command-workbench\".to_string()",
        "is_disabled=workbench_disabled.get()",
        "motion=workbench_motion.get()",
        "placeholder=if workbench_custom_text.get() {",
        "empty_label=if workbench_custom_text.get() {",
        "aria_label=if workbench_custom_text.get() {",
        "class_name=if workbench_custom_text.get() {",
    ] {
        assert!(
            docs_source.contains(marker),
            "command docs should keep snapshot baseline usage marker `{marker}`.",
        );
    }
}

#[test]
fn command_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-command --lib command_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = include_str!("../check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`command` 归类为 `Streaming Optional` 且当前实现为 `N/A`（`fallback=snapshot`）",
    ] {
        assert!(
            checklist_source.contains(required),
            "command checklist should keep streaming responsibility marker `{required}`.",
        );
    }
}

#[test]
fn command_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = include_str!("../src/view.rs");

    for required in [
        "role=input_a11y.role",
        "aria-autocomplete=input_a11y.aria_autocomplete",
        "aria-expanded=input_a11y.aria_expanded",
        "aria-label=aria_label.get_value()",
        "aria-controls=listbox_id.get_value()",
        "aria-activedescendant=move || listbox.attrs.aria_activedescendant.get()",
        "role=listbox.attrs.role",
        "aria-disabled=listbox.attrs.aria_disabled",
        "data-state=move || root_state.get().state_attr.as_attr()",
        "data-query=move || root_state.get().query_attr.as_attr()",
        "data-disabled=move || root_state.get().disabled_attr.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "command should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope.",
        );
    }

    for forbidden in [
        "data-ui-output-status",
        "data-output-status",
        "data-stream-mode",
        "data-stream-fallback",
        "data-stream-status",
        "data-status=\"draft\"",
        "data-status=\"verified\"",
        "data-status=\"committed\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "command should not mount fake streaming status marker `{forbidden}` when stream protocol is N/A.",
        );
    }
}

#[test]
fn command_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = include_str!("../src/view.rs");
    let logic_source = include_str!("../src/logic.rs");
    let mod_source = include_str!("../src/mod.rs");
    let motion_source = include_str!("../src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "command should keep validation/retry/resilience policy outside component layer; found `{forbidden}`.",
        );
    }
}

#[test]
fn command_streaming_script_covers_streaming_responsibility_contract() {
    let script_source = include_str!("../../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui-command --lib command_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-command --lib command_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-command --lib command_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mod_source = include_str!("../src/mod.rs");
    let logic_source = include_str!("../src/logic.rs");
    let styles_source = include_str!("../src/styles.rs");
    let view_source = include_str!("../src/view.rs");
    let motion_source = include_str!("../src/motion.rs");
    let protocol_source = include_str!("../src/protocol.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{protocol_source}"
    );

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "command non-test source should forbid rust-hygiene anti-pattern `{forbidden}`.",
        );
    }
}

#[test]
fn command_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow() {
    let logic_source = include_str!("../src/logic.rs");
    let view_source = include_str!("../src/view.rs");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> = vec![Cow::Borrowed(state.base_class)];",
        "classes.push(Cow::Borrowed(\"ui-command--custom-class\"));",
        "classes.push(Cow::Owned(class_name));",
        ".into_iter()",
        ".map(Cow::into_owned)",
        ".collect::<Vec<_>>()",
        ".join(\" \")",
    ] {
        assert!(
            logic_source.contains(required),
            "command logic should keep Cow-based string hotspot mitigation marker `{required}`.",
        );
    }

    for forbidden in [
        "\"ui-command--custom-class\".to_string()",
        "\"ui-command--custom-motion\".to_string()",
        "\"ui-command--querying\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "command string hotspot contract should avoid `{forbidden}`.",
        );
    }
}

#[test]
fn command_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = include_str!("../../../scripts/check-rust-hygiene.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "RUST_HYGIENE_SCOPE",
        "find \"${scope_roots[@]}\" -type f -name '*.rs' -path '*/src/*' | sort",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`.",
        );
    }
}

#[test]
fn command_engineering_script_covers_rust_hygiene_contract() {
    let script_source = include_str!("../../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui-command --lib command_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-command --lib command_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow",
        "cargo test -p ui-command --lib command_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn command_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = include_str!("../check2.md");

    for needle in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "components/command/src/logic.rs::compose_class_name",
        "Vec<Cow<'static, str>>",
        "command_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "command_rust_hygiene_string_clone_hotspots_converge_to_cow_or_static_borrow",
        "command_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "command_engineering_script_covers_rust_hygiene_contract",
        "./scripts/check-rust-hygiene.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "command check2 rust hygiene section should reference `{needle}`.",
        );
    }
}
