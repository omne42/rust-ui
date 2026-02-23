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

fn read_workspace_file(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let file = workspace_root.join(rel_path);
    fs::read_to_string(&file).unwrap_or_else(|e| panic!("read_to_string failed for {file:?}: {e}"))
}

#[test]
fn date_field_ui_exports_stable_public_surface() {
    let module_source = load_source("src/text_input/date_field/mod.rs");

    for needle in [
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::DateFieldMotion;",
        "pub use view::DateField;",
    ] {
        assert!(
            module_source.contains(needle),
            "DateField module should expose `{needle}` as the stable UI surface."
        );
    }

    for forbidden in ["pub mod view", "pub mod logic", "web_sys", "NodeRef"] {
        assert!(
            !module_source.contains(forbidden),
            "DateField public module surface must not expose `{forbidden}`."
        );
    }
}

#[test]
fn date_field_ui_files_follow_logic_view_styles_motion_boundaries() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let styles_source = load_source("src/text_input/date_field/styles.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");

    for needle in [
        "pub use ui_state_primitives::date_field::{",
        "normalize_date_value",
        "resolve_date_parts",
        "update_year_from_input",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateField logic.rs should keep normalization/derivation contract `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "use_date_field(",
        "ui_motion::web::animate(",
        "leptos::",
        "NodeRef",
        "class=\"ui-date-field",
        "var(--ui-",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic.rs must not contain `{forbidden}`."
        );
    }

    for needle in [
        "headless::use_controllable_state(",
        "use_date_field(DateFieldOptions {",
        "logic::resolve_state(DateFieldStateInput {",
        "motion::attach_motion(root_ref, has_value, motion);",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField view.rs should mount UI contracts with `{needle}`."
        );
    }
    for forbidden in [
        "ui_motion::web::animate(",
        "web_sys::",
        "DateFieldDataState::from_flags(",
        "normalize_date_value(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField view.rs must not contain `{forbidden}`."
        );
    }

    assert!(
        styles_source.contains("var(--ui-"),
        "DateField styles.rs should consume shared theme tokens via var(--ui-*)."
    );
    for forbidden in [
        "use_date_field(",
        "ui_motion::web::animate(",
        "web_sys::",
        "DateFieldStateInput",
        "DEFAULT_",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "DateField styles.rs must not contain `{forbidden}`."
        );
    }

    for needle in [
        "ui_motion::web::animate(",
        "default_text_field_motion_tokens()",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "DateField motion.rs should contain `{needle}` for motion contract + non-wasm stub."
        );
    }

    for forbidden in [
        "use_date_field(",
        "role=",
        "aria-label=",
        "data-state=",
        "normalize_date_value(",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "DateField motion.rs must not contain `{forbidden}`."
        );
    }
}

#[test]
fn date_field_feature_gate_is_anchored_at_ui_entrypoint() {
    let module_source = load_source("src/text_input/date_field/mod.rs");
    let ui_lib_source = read_workspace_file("crates/ui/src/lib.rs");

    assert!(
        ui_lib_source.contains("feature = \"component-date_field\""),
        "ui entrypoint should gate DateField behind `component-date_field` feature."
    );
    assert!(
        ui_lib_source
            .contains("pub use text_input::date_field::{DateField, DateFieldIds, DateFieldTone};"),
        "ui entrypoint should keep DateField stable export wiring."
    );

    assert!(
        !module_source.contains("pub mod view"),
        "date_field mod.rs should not expose implementation module `view`."
    );
    assert!(
        !module_source.contains("pub mod logic"),
        "date_field mod.rs should not expose implementation module `logic`."
    );
    assert!(
        !module_source.contains("pub fn "),
        "date_field mod.rs should not carry implementation functions."
    );
}

#[test]
fn date_field_tree_shaking_contract_is_feature_gated_and_budgeted_in_ci() {
    let ui_cargo = read_workspace_file("crates/ui/Cargo.toml");
    let ui_lib = read_workspace_file("crates/ui/src/lib.rs");
    let ui_css = read_workspace_file("crates/ui/src/css.rs");
    let tree_shaking_script = read_workspace_file("scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = read_workspace_file("scripts/tree_shaking_budget.env");

    for needle in [
        "component-date_field = [\"component-clear_button\"]",
        "all-components = [",
        "component-date_field",
    ] {
        assert!(
            ui_cargo.contains(needle),
            "ui feature graph should keep tree-shaking marker `{needle}`."
        );
    }

    for needle in [
        "feature = \"component-date_field\"",
        "pub use text_input::date_field::{DateField, DateFieldIds, DateFieldTone};",
        "#[cfg(feature = \"component-date_field\")]",
        "out.push_str(crate::text_input::date_field::styles::CSS);",
    ] {
        assert!(
            ui_lib.contains(needle) || ui_css.contains(needle),
            "DateField tree-shaking path should keep feature-gated export/css marker `{needle}`."
        );
    }

    for needle in [
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking CI contract should keep marker `{needle}`."
        );
    }
}

#[test]
fn date_field_simple_component_must_not_introduce_spec_rs() {
    let module_source = load_source("src/text_input/date_field/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let spec_path = workspace_root.join("components/text-input/src/date_field/spec.rs");

    assert!(
        !spec_path.exists(),
        "DateField is a simple component and must not introduce `spec.rs`: {spec_path:?}"
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !module_source.contains(forbidden),
            "DateField mod.rs must not wire `spec.rs` symbol `{forbidden}`."
        );
    }
}

#[test]
fn date_field_view_mounts_headless_semantics_contract() {
    let source = load_source("src/text_input/date_field/view.rs");

    for attr in [
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "aria-labelledby=move || group_aria_labelledby.get_value()",
        "lang=move || group_lang.get_value()",
        "dir=group_dir",
        "data-slot=\"date-field\"",
        "data-state=move || state.get().data_state_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-placeholder-source=move || state.get().placeholder_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=if has_custom_motion { \"custom\" } else { \"default\" }",
        "data-control-mode=control_mode.as_attr()",
        "data-value-source=value_source.as_attr()",
        "data-default-value-source=if has_default_value { \"custom\" } else { \"none\" }",
        "data-value-change-source=value_change_source.as_attr()",
        "data-interaction-source=move || interaction_source.get().as_attr()",
    ] {
        assert!(
            source.contains(attr),
            "DateField view should expose headless semantics contract marker `{attr}`."
        );
    }
}

#[test]
fn date_field_a11y_i18n_l10n_contract_is_wired_and_not_hardcoded() {
    let view_source = load_source("src/text_input/date_field/view.rs");
    let i18n_source = load_source("src/text_input/date_field/i18n.rs");

    for needle in [
        "let i18n = i18n::use_ui_i18n();",
        "let strings = i18n.strings::<DateFieldStrings>();",
        "logic::normalize_label(label, strings.label.as_ref())",
        "logic::normalize_placeholder(placeholder, strings.placeholder.as_ref())",
        "logic::normalize_aria_label(aria_label, strings.aria_label.as_ref())",
        "logic::normalize_year_aria_label(year_aria_label, strings.year_aria_label.as_ref())",
        "logic::normalize_month_aria_label(month_aria_label, strings.month_aria_label.as_ref())",
        "logic::normalize_day_aria_label(day_aria_label, strings.day_aria_label.as_ref())",
        "logic::normalize_clear_label(clear_label, strings.clear_label.as_ref())",
        "logic::normalize_clear_aria_label(clear_aria_label, strings.clear_aria_label.as_ref())",
        "use_date_field(DateFieldOptions {",
        "lang,",
        "dir,",
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "aria-labelledby=move || group_aria_labelledby.get_value()",
        "lang=move || group_lang.get_value()",
        "dir=group_dir",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField view should keep A11y+i18n/l10n wiring marker `{needle}`."
        );
    }

    for forbidden in [
        "\"Date field\"",
        "\"Date\"",
        "\"yyyy-mm-dd\"",
        "\"Year\"",
        "\"Month\"",
        "\"Day\"",
        "\"Clear\"",
        "\"Clear date\"",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField view.rs must not hardcode user-visible default copy marker `{forbidden}`."
        );
    }

    for needle in [
        "pub struct DateFieldStrings",
        "impl Default for DateFieldStrings",
        "label: DEFAULT_LABEL.into()",
        "placeholder: DEFAULT_PLACEHOLDER.into()",
        "aria_label: DEFAULT_ARIA_LABEL.into()",
        "year_aria_label: DEFAULT_YEAR_ARIA_LABEL.into()",
        "month_aria_label: DEFAULT_MONTH_ARIA_LABEL.into()",
        "day_aria_label: DEFAULT_DAY_ARIA_LABEL.into()",
        "clear_label: DEFAULT_CLEAR_LABEL.into()",
        "clear_aria_label: DEFAULT_CLEAR_ARIA_LABEL.into()",
    ] {
        assert!(
            i18n_source.contains(needle),
            "DateField i18n defaults should stay centralized marker `{needle}`."
        );
    }
}

#[test]
fn date_field_observability_contract_uses_stable_data_aria_markers() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");

    for needle in [
        "pub enum DateFieldDataState",
        "pub enum DateFieldControlMode",
        "pub enum DateFieldValueSource",
        "pub enum DateFieldValueChangeSource",
        "pub enum DateFieldInteractionSource",
        "DateFieldDataState::from_flags(",
        "DateFieldControlMode::from_is_controlled(is_controlled)",
        "DateFieldValueSource::from_control_mode(control_mode, has_default_value)",
        "DateFieldValueChangeSource::from_has_handler(has_value_change_handler)",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "DateField observability contract should keep closed marker definition `{needle}`."
        );
    }

    for needle in [
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "aria-labelledby=move || group_aria_labelledby.get_value()",
        "data-state=move || state.get().data_state_attr",
        "data-control-mode=control_mode.as_attr()",
        "data-value-source=value_source.as_attr()",
        "data-default-value-source=if has_default_value { \"custom\" } else { \"none\" }",
        "data-value-change-source=value_change_source.as_attr()",
        "data-interaction-source=move || interaction_source.get().as_attr()",
        "data-slot=\"date-field\"",
        "data-slot=\"date-field-year\"",
        "data-slot=\"date-field-month\"",
        "data-slot=\"date-field-day\"",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField observability surface should expose stable selector marker `{needle}`."
        );
    }
}

#[test]
fn date_field_semantics_matrix_covers_control_disabled_keyboard_pointer_and_platform_paths() {
    let view_source = load_source("src/text_input/date_field/view.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");
    let clear_button_source = read_workspace_file("components/button/src/clear_button/view.rs");

    for needle in [
        "let is_controlled = value.is_some();",
        "let control_mode = logic::DateFieldControlMode::from_is_controlled(is_controlled);",
        "let value_state = headless::use_controllable_state(value, Some(default_value), on_value_change);",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "disabled=is_disabled",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField semantics matrix should keep controlled/uncontrolled + disabled marker `{needle}`."
        );
    }

    for needle in [
        "on:input=move |ev| on_year_input_handler.run(event_target_value(&ev))",
        "on:input=move |ev| on_month_input_handler.run(event_target_value(&ev))",
        "on:input=move |ev| on_day_input_handler.run(event_target_value(&ev))",
        "set_interaction_source_for_year.set(logic::DateFieldInteractionSource::YearInput);",
        "set_interaction_source_for_month.set(logic::DateFieldInteractionSource::MonthInput);",
        "set_interaction_source_for_day.set(logic::DateFieldInteractionSource::DayInput);",
        "<ClearButton",
        "on_press=on_clear_handler",
        "set_interaction_source_for_clear.set(logic::DateFieldInteractionSource::ClearButton);",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField semantics matrix should expose keyboard/pointer interaction path marker `{needle}`."
        );
    }

    for needle in [
        "on:pointerdown=",
        "on:pointerup=",
        "on:keydown=",
        "on:keyup=",
        "on:click=",
    ] {
        assert!(
            clear_button_source.contains(needle),
            "ClearButton press semantics bridge should expose pointer/keyboard marker `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::animate(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "DateField platform branch matrix should keep SSR/wasm marker `{needle}`."
        );
    }
}

