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
fn native_select_is_exported_from_module_and_crate_root() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::NativeSelect;"),
        "native_select module should export `NativeSelect`."
    );
    assert!(
        module_source.contains("pub use logic::{")
            && module_source.contains("DEFAULT_ARIA_LABEL")
            && module_source.contains("NativeSelectSize")
            && module_source.contains("NativeSelectState"),
        "native_select module should export `NativeSelectSize`, `NativeSelectState`, and `DEFAULT_ARIA_LABEL`."
    );
    assert!(
        crate_source.contains(
            "pub use native_select::{NativeSelect, NativeSelectOption, NativeSelectSize};"
        ),
        "crate root should re-export NativeSelect contracts."
    );
}

#[test]
fn native_select_component_file_responsibilities_are_scoped_and_motion_is_na() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "DEFAULT_ARIA_LABEL",
        "NativeSelectSize",
        "NativeSelectState",
        "pub use view::NativeSelect;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep export boundary responsibility via `{required}`."
        );
    }

    for forbidden in ["mod motion;", "pub mod motion", "ui_motion::"] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should keep motion.rs as N/A without local motion engine path `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs must not leak render/DOM responsibility `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs must stay static token-first CSS without render logic `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_native_select(NativeSelectOptions {",
        "on:change=on_change",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should focus on structure + headless mount via `{required}`."
        );
    }
}

#[test]
fn native_select_does_not_introduce_spec_rs_for_simple_component() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let spec_path = workspace_dir.join("components/native-select/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "NativeSelect should not introduce src/spec.rs for a simple component."
    );

    for forbidden in ["mod spec;", "pub mod spec", "pub use spec::"] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not wire spec module path `{forbidden}`."
        );
    }

    for forbidden in ["serde::", "#[derive(Serialize", "#[derive(Deserialize"] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "simple NativeSelect should not carry schema-versioning surface `{forbidden}`."
        );
    }
}

#[test]
fn native_select_engineering_contract_uses_structured_protocol_and_avoids_runtime_leakage() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let protocol_source = load_source("../../components/native-select/src/protocol.rs");
    let cargo_source = load_source("../../components/native-select/Cargo.toml");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
        "#[serde(rename_all = \"snake_case\")]",
        "#[serde(default)]",
        "schema_version",
    ] {
        assert!(
            protocol_source.contains(required),
            "NativeSelect protocol serialization contract should include `{required}`."
        );
    }

    for forbidden in [
        "tracing::",
        "#[instrument",
        "tokio::",
        "async_std",
        "async-std",
        "Runtime",
        "JoinHandle",
        "spawn_local(",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !protocol_source.contains(forbidden)
                && !cargo_source.contains(forbidden),
            "NativeSelect engineering contract should not leak runtime/tracing detail `{forbidden}`."
        );
    }

    for forbidden in ["tokio", "async-std", "tracing"] {
        assert!(
            !cargo_source.contains(forbidden),
            "NativeSelect Cargo dependencies should stay runtime-agnostic without `{forbidden}`."
        );
    }
}

#[test]
fn native_select_version_deprecation_registry_is_na_without_breaking_upgrade() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let protocol_source = load_source("../../components/native-select/src/protocol.rs");

    for required in ["schema_version", "#[serde(default)]", "V1"] {
        assert!(
            protocol_source.contains(required),
            "native-select protocol should preserve versioned baseline entry `{required}`."
        );
    }

    for forbidden in [
        "migrate_v1_to_v2",
        "migrate_v2_to_v3",
        "SchemaRegistry",
        "deprecation_window",
        "deprecated_since",
        "codemod",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "native-select should stay N/A for version-migration registry surface `{forbidden}` when no breaking upgrade is introduced."
        );
    }
}

#[test]
fn native_select_uses_logic_state_model() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for needle in [
        "pub enum NativeSelectSize",
        "pub struct NativeSelectStateParams",
        "pub fn normalize_options(",
        "pub fn normalize_default_selected_index(",
        "pub fn resolve_control_value(",
        "pub fn resolve_options(",
        "pub fn sanitize_selected_index(",
        "pub fn resolve_selected_index_correction(",
        "pub fn resolve_states_for_render(",
        "pub fn find_index_by_value(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            logic_source.contains(needle),
            "NativeSelect logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "let default_selected_index = logic::normalize_default_selected_index(default_selected_index);",
        "let selected_state = overlay_open::use_controllable_state(",
        "let selected_index = selected_state.value;",
        "let request_selected_index_change = selected_state.request_change;",
        "logic::resolve_selected_index_correction(selected_index.get(), &resolved_options.get())",
        "logic::resolve_states_for_render(NativeSelectStateParams {",
        "let state = Signal::derive(move ||",
        "logic::compose_class_name(class_name.get_value(), &state.get())",
        "logic::resolve_control_value(state.get().selected_value.as_deref())",
    ] {
        assert!(
            view_source.contains(needle),
            "NativeSelect view should derive wrapper state through logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn native_select_supports_controlled_uncontrolled_and_placeholder_contracts() {
    let source = load_source("../../components/native-select/src/view.rs");

    for needle in [
        "selected_index: Option<Signal<Option<usize>>>",
        "default_selected_index: Option<usize>",
        "on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_required: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional, into)] placeholder: Option<String>",
        "prop:value=move || selected_value.get()",
        "let on_change = move |ev: ev::Event| {",
        "let next_value = event_target_value(&ev);",
        "resolve_native_select_change_index(&next_value, &resolved_options.get_untracked())",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect should include `{needle}` for controlled/uncontrolled and placeholder behavior."
        );
    }
}

#[test]
fn native_select_discrete_axes_are_type_constrained() {
    let source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    assert!(
        logic_source.contains("pub enum NativeSelectSize"),
        "NativeSelect should model discrete size axis with an enum."
    );
    assert!(
        source.contains("#[prop(optional)] size: NativeSelectSize"),
        "NativeSelect view should expose size via NativeSelectSize instead of stringly input."
    );

    for forbidden in [
        "size: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !source.contains(forbidden),
            "NativeSelect should not expose string/Option<bool> discrete inputs `{forbidden}`."
        );
    }
}

#[test]
fn native_select_consumes_state_primitives_without_business_store_binding() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    assert!(
        module_source.contains("pub use ui_state_primitives::native_select::{"),
        "native-select module should consume contracts from ui-state-primitives."
    );

    for needle in [
        "ui_state_primitives::native_select::normalize_options(options)",
        "ui_state_primitives::native_select::resolve_options(id_base, options)",
        "ui_state_primitives::native_select::sanitize_selected_index(selected_index, options)",
        "ui_state_primitives::native_select::resolve_state(input);",
    ] {
        assert!(
            logic_source.contains(needle),
            "NativeSelect logic should map state primitives via `{needle}`."
        );
    }

    for forbidden in [
        "use_store(",
        "Store<",
        "Redux",
        "mobx",
        "zustand",
        "app_state",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not bind business store directly via `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_component_local_async_protocol_surface() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "use_async_action",
        "spawn_local(",
        "tokio::",
        "Future<",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define component-local async protocol `{forbidden}`."
        );
    }
}

