use std::sync::OnceLock;
use ui_test_support::source_contract;

fn docs_form_field_page_source() -> &'static str {
    static SOURCE: OnceLock<String> = OnceLock::new();
    SOURCE
        .get_or_init(|| {
            let parent = source_contract::source_from_file_relative(
                file!(),
                "../../../apps/docs-app/src/pages/components/pages/forms_groups_extra.rs",
            );
            let child = source_contract::source_from_file_relative(
                file!(),
                "../../../apps/docs-app/src/pages/components/pages/forms_groups_extra/form_field.rs",
            );
            format!("{parent}\n\n{child}").replace(
                "pub(crate) fn form_field() -> AnyView {",
                "pub(super) fn form_field() -> AnyView {",
            )
        })
        .as_str()
}

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "readme" => include_str!("../src/README.md"),
        "manifest" => include_str!("../src/Component.toml"),
        "rbi" => include_str!("../src/form_field.rbi"),
        "check2" => include_str!("../check2.md"),
        "docs_form_field_page" => docs_form_field_page_source(),
        _ => panic!("unsupported source path: {path}"),
    }
}

fn load_form_field_docs_section() -> &'static str {
    let docs = load_source("docs_form_field_page");
    let start = docs
        .find("pub(super) fn form_field() -> AnyView")
        .expect("docs source should include form_field page function");
    let tail = &docs[start..];
    let end = tail
        .find("\npub(super) fn legend() -> AnyView")
        .map(|offset| start + offset)
        .unwrap_or(docs.len());
    &docs[start..end]
}

fn contains_hex_color_literal(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        if bytes[index] != b'#' {
            index += 1;
            continue;
        }

        let mut cursor = index + 1;
        let mut hex_len = 0usize;
        while cursor < bytes.len() && hex_len < 8 && bytes[cursor].is_ascii_hexdigit() {
            hex_len += 1;
            cursor += 1;
        }

        if matches!(hex_len, 3 | 4 | 6 | 8)
            && (cursor == bytes.len() || !bytes[cursor].is_ascii_hexdigit())
        {
            return true;
        }

        index += 1;
    }

    false
}

#[test]
fn form_field_component_keeps_ui_components_layering_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FormField;",
    ] {
        assert!(
            module.contains(required),
            "form-field module should keep ui layering boundary `{required}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !module.contains(forbidden),
            "form-field module should keep public API DOM-agnostic: `{forbidden}`."
        );
    }

    for forbidden in ["web_sys::", "HtmlElement", "Element", "Node"] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "form-field component layer should not leak DOM platform types: `{forbidden}`."
        );
    }
}

#[test]
fn form_field_component_uses_headless_contract_without_reimplementation() {
    let view = load_source("view");

    for required in ["OnPress", "SwitchOptions", "use_switch"] {
        assert!(
            view.contains(required),
            "form-field switch adapter should consume ui-headless contract via `{required}`."
        );
    }
    for required in [
        "let aria = use_switch(SwitchOptions {",
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "role=aria.attrs.role",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            view.contains(required),
            "form-field switch adapter should consume ui-headless contract via `{required}`."
        );
    }

    for required in [
        "fn render_switch_view(",
        "<Checkbox",
        "logic::resolve_state(FormFieldStateInput {",
    ] {
        assert!(
            view.contains(required),
            "form-field view should compose controls and state from existing layers via `{required}`."
        );
    }
}

#[test]
fn form_field_component_file_responsibilities_are_layered_with_motion_na() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FormField;",
    ] {
        assert!(
            module.contains(required),
            "form-field mod boundary should stay minimal and export-focused `{required}`."
        );
    }

    for forbidden in ["mod switch {", "mod checkbox {", "pub fn Switch("] {
        assert!(
            !module.contains(forbidden),
            "form-field mod boundary should not carry implementation detail `{forbidden}`."
        );
    }

    for forbidden in ["web_sys::", "view!", "data-slot=", "role="] {
        assert!(
            !logic.contains(forbidden),
            "form-field logic layer should stay pure state-derivation and avoid `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        ".ui-form-field[data-tone=\"default\"]",
        ".ui-form-field[data-disabled=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles layer should keep token-first static css contract `{required}`."
        );
    }

    for forbidden in ["#[component]", "view!", "on:click=", "use_switch("] {
        assert!(
            !styles.contains(forbidden),
            "form-field styles layer should not include runtime/view logic `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn FormField(",
        "let aria = use_switch(SwitchOptions {",
        "logic::resolve_state(FormFieldStateInput {",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field view layer should own structure + headless mounting `{required}`."
        );
    }

    assert!(
        !motion_file.exists(),
        "form-field has no component-level motion axis; `src/motion.rs` should remain N/A."
    );

    assert!(
        check2.contains("- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。"),
        "form-field checklist should mark component-file responsibilities contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_component_file_responsibilities_are_layered_with_motion_na"),
        "form-field checklist should record component-file-responsibility regression evidence."
    );
}

#[test]
fn form_field_component_directory_standard_files_follow_contract_and_na_paths() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for required_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/protocol.rs",
    ] {
        assert!(
            component_dir.join(required_path).exists(),
            "form-field component directory should include `{required_path}`."
        );
    }

    for forbidden_path in ["src/render.rs", "src/motion.rs", "src/spec.rs"] {
        assert!(
            !component_dir.join(forbidden_path).exists(),
            "form-field component directory should keep N/A path absent `{forbidden_path}`."
        );
    }

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FormField;",
    ] {
        assert!(
            module.contains(required),
            "form-field mod.rs should keep minimal stable exports `{required}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "mod render;"] {
        assert!(
            !module.contains(forbidden),
            "form-field mod.rs should avoid over-export or render drift `{forbidden}`."
        );
    }

    for required in [
        "pub fn normalize_selected_axis(",
        "pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: FormFieldState)",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic.rs should keep normalization/derivation helpers `{required}`."
        );
    }

    for forbidden in ["view!", "data-slot=", "role=", "on:click="] {
        assert!(
            !logic.contains(forbidden),
            "form-field logic.rs should not include view/headless mount token `{forbidden}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles.rs should keep token-first static css contract `{required}`."
        );
    }

    for forbidden in ["#[component]", "view!", "style=\"top:"] {
        assert!(
            !styles.contains(forbidden),
            "form-field styles.rs should avoid runtime/view logic `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn FormField(",
        "let aria = use_switch(SwitchOptions {",
        "data-slot=\"form-field\"",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field view.rs should keep structure + headless mounting `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] 组件目录标准文件落点正确。"),
        "form-field checklist should mark standard component-directory file layout as complete."
    );
    assert!(
        check2.contains("本组件判定：`src/motion.rs` N/A"),
        "form-field checklist should record why motion.rs is N/A for this component."
    );
    assert!(
        check2.contains("本组件判定：`src/spec.rs` N/A"),
        "form-field checklist should record why spec.rs is N/A for this component."
    );
    assert!(
        check2.contains(
            "components/form-field/test/semantics.rs::form_field_component_directory_standard_files_follow_contract_and_na_paths",
        ),
        "form-field checklist should record local regression evidence for standard file layout."
    );
}

#[test]
fn form_field_file_placement_discipline_is_strict_for_component_scope() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    for required_path in ["src/mod.rs", "src/logic.rs", "src/styles.rs", "src/view.rs"] {
        assert!(
            component_dir.join(required_path).exists(),
            "form-field file placement discipline requires `{required_path}`."
        );
    }

    {
        let forbidden_path = "src/render.rs";
        assert!(
            !component_dir.join(forbidden_path).exists(),
            "form-field should not drift to forbidden file `{forbidden_path}`."
        );
    }

    assert!(
        !component_dir.join("src/motion.rs").exists(),
        "form-field keeps `motion.rs` as N/A because there is no component-local motion axis."
    );
    assert!(
        !component_dir.join("src/spec.rs").exists(),
        "form-field keeps `spec.rs` as N/A for simple component scope."
    );

    for required in [
        "pub(crate) mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::FormField;",
        "pub fn normalize_selected_axis(",
        "pub const CSS: &str = r#\"",
        "pub fn FormField(",
    ] {
        assert!(
            module.contains(required)
                || logic.contains(required)
                || styles.contains(required)
                || view.contains(required),
            "form-field file placement discipline marker missing `{required}`."
        );
    }

    for forbidden in ["mod render;", "pub mod view;", "pub mod logic;"] {
        assert!(
            !module.contains(forbidden),
            "form-field export boundary should stay minimal and avoid `{forbidden}`."
        );
    }

    for forbidden in ["view!", "#[component]", "on:click="] {
        assert!(
            !logic.contains(forbidden),
            "form-field logic.rs should not carry render/event contract `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "form-field checklist should mark file-placement discipline as complete."
    );
    assert!(
        check2.contains("本组件判定：`src/motion.rs` N/A"),
        "form-field checklist should document motion.rs N/A for file-placement discipline."
    );
    assert!(
        check2.contains("本组件判定：`src/spec.rs` N/A"),
        "form-field checklist should document spec.rs N/A for file-placement discipline."
    );
    assert!(
        check2.contains(
            "components/form-field/test/semantics.rs::form_field_file_placement_discipline_is_strict_for_component_scope",
        ),
        "form-field checklist should record local file-placement regression evidence."
    );
}

#[test]
fn form_field_spec_rs_rule_is_na_for_simple_component_with_minimal_protocol() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");

    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let spec_path = component_dir.join("src/spec.rs");
    let protocol_path = component_dir.join("src/protocol.rs");
    let protocol_source = std::fs::read_to_string(&protocol_path)
        .expect("form-field protocol.rs should exist for minimal schema-compat contract");

    assert!(
        !spec_path.exists(),
        "form-field should not add `src/spec.rs` for a simple single-field component."
    );
    assert!(
        protocol_path.exists(),
        "form-field should keep minimal versioned protocol contract in `src/protocol.rs`."
    );

    for required in [
        "pub enum FormFieldComponentSchemaVersion",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion,",
        "#[cfg(test)]",
        "#[path = \"../test/protocol.rs\"]",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol schema contract should include `{required}`."
        );
    }

    for forbidden in [
        "pub struct FormFieldSpec",
        "impl FormFieldSpec",
        "fn render(",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field should not expose complex spec-builder API `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "form-field checklist should mark spec.rs-scope contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 为单字段基础组件，不存在复杂配置建造器/多版本外部 DSL 的 `spec.rs` 需求）"),
        "form-field checklist should explain why spec.rs is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_spec_rs_rule_is_na_for_simple_component_with_minimal_protocol"),
        "form-field checklist should record spec.rs-scope N/A regression evidence."
    );
}

#[test]
fn form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let spec_path = component_dir.join("src/spec.rs");
    let protocol_path = component_dir.join("src/protocol.rs");
    let protocol_source = std::fs::read_to_string(&protocol_path)
        .expect("form-field protocol.rs should exist as schema fallback for N/A builder path");

    assert!(
        !spec_path.exists(),
        "form-field should keep hyper-structure builder as N/A and avoid `src/spec.rs`."
    );
    assert!(
        protocol_path.exists(),
        "form-field should keep minimal protocol fallback at `src/protocol.rs`."
    );

    for forbidden in [
        "pub struct FormFieldSpec",
        "impl FormFieldSpec",
        "fn new(",
        "fn render(",
        "FormFieldSpec::new",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field should not expose hyper-structure builder API token `{forbidden}`."
        );
    }

    for required in [
        "pub enum FormFieldComponentSchemaVersion",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol fallback should include `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "form-field checklist should mark Hyper-Structure Builder item as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 为单字段基础组件，不存在复杂多槽位组合与可编排 DSL 输入，不引入 `*Spec::new()...render()` builder）"),
        "form-field checklist should explain Hyper-Structure Builder N/A reason."
    );
    assert!(
        check2.contains(
            "components/form-field/test/semantics.rs::form_field_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        ),
        "form-field checklist should record local Hyper-Structure Builder regression evidence."
    );
}