#[test]
fn date_field_tests_prioritize_semantic_contract_assertions_over_snapshot_only() {
    let semantics_source =
        read_workspace_file("components/text-input/test/date_field/semantics.rs");
    let logic_source = read_workspace_file("components/text-input/test/date_field/logic.rs");
    let motion_source = read_workspace_file("components/text-input/test/date_field/motion.rs");
    let styles_source = read_workspace_file("components/text-input/test/date_field/styles.rs");
    let protocol_source = read_workspace_file("components/text-input/test/date_field/protocol.rs");

    for source in [
        &logic_source,
        &motion_source,
        &styles_source,
        &protocol_source,
    ] {
        for forbidden in [
            "assert_snapshot!",
            "assert_debug_snapshot!",
            "to_match_snapshot(",
            "insta::",
        ] {
            assert!(
                !source.contains(forbidden),
                "DateField tests should not rely on snapshot-only assertion marker `{forbidden}`."
            );
        }
    }

    for needle in [
        "fn date_field_view_mounts_headless_semantics_contract()",
        "fn date_field_observability_contract_uses_stable_data_aria_markers()",
        "role=group_role",
        "aria-label=group_aria_label.get_value()",
        "data-state=move || state.get().data_state_attr",
        "data-value-source=value_source.as_attr()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "DateField test suite should keep semantic-contract assertion marker `{needle}`."
        );
    }
}