#[test]
fn native_select_dx_paradox_keeps_minimal_api_and_hello_world_docs() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");

    for required in [
        "id_base: String,",
        "options: Vec<NativeSelectOption>,",
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect API should keep required/advanced boundary via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] state:",
        "state: NativeSelectState",
        "state: Signal<",
        "state=state",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "NativeSelect should not expose mandatory internal state object `{forbidden}`."
        );
    }

    for required in [
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "id_base=\"docs-native-select-hello\".to_string()",
        "options=vec![NativeSelectOption::new(\"system\", \"System\"), NativeSelectOption::new(\"manual\", \"Manual\")]",
    ] {
        assert!(
            docs_source.contains(required),
            "NativeSelect docs should include minimal hello-world DX entry `{required}`."
        );
    }
}

#[test]
fn native_select_dx_workbench_supports_live_css_and_optional_state_persistence() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for required in [
        "title=\"Interactive Playground\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"crates/ui/src/native_select/styles.rs\".to_string()",
        "scoped CSS live-edit（CSS Test）+ optional state persistence across reload",
        "let persisted_workbench_state = load_native_select_workbench_state();",
        "let (workbench_persist_state, set_workbench_persist_state) =",
        "save_native_select_workbench_state(NativeSelectWorkbenchState {",
        "clear_native_select_workbench_state();",
        "Persist workbench state",
        "data-slot=\"native-select-workbench-controls\"",
        "data-slot=\"native-select-workbench-canvas\"",
    ] {
        assert!(
            docs_source.contains(required),
            "NativeSelect docs workbench DX contract should include `{required}`."
        );
    }

    for required in [
        "const NATIVE_SELECT_WORKBENCH_STORAGE_KEY: &str = \"docs:native-select:workbench:state\";",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "fn load_native_select_workbench_state() -> Option<NativeSelectWorkbenchState>",
        "fn save_native_select_workbench_state(state: NativeSelectWorkbenchState)",
        "fn save_native_select_workbench_state(_state: NativeSelectWorkbenchState) {}",
        "fn clear_native_select_workbench_state()",
    ] {
        assert!(
            docs_source.contains(required),
            "NativeSelect docs workbench state-persistence boundary should include `{required}`."
        );
    }

    for required in [
        "let (test_css, set_test_css) = signal(default_test_css.get_untracked());",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let on_reset_test_css: OnPress =",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground css-test feedback loop should include `{required}`."
        );
    }
}

#[test]
fn native_select_docs_are_copy_paste_ready_with_matrix_and_streaming_snapshot_contract() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let code_block_source = load_source("../../components/code-block/src/view.rs");

    for required in [
        "const NATIVE_SELECT_DOC_IMPORTS: &str =",
        "<Playground title=\"Hello World (Uncontrolled)\"",
        "<Playground title=\"Controlled vs Uncontrolled\"",
        "<Playground title=\"State Matrix (Controlled / Uncontrolled / Disabled)\"",
        "title=\"Streaming Optional (fallback=snapshot)\"",
        "code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()",
        "data-slot=\"native-select-controlled-uncontrolled\"",
        "data-slot=\"native-select-state-matrix\"",
        "data-slot=\"native-select-streaming-snapshot\"",
        "Inspect `data-streaming-mode/data-streaming-fallback/data-output-status`.",
        "data-slot=\"native-select-source-first\"",
        "data-slot=\"native-select-source-paths\"",
        "component-native_select + inject-css",
    ] {
        assert!(
            docs_source.contains(required),
            "NativeSelect docs copy-ready contract should include `{required}`."
        );
    }

    for required in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
        "CodeBlock code=resolved_code.get()",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground import-completion pipeline should include `{required}`."
        );
    }

    for required in [
        "class_name=\"ui-code-block__copy-button\"",
        "aria_label=copy_to_clipboard_aria_label.get_value()",
        "on_press=on_copy_press",
    ] {
        assert!(
            code_block_source.contains(required),
            "CodeBlock should provide one-click copy entry via `{required}`."
        );
    }
}

#[test]
fn native_select_docs_matrix_and_api_contract_are_synced_with_logic_defaults() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");

    for required in [
        "<Playground title=\"Controlled vs Uncontrolled\"",
        "<Playground title=\"State Matrix (Controlled / Uncontrolled / Disabled)\"",
        "data-slot=\"native-select-controlled-uncontrolled\"",
        "data-slot=\"native-select-state-matrix\"",
        "default_selected_index=0",
        "selected_index=Signal::derive(|| Some(2usize))",
        "is_disabled=true",
    ] {
        assert!(
            docs_source.contains(required),
            "NativeSelect docs should keep state-matrix branch coverage via `{required}`."
        );
    }

    for required in [
        "selected_index=selected_signal",
        "on_selected_index_change=on_selected_change",
        "default_selected_index=1",
        "is_required=true",
        "is_invalid=true",
        "size=NativeSelectSize::Lg",
        "_ => NativeSelectSize::Md,",
    ] {
        assert!(
            docs_source.contains(required),
            "NativeSelect docs should keep API naming/default-value usage via `{required}`."
        );
    }

    for required in [
        "#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]",
        "#[default]",
        "Md,",
        "pub fn normalize_default_selected_index(",
        "default_selected_index.map(Some)",
    ] {
        assert!(
            logic_source.contains(required),
            "NativeSelect logic default contract should include `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] on_selected_index_change: Option<Callback<Option<usize>>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_required: bool",
        "#[prop(optional)] is_invalid: bool",
        "#[prop(optional)] size: NativeSelectSize",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect public API contract in view.rs should include `{required}`."
        );
    }
}

#[test]
fn native_select_readme_is_beginner_friendly_documentation_product() {
    let readme_source = load_source("../../components/native-select/src/README.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");

    for required in [
        "# NativeSelect",
        "## 新手路径：先用起来，再进阶",
        "### Hello World（零门槛）",
        "use ui::{NativeSelect, NativeSelectOption};",
        "### 常见用法（在 docs-app 直接对照）",
        "Hello World (Uncontrolled)",
        "Controlled + Placeholder",
        "Required + Invalid + Disabled",
        "Controlled vs Uncontrolled",
        "State Matrix (Controlled / Uncontrolled / Disabled)",
        "## API（进阶参考）",
    ] {
        assert!(
            readme_source.contains(required),
            "NativeSelect README should include beginner-friendly docs contract `{required}`."
        );
    }

    let hello_index = readme_source
        .find("### Hello World（零门槛）")
        .unwrap_or_else(|| panic!("README should contain hello-world section heading"));
    let api_index = readme_source
        .find("## API（进阶参考）")
        .unwrap_or_else(|| panic!("README should contain advanced API section heading"));
    assert!(
        hello_index < api_index,
        "README should place hello-world path before advanced API reference."
    );

    for required in [
        "title=\"NativeSelect\"",
        "<Playground title=\"Hello World (Uncontrolled)\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app page should remain a valid equivalent docs entry via `{required}`."
        );
    }
}