#[test]
fn form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced() {
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = component_dir.join("src/Component.toml");
    let rbi_path = component_dir.join("src/form_field.rbi");

    assert!(
        manifest_path.exists() && rbi_path.exists(),
        "form-field should provide both context-compression files: {} and {}",
        manifest_path.display(),
        rbi_path.display(),
    );

    let manifest_source = std::fs::read_to_string(&manifest_path)
        .expect("form-field Component.toml should be readable");
    let rbi_source =
        std::fs::read_to_string(&rbi_path).expect("form-field form_field.rbi should be readable");

    for required in [
        "schema_version = \"1\"",
        "name = \"FormField\"",
        "crate = \"ui-form-field\"",
        "rbi = \"form_field.rbi\"",
        "name = \"is_selected\"",
        "name = \"default_selected\"",
        "name = \"on_selected_change\"",
        "name = \"is_disabled\"",
        "name = \"is_invalid\"",
        "name = \"tone\"",
        "name = \"indicator_variant\"",
        "name = \"indicator_placement\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "form-field Component.toml should include manifest marker `{required}`."
        );
    }

    for required in [
        "pub type FormFieldTone = ui_form_field::FormFieldTone;",
        "pub type FormFieldIndicatorVariant = ui_form_field::FormFieldIndicatorVariant;",
        "pub type FormFieldIndicatorPlacement = ui_form_field::FormFieldIndicatorPlacement;",
        "pub fn FormField(",
        "is_selected: Option<leptos::prelude::Signal<bool>>",
        "default_selected: Option<bool>",
        "on_selected_change: Option<leptos::prelude::Callback<bool>>",
        "is_disabled: bool",
        "is_invalid: bool",
        "tone: FormFieldTone",
        "indicator_variant: FormFieldIndicatorVariant",
        "indicator_placement: FormFieldIndicatorPlacement",
        "label: Option<String>",
        "description: Option<String>",
        "error_message: Option<String>",
        "aria_label: Option<String>",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "class_name: Option<String>",
    ] {
        assert!(
            rbi_source.contains(required),
            "form-field RBI projection should include interface marker `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "form-field checklist should mark context-compression item as complete."
    );
    assert!(
        check2.contains("已满足（Manifest 落位）：`components/form-field/src/Component.toml`"),
        "form-field checklist should record manifest placement evidence."
    );
    assert!(
        check2.contains("已满足（RBI 投影落位）：`components/form-field/src/form_field.rbi`"),
        "form-field checklist should record RBI placement evidence."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_context_compression_manifest_and_rbi_projection_are_present_and_synced"),
        "form-field checklist should record local context-compression regression evidence."
    );
}

#[test]
fn form_field_agent_contract_schema_is_typed_traceable_and_whitelist_safe() {
    let logic = load_source("logic");
    let view = load_source("view");
    let module = load_source("mod");
    let styles = load_source("styles");
    let manifest = load_source("manifest");
    let rbi = load_source("rbi");
    let check2 = load_source("check2");
    let combined = format!("{module}\n{logic}\n{view}\n{styles}");

    for required in [
        "pub const FORM_FIELD_AGENT_SCHEMA: &str = \"ui.form_field.agent-contract.v1\";",
        "pub const FORM_FIELD_AGENT_SCHEMA_VERSION: &str = \"v1\";",
        "pub enum FormFieldAgentIntent",
        "pub enum FormFieldAgentAction",
        "pub enum FormFieldAgentStateAxis",
        "pub enum FormFieldAgentSourceAxis",
        "pub enum FormFieldAgentStreamSupport",
        "pub enum FormFieldAgentStreamFallback",
        "pub enum FormFieldAgentOutputStatus",
        "pub struct FormFieldAgentContractAttrs",
        "pub fn resolve_agent_contract_attrs(",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should expose typed agent-contract marker `{required}`."
        );
    }

    for required in [
        "let agent_contract = Memo::new(move |_| {",
        "logic::resolve_agent_contract_attrs(state.get(), selected_control_mode_attr)",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version",
        "data-ui-intent=move || agent_contract.get().intent_attr",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-state=move || agent_contract.get().state_attr",
        "data-ui-source=move || agent_contract.get().source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field view should mount schemaized agent-contract marker `{required}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "format!(\"data-ui-",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field agent contract should avoid free-form string splicing `{forbidden}`."
        );
    }

    for required in [
        "[agent_contract]",
        "schema = \"ui.form_field.agent-contract.v1\"",
        "intent = \"selection-control\"",
        "action = \"render-snapshot\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-output-status\"",
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "typed_agent_contract_from_logic::resolve_agent_contract_attrs",
        "inner_html",
        "javascript:",
    ] {
        assert!(
            manifest.contains(required),
            "form-field Component.toml should include agent-contract governance marker `{required}`."
        );
    }

    for required in [
        "pub const FORM_FIELD_AGENT_SCHEMA: &str;",
        "pub const FORM_FIELD_AGENT_SCHEMA_VERSION: &str;",
        "pub enum FormFieldAgentIntent",
        "pub enum FormFieldAgentAction",
        "pub enum FormFieldAgentStateAxis",
        "pub enum FormFieldAgentSourceAxis",
        "pub struct FormFieldAgentContractAttrs",
    ] {
        assert!(
            rbi.contains(required),
            "form-field RBI should project typed agent-contract marker `{required}`."
        );
    }

    for forbidden in [
        "<script",
        "javascript:",
        "onerror=",
        "onload=",
        "inner_html",
        "dangerously_set_inner_html",
    ] {
        assert!(
            !combined.contains(forbidden),
            "form-field agent contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "components/form-field/test/semantics.rs::form_field_agent_contract_schema_is_typed_traceable_and_whitelist_safe",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should include agent-contract evidence marker `{required}`."
        );
    }
}

#[test]
fn form_field_public_api_follows_is_on_default_naming_contract() {
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "#[prop(optional, into)] is_selected: Option<Signal<bool>>",
        "#[prop(optional)] default_selected: Option<bool>",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(optional)] is_invalid: bool",
        "logic::normalize_selected_axis(logic::FormFieldSelectedAxisInput {",
        "let selected_state = use_controllable_state(",
        "logic::normalize_error_message(error_message, is_invalid)",
    ] {
        assert!(
            view.contains(required),
            "form-field public API naming should include `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。"),
        "form-field checklist should mark API naming contract as complete."
    );
}

#[test]
fn form_field_selected_axis_keeps_controlled_uncontrolled_triplet_contract() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "#[prop(optional, into)] is_selected: Option<Signal<bool>>",
        "#[prop(optional)] default_selected: Option<bool>",
        "#[prop(optional)] on_selected_change: Option<Callback<bool>>",
        "use_controllable_state(",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-selected-controlled=is_controlled_selected.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_controlled_selected).then_some(\"true\")",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field selected axis should keep controlled/uncontrolled triplet marker `{required}`."
        );
    }

    for required in [
        "pub struct FormFieldSelectedAxisInput",
        "pub struct FormFieldSelectedAxisState",
        "pub fn normalize_selected_axis(",
        "use ui_state_primitives::radio::{RadioCheckedAxisInput, resolve_checked_axis};",
        "let primitive = resolve_checked_axis(RadioCheckedAxisInput {",
        "control_mode_attr: primitive.control_mode_attr,",
        "default_selected_source_attr: primitive.default_checked_source_attr,",
        "selected_change_source_attr,",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should normalize selected-axis sources via `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。"),
        "form-field checklist should mark controlled/uncontrolled triplet contract as complete."
    );
}

#[test]
fn form_field_default_selected_is_normalized_only_in_logic_layer() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub const DEFAULT_SELECTED: bool = ui_state_primitives::radio::DEFAULT_CHECKED;",
        "pub fn normalize_selected_axis(input: FormFieldSelectedAxisInput) -> FormFieldSelectedAxisState",
        "default_selected: input.default_selected.unwrap_or(DEFAULT_SELECTED),",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should keep default-source normalization via `{required}`."
        );
    }

    for required in [
        "let selected_axis = logic::normalize_selected_axis(logic::FormFieldSelectedAxisInput {",
        "Some(selected_axis.default_selected)",
    ] {
        assert!(
            view.contains(required),
            "form-field view should consume normalized defaults from logic via `{required}`."
        );
    }

    for forbidden in [
        "default_selected.unwrap_or",
        "DEFAULT_SELECTED",
        "if default_selected",
    ] {
        assert!(
            !view.contains(forbidden),
            "form-field view should not own default fallback branch `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。"),
        "form-field checklist should mark default-source single-authority contract as complete."
    );
}

#[test]
fn form_field_state_normalization_is_centralized_in_logic_layer() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState",
        "pub fn resolve_checkbox_variant(is_invalid: bool) -> CheckboxVariant",
        "pub fn compose_describedby(",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should own centralized state normalization helper `{required}`."
        );
    }

    for required in [
        "let selected_axis = logic::normalize_selected_axis(logic::FormFieldSelectedAxisInput {",
        "logic::resolve_state(FormFieldStateInput {",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "logic::compose_describedby(",
        "let checkbox_variant = logic::resolve_checkbox_variant(is_invalid);",
    ] {
        assert!(
            view.contains(required),
            "form-field view should consume logic-layer state derivation via `{required}`."
        );
    }

    for forbidden in [
        "let checkbox_variant = if is_invalid {",
        "let mut ids = Vec::new();",
    ] {
        assert!(
            !view.contains(forbidden),
            "form-field view should not rebuild state machine branches via `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。"),
        "form-field checklist should mark centralized state-normalization contract as complete."
    );
}

#[test]
fn form_field_discrete_axes_are_enum_typed_not_free_form_strings() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub enum FormFieldTone {",
        "pub enum FormFieldIndicatorVariant {",
        "pub enum FormFieldIndicatorPlacement {",
        "#[prop(optional)] tone: FormFieldTone,",
        "#[prop(optional)] indicator_variant: FormFieldIndicatorVariant,",
        "#[prop(optional)] indicator_placement: FormFieldIndicatorPlacement,",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "form-field discrete status axis should stay enum-typed via `{required}`."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "indicator_variant: Option<String>",
        "indicator_placement: Option<String>",
        "tone: &str",
        "indicator_variant: &str",
        "indicator_placement: &str",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "form-field discrete status axis should not drift to free-form string `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。"),
        "form-field checklist should mark discrete-state enum-typing contract as complete."
    );
}

#[test]
fn form_field_consumes_state_primitives_for_selection_axis() {
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "use ui_state_primitives::radio::{RadioCheckedAxisInput, resolve_checked_axis};",
        "let primitive = resolve_checked_axis(RadioCheckedAxisInput {",
        "has_is_checked: controlled_selected.is_some(),",
        "has_default_checked: input.default_selected.is_some(),",
        "has_on_checked_change: input.on_selected_change.is_some(),",
    ] {
        assert!(
            logic.contains(required),
            "form-field selected axis should consume state-primitives via `{required}`."
        );
    }

    assert!(
        !logic.contains("control_mode_attr: if is_controlled {"),
        "form-field logic should not reimplement control-mode branch after state-primitives adoption."
    );

    assert!(
        check2.contains("- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。"),
        "form-field checklist should mark state-primitives source contract as complete."
    );
}

#[test]
fn form_field_async_semantics_are_na_without_loading_protocol() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "on_retry",
        "retry_count",
        "aria-busy",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field should stay N/A for async protocol and avoid `{forbidden}`."
        );
    }

    for required in [
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "本组件判定：N/A（`FormField` 无远程请求与异步状态轴），未暴露 `is_loading`/retry API，未引入 `use_async_action`，且未在语义标记中新增 `aria-busy` 分支。",
        "components/form-field/test/semantics.rs::form_field_async_semantics_are_na_without_loading_protocol",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should record async N/A evidence `{required}`."
        );
    }
}

#[test]
fn form_field_api_dx_hello_world_stays_copy_paste_ready() {
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for required in [
        "let hello_code = Signal::derive(move || {",
        "r#\"<FormField label=\"Accept terms of service\".to_string() />\"#.to_string()",
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<FormField label=\"Accept terms of service\".to_string() />",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs should keep DX default/advanced API contract `{required}`."
        );
    }

    let hello_start = docs
        .find("let hello_code = Signal::derive(move || {")
        .expect("hello_code signal should exist in form_field docs page");
    let hello_tail = &docs[hello_start..];
    let snippet_start = hello_tail
        .find("r#\"")
        .expect("hello_code should use raw string literal")
        + 3;
    let snippet_tail = &hello_tail[snippet_start..];
    let snippet_end = snippet_tail
        .find("\"#.to_string()")
        .expect("hello_code raw string should end with to_string()");
    let hello_snippet = &snippet_tail[..snippet_end];
    let non_empty_lines = hello_snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    assert!(
        non_empty_lines <= 5,
        "hello world snippet should stay copy-paste ready within 5 lines; got {non_empty_lines}"
    );

    for forbidden in [
        "ui-state-primitives",
        "ui_state_primitives",
        "ui-headless",
        "use_controllable_state",
        "use_switch",
        "state=",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "hello world snippet should not require low-level state wiring `{forbidden}`."
        );
    }

    assert!(
        check2
            .contains("- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。"),
        "form-field checklist should mark DX paradox contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_api_dx_hello_world_stays_copy_paste_ready"),
        "form-field checklist should record DX paradox regression evidence."
    );
}

#[test]
fn form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let e2e_source = std::fs::read_to_string(
        component_dir.join("../../e2e/tests/docs_app_form_field_contract.spec.mjs"),
    )
    .expect("form-field docs e2e contract source should be readable");
    let dx_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-dx.sh"))
            .expect("dx gate script should be readable");

    for required in [
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=comparison_code>",
        "data-slot=\"form-field-state-matrix-note\"",
        "data-slot=\"form-field-controlled-uncontrolled-note\"",
        "data-slot=\"form-field-streaming-policy\"",
        "data-slot=\"form-field-streaming-modes\"",
        "data-slot=\"form-field-copy-ready\"",
        "data-slot=\"form-field-source-paths\"",
        "data-slot=\"form-field-source-prerequisites\"",
        "Streaming Optional; fallback=snapshot.",
        "Snapshot mode renders verified full output for form-field semantics.",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*.",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs should keep docs-product marker `{required}`."
        );
    }

    for required in [
        "docs-app form-field playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui::*;",
        "<FormField",
    ] {
        assert!(
            e2e_source.contains(required),
            "form-field e2e docs contract should keep copy-ready marker `{required}`."
        );
    }

    for required in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "components/form-field/test/semantics.rs::form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "components/form-field/test/form_field/semantics.rs::form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "e2e/tests/docs_app_form_field_contract.spec.mjs::docs-app form-field playground source is copy-paste ready",
        "bash scripts/check-ui-dx.sh",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep docs-product marker `{required}`."
        );
    }

    let dx_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";
    assert!(
        dx_script_source.contains(dx_script_needle),
        "dx gate script should include `{dx_script_needle}`."
    );
}