#[test]
fn date_field_public_props_follow_is_on_default_naming_contract() {
    let source = load_source("src/text_input/date_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for needle in [
        "#[prop(optional)] is_disabled: Option<bool>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
    ] {
        assert!(
            source.contains(needle),
            "DateField API naming contract should include `{needle}`."
        );
    }

    assert!(
        docs_source.contains("is_disabled="),
        "DateField docs/playground should use `is_disabled` as the canonical bool prop name."
    );
}

#[test]
fn date_field_controlled_uncontrolled_contract_is_explicit_and_stable() {
    let source = load_source("src/text_input/date_field/view.rs");

    for needle in [
        "#[prop(optional, into)] value: Option<Signal<Option<String>>>",
        "#[prop(optional)] default_value: Option<String>",
        "#[prop(optional)] on_value_change: Option<Callback<Option<String>>>",
        "let default_value = logic::resolve_default_value(default_value);",
        "let value_state = headless::use_controllable_state(value, Some(default_value), on_value_change);",
        "let value = value_state.value;",
        "let request_value_change = value_state.request_change;",
        "request_value_change_for_year.run(next);",
        "request_value_change_for_month.run(next);",
        "request_value_change_for_day.run(next);",
        "let on_clear = Callback::new(move |_| {",
        "request_value_change_for_clear.run(None);",
    ] {
        assert!(
            source.contains(needle),
            "DateField should keep controlled/uncontrolled contract marker `{needle}`."
        );
    }

    assert!(
        !source.contains("normalize_date_value(default_value)"),
        "DateField view.rs should not normalize default values directly; this must stay in logic.rs."
    );
}