#[test]
fn native_select_heroui_strategy_doc_and_component_docs_are_synced() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let docs_catalog_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");
    let readme_source = load_source("../../components/native-select/src/README.md");

    for required in [
        "### NativeSelect 同步记录（2026-02-20）",
        "`selected_index/on_selected_index_change/default_selected_index`",
        "`is_disabled/is_required/is_invalid/size`",
        "`docs/spec/heroui-parameter-design-strategy.md`",
        "参数语义若变更，必须先同步本策略文档与 `components/native-select/src/README.md`、docs 入口，再推进实现",
    ] {
        assert!(
            strategy_source.contains(required),
            "HeroUI strategy doc should include NativeSelect sync evidence `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"NativeSelect\"",
        "\"native-select\"",
        "\"Forms\"",
        "forms_native::native_select",
    ] {
        assert!(
            docs_catalog_source.contains(required),
            "docs component catalog should keep NativeSelect indexable entry `{required}`."
        );
    }

    for required in ["title=\"NativeSelect\"", "slug=\"native-select\""] {
        assert!(
            docs_source.contains(required),
            "docs NativeSelect page should expose stable index keys via `{required}`."
        );
    }

    for required in [
        "# NativeSelect",
        "## 新手路径：先用起来，再进阶",
        "## Docs and Feature",
    ] {
        assert!(
            readme_source.contains(required),
            "component README should remain as equivalent documentation entry via `{required}`."
        );
    }
}

#[test]
fn native_select_item_semantics_are_bound_in_single_typed_option_model() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/native_select.rs");

    assert!(
        view_source.contains("options: Vec<NativeSelectOption>,"),
        "NativeSelect should expose typed option collection instead of parallel arrays."
    );

    for required in [
        "pub struct NativeSelectOption {",
        "pub value: String,",
        "pub label: String,",
        "pub disabled: bool,",
    ] {
        assert!(
            primitive_source.contains(required),
            "state primitive should bind option semantics in one item struct via `{required}`."
        );
    }

    for forbidden in [
        "labels: Vec<",
        "titles: Vec<",
        "children: Vec<",
        "panels: Vec<",
        "items_spec: Vec<",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden) && !docs_source.contains(forbidden),
            "NativeSelect should not expose parallel-array/spec-sugar API `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_dragging_macro_micro_state_machine_path() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for forbidden in [
        "Dragging",
        "Action::DragEnd",
        "on:pointermove",
        "on:mousemove",
        "on:touchmove",
        "on:drag",
        "requestAnimationFrame",
        "raf(",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define drag-loop macro/micro state machine path `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("on:change=on_change"),
        "NativeSelect interaction should remain discrete change event for this component."
    );
}

#[test]
fn native_select_has_no_two_pass_geometry_measurement_path() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

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
        "Intent",
        "Rectification",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define two-pass geometry pipeline `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("on:change=on_change"),
        "NativeSelect should keep discrete change-driven interaction instead of geometry measure loop."
    );
}

#[test]
fn native_select_has_no_registration_context_protocol_path() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/native_select.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "NativeSelect should not define dynamic registration protocol `{forbidden}`."
        );
    }

    for required in [
        "<For",
        "each=move || resolved_options.get()",
        "options: Vec<NativeSelectOption>,",
        "options.iter()",
        ".enumerate()",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || primitive_source.contains(required),
            "NativeSelect ordering should come from typed Vec flow `{required}`."
        );
    }
}

#[test]
fn native_select_has_no_slot_projection_lifecycle_protocol_path() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "suspend",
        "pause",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define slot projection lifecycle protocol `{forbidden}`."
        );
    }

    for required in ["<select", "<option", "<For"] {
        assert!(
            view_source.contains(required),
            "NativeSelect should remain native select + option rendering path `{required}`."
        );
    }
}

#[test]
fn native_select_has_no_environment_stream_subscription_pipeline() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "matchMedia",
        "prefers-color-scheme",
        "debounce",
        "throttle",
        "on:resize",
        "window.",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define env stream subscription pipeline `{forbidden}`."
        );
    }

    assert!(
        view_source.contains("on:change=on_change"),
        "NativeSelect interaction should remain change-driven instead of env-stream-driven."
    );
}

#[test]
fn native_select_has_no_event_light_cone_bulk_bus_path() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "broadcast",
        "bulk",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define event-light-cone bulk bus path `{forbidden}`."
        );
    }

    for required in [
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect should remain single-axis change propagation via `{required}`."
        );
    }
}

#[test]
fn native_select_has_no_causality_bus_trace_id_pipeline() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "trace-id",
        "Causality Bus",
        "publish",
        "subscribe",
        "subscriber",
        "command_bus",
        "EventBus",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "NativeSelect should not define causality-bus trace pipeline `{forbidden}`."
        );
    }

    for required in [
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect should keep direct user-change -> selected-index mapping via `{required}`."
        );
    }
}

#[test]
fn native_select_mounts_headless_a11y_contract_with_locale_hooks() {
    let source = load_source("../../components/native-select/src/view.rs");

    for needle in [
        "A11yDirection",
        "NativeSelectOptions",
        "use_native_select",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let semantics = Signal::derive(move || {",
        "use_native_select(NativeSelectOptions {",
        "lang: lang.get_value(),",
        "dir,",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect should consume ui-headless contract marker `{needle}`.",
        );
    }
}

#[test]
fn native_select_a11y_i18n_contract_uses_headless_locale_and_configurable_text_sources() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/native_select.rs");
    let headless_native_select_source =
        load_source("../../crates/ui-headless/src/native_select.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for required in [
        "#[prop(optional, into)] aria_label: Option<String>",
        "#[prop(optional, into)] placeholder: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "use_native_select(NativeSelectOptions {",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "{placeholder}",
        "{option.label}",
        "aria-hidden=\"true\"",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect view should expose a11y/i18n hook `{required}`."
        );
    }

    for required in [
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "ui_state_primitives::native_select::normalize_aria_label(value)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should normalize aria label through primitive source `{required}`."
        );
    }

    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Native select\";",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
        "(DEFAULT_ARIA_LABEL.into(), false)",
    ] {
        assert!(
            primitive_source.contains(required),
            "state primitive should provide fallback aria-label contract `{required}`."
        );
    }

    for required in [
        "use crate::a11y::{A11yDirection, locale_attrs};",
        "let locale = locale_attrs(options.lang, options.dir);",
    ] {
        assert!(
            headless_native_select_source.contains(required),
            "native-select headless contract should reuse shared a11y locale helper `{required}`."
        );
    }

    assert!(
        headless_a11y_source.contains(
            "pub fn locale_attrs(lang: Option<String>, dir: Option<A11yDirection>) -> A11yLocaleAttrs",
        ),
        "shared a11y helper should define locale_attrs in crates/ui-headless/src/a11y.rs."
    );

    for forbidden in ["\"Native select\"", "\"Select\"", "\"Choose\""] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not hardcode business-visible fallback copy `{forbidden}`."
        );
    }
}