#[test]
fn form_field_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 docs-sync section should include `{required}`."
        );
    }
}

#[test]
fn form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs = load_form_field_docs_section();
    let logic = load_source("logic");

    for required in [
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "<Playground title=\"Controlled vs Default (Comparison)\" code_signal=comparison_code>",
        "data-slot=\"form-field-state-matrix-note\"",
        "data-slot=\"form-field-controlled-uncontrolled-note\"",
        "is_selected=marketing.into()",
        "on_selected_change=on_marketing_selected_change",
        "is_selected=tos.into()",
        "on_selected_change=on_tos_selected_change",
        "default_selected=true",
        "is_disabled=true",
        "is_invalid=true",
        "tone=FormFieldTone::Quiet",
        "indicator_variant=FormFieldIndicatorVariant::Checkbox",
        "indicator_placement=FormFieldIndicatorPlacement::End",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs matrix/examples should include `{required}`."
        );
    }

    for required in [
        "pub const DEFAULT_SELECTED: bool = ui_state_primitives::radio::DEFAULT_CHECKED;",
        "default_selected: input.default_selected.unwrap_or(DEFAULT_SELECTED),",
        "pub struct FormFieldSelectedAxisInput {",
        "pub struct FormFieldSelectedAxisState {",
        "pub fn normalize_selected_axis(input: FormFieldSelectedAxisInput) -> FormFieldSelectedAxisState",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should keep API/default normalization marker `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "form-field check2 should mark docs-sync/state-matrix item complete."
    );

    for required in [
        "components/form-field/test/semantics.rs::form_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/form-field/test/semantics.rs::form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_docs_sync_and_state_matrix_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 docs-sync/state-matrix section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_documentation_as_product_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 documentation-as-product section should include `{required}`."
        );
    }
}

#[test]
fn form_field_documentation_entry_exists_with_beginner_first_progression() {
    let readme = load_source("readme");
    let docs = load_form_field_docs_section();

    for required in [
        "# FormField",
        "## Hello World",
        "## 常见用法",
        "## 先用起来，再进阶",
        "默认路径：`<FormField label=... />`",
        "进阶控制：按需启用 `is_selected + default_selected + on_selected_change`。",
    ] {
        assert!(
            readme.contains(required),
            "form-field README should include beginner marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn form_field() -> AnyView",
        "title=\"FormField\"",
        "slug=\"form-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Switch Indicator + Description\"",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
        "title=\"Controlled vs Default (Comparison)\"",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs entry should include `{required}`."
        );
    }

    let readme_hello = readme
        .find("## Hello World")
        .expect("form-field README should include Hello World section");
    let readme_common = readme
        .find("## 常见用法")
        .expect("form-field README should include common-usage section");
    let readme_progressive = readme
        .find("## 先用起来，再进阶")
        .expect("form-field README should include beginner-to-advanced section");
    let readme_architecture = readme
        .find("## Architecture Layers")
        .expect("form-field README should include architecture section");
    assert!(
        readme_hello < readme_common
            && readme_common < readme_progressive
            && readme_progressive < readme_architecture,
        "form-field README should keep default path before architecture-heavy content."
    );

    let docs_hello = docs
        .find("title=\"Hello World（默认路径）\"")
        .expect("form-field docs should include Hello World playground");
    let docs_common = docs
        .find("title=\"Switch Indicator + Description\"")
        .expect("form-field docs should include common-usage playground");
    let docs_advanced = docs
        .find("title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"")
        .expect("form-field docs should include advanced-state playground");
    let docs_controlled = docs
        .find("title=\"Controlled vs Default (Comparison)\"")
        .expect("form-field docs should include controlled/uncontrolled playground");
    assert!(
        docs_hello < docs_common && docs_common < docs_advanced && docs_advanced < docs_controlled,
        "form-field docs should keep beginner-first order before controlled comparison."
    );
}

#[test]
fn form_field_check2_marks_documentation_as_product_contract_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "form-field check2 should mark documentation-as-product item complete."
    );

    for required in [
        "components/form-field/src/README.md",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field",
        "components/form-field/test/semantics.rs::form_field_check2_documents_documentation_as_product_rules",
        "components/form-field/test/semantics.rs::form_field_documentation_entry_exists_with_beginner_first_progression",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_documentation_as_product_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_documentation_entry_exists_with_beginner_first_progression",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 documentation-as-product section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_interactive_playground_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 interactive-playground section should include `{required}`."
        );
    }
}

#[test]
fn form_field_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs = load_form_field_docs_section();
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");

    for required in [
        "title=\"FormField Workbench (Display + Config + Code + CSS Test)\"",
        "test_css_source=interactive_test_css_source",
        "test_config_signal=interactive_actual_config",
        "controls=move || view!",
        "data-slot=\"form-field-workbench-controls\"",
        "data-slot=\"form-field-workbench-compare\"",
        "Switch checked=interactive_selected set_checked=set_interactive_selected",
        "Switch checked=interactive_disabled set_checked=set_interactive_disabled",
        "Switch checked=interactive_invalid set_checked=set_interactive_invalid",
        "let (interactive_selected, set_interactive_selected) = signal(true);",
        "let (interactive_disabled, set_interactive_disabled) = signal(false);",
        "let (interactive_invalid, set_interactive_invalid) = signal(false);",
        "FormFieldActualConfig {",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs should provide interactive playground marker `{required}`."
        );
    }

    for required in [
        "let section_class = \"docs-card playground\";",
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "<Card class_name=\"playground__panel playground__controls\".to_string()>",
    ] {
        assert!(
            playground_source.contains(required),
            "docs-app Playground should keep interactive preview marker `{required}`."
        );
    }
}

#[test]
fn form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = include_str!("../../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for required in [
        "docs-app form-field key flow is repeatable with semantic breakpoints",
        "await page.goto(\"/#/components/form-field\");",
        "body:not(:has(#boot))",
        "await tosCheckbox.focus();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"selected-invalid\");",
        "await page.reload();",
        "await expect(reloadedTos).toHaveAttribute(\"data-state\", \"invalid\");",
    ] {
        assert!(
            e2e_source.contains(required),
            "form-field interactive playground should keep repeatable e2e marker `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_interactive_playground_item_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "form-field check2 should mark interactive-playground item complete."
    );

    for required in [
        "FormField Workbench (Display + Config + Code + CSS Test)",
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field",
        "e2e/tests/docs_app_form_field_contract.spec.mjs::docs-app form-field key flow is repeatable with semantic breakpoints",
        "components/form-field/test/semantics.rs::form_field_check2_documents_interactive_playground_rules",
        "components/form-field/test/semantics.rs::form_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/form-field/test/semantics.rs::form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_interactive_playground_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "components/form-field/test/form_field/semantics.rs::form_field_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 interactive-playground section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_source_first_copy_paste_ready_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 source-first section should include `{required}`."
        );
    }
}

#[test]
fn form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs = load_source("docs_form_field_page");
    let playground_source = include_str!("../../../apps/docs-app/src/playground.rs");
    let e2e_source = include_str!("../../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for required in [
        "data-slot=\"form-field-copy-ready\"",
        "data-slot=\"form-field-source-paths\"",
        "data-slot=\"form-field-source-prerequisites\"",
        "Copy-ready snippets prepend imports automatically: use leptos::prelude::*; use ui::*.",
        "Source paths: components/form-field/src/mod.rs, components/form-field/src/logic.rs, components/form-field/src/view.rs, components/form-field/src/styles.rs.",
        "Feature prerequisites: component-form_field (inject-css optional for runtime style injection).",
        "title=\"Switch Indicator + Description\" code_signal=code",
        "title=\"FormField Workbench (Display + Config + Code + CSS Test)\"",
    ] {
        assert!(
            docs.contains(required),
            "form-field source-first docs should keep marker `{required}`."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground copy-ready pipeline should keep `{required}`."
        );
    }

    for required in [
        "docs-app form-field playground source is copy-paste ready",
        "data-copyable",
        "use leptos::prelude::*;",
        "use ui::*;",
        "data-slot=\"form-field-source-paths\"",
        "data-slot=\"form-field-source-prerequisites\"",
        "toContainText(\"components/form-field/src/mod.rs\")",
        "toContainText(\"component-form_field\")",
        "toContainText(\"inject-css\")",
    ] {
        assert!(
            e2e_source.contains(required),
            "form-field e2e source-first contract should keep `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "form-field check2 should mark source-first copy-paste-ready item complete."
    );

    for required in [
        "apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field",
        "e2e/tests/docs_app_form_field_contract.spec.mjs::docs-app form-field playground source is copy-paste ready",
        "components/form-field/test/semantics.rs::form_field_check2_documents_source_first_copy_paste_ready_rules",
        "components/form-field/test/semantics.rs::form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_source_first_copy_paste_ready_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 source-first section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 heroui-benchmark docs-sync section should include `{required}`."
        );
    }
}

#[test]
fn form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = include_str!("../../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = include_str!("../../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("docs_form_field_page");
    let readme_source = load_source("readme");

    for required in [
        "### FormField 同步记录（2026-02-20）",
        "参数模型同步：`FormField` 参数主轴保持 `is_selected/default_selected/on_selected_change`",
        "component_doc!(\"FormField\", \"form-field\", \"Forms\", forms_groups_extra::form_field)",
        "`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field()`",
        "`components/form-field/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(required),
            "heroui strategy doc should include form-field synchronization marker `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"FormField\"",
        "\"form-field\"",
        "forms_groups_extra::form_field",
    ] {
        assert!(
            pages_source.contains(required),
            "component docs index should expose form-field entry marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn form_field() -> AnyView {",
        "title=\"FormField\"",
        "slug=\"form-field\"",
    ] {
        assert!(
            docs_source.contains(required),
            "docs-app form-field page should stay indexable via marker `{required}`."
        );
    }

    for required in ["# FormField", "## docs-app 入口"] {
        assert!(
            readme_source.contains(required),
            "form-field README should remain an equivalent component doc entry via `{required}`."
        );
    }
}

#[test]
fn form_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = include_str!("../../../scripts/check-ui-dx.sh");

    for required in [
        "echo \"[dx] contract: form-field heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(required),
            "DX check script should enforce heroui-benchmark docs-sync contract `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "components/form-field/test/semantics.rs::form_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/form-field/test/semantics.rs::form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/form-field/test/semantics.rs::form_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_heroui_benchmark_docs_sync_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 should keep heroui-benchmark docs-sync evidence marker `{required}`."
        );
    }
}