#[test]
fn date_field_state_normalization_is_centralized_in_logic_layer() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let styles_source = load_source("src/text_input/date_field/styles.rs");

    for needle in [
        "pub fn resolve_state(input: DateFieldStateInput) -> DateFieldState",
        "label_source_attr",
        "placeholder_source_attr",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateField logic.rs should centralize state derivation marker `{needle}`."
        );
    }

    for needle in [
        "logic::resolve_state(DateFieldStateInput {",
        "let on_year_input = Callback::new(move |year_input: String| {",
        "let next = logic::update_year_from_input(",
        "let on_month_input = Callback::new(move |month_input: String| {",
        "let next = logic::update_month_from_input(",
        "let on_day_input = Callback::new(move |day_input: String| {",
        "let next = logic::update_day_from_input(",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField view.rs should route normalization through logic marker `{needle}`."
        );
    }

    for forbidden in [
        "parse_date_value(",
        "days_in_month(",
        "normalize_day(",
        "normalize_month(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField view.rs must not implement state normalization rule `{forbidden}`."
        );
    }

    for needle in [
        ".ui-date-field[data-tone=\"default\"]",
        ".ui-date-field[data-disabled=\"true\"]",
        ".ui-date-field[data-has-value=\"true\"] .ui-date-field__control",
        ".ui-date-field[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "DateField styles.rs should consume stable state marker selector `{needle}`."
        );
    }

    assert!(
        !styles_source.contains(":nth-child"),
        "DateField styles.rs should not infer state via fragile DOM-structure selectors."
    );
}