#[test]
fn native_select_exposes_observable_retrievable_verifiable_state_markers() {
    let view_source = load_source("../../components/native-select/src/view.rs");

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().control_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-selection-mode=if is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "data-selection-source=if is_controlled {",
        "\"external\"",
        "\"default\"",
        "\"internal\"",
        "data-change-source=move || selection_change_source_attr.get()",
        "signal(\"initial\")",
        "set_selection_change_source_attr.set(\"user\")",
        "set_selection_change_source_attr.set(\"external\")",
        "set_selection_change_source_attr.set(\"internal\")",
        "set_selection_change_source_attr.set(\"sync-effect\")",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect should expose stable state/source marker contract `{required}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", "querySelector(\"."] {
        assert!(
            !view_source.contains(forbidden),
            "marker contract should avoid brittle selector dependency `{forbidden}`."
        );
    }
}

#[test]
fn native_select_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for required in [
        ".ui-native-select--size-sm .ui-native-select__control",
        ".ui-native-select--size-md .ui-native-select__control",
        ".ui-native-select--size-lg .ui-native-select__control",
        ".ui-native-select--invalid .ui-native-select__control",
        ".ui-native-select--selected .ui-native-select__control",
        ".ui-native-select--empty .ui-native-select__control",
        ".ui-native-select--disabled .ui-native-select__control",
        ".ui-native-select__control:disabled",
        ".ui-native-select__control:focus-visible",
        "data-state=move || state.get().data_state_attr",
        "data-disabled=move || state.get().control_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
    ] {
        assert!(
            styles_source.contains(required) || view_source.contains(required),
            "NativeSelect style/state contract should include explicit selector or marker `{required}`."
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        "style=",
    ] {
        assert!(
            !styles_source.contains(forbidden) && !view_source.contains(forbidden),
            "NativeSelect should not rely on brittle structure or inline-style business logic `{forbidden}`."
        );
    }
}

#[test]
fn native_select_semantics_contract_tests_cover_key_matrix_without_snapshot_dependency() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let component_semantics_source =
        load_source("../../components/native-select/test/semantics.rs");
    let workspace_semantics_source = include_str!("native_select_semantics.rs");

    for required in [
        "fn native_select_controlled_uncontrolled_axis_is_triplet_and_not_half_controlled()",
        "fn native_select_has_no_component_local_async_protocol_surface()",
        "fn native_select_mounts_headless_a11y_contract_with_locale_hooks()",
        "fn native_select_exposes_observable_retrievable_verifiable_state_markers()",
        "fn native_select_styles_depend_on_explicit_state_markers_not_dom_guessing()",
        "fn native_select_token_first_static_style_contract_is_enforced()",
        "fn native_select_styles_use_defensive_variable_fallback_chain()",
        "fn native_select_css_cascade_layer_contract_is_enforced()",
        "fn native_select_visual_desire_theme_baseline_is_enforced()",
        "fn native_select_tree_shaking_feature_gates_are_component_scoped()",
        "fn native_select_ui_components_fixed_entry_files_are_in_correct_locations()",
        "fn native_select_component_directory_standard_file_placement_is_correct()",
        "fn native_select_type_system_and_semantic_markers_form_machine_readable_contract()",
        "fn native_select_has_no_overlay_focus_stack_gc_path()",
        "fn native_select_has_no_foreign_zone_escape_hatch_path()",
        "fn native_select_hydration_ids_are_deterministic_without_time_or_random_sources()",
        "fn native_select_ssr_and_cross_platform_compile_contract_is_preserved()",
        "fn native_select_respects_ui_headless_web_ssr_compile_error_mutex()",
        "fn native_select_respects_ui_motion_non_wasm_noop_contract()",
        "fn native_select_reduced_motion_ssr_wasm_contract_is_preserved()",
        "fn native_select_performance_budget_contract_is_guarded_without_render_count_harness()",
        "fn native_select_view_macro_complexity_is_controlled_by_semantic_splitting()",
        "fn native_select_functional_fragment_split_prefers_plain_functions()",
        "fn native_select_static_fragment_is_constantized_with_stable_a11y_contract()",
        "fn native_select_inner_html_contract_disallows_injection_surface()",
        "fn native_select_wasm_debug_contract_tracks_state_and_keeps_api_clean()",
        "fn native_select_dx_workbench_supports_live_css_and_optional_state_persistence()",
        "fn native_select_docs_are_copy_paste_ready_with_matrix_and_streaming_snapshot_contract()",
        "fn native_select_heroui_strategy_doc_and_component_docs_are_synced()",
        "fn native_select_context_compression_manifest_and_rbi_are_present_and_synced()",
        "fn native_select_agent_contract_schema_markers_are_typed_and_whitelisted()",
        "fn native_select_streaming_term_is_scoped_to_llm_output_rendering_only()",
        "fn native_select_streaming_policy_is_optional_with_snapshot_fallback_and_readable_status_markers()",
        "fn native_select_rust_hygiene_disallows_unwrap_expect_let_underscore_and_string_clone_churn()",
        "fn native_select_snapshot_baseline_renders_complete_config_stably()",
        "fn native_select_engineering_contract_uses_structured_protocol_and_avoids_runtime_leakage()",
        "fn native_select_version_deprecation_registry_is_na_without_breaking_upgrade()",
    ] {
        assert!(
            component_semantics_source.contains(required)
                || workspace_semantics_source.contains(required),
            "semantic contract matrix should include `{required}`."
        );
    }

    for required in [
        "#[prop(optional)] selected_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_selected_index: Option<usize>",
        "#[prop(optional)] is_disabled: bool",
        "on:change=on_change",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "data-state=move || state.get().data_state_attr",
        "data-selection-mode=if is_controlled {",
        "data-selection-source=if is_controlled {",
        "data-change-source=move || selection_change_source_attr.get()",
    ] {
        assert!(
            view_source.contains(required),
            "view semantic markers/interaction path should include `{required}`."
        );
    }

    for forbidden in ["#[cfg(", "web_sys::", "wasm_bindgen::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "native-select has no component-local SSR/wasm split branch `{forbidden}`."
        );
    }

    for forbidden in [
        "insta::",
        "assert_snapshot!",
        "to_match_snapshot",
        "snapbox",
    ] {
        assert!(
            !component_semantics_source.contains(forbidden)
                && !workspace_semantics_source.contains(forbidden),
            "semantic contract should not depend on snapshot assertion `{forbidden}`."
        );
    }
}

#[test]
fn native_select_token_first_static_style_contract_is_enforced() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let css_aggregator_source = load_source("src/css.rs");
    let cargo_toml_source = load_source("Cargo.toml");

    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should be native-select static CSS source of truth."
    );
    assert!(
        styles_source.contains("var(--ui-"),
        "styles.rs should consume ui-theme token variables via `var(--ui-*)`."
    );
    assert!(
        css_aggregator_source.contains("#[cfg(feature = \"component-native_select\")]")
            && css_aggregator_source.contains("out.push_str(crate::native_select::styles::CSS);"),
        "ui css aggregator should feature-gate native-select style injection."
    );
    assert!(
        cargo_toml_source.contains("inject-css = []")
            && cargo_toml_source.contains("component-native_select = [\"dep:ui-native-select\"]"),
        "ui feature map should keep inject-css + component-native_select wiring."
    );

    for forbidden in [
        "style=",
        "style!(",
        "css!(",
        "styled_",
        "stylist::",
        "tailwind",
        "tw-",
        "class=\"flex",
        "class=\"grid",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect component source should avoid Utility-First / CSS-in-Rust default path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-sm, var(--ui-fallback-shadow-sm))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))",
        "var(--ui-text-field-motion-easing, var(--ui-fallback-text-field-motion-easing))",
    ] {
        assert!(
            styles_source.contains(required),
            "NativeSelect defensive variable contract should include `{required}`."
        );
    }

    for forbidden in [
        "var(--ui-motion-duration-fast, 120ms)",
        "var(--ui-motion-ease-standard, ease)",
        "font-size: var(--ui-button-size-s-font-size, 13px);",
        "line-height: var(--ui-button-size-s-line-height, 18px);",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg);",
        "color: var(--ui-fg);",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "NativeSelect defensive variable contract should not keep bare fallback literal `{forbidden}`."
        );
    }

    for required in [
        "--ui-fallback-border-width:",
        "--ui-fallback-border:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-shadow-sm:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-accent:",
        "--ui-fallback-danger:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-bg-muted:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-space-lg:",
        "--ui-fallback-component-height-100:",
        "--ui-fallback-text-field-motion-duration:",
        "--ui-fallback-text-field-motion-easing:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme fallback SSOT should include `{required}`."
        );
    }
}

#[test]
fn native_select_css_cascade_layer_contract_is_enforced() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let css_aggregator_source = load_source("src/css.rs");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-native_select\")]",
        "out.push_str(crate::native_select::styles::CSS);",
        "out.push_str(\"}\\n\");",
    ] {
        assert!(
            css_aggregator_source.contains(required),
            "NativeSelect cascade-layer contract should include `{required}`."
        );
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"width:",
        "style=\"height:",
        "style=\"transform:",
    ] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not rely on ordinary inline style value `{forbidden}`."
        );
    }

    assert!(
        !view_source.contains("style="),
        "NativeSelect view should avoid ordinary inline style attributes and keep runtime numeric adjustments out of DOM style strings."
    );
}

#[test]
fn native_select_visual_desire_theme_baseline_is_enforced() {
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");

    for required in [
        ".ui-native-select__control:not(:disabled):hover",
        ".ui-native-select__control:not(:disabled):active",
        ".ui-native-select__control:focus-visible",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "border: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
    ] {
        assert!(
            styles_source.contains(required),
            "NativeSelect default-theme visual baseline should include `{required}`."
        );
    }

    for required in [
        "title=\"NativeSelect\"",
        "slug=\"native-select\"",
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "<Playground title=\"Controlled + Placeholder\" code_signal=code>",
        "<Playground title=\"Required + Invalid + Disabled\" code_signal=states_code>",
    ] {
        assert!(
            docs_source.contains(required),
            "docs baseline should include `{required}` for NativeSelect visual acceptance."
        );
    }
}

#[test]
fn native_select_tree_shaking_feature_gates_are_component_scoped() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let cargo_toml_source = load_source("Cargo.toml");

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-native_select\")]\npub use ui_native_select as native_select;"
        ),
        "ui lib export should gate NativeSelect behind `component-native_select`."
    );
    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-native_select\")]\n    out.push_str(crate::native_select::styles::CSS);"
        ),
        "ui css aggregation should gate NativeSelect CSS behind `component-native_select`."
    );
    assert!(
        lib_source.contains("#[cfg(feature = \"all-components\")]")
            && lib_source.contains("mod all_components {")
            && lib_source.contains(
                "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]"
            ),
        "ui central aggregation paths should remain feature-gated (`all-components` / `web-demo-components`)."
    );
    assert!(
        cargo_toml_source.contains("component-native_select = [\"dep:ui-native-select\"]")
            && cargo_toml_source.contains("default = [\"inject-css\", \"all-components\"]")
            && cargo_toml_source.contains("all-components = [")
            && cargo_toml_source.contains("\"component-native_select\""),
        "ui Cargo feature graph should keep component-scoped feature + optional all-components aggregate."
    );

    for forbidden in ["all_components", "web_demo_components"] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect component source should not host global component registry path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_ui_components_fixed_entry_files_are_in_correct_locations() {
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let cargo_toml_source = load_source("Cargo.toml");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for required in [
        "mod css;",
        "mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-native_select\")]",
        "pub use ui_native_select as native_select;",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib.rs should include fixed entry contract `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "#[cfg(feature = \"component-native_select\")]",
        "out.push_str(crate::native_select::styles::CSS);",
    ] {
        assert!(
            css_source.contains(required),
            "ui css.rs should keep feature-gated aggregation contract `{required}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            root_source.contains(required),
            "ui root.rs should centralize theme/css/i18n injection via `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight_source.contains(required),
            "ui-visual-primitive active_highlight.rs should keep shared visual-motion primitive contract `{required}`."
        );
    }

    for forbidden in ["NativeSelect", "Accordion", "Dialog"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight.rs should stay generic without component business semantic `{forbidden}`."
        );
    }

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for absent in [
        "crates/ui/src/overlay_open.rs",
        "crates/ui/src/presence.rs",
        "crates/ui/src/a11y.rs",
    ] {
        assert!(
            !workspace_dir.join(absent).exists(),
            "ui should not define forbidden fixed-entry file `{absent}`."
        );
    }

    assert!(
        controllable_state_source.contains("pub fn use_controllable_state"),
        "open-state primitive source should stay in ui-headless controllable_state.rs."
    );
    assert!(
        presence_source.contains("pub fn use_presence"),
        "presence primitive source should stay in ui-headless presence.rs."
    );
    assert!(
        a11y_source.contains("pub fn locale_attrs("),
        "shared a11y utility source should stay in ui-headless a11y.rs."
    );
    assert!(
        cargo_toml_source.contains("component-native_select = [\"dep:ui-native-select\"]"),
        "ui Cargo features should keep component-level fixed-entry gate for native-select."
    );
}