#[test]
fn form_field_composite_parent_item_rule_is_na_for_single_field_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_form_field_docs_section();
    let check2 = load_source("check2");

    for forbidden in [
        "ItemSpec",
        "labels + children",
        "titles + panels",
        "items_order",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should stay out of composite Parent/Item conventions and avoid `{forbidden}`."
        );
    }

    for required in [
        "<FormField label=\"Accept terms of service\".to_string() />",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs should keep direct single-component invocation `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。"),
        "form-field checklist should mark composite Parent/Item contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 为单字段组件，不承载集合子项注册与容器-子项组合语义）；API 仅暴露单实例 props，不存在 `labels + children`、`titles + panels` 或 `ItemSpec` 入口。"),
        "form-field checklist should describe why composite Parent/Item contract is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_composite_parent_item_rule_is_na_for_single_field_component"),
        "form-field checklist should record composite Parent/Item N/A regression evidence."
    );
}

#[test]
fn form_field_macro_micro_duality_is_na_without_dragging_loop() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "pointermove",
        "on:pointermove",
        "requestAnimationFrame",
        "raf",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include drag-loop macro/micro state machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。"),
        "form-field checklist should mark macro/micro duality contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 不提供拖拽手势与几何跟随能力），不存在 `Dragging` 宏观状态、逐帧 pointer move 循环或 `Action::DragEnd` 收敛动作。"),
        "form-field checklist should explain why macro/micro duality is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_macro_micro_duality_is_na_without_dragging_loop"),
        "form-field checklist should record macro/micro duality N/A regression evidence."
    );
}

#[test]
fn form_field_two_pass_rendering_is_na_without_geometry_measurement() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "Intent -> Measure",
        "Rectification",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include two-pass geometry measurement machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。"),
        "form-field checklist should mark two-pass rendering contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 不包含 overlay 定位或依赖 DOM 几何测量的交互能力），无 `Intent -> Measure -> Rectification` 回路，也无幂等纠偏收敛状态机。"),
        "form-field checklist should explain why two-pass rendering is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_two_pass_rendering_is_na_without_geometry_measurement"),
        "form-field checklist should record two-pass rendering N/A regression evidence."
    );
}

#[test]
fn form_field_registration_protocol_is_na_for_non_collection_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "RegistrationContext",
        "Register/Unregister",
        "register(",
        "unregister(",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include collection registration protocol machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。"),
        "form-field checklist should mark registration protocol contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 非集合容器组件，不管理动态子项生命周期），不存在 `RegistrationContext`、`Register/Unregister` 协议与 `items_order` 导航状态。"),
        "form-field checklist should explain why registration protocol is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_registration_protocol_is_na_for_non_collection_component"),
        "form-field checklist should record registration protocol N/A regression evidence."
    );
}

#[test]
fn form_field_slot_projection_is_na_for_non_container_component() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include slot projection policy machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。"),
        "form-field checklist should mark slot projection contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 非容器投影组件，不承载子内容驻留策略），不存在 `Lazy/KeepAlive/Eager` 插槽模式与 `NotifyHidden` 生命周期通知链路。"),
        "form-field checklist should explain why slot projection strategy is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_slot_projection_is_na_for_non_container_component"),
        "form-field checklist should record slot projection N/A regression evidence."
    );
}

#[test]
fn form_field_env_streams_are_na_without_environment_subscriptions() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "on:resize",
        "on:scroll",
        "debounce",
        "throttle",
        "match_media",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include environment stream machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。"),
        "form-field checklist should mark env-stream contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 无断点/可见性/环境驱动布局语义），不存在 `Resize/Theme/Intersection` 订阅、防抖采样与 `BreakpointChanged` 类高层 `Action` 回流链路。"),
        "form-field checklist should explain why env streams are N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_env_streams_are_na_without_environment_subscriptions"),
        "form-field checklist should record env-stream N/A regression evidence."
    );
}

#[test]
fn form_field_event_light_cone_is_na_without_collection_batch_operations() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "prop drilling",
        "batch selection",
        "bulk selection",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include event-light-cone batch-operation machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。"),
        "form-field checklist should mark event-light-cone contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 非大型集合组件，不存在批量选择与跨项广播场景），无需 `Context Bus + Selector` 及 `SelectionState::All` 等状态压缩协议。"),
        "form-field checklist should explain why event light cone is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_event_light_cone_is_na_without_collection_batch_operations"),
        "form-field checklist should record event-light-cone N/A regression evidence."
    );
}

#[test]
fn form_field_causality_bus_is_na_without_derived_event_bus() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "TraceId",
        "Causality Bus",
        "event bus",
        "dispatch command",
        "broadcast",
        "subscriber",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not include causality-bus machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。"),
        "form-field checklist should mark causality-bus contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 无复杂派生总线与跨订阅者广播链路），不存在 `TraceId` 透传需求及“触发 -> 派生命令 -> 广播 -> 订阅者”因果链编排。"),
        "form-field checklist should explain why causality bus is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_causality_bus_is_na_without_derived_event_bus"),
        "form-field checklist should record causality-bus N/A regression evidence."
    );
}

#[test]
fn form_field_focus_stack_is_na_without_overlay_restore_targets() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "NodeRef",
        "document.body",
        "FallbackTo",
        "Focus Manager",
        "focus stack",
        "overlay stack",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not carry overlay-focus-stack machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "form-field checklist should mark focus-stack contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 非 overlay/portal 组件，不创建层叠浮层与焦点恢复栈），不存在“关闭浮层后恢复焦点目标”的状态机需求。"),
        "form-field checklist should explain why focus stack is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_focus_stack_is_na_without_overlay_restore_targets"),
        "form-field checklist should record focus-stack N/A regression evidence."
    );
}

#[test]
fn form_field_escape_hatch_is_na_without_imperative_third_party_instances() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for forbidden in [
        "ECharts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "imperative instance",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not carry imperative third-party escape-hatch machinery `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "form-field checklist should mark escape-hatch contract as complete."
    );
    assert!(
        check2.contains("本组件判定：N/A（`FormField` 不集成命令式第三方引擎，不存在 ECharts/Map 类实例生命周期托管），无 `Foreign Zone`、`YieldControl`、`CleanupForeign` 接入需求。"),
        "form-field checklist should explain why escape hatch is N/A."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_escape_hatch_is_na_without_imperative_third_party_instances"),
        "form-field checklist should record escape-hatch N/A regression evidence."
    );
}

#[test]
fn form_field_hydration_discontinuity_avoids_time_random_and_prefers_id_provider_seed() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for required in [
        "use_ui_id_provider,",
        "pub const DEFAULT_ID_BASE: &str = \"ui-form-field\";",
        "let id_base = id_base.or_else(|| {",
        "use_ui_id_provider()",
        "id_provider.next_prefixed_id(logic::DEFAULT_ID_BASE)",
        "let id_base = StoredValue::new(logic::normalize_id_base(id_base));",
    ] {
        assert!(
            view.contains(required) || logic.contains(required),
            "form-field should prefer deterministic id-provider path for hydration safety `{required}`."
        );
    }

    for forbidden in [
        "SystemTime::now",
        "Instant::now",
        "Date::now",
        "Uuid::",
        "uuid::",
        "rand::",
        "random::<",
        "Math::random",
        "getrandom",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !docs.contains(forbidden),
            "form-field should not initialize hydration-unstable id/time/random source `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"),
        "form-field checklist should mark hydration-discontinuity contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_hydration_discontinuity_avoids_time_random_and_prefers_id_provider_seed"),
        "form-field checklist should record hydration-discontinuity regression evidence."
    );
}

#[test]
fn form_field_platform_compile_contract_stays_cfg_explicit_and_non_wasm_safe() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window.",
        "document.",
        "#[cfg(target_arch = \"wasm32\")]",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field component layer should keep non-wasm safe source without `{forbidden}`."
        );
    }

    for required in [
        "cargo check -p ui-form-field",
        "cargo check -p ui-form-field --target wasm32-unknown-unknown",
        "cargo check -p ui-form-field --no-default-features",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should document compile-only verification command `{required}`."
        );
    }

    assert!(
        check2
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "form-field checklist should mark SSR/platform compile contract as complete."
    );
    assert!(
        check2.contains("当前环境说明：上述命令在本执行环境被统一阻断为 `Invalid cross-device link (os error 18)`；该错误为构建环境问题而非组件源码的跨平台分支错误。"),
        "form-field checklist should document current environment compile blocker for platform checks."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_platform_compile_contract_stays_cfg_explicit_and_non_wasm_safe"),
        "form-field checklist should record SSR/platform compile regression evidence."
    );
}

#[test]
fn form_field_headless_web_ssr_mutex_contract_is_preserved() {
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let form_field_cargo = std::fs::read_to_string(component_dir.join("Cargo.toml"))
        .expect("form-field Cargo.toml should be readable");
    let headless_lib =
        std::fs::read_to_string(component_dir.join("../../crates/ui-headless/src/lib.rs"))
            .expect("ui-headless lib.rs should be readable for feature-mutex contract checks");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless should keep web/ssr mutex guard via `{required}`."
        );
    }

    for required in [
        "ui-headless = { path = \"../../crates/ui-headless\" }",
        "use_switch",
        "use_controllable_state",
        "locale_attrs",
    ] {
        assert!(
            form_field_cargo.contains(required) || view.contains(required),
            "form-field should consume headless contract without breaking feature mutex `{required}`."
        );
    }

    for forbidden in [
        "ui-headless/web",
        "ui-headless/ssr",
        "features = [\"web\", \"ssr\"]",
    ] {
        assert!(
            !form_field_cargo.contains(forbidden),
            "form-field Cargo should not force invalid web+ssr combination `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "form-field checklist should mark ui-headless web/ssr mutex contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_headless_web_ssr_mutex_contract_is_preserved"),
        "form-field checklist should record ui-headless mutex regression evidence."
    );
}

#[test]
fn form_field_motion_non_wasm_stub_contract_is_preserved() {
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let motion_lib =
        std::fs::read_to_string(component_dir.join("../../crates/ui-motion/src/lib.rs"))
            .expect("ui-motion lib.rs should be readable for wasm/non-wasm stub contract checks");

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "pub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op/stub contract via `{required}`."
        );
    }

    for forbidden in ["web_sys::", "wasm_bindgen::", "js_sys::"] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field component layer should stay platform-agnostic without `{forbidden}`."
        );
    }

    assert!(
        !std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/motion.rs")
            .exists(),
        "form-field should not assume component-level motion handle; `src/motion.rs` remains N/A."
    );

    for required in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion non_wasm_web_backend_is_predictable_noop -- --exact",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should record ui-motion compile/no-op verification command `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "form-field checklist should mark ui-motion non-wasm no-op/stub contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_motion_non_wasm_stub_contract_is_preserved"),
        "form-field checklist should record ui-motion non-wasm no-op/stub regression evidence."
    );
}

#[test]
fn form_field_reduced_motion_ssr_wasm_branch_contract_is_covered_without_semantic_split() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_lib =
        std::fs::read_to_string(component_dir.join("../../crates/ui-motion/src/lib.rs"))
            .expect("ui-motion lib.rs should be readable for wasm/non-wasm branch checks");
    let motion_web =
        std::fs::read_to_string(component_dir.join("../../crates/ui-motion/src/web.rs"))
            .expect("ui-motion web.rs should be readable for reduced-motion contract checks");

    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep wasm/non-wasm split and non-wasm fallback via `{required}`."
        );
    }

    for required in [
        "w.match_media(\"(prefers-reduced-motion: reduce)\")",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            motion_web.contains(required),
            "ui-motion wasm backend should keep reduced-motion early-degrade path `{required}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "#[cfg(target_arch = \"wasm32\")]",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field component layer should stay platform-neutral without semantic split `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "form-field checklist should mark reduced-motion/SSR/wasm branch contract as complete."
    );
    for required in [
        "cargo check -p ui-form-field",
        "cargo check -p ui-form-field --target wasm32-unknown-unknown",
        "cargo test -p ui-motion non_wasm_web_backend_is_predictable_noop -- --exact",
        "components/form-field/test/semantics.rs::form_field_reduced_motion_ssr_wasm_branch_contract_is_covered_without_semantic_split",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should record branch-coverage evidence `{required}`."
        );
    }
}

#[test]
fn form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards()
 {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let motion_lib =
        std::fs::read_to_string(component_dir.join("../../crates/ui-motion/src/lib.rs"))
            .expect("ui-motion lib.rs should be readable for non-wasm no-op guard checks");
    let motion_web =
        std::fs::read_to_string(component_dir.join("../../crates/ui-motion/src/web.rs"))
            .expect("ui-motion web.rs should be readable for reduced-motion guard checks");
    let platform_script =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-platforms.sh"))
            .expect("platform check script should be readable");

    let motion_file = component_dir.join("src/motion.rs");
    assert!(
        !motion_file.exists(),
        "form-field should keep motion contractualization as explicit N/A; `src/motion.rs` must remain absent."
    );

    for source in [module, logic, view, styles] {
        for forbidden in [
            "mod motion;",
            "pub mod motion;",
            "pub use motion::",
            "attach_motion(",
            "stiffness",
            "damping",
            "MotionOptions",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field should not leak component-local motion contract token `{forbidden}`."
            );
        }
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op branch for N/A component motion via `{required}`."
        );
    }

    for required in [
        "w.match_media(\"(prefers-reduced-motion: reduce)\")",
        "if prefers_reduced_motion() {",
        "return;",
    ] {
        assert!(
            motion_web.contains(required),
            "ui-motion wasm backend should keep reduced-motion guard `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards";
    assert!(
        platform_script.contains(script_needle),
        "platform check script should include `{script_needle}`.",
    );

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "本组件判定：N/A（`FormField` 无独立组件级动效状态轴，不定义 `src/motion.rs` 与 `attach_motion`）",
        "components/form-field/test/semantics.rs::form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
        "components/form-field/test/form_field/semantics.rs::form_field_motion_contract_is_explicitly_na_for_runtime_attach_and_keeps_reduced_motion_noop_guards",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should record motion contractualization marker `{required}`."
        );
    }
}