#[test]
fn date_field_discrete_state_axes_are_type_constrained() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");

    for needle in [
        "pub enum DateFieldTone",
        "pub enum DateFieldDataState",
        "DateFieldDataState::from_flags(",
        "#[prop(optional)] tone: DateFieldTone",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "DateField should type discrete axes with enum marker `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] tone: Option<String>",
        "#[prop(optional, into)] status: Option<String>",
        "#[prop(optional)] is_quiet: Option<bool>",
        "#[prop(optional)] is_strong: Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField must not expose free-form/string-bool state machine marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_state_primitives_source_is_correct() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");

    for needle in [
        "pub use ui_state_primitives::date_field::{",
        "normalize_date_value",
        "resolve_date_parts",
        "update_year_from_input",
        "update_month_from_input",
        "update_day_from_input",
    ] {
        assert!(
            logic_source.contains(needle),
            "DateField logic should consume state primitives marker `{needle}`."
        );
    }

    for forbidden in ["use leptos", "use_controllable_state(", "use_date_field("] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic.rs should remain an assembly/mapping layer, not own `{forbidden}`."
        );
    }

    for needle in [
        "headless::use_controllable_state(",
        "use_date_field(DateFieldOptions {",
    ] {
        assert!(
            view_source.contains(needle),
            "DateField view should bridge through headless contract marker `{needle}`."
        );
    }

    for forbidden in [
        "ui_state_primitives::date_field",
        "crate::app_state",
        "crate::store",
        "RwSignal<",
        "Store<",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField view must not directly bind business/global store marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_async_contract_is_not_applicable_and_not_exposed() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "use_async_action",
        "on_retry",
        "retry",
        "async fn",
        ".await",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not expose async protocol marker `{forbidden}` in non-async component."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not expose async protocol marker `{forbidden}` in non-async component."
        );
    }
}

#[test]
fn date_field_dx_default_path_is_simple_and_advanced_path_is_optional() {
    let view_source = load_source("src/text_input/date_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for forbidden in [
        "#[prop(optional)] state:",
        "#[prop(optional, into)] state:",
        "#[prop(optional)] headless:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField public API must not require internal state wiring marker `{forbidden}`."
        );
    }

    for needle in [
        "title=\"Hello World (Default DateField)\"",
        "r#\"<DateField id_base=\"invoice-date\".to_string() />\"#",
        "<DateField id_base=\"docs-date-field-showcase\".to_string() />",
        "title=\"Workbench (All API + Actual Config)\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "DateField docs should keep DX marker `{needle}`."
        );
    }
}

#[test]
fn date_field_docs_include_default_theme_visual_baseline_matrix() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for needle in [
        "title=\"Hello World (Default DateField)\"",
        "title=\"State Matrix (Default / Strong / Disabled)\"",
        "data-slot=\"date-field-state-matrix\"",
        "id_base=\"docs-date-field-matrix-default\".to_string()",
        "id_base=\"docs-date-field-matrix-strong\".to_string()",
        "id_base=\"docs-date-field-matrix-disabled\".to_string()",
        "tone=DateFieldTone::Strong",
        "is_disabled=true",
        "motion=ui::text_input::date_field::DateFieldMotion::disabled()",
    ] {
        assert!(
            docs_source.contains(needle),
            "DateField docs should expose default-theme baseline marker `{needle}`."
        );
    }
}