#[test]
fn native_select_component_directory_standard_file_placement_is_correct() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let component_src_dir = workspace_dir.join("components/native-select/src");

    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs"] {
        assert!(
            component_src_dir.join(required).exists(),
            "native-select component directory should contain `{required}`."
        );
    }

    for absent in ["render.rs", "motion.rs", "spec.rs"] {
        assert!(
            !component_src_dir.join(absent).exists(),
            "native-select component directory should not contain `{absent}` for current scope."
        );
    }

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{",
        "pub use view::NativeSelect;",
    ] {
        assert!(
            module_source.contains(required),
            "mod.rs should keep minimal export boundary via `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic",
        "pub mod view",
        "mod motion;",
        "mod spec;",
        "mod render;",
    ] {
        assert!(
            !module_source.contains(forbidden),
            "mod.rs should not over-export or drift file entry via `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_options(",
        "pub fn resolve_states_for_render(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation responsibility via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not carry render/headless mount responsibility `{forbidden}`."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should keep token-first static CSS via `{required}`."
        );
    }

    for forbidden in ["view! {", "on:change=", "use_native_select("] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not carry rendering logic `{forbidden}`."
        );
    }

    for required in [
        "view! {",
        "use_native_select(NativeSelectOptions {",
        "on:change=on_change",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should keep Leptos structure + headless semantic mount via `{required}`."
        );
    }
}

#[test]
fn native_select_context_compression_manifest_and_rbi_are_present_and_synced() {
    let manifest_source = load_source("../../components/native-select/src/Component.toml");
    let rbi_source = load_source("../../components/native-select/src/native_select.rbi");

    for required in [
        "schema_version = \"1\"",
        "name = \"NativeSelect\"",
        "crate = \"ui-native-select\"",
        "name = \"id_base\"",
        "name = \"options\"",
        "name = \"selected_index\"",
        "name = \"default_selected_index\"",
        "name = \"on_selected_index_change\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
        "name = \"ui-headless\"",
        "name = \"ui-state-primitives\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should include manifest contract `{required}`."
        );
    }

    for required in [
        "pub use ui_state_primitives::native_select::{",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub enum NativeSelectSize {",
        "pub struct NativeSelectState {",
        "pub fn NativeSelect(",
        "id_base: String",
        "options: Vec<NativeSelectOption>",
        "selected_index: Option<leptos::prelude::Signal<Option<usize>>>",
        "on_selected_index_change: Option<leptos::prelude::Callback<Option<usize>>>",
        "dir: Option<ui_headless::A11yDirection>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(required),
            "native_select.rbi should include API signature projection `{required}`."
        );
    }
}