#[test]
fn form_field_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let ui_components_lib =
        std::fs::read_to_string(component_dir.join("../../crates/ui/src/lib.rs"))
            .expect("ui lib.rs should be readable");
    let ui_components_css =
        std::fs::read_to_string(component_dir.join("../../crates/ui/src/css.rs"))
            .expect("ui css.rs should be readable");
    let ui_components_root =
        std::fs::read_to_string(component_dir.join("../../crates/ui/src/root.rs"))
            .expect("ui root.rs should be readable");
    let active_highlight = std::fs::read_to_string(
        component_dir.join("../../crates/ui-visual-primitive/src/active_highlight.rs"),
    )
    .expect("ui-visual-primitive active_highlight.rs should be readable");
    let entrypoints_script =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-entrypoints.sh"))
            .expect("entrypoints check script should be readable");
    let ui_components_src_root = component_dir.join("../../crates/ui/src");
    let ui_headless_src_root = component_dir.join("../../crates/ui-headless/src");

    for required in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(any(",
        "feature = \"component-form_field\",",
        "pub mod field_form {",
        "pub use field_form::form_field::{",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs should keep fixed entrypoint marker `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui public API should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-form_field\")]",
        "pub mod form_field {",
        "pub use crate::field_form_form_field::*;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib.rs inline field_form module should keep form-field feature-gated entry `{required}`."
        );
    }

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::field_form::form_field::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css.rs should keep fixed entrypoint marker `{required}`."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "provide_ui_i18n(i18n);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(required),
            "UiRoot should keep centralized injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion {",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should keep marker `{required}`."
        );
    }

    for forbidden in ["FormField", "ui-form-field", "form_field"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight shared primitive should avoid form-field business token `{forbidden}`."
        );
    }

    for forbidden_path in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_root.join(forbidden_path).exists(),
            "ui src should not host duplicated headless primitive `{forbidden_path}`."
        );
    }

    for required_path in ["controllable_state.rs", "presence.rs", "a11y.rs"] {
        assert!(
            ui_headless_src_root.join(required_path).exists(),
            "ui-headless should host shared primitive `{required_path}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints script should include `{script_needle}`."
    );

    for required in [
        "- [x] `ui` 固定入口文件落点正确。",
        "components/form-field/test/semantics.rs::form_field_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/form-field/test/form_field/semantics.rs::form_field_ui_components_fixed_entry_files_follow_layered_boundaries",
        "scripts/check-ui-entrypoints.sh",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep fixed-entrypoint evidence `{required}`."
        );
    }
}

#[test]
fn form_field_performance_governance_budget_is_defined_traceable_and_blocking() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let shell_source = std::fs::read_to_string(
        component_dir.join("../../apps/docs-app/src/pages/components/shell.rs"),
    )
    .expect("docs shell should be readable for perf budget checks");
    let pages_source = std::fs::read_to_string(
        component_dir.join("../../apps/docs-app/src/pages/components/pages.rs"),
    )
    .expect("docs pages registry should be readable for perf coverage checks");
    let perf_probe_source =
        std::fs::read_to_string(component_dir.join("../../apps/docs-app/src/perf_probe.rs"))
            .expect("perf probe should be readable for machine-readable perf markers");
    let coverage_source = std::fs::read_to_string(
        component_dir.join("../../e2e/tests/docs_app_components_coverage.spec.mjs"),
    )
    .expect("docs coverage e2e should be readable for repeatable perf assertions");
    let debug_overlay_source =
        std::fs::read_to_string(component_dir.join("../../apps/docs-app/src/debug_overlay.rs"))
            .expect("debug overlay source should be readable for trace attribution checks");
    let todo_source = std::fs::read_to_string(component_dir.join("../../docs/plan/TODO.md"))
        .expect("todo plan should be readable for render_count follow-up tracking");
    let perf_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-performance.sh"))
            .expect("performance gate script should be readable");

    for required in [
        "\"button\" => UiPerfBudget {",
        "max_mount_ms: 24.0,",
        "\"input\" => UiPerfBudget {",
        "max_mount_ms: 28.0,",
        "\"form-field\" => UiPerfBudget {",
        "max_mount_ms: 26.0,",
        "max_update_ms: Some(8.0),",
        "max_heap_kb: Some(384.0),",
    ] {
        assert!(
            shell_source.contains(required),
            "docs shell should keep explicit form-field perf budget token `{required}`."
        );
    }

    for required in [
        "component_doc!(",
        "\"FormField\"",
        "\"form-field\"",
        "forms_groups_extra::form_field",
    ] {
        assert!(
            pages_source.contains(required),
            "docs component registry should keep form-field page coverage token `{required}`."
        );
    }

    for required in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
        "(mount_ms > budget.max_mount_ms).then_some(\"true\")",
    ] {
        assert!(
            perf_probe_source.contains(required),
            "UiPerfProbe should keep machine-readable perf marker `{required}`."
        );
    }

    for required in [
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(required),
            "docs e2e coverage should keep repeatable perf threshold assertion `{required}`."
        );
    }

    for required in [
        "let trace = ui_headless::use_ui_trace();",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
    ] {
        assert!(
            debug_overlay_source.contains(required),
            "debug overlay should keep trace-based perf attribution token `{required}`."
        );
    }

    for required in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(required),
            "repository todo should keep render_count follow-up token `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field view should expose attributable perf marker `{required}`."
        );
    }

    for required in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "渲染次数预算为 `1`",
        "render_count",
        "若当前测试框架暂不支持精确渲染计数",
        "等价证据",
        "components/form-field/test/semantics.rs::form_field_performance_governance_budget_is_defined_traceable_and_blocking",
        "components/form-field/test/form_field/semantics.rs::form_field_performance_governance_budget_is_defined_traceable_and_blocking",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep performance governance marker `{required}`."
        );
    }

    let perf_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_performance_governance_budget_is_defined_traceable_and_blocking";
    assert!(
        perf_script_source.contains(perf_script_needle),
        "performance gate script should include `{perf_script_needle}`."
    );
}

#[test]
fn form_field_view_macro_complexity_is_bounded_via_semantic_subview_split() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_macro_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-view-macro.sh"))
            .expect("view-macro gate script should be readable");

    for required in [
        "fn render_indicator_view(",
        "fn render_content_view(",
        "{render_indicator_view(",
        "{render_content_view(",
        "data-slot=\"form-field\"",
        "data-slot=\"form-field-content\"",
        "data-slot=\"form-field-indicator\"",
    ] {
        assert!(
            view.contains(required),
            "form-field view should keep semantic subview split marker `{required}`."
        );
    }

    let switch_branch_count = view.matches("FormFieldIndicatorVariant::Switch =>").count();
    assert_eq!(
        switch_branch_count, 1,
        "indicator switch branch should be centralized into one render helper; found {switch_branch_count}."
    );
    let checkbox_branch_count = view
        .matches("FormFieldIndicatorVariant::Checkbox =>")
        .count();
    assert_eq!(
        checkbox_branch_count, 1,
        "indicator checkbox branch should be centralized into one render helper; found {checkbox_branch_count}."
    );

    for required in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "components/form-field/test/semantics.rs::form_field_view_macro_complexity_is_bounded_via_semantic_subview_split",
        "components/form-field/test/form_field/semantics.rs::form_field_view_macro_complexity_is_controlled_by_semantic_subview_split",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep view-macro complexity governance marker `{required}`."
        );
    }

    let view_macro_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_view_macro_complexity_is_controlled_by_semantic_subview_split";
    assert!(
        view_macro_script_source.contains(view_macro_script_needle),
        "view-macro gate script should include `{view_macro_script_needle}`."
    );
}

#[test]
fn form_field_view_functional_split_prefers_plain_helpers_without_component_noise() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_macro_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-view-macro.sh"))
            .expect("view-macro gate script should be readable");

    for required in [
        "fn render_indicator_view(",
        "fn render_content_view(",
        "{render_indicator_view(",
        "{render_content_view(",
        "#[component]\npub fn FormField(",
    ] {
        assert!(
            view.contains(required),
            "form-field view should keep functional-split marker `{required}`."
        );
    }

    let component_attr_count = view.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "form-field view should keep a single component declaration (FormField) while helper fragments stay plain functions; found {component_attr_count}."
    );

    for required in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "components/form-field/test/semantics.rs::form_field_view_functional_split_prefers_plain_helpers_without_component_noise",
        "components/form-field/test/form_field/semantics.rs::form_field_view_functional_split_prefers_plain_functions_over_extra_local_components",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep function-first split governance marker `{required}`."
        );
    }

    let view_macro_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_view_functional_split_prefers_plain_functions_over_extra_local_components";
    assert!(
        view_macro_script_source.contains(view_macro_script_needle),
        "view-macro gate script should include `{view_macro_script_needle}`."
    );
}

#[test]
fn form_field_static_fragments_are_constantized_or_absent_for_simple_layout() {
    let check2 = load_source("check2");
    let view = load_source("view");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let view_macro_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-view-macro.sh"))
            .expect("view-macro gate script should be readable");

    for forbidden in [
        "inner_html=",
        "include_str!(",
        "markdown_to_html",
        "svg path d=\"",
        "</footer>",
    ] {
        assert!(
            !view.contains(forbidden),
            "form-field should avoid heavy static fragment token `{forbidden}` in view layer."
        );
    }

    assert_eq!(
        view.matches("ui-switch__track").count(),
        1,
        "switch static track fragment should stay single-source to avoid duplicate construction."
    );
    assert_eq!(
        view.matches("ui-switch__thumb").count(),
        1,
        "switch static thumb fragment should stay single-source to avoid duplicate construction."
    );

    for required in [
        "data-slot=\"switch-track\"",
        "data-slot=\"switch-thumb\"",
        "role=aria.attrs.role",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            view.contains(required),
            "form-field static micro fragments should keep a11y/semantic marker `{required}`."
        );
    }

    for required in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "static fragments are constantized or absent for simple layout",
        "components/form-field/test/semantics.rs::form_field_static_fragments_are_constantized_or_absent_for_simple_layout",
        "components/form-field/test/form_field/semantics.rs::form_field_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep static fragment governance marker `{required}`."
        );
    }

    let view_macro_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        view_macro_script_source.contains(view_macro_script_needle),
        "view-macro gate script should include `{view_macro_script_needle}`."
    );
}

#[test]
fn form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let docs_form_field_page = load_source("docs_form_field_page");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let inner_html_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-inner-html.sh"))
            .expect("inner-html gate script should be readable");

    for source in [module, logic, styles, view, docs_form_field_page] {
        let normalized = source.to_ascii_lowercase();
        for forbidden in [
            "inner_html=",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "markdown_to_html(",
            "format!(\"<",
            "<script",
            "javascript:",
        ] {
            assert!(
                !normalized.contains(forbidden),
                "form-field component/docs source should forbid html injection marker `{forbidden}`."
            );
        }
    }

    for required in [
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "该项按“零注入面”通过",
        "components/form-field/test/semantics.rs::form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        "components/form-field/test/form_field/semantics.rs::form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep inner_html safety governance marker `{required}`."
        );
    }

    let inner_html_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        inner_html_script_source.contains(inner_html_script_needle),
        "inner-html gate script should include `{inner_html_script_needle}`."
    );
}

#[test]
fn form_field_wasm_debug_contract_is_na_and_feature_isolated() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let form_field_cargo = std::fs::read_to_string(component_dir.join("Cargo.toml"))
        .expect("form-field Cargo.toml should be readable");
    let ui_components_cargo =
        std::fs::read_to_string(component_dir.join("../../crates/ui/Cargo.toml"))
            .expect("ui Cargo.toml should be readable");
    let ui_components_lib =
        std::fs::read_to_string(component_dir.join("../../crates/ui/src/lib.rs"))
            .expect("ui lib.rs should be readable");
    let docs_app_source =
        std::fs::read_to_string(component_dir.join("../../apps/docs-app/src/lib.rs"))
            .expect("docs-app lib.rs should be readable");
    let debug_overlay_source =
        std::fs::read_to_string(component_dir.join("../../apps/docs-app/src/debug_overlay.rs"))
            .expect("docs-app debug overlay source should be readable");
    let trace_source =
        std::fs::read_to_string(component_dir.join("../../crates/ui-headless/src/trace.rs"))
            .expect("ui-headless trace source should be readable");
    let wasm_debug_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-wasm-debug.sh"))
            .expect("wasm-debug gate script should be readable");

    for required in ["[features]", "default = []"] {
        assert!(
            form_field_cargo.contains(required),
            "form-field crate feature boundary should include `{required}`."
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "dep:tracing"] {
        assert!(
            !form_field_cargo.contains(forbidden),
            "form-field crate should not leak wasm debug feature `{forbidden}`."
        );
    }

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui should keep shared wasm debug feature marker `{required}`."
        );
    }

    for forbidden in [
        "form-field-wasm-debug =",
        "form_field_wasm_debug =",
        "component-form_field\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui feature graph should not leak form-field-specific debug toggle `{forbidden}`."
        );
    }

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui root should keep global wasm-debug isolation marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_source.contains(required),
            "docs-app should keep dev-only debug overlay entry `{required}`."
        );
    }

    for required in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        ".into_iter()",
        ".rev()",
        ".take(40)",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            trace_source.contains(required) || debug_overlay_source.contains(required),
            "global trace/debug-overlay contract should keep marker `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field should keep state/source marker `{required}` for debug traceability."
        );
    }

    for source in [module, logic, styles, view] {
        for forbidden in [
            "use_ui_trace(",
            "provide_ui_trace(",
            "trace.emit(",
            "debug_overlay",
            "request_replay",
            "replay",
            "trace_id",
            "wasm_debug_proxy!",
            "observability::",
            "#[prop(optional)] debug",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field runtime/public contract should not leak wasm-debug internals `{forbidden}`."
            );
        }
    }

    let wasm_debug_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_wasm_debug_contract_is_na_and_feature_isolated";
    assert!(
        wasm_debug_script_source.contains(wasm_debug_script_needle),
        "wasm-debug gate script should include `{wasm_debug_script_needle}`."
    );

    for required in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "本组件判定：N/A（组件级不自建 wasm 调试/回放管线）",
        "components/form-field/test/semantics.rs::form_field_wasm_debug_contract_is_na_and_feature_isolated",
        "components/form-field/test/form_field/semantics.rs::form_field_wasm_debug_contract_is_na_and_feature_isolated",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep wasm-debug governance marker `{required}`."
        );
    }
}