#[test]
fn date_field_parent_item_composition_contract_is_not_applicable() {
    let view_source = load_source("src/text_input/date_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for forbidden in [
        "#[prop(optional)] items:",
        "#[prop(optional)] labels:",
        "#[prop(optional)] titles:",
        "#[prop(optional)] panels:",
        "ItemSpec",
        "DateFieldItem",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "DateField should not expose Parent/Item composition API marker `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels", "<DateFieldItem"] {
        assert!(
            !docs_source.contains(forbidden),
            "DateField docs should not recommend implicit parallel-array composition marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_macro_micro_duality_is_not_applicable_without_drag_interaction() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "on_drag",
        "on_drag_end",
        "pointermove",
        "pointerdown",
        "pointerup",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain drag state machine marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain drag interaction marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "DateField motion should not contain drag physics marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_two_pass_geometry_rendering_is_not_applicable() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "Tooltip",
        "Popover",
        "Menu",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "ResizeObserver",
        "IntersectionObserver",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain geometry two-pass marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain geometry measurement marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "DateField motion should not contain geometry rectification marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_registration_protocol_is_not_applicable_for_non_collection_component() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "Accordion",
        "Tabs",
        "Menu",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain collection registration marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain collection registration marker `{forbidden}`."
        );
    }

    for forbidden in ["labels + children", "titles + panels", "HashSet"] {
        assert!(
            !docs_source.contains(forbidden),
            "DateField docs should not suggest implicit collection navigation marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_slot_projection_strategy_is_not_applicable_for_non_container_component() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "on_hidden",
        "on_shown",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain slot projection marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain keep-alive lifecycle marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "DateField motion should not contain keep-alive side-effect marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_env_streams_contract_is_not_applicable_without_env_subscriptions() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "onresize",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain env-stream action marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain raw env-subscription marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "DateField motion should not contain env-stream side-effect marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_event_light_cone_is_not_applicable_for_non_large_collection_component() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "RegisterBatch",
        "BulkSelection",
        "Table",
        "Grid",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain event light-cone marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain large-collection bus marker `{forbidden}`."
        );
    }

    for forbidden in ["prop drilling", "labels + children", "titles + panels"] {
        assert!(
            !docs_source.contains(forbidden),
            "DateField docs should not suggest O(N) fan-out marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_causality_bus_is_not_applicable_for_non_bus_component() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "Broadcast",
        "Subscriber",
        "CommandBus",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain causality bus marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain causality bus marker `{forbidden}`."
        );
    }

    for forbidden in ["用户触发", "派生命令", "总线广播", "订阅者"] {
        assert!(
            !docs_source.contains(forbidden),
            "DateField docs should not suggest causality-bus chain marker `{forbidden}`."
        );
    }
}

#[test]
fn date_field_focus_stack_gc_is_not_applicable_without_overlay_layering() {
    let logic_source = load_source("src/text_input/date_field/logic.rs");
    let view_source = load_source("src/text_input/date_field/view.rs");
    let motion_source = load_source("src/text_input/date_field/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/forms_extra/date_field.rs");

    for forbidden in [
        "FocusManager",
        "FocusStack",
        "FallbackTo",
        "focus_restore_target",
        "restore_focus_to",
        "OverlayLayer",
        "overlay_stack",
        "document.body",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "DateField logic should not contain overlay focus-stack marker `{forbidden}`."
        );
        assert!(
            !view_source.contains(forbidden),
            "DateField view should not contain overlay focus-stack marker `{forbidden}`."
        );
        assert!(
            !motion_source.contains(forbidden),
            "DateField motion should not contain overlay focus-stack marker `{forbidden}`."
        );
    }

    for forbidden in [
        "Focus Stack",
        "Focus Manager",
        "FallbackTo",
        "document.body",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "DateField docs should not suggest overlay focus-stack protocol marker `{forbidden}`."
        );
    }
}