#[test]
fn native_select_agent_contract_schema_markers_are_typed_and_whitelisted() {
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let manifest_source = load_source("../../components/native-select/src/Component.toml");

    for required in [
        "pub const NATIVE_SELECT_AGENT_SCHEMA_NAME",
        "pub const NATIVE_SELECT_AGENT_SCHEMA_VERSION",
        "pub enum NativeSelectAgentIntent",
        "pub enum NativeSelectAgentAction",
        "pub enum NativeSelectAgentState",
        "pub enum NativeSelectAgentSource",
        "pub enum NativeSelectChangeSource",
        "pub enum NativeSelectAgentConfigPolicy",
        "pub struct NativeSelectAgentContract",
        "pub struct NativeSelectAgentContractInput",
        "pub fn resolve_agent_contract(input: NativeSelectAgentContractInput<'_>)",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should define typed agent-contract schema generation via `{required}`."
        );
    }

    for required in [
        "logic::resolve_agent_contract(logic::NativeSelectAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version",
        "data-ui-intent=move || agent_contract.get().intent.as_attr()",
        "data-ui-action=move || agent_contract.get().action.as_attr()",
        "data-ui-state=move || agent_contract.get().state.as_attr()",
        "data-ui-source=move || agent_contract.get().source.as_attr()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_attr()",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should mount typed agent-contract markers via `{required}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.native_select.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should carry agent-contract markers + whitelist boundary via `{required}`."
        );
    }
}

#[test]
fn native_select_streaming_term_is_scoped_to_llm_output_rendering_only() {
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let manifest_source = load_source("../../components/native-select/src/Component.toml");

    for required in [
        "pub fn NativeSelect(",
        "options: Vec<NativeSelectOption>",
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "native-select should stay on discrete selection interaction path `{required}`."
        );
    }

    for forbidden in [
        "EventSource",
        "WebSocket",
        "ReadableStream",
        "text/event-stream",
        "render_markdown",
        "markdown",
        "stream_chunk",
        "token_delta",
        "llm_stream",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden),
            "native-select should not introduce LLM streaming output pipeline surface `{forbidden}`."
        );
    }
}

#[test]
fn native_select_streaming_policy_is_optional_with_snapshot_fallback_and_readable_status_markers() {
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let manifest_source = load_source("../../components/native-select/src/Component.toml");

    for required in [
        "data-streaming-mode=\"optional\"",
        "data-streaming-fallback=\"snapshot\"",
        "data-output-status=move || output_status.get().as_attr()",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
        "data-state=move || state.get().data_state_attr",
    ] {
        assert!(
            view_source.contains(required),
            "native-select should expose optional-streaming snapshot fallback/status marker `{required}`."
        );
    }

    for required in [
        "pub enum NativeSelectOutputStatus",
        "Draft,",
        "Verified,",
        "Submittable,",
        "pub fn resolve_output_status(state: &NativeSelectState) -> NativeSelectOutputStatus",
        "state.is_invalid || state.is_empty",
    ] {
        assert!(
            logic_source.contains(required),
            "native-select output status contract should stay typed in logic.rs via `{required}`."
        );
    }

    for required in [
        "name = \"streaming_optional_fallback_snapshot\"",
        "name = \"snapshot_rendering\"",
        "data-streaming-mode + data-streaming-fallback + data-output-status",
    ] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should declare streaming-optional snapshot fallback capability via `{required}`."
        );
    }
}

#[test]
fn native_select_rust_hygiene_disallows_unwrap_expect_let_underscore_and_string_clone_churn() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let protocol_source = load_source("../../components/native-select/src/protocol.rs");

    for forbidden in [".unwrap(", ".expect(", ".unwrap_err(", "let _ ="] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "native-select non-test source should not contain forbidden hygiene pattern `{forbidden}`."
        );
    }

    for required in [
        "use std::borrow::Cow;",
        "Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-native-select\")",
        "Cow::Borrowed(\"ui-native-select--disabled\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic_source.contains(required),
            "native-select should use Cow-backed class assembly in logic.rs via `{required}`."
        );
    }

    for forbidden in [
        "\"ui-native-select\".to_string()",
        "\"ui-native-select--disabled\".to_string()",
        "\"ui-native-select--invalid\".to_string()",
        "\"ui-native-select--empty\".to_string()",
        "\"ui-native-select--selected\".to_string()",
        "\"ui-native-select--has-placeholder\".to_string()",
        "\"ui-native-select--custom-class\".to_string()",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "native-select should remove class-string clone hotspot `{forbidden}`."
        );
    }
}

#[test]
fn native_select_snapshot_baseline_renders_complete_config_stably() {
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let manifest_source = load_source("../../components/native-select/src/Component.toml");

    for required in [
        "pub fn NativeSelect(",
        "options: Vec<NativeSelectOption>",
        "let resolved_options =",
        "logic::resolve_states_for_render(NativeSelectStateParams {",
        "<For",
        "each=move || resolved_options.get()",
        "children=render_native_select_option",
        "prop:value=move || selected_value.get()",
    ] {
        assert!(
            view_source.contains(required) || logic_source.contains(required),
            "native-select snapshot baseline should accept complete config and render stably via `{required}`."
        );
    }

    for required in ["name = \"snapshot_rendering\"", "enabled = true"] {
        assert!(
            manifest_source.contains(required),
            "Component.toml should declare snapshot baseline capability via `{required}`."
        );
    }
}

#[test]
fn native_select_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");

    for required in [
        "pub enum NativeSelectSize",
        "pub struct NativeSelectStateParams<'a>",
        "pub fn sanitize_selected_index(",
        "pub fn resolve_selected_index_correction(",
        "pub fn resolve_states_for_render(",
        "ui_state_primitives::native_select::resolve_state(input);",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should provide typed state modeling and normalization entry `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-size=move || state.get().size_attr",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-selection-mode=if is_controlled {",
        "\"controlled\"",
        "\"uncontrolled\"",
        "data-selection-source=if is_controlled {",
        "\"external\"",
        "\"default\"",
        "\"internal\"",
        "data-change-source=move || selection_change_source_attr.get()",
        "set_selection_change_source_attr.set(\"user\")",
        "set_selection_change_source_attr.set(\"external\")",
        "set_selection_change_source_attr.set(\"internal\")",
        "set_selection_change_source_attr.set(\"sync-effect\")",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should expose machine-readable semantic contract marker `{required}`."
        );
    }

    for forbidden in [
        "size: Option<String>",
        "variant: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "Option<bool>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not expose stringly/boolean-explosion typed inputs `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_overlay_focus_stack_gc_path() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for forbidden in [
        "NodeRef",
        "use_focus_trap(",
        "FocusTrapOptions",
        "FocusTrapFrame",
        "RestorePolicy",
        "FallbackTo(",
        "Selector(",
        "document.body",
        "body()",
        "provide_overlay_stack(",
        "use_overlay_stack(",
        "use_overlay_stack_registration(",
        "data-ui-overlay-portal",
        "focus_manager_push_trap",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not carry overlay focus-stack/GC internals `{forbidden}`."
        );
    }
}

#[test]
fn native_select_has_no_foreign_zone_escape_hatch_path() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "OpenLayers",
        "YieldControl",
        "CleanupForeign",
        "ForeignZone",
        "foreign_zone",
        "yield_control",
        "cleanup_foreign",
        "js_sys::",
        "wasm_bindgen::JsValue",
        "web_sys::HtmlCanvasElement",
        "extern \"C\"",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not carry imperative third-party integration escape-hatch path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_hydration_ids_are_deterministic_without_time_or_random_sources() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/native_select.rs");

    for required in [
        "id_base: String,",
        "let id_base = StoredValue::new(id_base);",
        "id=move || format!(\"{}-root\", id_base.get_value())",
        "id=move || format!(\"{}-control\", id_base.get_value())",
        "ui_state_primitives::native_select::resolve_options(id_base, options)",
        "id: format!(\"{id_base}-option-{index}\")",
    ] {
        assert!(
            view_source.contains(required)
                || logic_source.contains(required)
                || primitive_source.contains(required),
            "NativeSelect hydration id path should keep deterministic seed mapping `{required}`."
        );
    }

    for forbidden in [
        "SystemTime",
        "UNIX_EPOCH",
        "Instant::now",
        "Utc::now",
        "now()",
        "uuid::",
        "Uuid",
        "rand::",
        "thread_rng",
        "getrandom",
        "nanoid",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !primitive_source.contains(forbidden),
            "NativeSelect should not introduce non-deterministic hydration id source `{forbidden}`."
        );
    }
}

#[test]
fn native_select_ssr_and_cross_platform_compile_contract_is_preserved() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let native_select_cargo_source = load_source("../../components/native-select/Cargo.toml");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");

    for required in [
        "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }",
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            native_select_cargo_source.contains(required)
                || headless_cargo_source.contains(required)
                || headless_lib_source.contains(required),
            "cross-platform compile contract should include `{required}`."
        );
    }

    for forbidden in [
        "#[cfg(",
        "cfg!(",
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window.",
        "document.",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect component source should stay platform-neutral without `{forbidden}`."
        );
    }
}

#[test]
fn native_select_respects_ui_headless_web_ssr_compile_error_mutex() {
    let native_select_cargo_source = load_source("../../components/native-select/Cargo.toml");
    let headless_cargo_source = load_source("../../crates/ui-headless/Cargo.toml");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");

    for required in [
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            native_select_cargo_source.contains(required)
                || headless_cargo_source.contains(required)
                || headless_lib_source.contains(required),
            "ui-headless web/ssr mutual-exclusion contract should include `{required}`."
        );
    }
}

#[test]
fn native_select_respects_ui_motion_non_wasm_noop_contract() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib_source.contains(required),
            "ui-motion non-wasm noop/stub contract should include `{required}`."
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion",
        "ui_motion::",
        "attach_motion(",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not assume component-level motion runtime path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_reduced_motion_ssr_wasm_contract_is_preserved() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/native_select.rs");
    let ui_motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib_source.contains(required),
            "ui-motion reduced-motion/non-wasm contract should include `{required}`."
        );
    }

    for required in [
        "id=move || format!(\"{}-root\", id_base.get_value())",
        "id=move || format!(\"{}-control\", id_base.get_value())",
        "id: format!(\"{id_base}-option-{index}\")",
        "use_native_select(NativeSelectOptions {",
        "aria-label=move || semantics.get().attrs.aria_label",
        "aria-invalid=move || semantics.get().attrs.aria_invalid",
    ] {
        assert!(
            view_source.contains(required) || primitive_source.contains(required),
            "NativeSelect SSR/wasm semantic parity should include `{required}`."
        );
    }

    for forbidden in ["#[cfg(", "web_sys::", "wasm_bindgen::", "js_sys::"] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not fork reduced-motion/SSR/wasm behavior in component source `{forbidden}`."
        );
    }
}

#[test]
fn native_select_view_macro_complexity_is_controlled_by_semantic_splitting() {
    let view_source = load_source("../../components/native-select/src/view.rs");

    for required in [
        "fn render_placeholder_option(",
        "fn render_native_select_option(",
        "fn render_static_indicator() -> impl IntoView",
        "let render_placeholder = move || {",
        "{render_placeholder}",
        "<For",
        "children=render_native_select_option",
        "{render_static_indicator()}",
        "on:change=on_change",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect view-macro complexity control should include `{required}`."
        );
    }

    for forbidden in [
        "children=move |option| {",
        ".map(|placeholder| {",
        "let render_option =",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "NativeSelect should avoid inline repeated nested view fragments `{forbidden}`."
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert_eq!(
        view_macro_count, 4,
        "NativeSelect should keep a bounded number of `view!` expansions (4)."
    );
}

#[test]
fn native_select_functional_fragment_split_prefers_plain_functions() {
    let view_source = load_source("../../components/native-select/src/view.rs");

    for required in [
        "fn render_placeholder_option(placeholder_label: String, is_required: bool) -> impl IntoView",
        "fn render_native_select_option(option: crate::NativeSelectOptionResolved) -> impl IntoView",
        "render_placeholder_option(placeholder_label, is_required)",
        "children=render_native_select_option",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect should prefer plain function split for lightweight UI fragment `{required}`."
        );
    }

    for forbidden in ["#[component]\nfn render_", "#[component]\r\nfn render_"] {
        assert!(
            !view_source.contains(forbidden),
            "NativeSelect fragment split should not introduce component-level abstraction noise `{forbidden}`."
        );
    }
}

#[test]
fn native_select_static_fragment_is_constantized_with_stable_a11y_contract() {
    let view_source = load_source("../../components/native-select/src/view.rs");

    for required in [
        "const NATIVE_SELECT_INDICATOR_SYMBOL: &str = \"▾\";",
        "fn render_static_indicator() -> impl IntoView",
        "data-slot=\"native-select-indicator\"",
        "aria-hidden=\"true\"",
        "{NATIVE_SELECT_INDICATOR_SYMBOL}",
        "{render_static_indicator()}",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect static fragment constantization contract should include `{required}`."
        );
    }

    assert!(
        !view_source.contains("\n                \"▾\"\n"),
        "NativeSelect should avoid scattering static indicator literal directly in root view tree."
    );
}

#[test]
fn native_select_inner_html_contract_disallows_injection_surface() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "insert_adjacent_html",
        "innerHTML",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should not expose HTML injection surface `{forbidden}`."
        );
    }

    for required in [
        "{placeholder_label}",
        "{option.label}",
        "{NATIVE_SELECT_INDICATOR_SYMBOL}",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect should keep text rendering path via `{required}`."
        );
    }
}

#[test]
fn native_select_wasm_debug_contract_tracks_state_and_keeps_api_clean() {
    let view_source = load_source("../../components/native-select/src/view.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");
    let cargo_source = load_source("../../components/native-select/Cargo.toml");
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for required in [
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-selection-mode=if is_controlled {",
        "data-selection-source=if is_controlled {",
        "data-change-source=move || selection_change_source_attr.get()",
        "let current_selected_index = selected_index.get();",
        "let previous_selected_index = last_selected_index.get_untracked();",
        "let on_change = move |ev: ev::Event| {",
        "resolve_native_select_change_index(&next_value, &resolved_options.get_untracked())",
        "request_selected_index_change.run(next_index);",
        "Interactive Playground",
    ] {
        assert!(
            view_source.contains(required) || docs_source.contains(required),
            "NativeSelect wasm debug contract should include `{required}`."
        );
    }

    for required in ["[features]", "default = []"] {
        assert!(
            cargo_source.contains(required),
            "NativeSelect cargo feature surface should include `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] debug",
        "#[prop(optional)] debug_trace",
        "feature = \"debug\"",
        "feature = \"wasm-debug\"",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should keep debug hooks out of public API surface `{forbidden}`."
        );
    }
}