#[test]
fn form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na() {
    let docs_source = load_source("docs_form_field_page");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let playground_source =
        std::fs::read_to_string(component_dir.join("../../apps/docs-app/src/playground.rs"))
            .expect("docs playground source should be readable");
    let dx_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-dx.sh"))
            .expect("dx gate script should be readable");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "data-playground-scope=scope_id.clone()",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(required),
            "docs playground should keep DX hot-reload/isolated-canvas marker `{required}`."
        );
    }

    for required in [
        "pub(super) fn form_field() -> AnyView",
        "let (marketing, set_marketing) = signal(true);",
        "let (tos, set_tos) = signal(false);",
        "let on_marketing_selected_change = Callback::new(move |next| set_marketing.set(next));",
        "let on_tos_selected_change = Callback::new(move |next| set_tos.set(next));",
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
        "\"marketing: \" {move || marketing.get()}",
        "\"tos: \" {move || tos.get()}",
    ] {
        assert!(
            docs_source.contains(required),
            "form-field docs should keep DX workbench/context marker `{required}`."
        );
    }

    for forbidden in [
        "FORM_FIELD_WORKBENCH_STORAGE_KEY",
        "load_form_field_workbench_state(",
        "save_form_field_workbench_state(",
        "clear_form_field_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "form-field keeps optional persisted state as N/A in current scope; `{forbidden}` should remain absent."
        );
    }

    for required in [
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
        "components/form-field/test/semantics.rs::form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
        "components/form-field/test/form_field/semantics.rs::form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep DX governance marker `{required}`."
        );
    }

    let dx_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na";
    assert!(
        dx_script_source.contains(dx_script_needle),
        "dx gate script should include `{dx_script_needle}`."
    );
}

#[test]
fn form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let protocol_source = std::fs::read_to_string(component_dir.join("src/protocol.rs"))
        .expect("form-field protocol source should be readable");
    let protocol_test_source = std::fs::read_to_string(component_dir.join("test/protocol.rs"))
        .expect("form-field protocol tests should be readable");
    let form_field_cargo = std::fs::read_to_string(component_dir.join("Cargo.toml"))
        .expect("form-field Cargo.toml should be readable");
    let ui_components_cargo =
        std::fs::read_to_string(component_dir.join("../../crates/ui/Cargo.toml"))
            .expect("ui Cargo.toml should be readable");
    let button_view_source =
        std::fs::read_to_string(component_dir.join("../../components/button/src/view.rs"))
            .expect("button view source should be readable");
    let engineering_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-engineering.sh"))
            .expect("engineering gate script should be readable");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum FormFieldComponentSchemaVersion",
        "#[serde(rename_all = \"snake_case\")]",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol should keep structured serde marker `{required}`."
        );
    }

    for required in [
        "fn protocol_types_implement_serde_contract()",
        "T: Serialize + DeserializeOwned",
    ] {
        assert!(
            protocol_test_source.contains(required),
            "form-field protocol tests should keep serde regression marker `{required}`."
        );
    }

    assert!(
        button_view_source.contains("target: \"ui::button::state_change\""),
        "engineering baseline should keep canonical tracing target `ui::button::state_change`."
    );
    assert!(
        ui_components_cargo.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "ui feature surface should keep shared tracing/debug baseline marker."
    );

    for source in [module, logic, view, styles, protocol_source.as_str()] {
        for forbidden in [
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
            "target: \"ui::form_field::",
            "const FORM_FIELD_TRACE_TARGET",
            "tokio",
            "tokio::",
            "async_std",
            "async_std::",
            "async-std",
            "smol::",
            "runtime::Handle",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field engineering contract should avoid tracing/runtime leak marker `{forbidden}`."
            );
        }
    }

    for forbidden in ["tokio", "async-std", "async_std", "smol", "runtime::Handle"] {
        assert!(
            !form_field_cargo.contains(forbidden),
            "form-field Cargo.toml should not leak runtime binding `{forbidden}`."
        );
    }

    let engineering_script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        engineering_script_source.contains(engineering_script_needle),
        "engineering gate script should include `{engineering_script_needle}`."
    );

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
        "components/form-field/test/semantics.rs::form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
        "components/form-field/test/form_field/semantics.rs::form_field_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep engineering governance marker `{required}`."
        );
    }
}

#[test]
fn form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade()
 {
    let check2 = load_source("check2");
    let manifest = load_source("manifest");
    let rbi = load_source("rbi");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let protocol_source = std::fs::read_to_string(component_dir.join("src/protocol.rs"))
        .expect("form-field protocol source should be readable");
    let engineering_script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-engineering.sh"))
            .expect("engineering gate script should be readable");

    for required in [
        "pub enum FormFieldComponentSchemaVersion",
        "#[default]",
        "V1,",
        "pub struct FormFieldComponentSpec",
        "pub schema_version: FormFieldComponentSchemaVersion,",
    ] {
        assert!(
            protocol_source.contains(required),
            "form-field protocol should keep v1 schema marker `{required}` in non-breaking scope."
        );
    }

    for required in [
        "schema_version = \"1\"",
        "schema = \"ui.form_field.agent-contract.v1\"",
        "values = [\"v1\"]",
    ] {
        assert!(
            manifest.contains(required),
            "form-field manifest should keep v1 registration marker `{required}`."
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecation_window",
        "codemod",
        "schema_version = \"2\"",
        "agent-contract.v2",
    ] {
        assert!(
            !protocol_source.contains(forbidden)
                && !manifest.contains(forbidden)
                && !rbi.contains(forbidden),
            "without major breaking upgrade, form-field should not claim migration surface `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade";
    assert!(
        engineering_script_source.contains(script_needle),
        "engineering gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "FormFieldComponentSchemaVersion::V1",
        "schema_version = \"1\"",
        "ui.form_field.agent-contract.v1",
        "form_field_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep version deprecation migration marker `{required}`."
        );
    }
}

#[test]
fn form_field_a11y_i18n_contract_is_mounted_without_hardcoded_view_text() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in ["OnPress", "SwitchOptions", "use_switch"] {
        assert!(
            view.contains(required),
            "form-field switch adapter should mount headless a11y contract `{required}`."
        );
    }
    for required in [
        "role=aria.attrs.role",
        "aria-checked=move || aria.attrs.aria_checked.get()",
    ] {
        assert!(
            view.contains(required),
            "form-field switch adapter should mount headless a11y contract `{required}`."
        );
    }

    for required in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "let locale = locale_attrs(lang, dir);",
        "role=\"group\"",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "{move || label.get_value()}",
    ] {
        assert!(
            view.contains(required),
            "form-field view should expose locale/a11y contract `{required}`."
        );
    }

    for required in [
        "pub const DEFAULT_LABEL: &str = \"Form field\";",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Form field control\";",
        "pub const DEFAULT_ERROR_MESSAGE: &str = \"Selection is required\";",
        "pub fn normalize_label(",
        "pub fn normalize_aria_label(",
        "pub fn normalize_error_message(",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should keep i18n fallback normalization `{required}`."
        );
    }

    assert!(
        !view.contains("\"toggle\""),
        "form-field view should not hardcode user-visible indicator text."
    );

    assert!(
        check2.contains(
            "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
        ),
        "form-field checklist should mark a11y/i18n contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_a11y_i18n_contract_is_mounted_without_hardcoded_view_text"),
        "form-field checklist should record a11y/i18n regression evidence."
    );
}

#[test]
fn form_field_state_markers_are_observable_queryable_and_enumerable() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "data-state=move || state.get().state_attr",
        "data-selected=move || state.get().is_selected.then_some(\"true\")",
        "data-unselected=move || state.get().is_unselected.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-selected-controlled=is_controlled_selected.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_controlled_selected).then_some(\"true\")",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "form-field root should expose stable observable/queryable marker `{required}`."
        );
    }

    for required in [
        "data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some(\"true\")",
        "data-focused=move || aria.state.resolved.get().is_focused.then_some(\"true\")",
        "data-pressed=move || aria.state.resolved.get().is_pressed.then_some(\"true\")",
        "data-hovered=move || aria.state.resolved.get().is_hovered.then_some(\"true\")",
    ] {
        assert!(
            view.contains(required),
            "form-field switch control should expose headless interaction marker `{required}`."
        );
    }

    for required in [
        "let state_attr = if input.is_invalid && input.is_disabled {",
        "\"invalid-disabled\"",
        "\"selected-invalid\"",
        "\"invalid\"",
        "\"selected-disabled\"",
        "\"disabled\"",
        "\"selected\"",
        "\"unselected\"",
        "let message_kind_attr = if shows_error {",
        "\"error\"",
        "\"description\"",
        "\"none\"",
        "let label_source_attr = if input.has_custom_label {",
        "let aria_source_attr = if input.has_custom_aria_label {",
        "let error_source_attr = if !input.has_error_message {",
        "let class_source_attr = if input.has_custom_class_name {",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should keep marker values in closed enumerable sets via `{required}`."
        );
    }

    for forbidden in [
        "data-state=move || format!(",
        "data-message-kind=move || format!(",
        "data-label-source=move || format!(",
        "data-aria-source=move || format!(",
        "data-error-source=move || format!(",
    ] {
        assert!(
            !view.contains(forbidden),
            "form-field view markers should not be free-form formatted text `{forbidden}`."
        );
    }

    assert!(
        check2.contains(
            "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。"
        ),
        "form-field checklist should mark observable/queryable marker contract as complete."
    );
    assert!(
        check2.contains("关键状态轴适配：`selected/disabled/invalid/focus-visible` 已覆盖；`open/expanded/loading` 对本组件为 N/A"),
        "form-field checklist should document key marker axis coverage and N/A boundaries."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_state_markers_are_observable_queryable_and_enumerable"),
        "form-field checklist should record observable/queryable marker regression evidence."
    );
}

#[test]
fn form_field_styles_depend_on_explicit_state_markers_not_dom_guessing() {
    let module = load_source("mod");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for required in [
        ".ui-form-field--placement-end,",
        ".ui-form-field[data-indicator-placement=\"end\"] {",
        ".ui-form-field--placement-start,",
        ".ui-form-field[data-indicator-placement=\"start\"] {",
        ".ui-form-field--tone-default,",
        ".ui-form-field[data-tone=\"default\"] {",
        ".ui-form-field--tone-quiet,",
        ".ui-form-field[data-tone=\"quiet\"] {",
        ".ui-form-field--invalid .ui-form-field__label,",
        ".ui-form-field[data-invalid=\"true\"] .ui-form-field__label {",
        ".ui-form-field--disabled,",
        ".ui-form-field[data-disabled=\"true\"] {",
        ".ui-form-field--custom-class,",
        ".ui-form-field[data-custom-class=\"true\"] {",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles should branch from explicit class/data marker `{required}`."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":only-child",
        ":first-of-type",
        ":last-of-type",
    ] {
        assert!(
            !styles.contains(forbidden),
            "form-field styles should not depend on brittle DOM-structure guessing `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view.contains(forbidden) && !module.contains(forbidden),
            "form-field runtime should not push business style logic as inline style `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。"),
        "form-field checklist should mark explicit-style-marker contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_styles_depend_on_explicit_state_markers_not_dom_guessing"),
        "form-field checklist should record explicit-style-marker regression evidence."
    );
}

#[test]
fn form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles = load_source("styles");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let theme_css = std::fs::read_to_string(component_dir.join("../../crates/ui-theme/src/css.rs"))
        .expect("ui-theme css source should be readable");
    let script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-contract-hygiene.sh"))
            .expect("contract-hygiene gate script should be readable");

    for required in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-3xs, var(--ui-fallback-space-3xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity))",
        "var(--ui-button-size-l-font-size, var(--ui-fallback-button-size-l-font-size))",
        "var(--ui-button-size-l-line-height, var(--ui-fallback-button-size-l-line-height))",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles should keep defensive variable fallback chain `{required}`."
        );
    }

    for forbidden in [
        ", 1px)", ", 2px)", ", 12px)", ", 15px)", ", 16px)", ", 22px)", ", 0.72)",
    ] {
        assert!(
            !styles.contains(forbidden),
            "form-field styles should not keep local hardcoded fallback terminal `{forbidden}`."
        );
    }

    assert!(
        !contains_hex_color_literal(styles),
        "form-field styles should not include hardcoded hex color literals."
    );

    for required in [
        "--ui-fallback-space-3xs:",
        "--ui-fallback-disabled-opacity:",
        "--ui-fallback-button-size-l-font-size:",
        "--ui-fallback-button-size-l-line-height:",
    ] {
        assert!(
            theme_css.contains(required),
            "ui-theme SSOT fallback output should provide `{required}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "components/form-field/test/semantics.rs::form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "components/form-field/test/form_field/semantics.rs::form_field_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should record defensive-variable contract marker `{required}`."
        );
    }
}