#[test]
fn native_select_performance_budget_contract_is_guarded_without_render_count_harness() {
    let module_source = load_source("../../components/native-select/src/mod.rs");
    let logic_source = load_source("../../components/native-select/src/logic.rs");
    let view_source = load_source("../../components/native-select/src/view.rs");
    let styles_source = load_source("../../components/native-select/src/styles.rs");

    for required in [
        "let resolved_options =",
        "Signal::derive(move || logic::resolve_options(&id_base.get_value(), &options.get_value()))",
        "let resolved_states = Signal::derive(move || {",
        "let selected_value =",
        "Signal::derive(move || logic::resolve_control_value(state.get().selected_value.as_deref()))",
        "let on_change = move |ev: ev::Event| {",
        "resolve_native_select_change_index(&next_value, &resolved_options.get_untracked())",
        "on:change=on_change",
        "request_selected_index_change.run(next_index);",
    ] {
        assert!(
            view_source.contains(required),
            "NativeSelect performance budget baseline should include `{required}`."
        );
    }

    let effect_count = view_source.matches("Effect::new(move |_| {").count();
    assert_eq!(
        effect_count, 2,
        "NativeSelect should keep effect count stable (2) to avoid accidental reactive churn."
    );

    for forbidden in [
        "on:input=",
        "on:mousemove=",
        "on:pointermove=",
        "on:touchmove=",
        "request_animation_frame",
        "set_interval",
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "spawn_local(",
        "tokio::",
        "mod motion;",
        "ui_motion::",
    ] {
        assert!(
            !module_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "NativeSelect should avoid high-frequency/event-flood perf path `{forbidden}`."
        );
    }
}