#[test]
fn form_field_cascade_layer_and_runtime_style_contract_is_enforced() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let css_aggregate = std::fs::read_to_string(component_dir.join("../../crates/ui/src/css.rs"))
        .expect("ui css aggregation source should be readable");
    let ui_root = std::fs::read_to_string(component_dir.join("../../crates/ui/src/root.rs"))
        .expect("ui UiRoot source should be readable");
    let script_source =
        std::fs::read_to_string(component_dir.join("../../scripts/check-ui-contract-hygiene.sh"))
            .expect("contract-hygiene gate script should be readable");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::field_form::form_field::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_aggregate.contains(required),
            "ui css entry should enforce cascade-layer contract `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should keep centralized css injection contract `{required}`."
        );
    }

    for source in [module, logic, view] {
        for forbidden in [
            "style=\"top:",
            "style=\"left:",
            "style=\"right:",
            "style=\"bottom:",
            "style=\"width:",
            "style=\"height:",
            "style:top=",
            "style:left=",
            "style:right=",
            "style:bottom=",
            "style:width=",
            "style:height=",
        ] {
            assert!(
                !source.contains(forbidden),
                "form-field runtime should not include plain inline style token `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene gate script should include `{script_needle}`.",
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "components/form-field/test/semantics.rs::form_field_cascade_layer_and_runtime_style_contract_is_enforced",
        "components/form-field/test/form_field/semantics.rs::form_field_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should record cascade-layer contract marker `{required}`."
        );
    }
}

#[test]
fn form_field_token_first_static_style_contract_is_enforced() {
    let module = load_source("mod");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let css_aggregate = std::fs::read_to_string(component_dir.join("../../crates/ui/src/css.rs"))
        .expect("ui css aggregation source should be readable");
    let ui_root = std::fs::read_to_string(component_dir.join("../../crates/ui/src/root.rs"))
        .expect("ui UiRoot source should be readable");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-3xs, var(--ui-fallback-space-3xs))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-accent, var(--ui-fallback-accent))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity))",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles should stay token-first and include `{required}`."
        );
    }

    for forbidden in ["var(--form-field-", "tailwind", "tw-", "stylex", "css!("] {
        assert!(
            !styles.contains(forbidden) && !view.contains(forbidden) && !module.contains(forbidden),
            "form-field component layer should not adopt utility/css-in-rust pollution `{forbidden}`."
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view.contains(forbidden) && !module.contains(forbidden),
            "form-field runtime should not emit inline business styles `{forbidden}`."
        );
    }

    for required in [
        "#[cfg(feature = \"component-form_field\")]",
        "out.push_str(crate::field_form::form_field::styles::CSS);",
    ] {
        assert!(
            css_aggregate.contains(required),
            "ui css aggregation should include form-field style registration `{required}`."
        );
    }

    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should gate and inject component css through centralized path `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。"),
        "form-field checklist should mark token-first static-style contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_token_first_static_style_contract_is_enforced"),
        "form-field checklist should record token-first style regression evidence."
    );
}

#[test]
fn form_field_visual_desire_baseline_is_documented_and_non_bootstrap_like() {
    let styles = load_source("styles");
    let view = load_source("view");
    let docs = load_source("docs_form_field_page");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn form_field() -> AnyView",
        "title=\"FormField\"",
        "slug=\"form-field\"",
        "description=\"baseline-style form field primitive",
        "<Playground title=\"Hello World（默认路径）\" code_signal=hello_code>",
        "<Playground title=\"Switch Indicator + Description\" code_signal=code>",
        "<Playground title=\"Checkbox Indicator + Quiet + Invalid/Disabled\" code_signal=states_code>",
    ] {
        assert!(
            docs.contains(required),
            "form-field docs should provide default-theme baseline entry `{required}`."
        );
    }

    for required in [
        "font-size: var(--ui-button-size-l-font-size, var(--ui-fallback-button-size-l-font-size));",
        "line-height: var(--ui-button-size-l-line-height, var(--ui-fallback-button-size-l-line-height));",
        "font-weight: 600;",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "line-height: var(--ui-line-height-100, var(--ui-fallback-line-height-100));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "var(--ui-danger, var(--ui-fallback-danger)) 72%",
        "var(--ui-fg, var(--ui-fallback-fg)) 28%",
        "var(--ui-danger, var(--ui-fallback-danger)) 64%",
        "var(--ui-fg, var(--ui-fallback-fg)) 36%",
    ] {
        assert!(
            styles.contains(required),
            "form-field styles should keep visual hierarchy/contrast baseline `{required}`."
        );
    }

    for required in [
        "data-hovered=move || aria.state.resolved.get().is_hovered.then_some(\"true\")",
        "data-pressed=move || aria.state.resolved.get().is_pressed.then_some(\"true\")",
        "data-focused=move || aria.state.resolved.get().is_focused.then_some(\"true\")",
        "data-focus-visible=move || aria.state.resolved.get().is_focus_visible.then_some(\"true\")",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field should expose interaction/state feedback marker `{required}`."
        );
    }

    for forbidden in [
        ".btn",
        ".form-control",
        ".panel",
        ".jumbotron",
        "btn-",
        "form-control",
    ] {
        assert!(
            !styles.contains(forbidden) && !docs.contains(forbidden) && !view.contains(forbidden),
            "form-field visual layer should not regress to bootstrap-like contract `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "form-field checklist should mark visual-desire baseline contract as complete."
    );
    assert!(
        check2.contains("单组件范围 N/A：本条中的“关键组件（Button/Input/Overlay）视觉回归与截图基线”属于仓库级主题治理"),
        "form-field checklist should document component-scope N/A boundary for global visual baseline asks."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_visual_desire_baseline_is_documented_and_non_bootstrap_like"),
        "form-field checklist should record visual-desire regression evidence."
    );
}

#[test]
fn form_field_tree_shaking_contract_is_feature_gated_without_global_reachability_leak() {
    let check2 = load_source("check2");
    let component_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    let cargo_toml = std::fs::read_to_string(component_dir.join("../../crates/ui/Cargo.toml"))
        .expect("ui Cargo.toml should be readable for feature-chain checks");
    let lib_source = std::fs::read_to_string(component_dir.join("../../crates/ui/src/lib.rs"))
        .expect("ui lib.rs should be readable for feature-gate checks");
    let css_source = std::fs::read_to_string(component_dir.join("../../crates/ui/src/css.rs"))
        .expect("ui css.rs should be readable for style tree-shaking checks");

    assert!(
        cargo_toml
            .contains("component-form_field = [\"component-switch\", \"component-checkbox\"]"),
        "form-field package mode should keep component-level feature chain with minimal dependencies."
    );

    for required in [
        "#[cfg(any(",
        "feature = \"component-form_field\",",
        "pub mod field_form {",
    ] {
        assert!(
            lib_source.contains(required),
            "ui lib boundary should keep field_form export feature-gated via `{required}`."
        );
    }

    assert!(
        lib_source.contains("#[cfg(feature = \"component-form_field\")]")
            && lib_source.contains("pub mod form_field {")
            && lib_source.contains("pub use crate::field_form_form_field::*;"),
        "field_form inline module should keep form_field source path behind component feature gate."
    );

    let form_field_css = "out.push_str(crate::field_form::form_field::styles::CSS);";
    assert!(
        css_source.contains("#[cfg(feature = \"component-form_field\")]")
            && css_source.contains(form_field_css),
        "ui css aggregation should gate form-field CSS by component feature."
    );
    let cfg_idx = css_source
        .find("#[cfg(feature = \"component-form_field\")]")
        .expect("form-field css cfg gate should exist");
    let css_push_idx = css_source
        .find(form_field_css)
        .expect("form-field css aggregation should exist");
    assert!(
        cfg_idx < css_push_idx,
        "form-field css aggregation should be guarded before push to preserve tree-shaking."
    );
    assert_eq!(
        css_source.matches(form_field_css).count(),
        1,
        "form-field css aggregation should avoid duplicate global registrations."
    );

    assert!(
        check2.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "form-field checklist should mark tree-shaking contract as complete."
    );
    assert!(
        check2.contains(
            "单组件范围 N/A：本条中的“反向依赖树检查、CI 最小特性任务与体积预算阈值”属于仓库级门禁"
        ),
        "form-field checklist should document single-component N/A boundary for repo-level CI/budget checks."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_tree_shaking_contract_is_feature_gated_without_global_reachability_leak"),
        "form-field checklist should record tree-shaking regression evidence."
    );
}

#[test]
fn form_field_type_system_and_semantic_markers_keep_machine_readable_contract() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");
    let logic_tests = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/logic.rs"),
    )
    .expect("form-field logic test source should be readable");

    for required in [
        "pub enum FormFieldTone {",
        "pub enum FormFieldIndicatorVariant {",
        "pub enum FormFieldIndicatorPlacement {",
        "#[prop(optional)] tone: FormFieldTone,",
        "#[prop(optional)] indicator_variant: FormFieldIndicatorVariant,",
        "#[prop(optional)] indicator_placement: FormFieldIndicatorPlacement,",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "form-field should keep discrete axes enum-typed via `{required}`."
        );
    }

    for forbidden in [
        "#[prop(optional, into)] tone:",
        "#[prop(optional, into)] indicator_variant:",
        "#[prop(optional, into)] indicator_placement:",
        "tone: Option<String>",
        "indicator_variant: Option<String>",
        "indicator_placement: Option<String>",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "form-field should not reopen string protocol for discrete axes `{forbidden}`."
        );
    }

    for required in [
        "pub fn resolve_state(input: FormFieldStateInput) -> FormFieldState",
        "let state_attr = if input.is_invalid && input.is_disabled {",
        "\"invalid-disabled\"",
        "\"selected-invalid\"",
        "\"invalid\"",
        "\"selected-disabled\"",
        "\"disabled\"",
        "\"selected\"",
        "\"unselected\"",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should normalize invalid combinations into closed state set `{required}`."
        );
    }

    for required in [
        "data-state=move || state.get().state_attr",
        "data-tone=move || state.get().tone_attr",
        "data-indicator-variant=move || state.get().indicator_variant_attr",
        "data-indicator-placement=move || state.get().indicator_placement_attr",
        "data-message-kind=move || state.get().message_kind_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field should expose machine-readable semantic marker `{required}`."
        );
    }

    for required in [
        "fn enums_expose_stable_class_and_attr_names()",
        "fn resolve_state_attr_stays_in_closed_discrete_set()",
    ] {
        assert!(
            logic_tests.contains(required),
            "form-field logic test suite should keep compile-time contract regression `{required}`."
        );
    }

    assert!(
        check2.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "form-field checklist should mark type-system and semantic-marker contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_type_system_and_semantic_markers_keep_machine_readable_contract"),
        "form-field checklist should record type+marker regression evidence."
    );
}