#[test]
fn native_select_emits_baseline_root_state_data_attributes() {
    let source = load_source("../../components/native-select/src/view.rs");

    for needle in [
        "data-slot=\"native-select\"",
        "data-state=move || state.get().data_state_attr",
        "data-size=move || state.get().size_attr",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-options=move || state.get().has_options.then_some(\"true\")",
        "data-option-count=move || state.get().option_count.to_string()",
        "data-selected-index=move || state.get().selected_index.map(|index| index.to_string())",
        "data-selected-value=move || state.get().selected_value.clone()",
        "data-has-selection=move || state.get().has_selection.then_some(\"true\")",
        "data-disabled=move || state.get().control_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-has-placeholder=move || state.get().has_placeholder.then_some(\"true\")",
        "data-has-disabled-options=move || state.get().has_disabled_options.then_some(\"true\")",
        "data-has-enabled-options=move || state.get().has_enabled_options.then_some(\"true\")",
        "data-disabled-option-count=move || state.get().disabled_option_count.to_string()",
        "data-slot=move || semantics.get().attrs.data_slot",
        "data-slot=\"native-select-indicator\"",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect should set `{needle}` for baseline-compatible selectors and state inspection."
        );
    }
}

#[test]
fn native_select_styles_include_size_invalid_disabled_and_empty_markers() {
    let source = load_source("../../components/native-select/src/styles.rs");

    for needle in [
        ".ui-native-select {",
        ".ui-native-select__control {",
        ".ui-native-select__indicator {",
        ".ui-native-select--size-sm .ui-native-select__control",
        ".ui-native-select--size-md .ui-native-select__control",
        ".ui-native-select--size-lg .ui-native-select__control",
        ".ui-native-select--invalid .ui-native-select__control",
        ".ui-native-select--disabled .ui-native-select__control",
        ".ui-native-select--empty .ui-native-select__control",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect styles should include `{needle}` for stable visual state contracts."
        );
    }
}

#[test]
fn native_select_styles_consume_ui_theme_tokens_only() {
    let source = load_source("../../components/native-select/src/styles.rs");

    for needle in [
        "var(--ui-border)",
        "var(--ui-radius-md)",
        "var(--ui-bg)",
        "var(--ui-fg)",
        "var(--ui-shadow-sm)",
        "var(--ui-focus-ring)",
        "var(--ui-danger)",
        "var(--ui-accent)",
        "var(--ui-fg-muted)",
        "var(--ui-bg-muted)",
    ] {
        assert!(
            source.contains(needle),
            "NativeSelect styles should consume ui-theme token `{needle}`."
        );
    }

    let mut cursor = source.as_str();
    while let Some(start) = cursor.find("var(--") {
        let tail = &cursor[start + 6..];
        let end = tail.find([',', ')']).unwrap_or(tail.len());
        let token = tail[..end].trim();
        assert!(
            token.starts_with("ui-"),
            "NativeSelect styles should not introduce non-ui token namespace `{token}`."
        );
        cursor = &tail[end..];
    }
}

#[test]
fn native_select_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");

    for needle in [
        "pub(super) fn native_select() -> AnyView",
        "title=\"NativeSelect\"",
        "slug=\"native-select\"",
        "description=\"baseline-style native `<select>` wrapper with controllable selection, root `data-*` contracts, and stable option normalization.\"",
        "<Playground title=\"Hello World (Uncontrolled)\" code_signal=hello_code>",
        "<Playground title=\"Controlled + Placeholder\" code_signal=code>",
        "<Playground title=\"Required + Invalid + Disabled\" code_signal=states_code>",
        "<NativeSelect",
    ] {
        assert!(
            source.contains(needle),
            "forms_native docs should include `{needle}` for native-select primary playground coverage.",
        );
    }
}

#[test]
fn native_select_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/forms_native.rs");

    for needle in [
        "title=\"Controlled + Placeholder\"",
        "id_base=\"docs-native-select-controlled\".to_string()",
        "placeholder=\"Choose mode\".to_string()",
        "name=\"mode\".to_string()",
        "selected_index=selected_signal",
        "on_selected_index_change=on_selected_change",
        "title=\"Required + Invalid + Disabled\"",
        "id_base=\"docs-native-select-required\".to_string()",
        "default_selected_index=1",
        "is_required=true",
        "is_invalid=true",
        "size=NativeSelectSize::Lg",
        "class_name=\"docs-native-select-custom\".to_string()",
        "id_base=\"docs-native-select-disabled\".to_string()",
        "is_disabled=true",
        "placeholder=\"Disabled select\".to_string()",
        "size=NativeSelectSize::Sm",
    ] {
        assert!(
            source.contains(needle),
            "native-select docs playgrounds should contain `{needle}`.",
        );
    }
}