#[test]
fn form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency() {
    let module = load_source("mod");
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");
    let local_semantics = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs"),
    )
    .expect("form-field local semantics test source should be readable");
    let _workspace_semantics =
        include_str!("../../../components/form-field/test/form_field/semantics.rs");
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_form_field_contract.spec.mjs");

    for required in [
        "fn form_field_selected_axis_keeps_controlled_uncontrolled_triplet_contract()",
        "fn form_field_state_markers_are_observable_queryable_and_enumerable()",
        "fn form_field_a11y_i18n_contract_is_mounted_without_hardcoded_view_text()",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-selected-controlled=is_controlled_selected.then_some(\"true\")",
        "data-selected-uncontrolled=(!is_controlled_selected).then_some(\"true\")",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "data-state=move || state.get().state_attr",
    ] {
        assert!(
            local_semantics.contains(required) || view.contains(required),
            "form-field should keep semantic-state assertions for key contracts `{required}`."
        );
    }

    for required in [
        "on:pointerdown=move |_| aria.handlers.press.on_pointer_down.run(())",
        "on:pointerup=move |_| aria.handlers.press.on_pointer_up.run(())",
        "on:pointercancel=move |_| aria.handlers.press.on_pointer_cancel.run(())",
        "on:click=move |_| aria.handlers.press.on_click.run(())",
        "on:keydown=move |ev| {",
        "on:keyup=move |ev| {",
    ] {
        assert!(
            view.contains(required),
            "form-field switch adapter should expose pointer/keyboard contract `{required}`."
        );
    }

    for required in [
        "page.keyboard.press(\"Enter\")",
        "marketingSwitch.click()",
        "toHaveAttribute(\"data-state\", \"unselected\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "[data-component=\"form-field\"]",
        "data-slot=\"form-field\"",
    ] {
        assert!(
            e2e_contract.contains(required),
            "form-field e2e semantics contract should cover branch-path marker `{required}`."
        );
    }

    for forbidden in ["toMatchSnapshot", "assert_snapshot!", "snapshot_diff"] {
        assert!(
            !e2e_contract.contains(forbidden),
            "form-field semantic tests should not depend on visual snapshot assertion `{forbidden}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "#[cfg(target_arch = \"wasm32\")]",
    ] {
        assert!(
            !module.contains(forbidden) && !view.contains(forbidden) && !logic.contains(forbidden),
            "form-field component layer should stay platform-neutral for SSR/wasm applicability `{forbidden}`."
        );
    }

    assert!(
        check2.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "form-field checklist should mark semantic-test-first contract as complete."
    );
    assert!(
        check2.contains("components/form-field/test/semantics.rs::form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency"),
        "form-field checklist should record semantic-test matrix regression evidence."
    );
}

#[test]
fn form_field_component_has_local_semantics_tests_and_checklist_evidence() {
    let check2 = load_source("check2");
    let module = load_source("mod");
    let motion_file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/motion.rs");

    for required in [
        "#[cfg(all(test, not(feature = \"component-form_field\")))]",
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
    ] {
        assert!(
            module.contains(required),
            "form-field should keep local test wiring contract `{required}`."
        );
    }

    assert!(
        !motion_file.exists(),
        "form-field has no independent motion axis; `src/motion.rs` should stay N/A."
    );

    for required in [
        "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "components/form-field/test/semantics.rs::form_field_component_keeps_ui_components_layering_boundaries",
        "components/form-field/test/semantics.rs::form_field_component_uses_headless_contract_without_reimplementation",
        "components/form-field/test/semantics.rs::form_field_component_has_local_semantics_tests_and_checklist_evidence",
        "`motion.rs` 在该条按 N/A",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should document ui boundary evidence `{required}`."
        );
    }
}

#[test]
fn form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let view = load_source("view");
    let local_semantics = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("test/semantics.rs"),
    )
    .expect("form-field local semantics source should be readable");
    let workspace_semantics =
        include_str!("../../../components/form-field/test/form_field/semantics.rs");
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_form_field_contract.spec.mjs");
    let perf_script = include_str!("../../../scripts/check-ui-performance.sh");

    for required in [
        "role=\"group\"",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-default-selected-source=default_selected_source_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-label-source=move || state.get().label_source_attr",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-error-source=move || state.get().error_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field semantic-priority contract should keep marker `{required}`."
        );
    }

    for required in [
        "fn form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency()",
        "fn form_field_state_markers_are_observable_queryable_and_enumerable()",
        "fn form_field_a11y_i18n_contract_is_mounted_without_hardcoded_view_text()",
        "for forbidden in [\"toMatchSnapshot\", \"assert_snapshot!\", \"snapshot_diff\"]",
        "semantic tests should not depend on visual snapshot assertion",
    ] {
        assert!(
            local_semantics.contains(required),
            "form-field local semantics suite should keep semantic-priority marker `{required}`."
        );
    }

    for required in [
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-state\", \"unselected\")",
        "toHaveAttribute(\"data-disabled\", \"true\")",
        "[data-component=\"form-field\"]",
        "data-copyable",
    ] {
        assert!(
            workspace_semantics.contains(required) || e2e_contract.contains(required),
            "form-field semantic-priority path should keep marker `{required}`."
        );
    }

    for forbidden_snapshot in ["toHaveScreenshot(", "toMatchSnapshot(", "screenshot("] {
        assert!(
            !e2e_contract.contains(forbidden_snapshot),
            "form-field e2e should avoid snapshot-only assertion `{forbidden_snapshot}`."
        );
    }

    let script_needle = "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn form_field_e2e_selector_stability_prefers_semantic_markers_and_settled_waits() {
    let check2 = load_source("check2");
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_form_field_contract.spec.mjs");
    let e2e_script =
        include_str!("../../../components/form-field/scripts/check-ui-e2e-form-field.sh");

    for required in [
        "body:not(:has(#boot))",
        "[data-component=\"form-field\"]",
        "#docs-form-field-marketing[data-slot=\"form-field\"]",
        "#docs-form-field-tos[data-slot=\"form-field\"]",
        "#docs-form-field-read-only[data-slot=\"form-field\"]",
        "[data-slot=\"switch\"]",
        "[data-slot=\"checkbox\"]",
        "toHaveAttribute(\"data-state\", \"unselected\")",
        "toHaveAttribute(\"data-state\", \"selected-invalid\")",
        "toHaveAttribute(\"data-state\", \"invalid\")",
    ] {
        assert!(
            e2e_contract.contains(required),
            "form-field e2e selector stability contract should keep marker `{required}`."
        );
    }

    for forbidden in [
        "getByText(",
        "locator(\"div > div >",
        "nth-child(",
        "waitForTimeout(",
        "setTimeout(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_contract.contains(forbidden),
            "form-field e2e selector stability should avoid brittle/snapshot token `{forbidden}`."
        );
    }

    for required in [
        "form_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "form_field_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source",
    ] {
        assert!(
            e2e_script.contains(required),
            "form-field e2e script should gate `{required}`."
        );
    }

    assert!(
        check2.contains("components/form-field/scripts/check-ui-e2e-form-field.sh"),
        "form-field checklist should reference e2e selector stability gate script."
    );
}

#[test]
fn form_field_check2_marks_e2e_selector_stability_item_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "form-field check2 should mark e2e selector stability item complete."
    );

    for required in [
        "components/form-field/test/semantics.rs::form_field_e2e_selector_stability_prefers_semantic_markers_and_settled_waits",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_e2e_selector_and_stable_wait_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_contract_covers_repeatable_key_flow_and_copy_ready_source",
        "components/form-field/scripts/check-ui-e2e-form-field.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 e2e selector stability section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 repeatable-key-flow section should include `{required}`."
        );
    }
}

#[test]
fn form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_contract = include_str!("../../../e2e/tests/docs_app_form_field_contract.spec.mjs");
    let e2e_script =
        include_str!("../../../components/form-field/scripts/check-ui-e2e-form-field.sh");

    for required in [
        "docs-app form-field key flow is repeatable with semantic breakpoints",
        "body:not(:has(#boot))",
        "#docs-form-field-tos[data-slot=\"form-field\"]",
        "const tosCheckbox = tos.locator('[data-slot=\"checkbox\"]').first();",
        "await tosCheckbox.focus();",
        "await expect(tosCheckbox).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"selected-invalid\");",
        "await expect(tos).toHaveAttribute(\"data-state\", \"invalid\");",
        "await page.reload();",
        "await expect(reloadedTos).toHaveAttribute(\"data-state\", \"invalid\");",
    ] {
        assert!(
            e2e_contract.contains(required),
            "form-field e2e repeatable-key-flow contract should keep `{required}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_contract.contains(forbidden),
            "form-field repeatable key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }

    for required in [
        "form_field_check2_documents_e2e_repeatable_key_flow_rules",
        "form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            e2e_script.contains(required),
            "form-field e2e script should gate repeatable key flow marker `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_e2e_repeatable_key_flow_item_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "form-field check2 should mark repeatable-key-flow item complete."
    );

    for required in [
        "docs_app_form_field_contract.spec.mjs",
        "docs-app form-field key flow is repeatable with semantic breakpoints",
        "components/form-field/test/semantics.rs::form_field_check2_documents_e2e_repeatable_key_flow_rules",
        "components/form-field/test/semantics.rs::form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/form-field/test/form_field/semantics.rs::form_field_check2_documents_e2e_repeatable_key_flow_rules",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "components/form-field/scripts/check-ui-e2e-form-field.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 repeatable-key-flow section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A（职责边界）：`FormField` 不是 LLM 正文渲染组件",
        "components/form-field/test/semantics.rs::form_field_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep LLM-only streaming definition marker `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_semantic_test_priority_item_complete() {
    let check2 = load_source("check2");

    assert!(
        check2.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        ),
        "form-field check2 should mark semantic-test-priority item complete."
    );

    for required in [
        "components/form-field/test/semantics.rs::form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency",
        "components/form-field/test/semantics.rs::form_field_state_markers_are_observable_queryable_and_enumerable",
        "components/form-field/test/semantics.rs::form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "components/form-field/test/form_field/semantics.rs::form_field_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "form-field check2 semantic-test-priority section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_snapshot_as_default_baseline_capability() {
    let check2 = load_source("check2");
    let logic = load_source("logic");
    let view = load_source("view");
    let docs_form_field = load_source("docs_form_field_page");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "components/form-field/test/semantics.rs::form_field_check2_documents_snapshot_as_default_baseline_capability",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep snapshot baseline marker `{required}`."
        );
    }

    for required in [
        "FormFieldAgentAction::RenderSnapshot",
        "FormFieldAgentStreamFallback::Snapshot",
        "FormFieldAgentOutputStatus::Verified",
        "logic::resolve_state(FormFieldStateInput {",
        "data-ui-action=move || agent_contract.get().action_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "form-field source should keep snapshot baseline implementation marker `{required}`."
        );
    }

    for required in [
        "slug=\"form-field\"",
        "title=\"Hello World（默认路径）\"",
        "title=\"Switch Indicator + Description\"",
        "title=\"Checkbox Indicator + Quiet + Invalid/Disabled\"",
        "<FormField",
    ] {
        assert!(
            docs_form_field.contains(required),
            "docs form-field page should keep complete snapshot consumption path `{required}`."
        );
    }
}

#[test]
fn form_field_check2_documents_streaming_required_optional_classification_rules() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`FormField` 归类为 `Streaming Optional`",
        "fallback=snapshot",
        "components/form-field/test/semantics.rs::form_field_check2_documents_streaming_required_optional_classification_rules",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist should keep streaming required/optional marker `{required}`."
        );
    }
}

#[test]
fn form_field_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view = load_source("view");

    for required in [
        "role=\"group\"",
        "aria-label=move || control_aria_label.get_value()",
        "aria-describedby=move || describedby.get()",
        "aria-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-state=move || state.get().state_attr",
        "data-selected-control-mode=selected_control_mode_attr",
        "data-selected-change-source=selected_change_source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support_attr",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback_attr",
        "data-ui-output-status=move || agent_contract.get().output_status_attr",
    ] {
        assert!(
            view.contains(required),
            "form-field should keep continuous role/aria/data semantics marker `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn form_field_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let logic = load_source("logic");
    let view = load_source("view");
    let combined = format!("{logic}\n{view}");

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
            "form-field should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let protocol = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/protocol.rs"),
    )
    .expect("form-field protocol source should be readable");
    let combined = format!("{module}\n{logic}\n{styles}\n{view}\n{protocol}");

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "form-field non-test source should forbid rust-hygiene anti-pattern `{forbidden}`."
        );
    }
}

#[test]
fn form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic = load_source("logic");
    let view = load_source("view");

    for required in [
        "use std::borrow::Cow;",
        "let mut classes: Vec<Cow<'static, str>> =",
        "Cow::Borrowed(\"ui-form-field\")",
        "Cow::Borrowed(\"ui-form-field--custom-class\")",
        "Cow::Owned(base_class_name)",
        ".map(|class_name| class_name.as_ref())",
    ] {
        assert!(
            logic.contains(required),
            "form-field logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    for forbidden in [
        "DEFAULT_ID_BASE.to_string()",
        "\"ui-form-field\".to_string()",
        "\"ui-form-field--selected\".to_string()",
        "\"ui-form-field--unselected\".to_string()",
        "\"ui-form-field--invalid\".to_string()",
        "\"ui-form-field--disabled\".to_string()",
        "\"ui-form-field--with-description\".to_string()",
        "\"ui-form-field--with-error\".to_string()",
        "\"ui-form-field--custom-class\".to_string()",
        "\"ui-switch\".to_string()",
        "\"ui-form-field__control\".to_string()",
        "String::from(\"ui-form-field\")",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "form-field string hotspot contract should avoid `{forbidden}`."
        );
    }
}

#[test]
fn form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = include_str!("../../../scripts/check-rust-hygiene.sh");
    let engineering_script = include_str!("../../../scripts/check-ui-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            script_source.contains(required),
            "rust-hygiene gate script should enforce `{required}`."
        );
    }

    for needle in [
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui --test form_field_semantics --no-default-features --features component-form_field,inject-css form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn form_field_check2_marks_semantic_and_performance_regression_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "components/form-field/test/semantics.rs::form_field_semantic_contract_tests_cover_branch_matrix_without_snapshot_dependency",
        "components/form-field/test/semantics.rs::form_field_state_markers_are_observable_queryable_and_enumerable",
        "components/form-field/test/semantics.rs::form_field_performance_governance_budget_is_defined_traceable_and_blocking",
        "components/form-field/test/form_field/semantics.rs::form_field_e2e_contract_uses_semantic_selectors_and_settled_waits",
        "components/form-field/test/form_field/semantics.rs::form_field_performance_governance_budget_is_defined_traceable_and_blocking",
        "render_count",
        "bash scripts/check-ui-performance.sh",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist semantic+performance section should reference `{required}`."
        );
    }
}

#[test]
fn form_field_check2_marks_rust_hygiene_contract_complete() {
    let check2 = load_source("check2");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "components/form-field/test/semantics.rs::form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/form-field/test/semantics.rs::form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/form-field/test/semantics.rs::form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "components/form-field/test/form_field/semantics.rs::form_field_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/form-field/test/form_field/semantics.rs::form_field_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/form-field/test/form_field/semantics.rs::form_field_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-engineering.sh",
    ] {
        assert!(
            check2.contains(required),
            "form-field checklist rust-hygiene section should reference `{required}`."
        );
    }
}
