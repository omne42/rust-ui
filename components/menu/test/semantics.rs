use std::fs;
use std::path::{Path, PathBuf};

fn component_dir() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.parent().and_then(Path::parent);
    let mut candidates = vec![manifest_dir.join(file!())];
    if let Some(root) = workspace_root {
        candidates.push(root.join(file!()));
    }
    candidates.push(PathBuf::from(file!()));

    let resolved_test_path = candidates
        .into_iter()
        .find_map(|candidate| candidate.canonicalize().ok())
        .unwrap_or_else(|| panic!("failed to resolve test file path from file!()={}", file!()));

    resolved_test_path
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| {
            panic!("component root should be parent of test dir for {resolved_test_path:?}")
        })
        .to_path_buf()
}

fn load_component_source(rel_path: &str) -> String {
    let path = component_dir().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn load_workspace_source(rel_path: &str) -> String {
    let path = component_dir().join("../..").join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

fn collect_paths_with_extension(root: &Path, extension: &str, out: &mut Vec<PathBuf>) {
    if root.is_file() {
        if root
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext == extension)
        {
            out.push(root.to_path_buf());
        }
        return;
    }

    let entries =
        fs::read_dir(root).unwrap_or_else(|e| panic!("read_dir failed for {root:?}: {e}"));
    for entry in entries {
        let entry = entry.unwrap_or_else(|e| panic!("read_dir entry failed for {root:?}: {e}"));
        collect_paths_with_extension(&entry.path(), extension, out);
    }
}

fn extract_section(source: &str, section_header: &str) -> String {
    let start = source
        .find(section_header)
        .unwrap_or_else(|| panic!("missing section header `{section_header}`"));
    let tail = &source[start..];
    let end = tail.find("\npub(super) fn ").unwrap_or(tail.len());
    tail[..end].to_string()
}

#[test]
fn menu_component_has_standard_files_for_ui_components_assembly() {
    for rel_path in [
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "src/mod.rs",
    ] {
        let path = component_dir().join(rel_path);
        assert!(
            path.exists(),
            "menu component assembly file `{rel_path}` should exist."
        );
    }
}

#[test]
fn menu_module_exposes_stable_public_api_only() {
    let source = load_component_source("src/mod.rs");

    for needle in ["pub use motion::MenuMotion;", "pub use view::Menu;"] {
        assert!(
            source.contains(needle),
            "menu module should export `{needle}` as stable public API."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !source.contains(forbidden),
            "menu module should keep internals private; found `{forbidden}`."
        );
    }
}

#[test]
fn menu_logic_consumes_state_primitives_instead_of_reimplementing_state_machine() {
    let source = load_component_source("src/logic.rs");

    for needle in [
        "use ui_state_primitives::menu as menu_state;",
        "menu_state::resolve_menu_accessible_name(",
        "menu_state::resolve_menu_state(",
    ] {
        assert!(
            source.contains(needle),
            "menu logic should delegate state invariants to primitives via `{needle}`."
        );
    }
}

#[test]
fn menu_view_mounts_headless_contracts() {
    let source = load_component_source("src/view.rs");

    for needle in [
        "use ui_headless::{MenuItemKind, MenuItemOptions, MenuOptions, use_menu, use_menu_item};",
        "let aria = use_menu(MenuOptions {",
        "let item = use_menu_item(",
        "if aria.handlers.on_key_down.run(ev.key())",
    ] {
        assert!(
            source.contains(needle),
            "menu view should mount headless semantics through `{needle}`."
        );
    }
}

#[test]
fn menu_public_entrypoints_do_not_expose_dom_detail_types() {
    let lib_source = load_component_source("src/lib.rs");
    let mod_source = load_component_source("src/mod.rs");
    let combined = format!("{lib_source}\n{mod_source}");

    for forbidden in ["web_sys", "web-sys", "NodeRef<", "HtmlElement"] {
        assert!(
            !combined.contains(forbidden),
            "menu public entrypoints should not expose DOM detail token `{forbidden}`."
        );
    }
}

#[test]
fn menu_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let check2_source = load_component_source("check2.md");
    let ui_components_lib = load_workspace_source("crates/ui-components/src/lib.rs");
    let ui_components_css = load_workspace_source("crates/ui-components/src/css.rs");
    let ui_components_root = load_workspace_source("crates/ui-components/src/root.rs");
    let active_highlight =
        load_workspace_source("crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state = load_workspace_source("crates/ui-headless/src/controllable_state.rs");
    let presence = load_workspace_source("crates/ui-headless/src/presence.rs");
    let a11y = load_workspace_source("crates/ui-headless/src/a11y.rs");
    let entrypoints_script = load_workspace_source("scripts/check-ui-components-entrypoints.sh");

    for required in [
        "#[cfg(feature = \"component-menu\")]",
        "pub mod menu;",
        "#[cfg(feature = \"component-menu_trigger\")]",
        "pub mod menu_trigger;",
        "#[cfg(feature = \"component-menubar\")]",
        "pub mod menubar;",
        "#[cfg(feature = \"component-navigation_menu\")]",
        "pub mod navigation_menu;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "pub use menu::{Menu, MenuItemSpec};",
        "pub use menu_trigger::{MenuTrigger, MenuTriggerMotion};",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components lib.rs should keep fixed entry marker `{required}`."
        );
    }

    for forbidden in [
        "pub use web_sys",
        "pub use leptos::web_sys",
        "pub use wasm_bindgen",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui-components lib.rs should not leak platform detail `{forbidden}`."
        );
    }

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-menu\")]",
        "out.push_str(crate::menu::styles::CSS);",
        "#[cfg(feature = \"component-menu_item\")]",
        "out.push_str(crate::menu::item::styles::CSS);",
        "#[cfg(feature = \"component-menu_section\")]",
        "out.push_str(crate::menu::section::styles::CSS);",
        "#[cfg(feature = \"component-menu_trigger\")]",
        "out.push_str(crate::menu_trigger::styles::CSS);",
        "#[cfg(feature = \"component-menubar\")]",
        "out.push_str(crate::menubar::styles::CSS);",
        "#[cfg(feature = \"component-navigation_menu\")]",
        "out.push_str(crate::navigation_menu::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css.rs should keep fixed entry marker `{required}`."
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
            "ui-components root.rs should keep centralized injection marker `{required}`."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight shared primitive should contain `{required}`."
        );
    }

    for forbidden in ["Menu", "aria-", "data-state", "data-slot"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should stay generic and avoid component semantic token `{forbidden}`."
        );
    }

    for required in [
        "pub struct ControllableState<T>",
        "pub fn use_controllable_state<T>(",
    ] {
        assert!(
            controllable_state.contains(required),
            "ui-headless controllable_state canonical path should contain `{required}`."
        );
    }

    for required in [
        "pub struct Presence",
        "pub fn use_presence(is_open: Signal<bool>) -> Presence",
    ] {
        assert!(
            presence.contains(required),
            "ui-headless presence canonical path should contain `{required}`."
        );
    }

    for required in ["pub fn locale_attrs(", "pub fn aria_controls_when_open("] {
        assert!(
            a11y.contains(required),
            "ui-headless a11y canonical path should contain `{required}`."
        );
    }

    let ui_components_src_dir = component_dir()
        .join("../..")
        .join("crates/ui-components/src");
    for forbidden_file in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !ui_components_src_dir.join(forbidden_file).exists(),
            "ui-components/src/{forbidden_file} should be absent by fixed-entrypoint contract."
        );
    }

    let script_needle =
        "cargo test -p ui-menu menu_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints gate script should include `{script_needle}`."
    );

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "menu_ui_components_fixed_entry_files_follow_layered_boundaries",
    ] {
        assert!(
            check2_source.contains(required),
            "checklist should keep fixed-entrypoint governance marker `{required}`."
        );
    }
}

#[test]
fn menu_component_directory_standard_files_follow_contract_and_na_spec() {
    let mod_source = load_component_source("src/mod.rs");
    let logic_source = load_component_source("src/logic.rs");
    let styles_source = load_component_source("src/styles.rs");
    let view_source = load_component_source("src/view.rs");
    let motion_source = load_component_source("src/motion.rs");
    let check2_source = load_component_source("check2.md");
    let src_check2_source = load_component_source("src/check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");

    let src_dir = component_dir().join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "menu component should keep required standard file `{required}`."
        );
    }
    for forbidden in ["render.rs", "spec.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "menu component should keep non-required file absent `{forbidden}`."
        );
    }

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::MenuMotion;",
        "pub use view::Menu;",
    ] {
        assert!(
            mod_source.contains(needle),
            "mod.rs should keep minimal stable export boundary marker `{needle}`."
        );
    }
    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not over-export implementation module marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn normalize_props(input: MenuNormalizeInput) -> MenuNormalizedProps",
        "pub fn normalize_menu_items(input: MenuItemsInput) -> MenuItemsNormalized",
        "pub fn resolve_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "NodeRef",
        "on:click",
        "on:keydown",
        "web_sys",
        "wasm_bindgen",
    ] {
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
        "let aria = use_menu(MenuOptions {",
        "let item = use_menu_item(",
        "logic::normalize_props(",
        "logic::normalize_menu_items(",
        "crate::menu::motion::attach_motion(",
    ] {
        assert!(
            view_source.contains(needle),
            "view.rs should keep Leptos structure + headless mount marker `{needle}`."
        );
    }
    for forbidden in [
        "pub const CSS: &str",
        "attach_active_highlight_motion(",
        "mod render;",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should avoid style-engine/render-module drift marker `{forbidden}`."
        );
    }

    for needle in [
        "pub struct MenuMotion {",
        "pub fn sanitize_motion(motion: MenuMotion) -> MenuMotion",
        "pub fn attach_motion(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            motion_source.contains(needle),
            "motion.rs should keep semantic-to-motion mapping marker `{needle}`."
        );
    }
    for forbidden in ["view! {", "use_menu(", "on:click", "on:keydown"] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not include render or interaction semantics marker `{forbidden}`."
        );
    }

    let script_needle =
        "cargo test -p ui-menu menu_component_directory_standard_files_follow_contract_and_na_spec";
    assert!(
        script_source.contains(script_needle),
        "component-files script should include `{script_needle}`."
    );

    for check in [&check2_source, &src_check2_source] {
        for needle in [
            "- [x] 组件目录标准文件落点正确。",
            "menu_component_directory_standard_files_follow_contract_and_na_spec",
        ] {
            assert!(
                check.contains(needle),
                "menu check2 should keep component-directory evidence marker `{needle}`."
            );
        }
    }
}

#[test]
fn menu_file_placement_discipline_is_strict_for_component_scope() {
    let src_dir = component_dir().join("src");
    for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "menu should keep strict file-placement discipline with `{required}` present."
        );
    }
    assert!(
        !src_dir.join("render.rs").exists(),
        "menu should not introduce render.rs; view.rs is the fixed rendering slot."
    );
    assert!(
        !src_dir.join("spec.rs").exists(),
        "menu keeps spec.rs as N/A for current simple scope; only complex schema-heavy components should add it."
    );

    let mod_source = load_component_source("src/mod.rs");
    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::MenuMotion;",
        "pub use view::Menu;",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep fixed file-placement export marker `{required}`."
        );
    }
    for forbidden in [
        "mod render;",
        "pub mod render;",
        "mod spec;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not drift file placement via `{forbidden}`."
        );
    }

    let logic_source = load_component_source("src/logic.rs");
    assert!(
        logic_source.contains("pub fn normalize_props("),
        "logic.rs should preserve normalization responsibilities."
    );
    for forbidden in ["view! {", "on:click", "on:keydown"] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not absorb view/event responsibilities token `{forbidden}`."
        );
    }

    let styles_source = load_component_source("src/styles.rs");
    assert!(
        styles_source.contains("pub const CSS: &str"),
        "styles.rs should keep static CSS contract slot."
    );

    let view_source = load_component_source("src/view.rs");
    for required in ["#[component]", "view! {"] {
        assert!(
            view_source.contains(required),
            "view.rs should keep rendering slot marker `{required}`."
        );
    }

    let motion_source = load_component_source("src/motion.rs");
    assert!(
        motion_source.contains("pub fn attach_motion("),
        "motion.rs should keep motion attach slot."
    );

    let check2_source = load_component_source("check2.md");
    assert!(
        check2_source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`")
            && check2_source
                .contains("menu_file_placement_discipline_is_strict_for_component_scope"),
        "check2 should record AI struct-first file-placement discipline and regression reference."
    );
}

#[test]
fn menu_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    let spec_path = component_dir().join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "menu should keep spec.rs absent unless complexity reaches external schema/builder threshold."
    );

    let mod_source = load_component_source("src/mod.rs");
    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub struct MenuSpec",
        "impl MenuSpec",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "menu mod.rs should not expose Hyper-Structure builder token `{forbidden}` for this simple component."
        );
    }

    let mut rust_sources = Vec::new();
    collect_paths_with_extension(&component_dir().join("src"), "rs", &mut rust_sources);
    for source_path in rust_sources {
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {source_path:?}: {e}"));
        assert!(
            !source.contains("MenuSpec::new("),
            "menu source `{source_path:?}` should not contain complex-only builder entry `MenuSpec::new(`."
        );
    }

    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");
    let script_needle = "cargo test -p ui-menu menu_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        script_source.contains(script_needle),
        "component-files gate should include `{script_needle}`."
    );

    let check2_source = load_component_source("check2.md");
    assert!(
        check2_source.contains(
            "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"
        ) && check2_source.contains(
            "本组件判定：N/A（`menu` 为 simple component，当前不需要独立 `spec.rs` 与 `*Spec::new()...render()` builder 链路）。"
        ) && check2_source
            .contains("menu_hyper_structure_builder_spec_is_not_applicable_for_simple_component"),
        "checklist should explicitly record Hyper-Structure Builder N/A rationale and regression binding."
    );
}

#[test]
fn menu_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_path = component_dir().join("src/Component.toml");
    let rbi_path = component_dir().join("src/menu.rbi");
    assert!(
        manifest_path.exists(),
        "menu context-compression manifest should exist at `{}`.",
        manifest_path.display()
    );
    assert!(
        rbi_path.exists(),
        "menu RBI signature projection should exist at `{}`.",
        rbi_path.display()
    );

    let manifest_source = load_component_source("src/Component.toml");
    let rbi_source = load_component_source("src/menu.rbi");
    let script_source = load_workspace_source("scripts/check-ui-components-component-files.sh");
    let check2_source = load_component_source("check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Menu\"",
        "crate = \"ui-menu\"",
        "rbi = \"menu.rbi\"",
        "name = \"id_base\"",
        "name = \"item_specs\"",
        "name = \"on_action\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "menu Component.toml should keep context-compression marker `{needle}`."
        );
    }

    for needle in [
        "pub struct MenuItemSpec {",
        "pub fn action(label: impl Into<String>) -> Self;",
        "pub struct MenuMotion {",
        "pub fn sanitize_motion(",
        "pub fn attach_motion(",
        "pub fn Menu(",
    ] {
        assert!(
            rbi_source.contains(needle),
            "menu RBI projection should keep interface signature marker `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_needle),
        "component-files gate script should include `{script_needle}`."
    );

    for needle in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/menu/src/Component.toml",
        "components/menu/src/menu.rbi",
        "menu_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "scripts/check-ui-components-component-files.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "menu check2 should record context-compression manifest/rbi evidence marker `{needle}`."
        );
    }
}

#[test]
fn menu_check2_documents_agent_contract_schema_governance_rules() {
    let check2_source = load_component_source("check2.md");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "menu_agent_contract_is_schema_typed_and_machine_readable",
        "menu_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "menu_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "menu check2 should record Agent Contract governance marker `{required}`."
        );
    }

    for script_needle in [
        "cargo test -p ui-menu menu_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-menu menu_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-menu menu_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(script_needle),
            "contract-hygiene script should include `{script_needle}`."
        );
    }
}

#[test]
fn menu_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_component_source("src/logic.rs");
    let view_source = load_component_source("src/view.rs");
    let component_manifest = load_component_source("src/Component.toml");
    let component_rbi = load_component_source("src/menu.rbi");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_component_source("check2.md");

    for typed_source in [
        "pub const MENU_AGENT_SCHEMA: &str = \"ui.menu.agent-contract\";",
        "pub enum MenuAgentSchemaVersion",
        "pub enum MenuAgentIntent",
        "pub enum MenuAgentAction",
        "pub enum MenuAgentState",
        "pub enum MenuAgentSource",
        "pub struct MenuAgentContract",
        "pub struct MenuAgentContractInput",
        "fn resolve_agent_state(input: MenuAgentContractInput) -> MenuAgentState",
        "pub fn resolve_agent_contract(input: MenuAgentContractInput) -> MenuAgentContract",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "menu Agent Contract should stay type-derived via `{typed_source}`."
        );
    }

    for marker in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-state-source=move || agent_contract.get().state_source",
        "data-ui-motion-source=move || agent_contract.get().motion_source",
        "data-ui-items-source=move || agent_contract.get().items_source",
        "data-ui-config-policy=move || agent_contract.get().config_policy",
    ] {
        assert!(
            view_source.contains(marker),
            "menu view should mount Agent Contract marker `{marker}`."
        );
    }

    for required in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "[[agent_contract]]",
        "schema = \"ui.menu.agent-contract.v1\"",
        "intent = \"menu.interaction\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "MENU_AGENT_SCHEMA",
        "MenuAgentContract",
        "resolve_agent_contract",
    ] {
        assert!(
            component_manifest.contains(required) || component_rbi.contains(required),
            "menu context-compression assets should keep Agent Contract marker `{required}`."
        );
    }

    let script_needle =
        "cargo test -p ui-menu menu_agent_contract_is_schema_typed_and_machine_readable";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    assert!(
        check2_source.contains("menu_agent_contract_is_schema_typed_and_machine_readable"),
        "menu check2 should reference machine-readable Agent Contract regression."
    );
}

#[test]
fn menu_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_component_source("src/logic.rs");
    let view_source = load_component_source("src/view.rs");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_component_source("check2.md");

    for marker in [
        "MenuAgentSchemaVersion::V1 => \"v1\"",
        "MenuAgentIntent::MenuInteraction => \"menu.interaction\"",
        "MenuAgentAction::NavigateSelect => \"navigate-select\"",
        "MenuAgentState::Disabled => \"disabled\"",
        "MenuAgentState::ReadyChecked => \"ready-checked\"",
        "MenuAgentSource::StatePrimitives => \"state-primitives\"",
    ] {
        assert!(
            logic_source.contains(marker),
            "menu Agent Contract should keep closed typed mapping marker `{marker}`."
        );
    }

    for forbidden in [
        "data-ui-schema=\"",
        "data-ui-intent=\"",
        "data-ui-action=\"",
        "data-ui-state=\"",
        "data-ui-source=\"",
        "format!(\"data-ui-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "menu view should not splice free-form Agent Contract marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    assert!(
        check2_source.contains(
            "menu_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing"
        ),
        "menu check2 should reference type-derived Agent Contract regression."
    );
}

#[test]
fn menu_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_component_source("src/view.rs");
    let component_manifest = load_component_source("src/Component.toml");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_component_source("check2.md");

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "\"logic::normalize_props(...)\"",
        "\"logic::normalize_menu_items(...)\"",
        "\"logic::resolve_state(...)\"",
        "\"logic::resolve_agent_contract(...)\"",
        "\"menu_motion::attach_motion(...)\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\", \"eval(\"]",
        "name = \"agent_contract_whitelist_boundary\"",
    ] {
        assert!(
            component_manifest.contains(required),
            "menu Component.toml should keep whitelist boundary marker `{required}`."
        );
    }

    for forbidden in [
        "inner_html=",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
        "eval(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "menu render path should remain whitelist-safe and injection-free; forbidden marker `{forbidden}` was found."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_agent_contract_render_path_is_whitelist_safe_and_script_injection_free";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    assert!(
        check2_source.contains(
            "menu_agent_contract_render_path_is_whitelist_safe_and_script_injection_free"
        ),
        "menu check2 should reference whitelist-safe Agent Contract regression."
    );
}

#[test]
fn menu_streaming_term_is_limited_to_llm_output_render_modes() {
    let logic_source = load_component_source("src/logic.rs");
    let view_source = load_component_source("src/view.rs");
    let component_manifest = load_component_source("src/Component.toml");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");
    let check2_source = load_component_source("check2.md");

    for marker in [
        "pub enum MenuAgentStreamMode",
        "MenuAgentStreamMode::Streaming => \"streaming\"",
        "MenuAgentStreamMode::Snapshot => \"snapshot\"",
        "stream_support: MenuAgentStreamSupport::Unsupported,",
        "stream_fallback: MenuAgentStreamFallback::Snapshot,",
        "stream_mode: MenuAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "menu logic should keep typed stream/snapshot marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "menu view should expose stream/snapshot contract marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "term_scope = \"llm-output-rendering\"",
        "defined_modes = [\"streaming\", \"snapshot\"]",
        "required = false",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-support\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
    ] {
        assert!(
            component_manifest.contains(marker),
            "menu manifest should keep stream/snapshot boundary marker `{marker}`."
        );
    }

    let script_needle =
        "cargo test -p ui-menu menu_streaming_term_is_limited_to_llm_output_render_modes";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    for marker in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "menu_streaming_term_is_limited_to_llm_output_render_modes",
    ] {
        assert!(
            check2_source.contains(marker),
            "menu check2 should keep streaming-definition marker `{marker}`."
        );
    }
}

#[test]
fn menu_snapshot_is_foundational_and_complete_config_renders_stably() {
    let check2_source = load_component_source("check2.md");
    let logic_source = load_component_source("src/logic.rs");
    let view_source = load_component_source("src/view.rs");
    let component_manifest = load_component_source("src/Component.toml");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "menu_snapshot_is_foundational_and_complete_config_renders_stably",
    ] {
        assert!(
            check2_source.contains(marker),
            "menu check2 should keep snapshot-foundation marker `{marker}`."
        );
    }

    for marker in [
        "pub fn Menu(",
        "id_base: String,",
        "#[prop(optional, into)] items: Arc<[String]>",
        "on_action: Callback<usize>,",
        "#[prop(optional)] item_specs: Vec<MenuItemSpec>,",
        "#[prop(optional, into)] id: Option<String>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] aria_labelledby: Option<String>,",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] disabled: bool,",
        "#[prop(optional)] disabled_indices: Vec<usize>,",
        "#[prop(optional)] item_kinds: Vec<MenuItemKind>,",
        "#[prop(optional, default = 0)] default_index: usize,",
        "#[prop(optional)] motion: MenuMotion,",
        "#[prop(optional, into)] class_name: Option<String>,",
    ] {
        assert!(
            view_source.contains(marker),
            "menu snapshot render should keep complete-config marker `{marker}`."
        );
    }

    for marker in [
        "MenuAgentOutputStatus::Verified => \"verified\"",
        "output_status: MenuAgentOutputStatus::Verified,",
        "MenuAgentStreamMode::Snapshot => \"snapshot\"",
        "stream_mode: MenuAgentStreamMode::Snapshot,",
    ] {
        assert!(
            logic_source.contains(marker),
            "menu logic should keep snapshot/output-status marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "menu view should expose snapshot output marker `{marker}`."
        );
    }

    for marker in [
        "name = \"snapshot_rendering\"",
        "[streaming_policy]",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_manifest.contains(marker),
            "menu manifest should keep snapshot-foundation marker `{marker}`."
        );
    }

    let script_needle =
        "cargo test -p ui-menu menu_snapshot_is_foundational_and_complete_config_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn menu_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status() {
    let check2_source = load_component_source("check2.md");
    let logic_source = load_component_source("src/logic.rs");
    let view_source = load_component_source("src/view.rs");
    let component_manifest = load_component_source("src/Component.toml");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    for marker in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "menu_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status",
    ] {
        assert!(
            check2_source.contains(marker),
            "menu check2 should keep streaming-requirement marker `{marker}`."
        );
    }

    for marker in [
        "[streaming_policy]",
        "required = false",
        "owner = \"upstream\"",
        "fallback = \"snapshot\"",
        "default = \"snapshot\"",
        "attr = \"data-ui-stream-fallback\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"snapshot\"]",
        "attr = \"data-ui-output-status\"",
        "values = [\"verified\"]",
    ] {
        assert!(
            component_manifest.contains(marker),
            "menu manifest should keep streaming-optional marker `{marker}`."
        );
    }

    for marker in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "role=aria.attrs.role",
        "aria-disabled=aria.attrs.aria_disabled",
    ] {
        assert!(
            view_source.contains(marker),
            "menu view should keep continuous stream/output/a11y marker `{marker}`."
        );
    }

    for marker in [
        "stream_support: MenuAgentStreamSupport::Unsupported,",
        "stream_fallback: MenuAgentStreamFallback::Snapshot,",
        "stream_mode: MenuAgentStreamMode::Snapshot,",
        "output_status: MenuAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(marker),
            "menu logic should keep stream/output decision marker `{marker}`."
        );
    }

    let combined = format!("{logic_source}\n{view_source}");
    for forbidden in [
        "on_retry",
        "retry_count",
        "is_loading",
        "set_loading",
        "fetch(",
        "reqwest::",
        "tokio::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "menu component layer should not absorb upstream retry/validation concerns via `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );
}

#[test]
fn menu_boolean_props_expose_is_prefixed_names_with_disabled_alias_compatibility() {
    for (rel_path, logic_path, logic_needle) in [
        (
            "src/view.rs",
            "src/logic.rs",
            "pub fn normalize_props(input: MenuNormalizeInput)",
        ),
        (
            "src/trigger/view.rs",
            "src/trigger/logic.rs",
            "pub fn normalize_discrete_props(input: MenuTriggerDiscreteInput)",
        ),
        (
            "src/dropdown_menu/view.rs",
            "src/dropdown_menu/logic.rs",
            "pub fn normalize_discrete_props(input: DropdownMenuDiscreteInput)",
        ),
        (
            "src/item/view.rs",
            "src/item/logic.rs",
            "pub fn normalize_interaction(input: MenuItemInteractionInput)",
        ),
        (
            "src/section/view.rs",
            "src/section/logic.rs",
            "pub fn normalize_props(input: MenuSectionNormalizeInput)",
        ),
    ] {
        let source = load_component_source(rel_path);
        let logic_source = load_component_source(logic_path);
        assert!(
            source.contains("#[prop(optional)] is_disabled: Option<bool>"),
            "{rel_path} should expose is_disabled as the canonical boolean prop."
        );
        assert!(
            !source.contains("is_disabled.unwrap_or(disabled)"),
            "{rel_path} should delegate disabled alias precedence to logic.rs."
        );
        assert!(
            logic_source.contains(logic_needle),
            "{logic_path} should own disabled alias normalization via `{logic_needle}`."
        );
    }
}

#[test]
fn menu_open_axis_keeps_open_default_open_on_open_change_and_is_open_bridge() {
    for rel_path in [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown_menu/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for needle in [
            "#[prop(optional)] is_open: Option<Signal<bool>>",
            "#[prop(optional)] open: Option<Signal<bool>>",
            "#[prop(optional)] default_open: Option<bool>",
            "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        ] {
            assert!(
                source.contains(needle),
                "{rel_path} should include `{needle}` for naming compatibility."
            );
        }
        assert!(
            source.contains("normalize_open_state("),
            "{rel_path} should delegate open-axis priority to logic::normalize_open_state."
        );
        assert!(
            !source.contains("is_open.or(open)"),
            "{rel_path} should not resolve open-axis priority directly in view."
        );
    }

    let dropdown_source = load_component_source("src/dropdown/view.rs");
    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
    ] {
        assert!(
            dropdown_source.contains(needle),
            "dropdown view should keep the unified open-axis naming via `{needle}`."
        );
    }
}

#[test]
fn menu_open_axis_normalization_comes_from_state_primitives() {
    for rel_path in [
        "src/trigger/logic.rs",
        "src/dropdown_menu/logic.rs",
        "src/dropdown/logic.rs",
        "src/context_menu/logic.rs",
        "src/action_menu/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("normalize_controlled_prop_alias("),
            "{rel_path} should resolve open alias priority via ui-state-primitives."
        );
        assert!(
            source.contains("is_controlled_prop(&open)"),
            "{rel_path} should resolve control mode via ui-state-primitives."
        );
        assert!(
            !source.contains("is_open.or(input.open)"),
            "{rel_path} should not reimplement open alias priority in component logic."
        );
    }
}

#[test]
fn menu_close_on_action_boolean_keeps_is_prefixed_bridge() {
    for rel_path in [
        "src/trigger/view.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown/view.rs",
        "src/context_menu/view.rs",
        "src/menubar/view.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("#[prop(optional)] is_close_on_action: Option<bool>"),
            "{rel_path} should expose is_close_on_action for is_* naming compatibility."
        );
        assert!(
            !source.contains("is_close_on_action.unwrap_or(close_on_action)"),
            "{rel_path} should delegate close_on_action alias precedence to logic.rs."
        );
    }

    for (logic_path, needle) in [
        (
            "src/trigger/logic.rs",
            "pub fn normalize_discrete_props(input: MenuTriggerDiscreteInput)",
        ),
        (
            "src/dropdown_menu/logic.rs",
            "pub fn normalize_discrete_props(input: DropdownMenuDiscreteInput)",
        ),
        (
            "src/dropdown/logic.rs",
            "pub fn normalize_close_on_action(input: ActionModeInput)",
        ),
        (
            "src/context_menu/logic.rs",
            "pub fn normalize_discrete_props(input: ContextMenuDiscreteInput)",
        ),
        (
            "src/menubar/logic.rs",
            "pub fn normalize_close_on_action(input: MenubarActionModeInput)",
        ),
    ] {
        let source = load_component_source(logic_path);
        assert!(
            source.contains(needle),
            "{logic_path} should define close_on_action alias normalization via `{needle}`."
        );
    }
}

#[test]
fn view_default_resolution_is_centralized_in_logic() {
    for rel_path in [
        "src/view.rs",
        "src/trigger/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/context_menu/view.rs",
        "src/action_menu/view.rs",
        "src/item/view.rs",
        "src/section/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in ["unwrap_or(", "unwrap_or_else(", ".or("] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not resolve defaults/priority in view.rs; found `{forbidden}`."
            );
        }
    }
}

#[test]
fn view_event_and_state_decisions_delegate_to_logic_layer() {
    for rel_path in [
        "src/trigger/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/context_menu/view.rs",
        "src/action_menu/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "ui_headless::menu_trigger_open_focus_strategy(",
            "ui_headless::context_menu_open_focus_strategy(",
            "ui_headless::menubar_key_command(",
            "ui_headless::navigation_menu_key_command(",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should delegate interaction semantics to logic.rs, found `{forbidden}`."
            );
        }
    }

    for (logic_path, needle) in [
        (
            "src/trigger/logic.rs",
            "pub fn resolve_open_focus_strategy(",
        ),
        (
            "src/dropdown/logic.rs",
            "pub fn resolve_open_focus_strategy(",
        ),
        (
            "src/dropdown_menu/logic.rs",
            "pub fn resolve_open_focus_strategy(",
        ),
        (
            "src/context_menu/logic.rs",
            "pub fn resolve_open_focus_strategy(",
        ),
        (
            "src/action_menu/logic.rs",
            "pub fn resolve_open_focus_strategy(",
        ),
        ("src/menubar/logic.rs", "pub fn resolve_key_decision("),
        (
            "src/navigation_menu/logic.rs",
            "pub fn resolve_key_decision(",
        ),
    ] {
        let source = load_component_source(logic_path);
        assert!(
            source.contains(needle),
            "{logic_path} should define typed interaction decision helper `{needle}`."
        );
    }
}

#[test]
fn discrete_modes_and_statuses_are_type_constrained_with_enums() {
    for (logic_path, required_tokens) in [
        (
            "src/action_menu/logic.rs",
            vec![
                "pub enum ActionMenuDisabledState",
                "pub enum ActionMenuActionMode",
            ],
        ),
        (
            "src/trigger/logic.rs",
            vec![
                "pub enum MenuTriggerActionMode",
                "pub action_mode: MenuTriggerActionMode",
            ],
        ),
        (
            "src/dropdown_menu/logic.rs",
            vec![
                "pub enum DropdownMenuActionMode",
                "pub action_mode: DropdownMenuActionMode",
            ],
        ),
        (
            "src/dropdown/logic.rs",
            vec![
                "pub enum DropdownActionMode",
                "pub fn normalize_close_on_action(input: ActionModeInput) -> DropdownActionMode",
            ],
        ),
        (
            "src/context_menu/logic.rs",
            vec![
                "pub enum ContextMenuDisabledState",
                "pub enum ContextMenuActionMode",
            ],
        ),
        (
            "src/menubar/logic.rs",
            vec![
                "pub enum MenubarActionMode",
                "pub fn normalize_close_on_action(input: MenubarActionModeInput) -> MenubarActionMode",
            ],
        ),
    ] {
        let source = load_component_source(logic_path);
        for token in required_tokens {
            assert!(
                source.contains(token),
                "{logic_path} should type-constrain discrete state via `{token}`."
            );
        }
    }
}

#[test]
fn controllable_axes_provide_value_default_and_change_triplets() {
    let open_axis_components = [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/trigger/view.rs",
    ];
    for rel_path in open_axis_components {
        let source = load_component_source(rel_path);
        for needle in [
            "#[prop(optional)] open: Option<Signal<bool>>",
            "#[prop(optional)] default_open: Option<bool>",
            "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        ] {
            assert!(
                source.contains(needle),
                "{rel_path} should include controlled/uncontrolled open triplet `{needle}`."
            );
        }
    }

    let menubar_source = load_component_source("src/menubar/view.rs");
    for needle in [
        "#[prop(optional)] open_index: Option<Signal<Option<usize>>>",
        "#[prop(optional)] default_open_index: Option<usize>",
        "#[prop(optional)] on_open_index_change: Option<Callback<Option<usize>>>",
    ] {
        assert!(
            menubar_source.contains(needle),
            "menubar should include controlled/uncontrolled open_index triplet `{needle}`."
        );
    }

    let navigation_source = load_component_source("src/navigation_menu/view.rs");
    for needle in [
        "#[prop(optional)] selected_id: Option<Signal<Option<String>>>",
        "#[prop(optional, into)] default_selected_id: Option<String>",
        "#[prop(optional)] on_selected_id_change: Option<Callback<Option<String>>>",
    ] {
        assert!(
            navigation_source.contains(needle),
            "navigation_menu should include selected_id controlled/uncontrolled triplet `{needle}`."
        );
    }
}

#[test]
fn controllable_axes_use_controllable_primitives_and_expose_mode_markers() {
    for rel_path in [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("use_controllable_open_state_traced("),
            "{rel_path} should delegate open-state control to shared controllable primitive."
        );
        assert!(
            source.contains("data-controlled=") && source.contains("data-uncontrolled="),
            "{rel_path} should expose controlled/uncontrolled mode markers for regression checks."
        );
    }

    let menubar_source = load_component_source("src/menubar/view.rs");
    assert!(
        menubar_source.contains("use_controllable_state("),
        "menubar should delegate open_index control to shared controllable primitive."
    );
    assert!(
        menubar_source.contains("data-controlled=")
            && menubar_source.contains("data-uncontrolled="),
        "menubar should expose controlled/uncontrolled markers."
    );

    let navigation_source = load_component_source("src/navigation_menu/view.rs");
    assert!(
        navigation_source.contains("use_controllable_state("),
        "navigation_menu should delegate selected_id control to shared controllable primitive."
    );
    assert!(
        navigation_source
            .contains("data-selection-mode=move || root_state.get().selection_mode_attr"),
        "navigation_menu should expose selected mode marker for controlled/uncontrolled regressions."
    );
}

#[test]
fn menu_async_interaction_contract_is_not_applicable_and_documented() {
    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "is_loading",
            "aria-busy",
            "use_async_action",
            "create_resource(",
            "spawn_local(",
            "retry",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not define async interaction contract token `{forbidden}`."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("本组件判定：N/A（组件无远程请求与异步状态；当前仅包含同步菜单开关、焦点与选择交互，不涉及 `is_loading`/`aria-busy`/retry 协议）。"),
        "check2.md should document why async contract is N/A for menu."
    );
}

#[test]
fn menu_api_dx_paradox_keeps_minimal_path_simple_and_docs_visible() {
    let menu_view = load_component_source("src/view.rs");
    for required in [
        "pub fn Menu(",
        "id_base: String,",
        "#[prop(optional, into)] items: Arc<[String]>",
        "#[prop(optional)] item_specs: Vec<MenuItemSpec>",
        "on_action: Callback<usize>,",
    ] {
        assert!(
            menu_view.contains(required),
            "menu public API should keep minimal required props via `{required}`."
        );
    }
    assert!(
        !menu_view.contains("state:"),
        "menu public API should not require internal state object wiring."
    );

    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/collections.rs");
    let menu_docs_block = docs_source
        .split("pub(super) fn menu() -> AnyView {")
        .nth(1)
        .and_then(|tail| {
            tail.split("pub(super) fn menu_trigger() -> AnyView {")
                .next()
        })
        .expect("collections.rs should contain menu() section");
    assert!(
        menu_docs_block
            .contains("<Playground\n                title=\"Hello World (Default Path)\"")
            && menu_docs_block.contains("code_signal=hello_code"),
        "docs-app should expose explicit menu hello-world default path."
    );

    let hello_block = menu_docs_block
        .split("let hello_code = Signal::derive(move || {")
        .nth(1)
        .and_then(|tail| tail.split("let code = Signal::derive(move || {").next())
        .expect("menu docs should define hello_code block before code block");
    let hello_literal_start = hello_block
        .find("r#\"")
        .map(|index| index + 3)
        .expect("hello_code should use raw string literal");
    let hello_literal_end = hello_block[hello_literal_start..]
        .find("\"#")
        .map(|index| index + hello_literal_start)
        .expect("hello_code raw string should be closed");
    let hello_snippet = &hello_block[hello_literal_start..hello_literal_end];

    let non_empty_lines = hello_snippet
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .count();
    assert!(
        non_empty_lines <= 5,
        "menu hello-world should stay within 5 non-empty lines, got {non_empty_lines} lines:\n{hello_snippet}"
    );

    for required in [
        "<Menu",
        "id_base=\"menu-hello\".to_string()",
        "item_specs=vec![MenuItemSpec::action(\"New file\"), MenuItemSpec::action(\"Share with team\")]",
        "on_action=Callback::new(move |_: usize| {})",
        "/>",
    ] {
        assert!(
            hello_snippet.contains(required),
            "menu hello-world snippet should include `{required}`."
        );
    }

    for forbidden in [
        "ui_state_primitives",
        "ui_headless",
        "use_menu(",
        "MenuOptions",
        "state=",
    ] {
        assert!(
            !hello_snippet.contains(forbidden),
            "menu hello-world should not require internal state/headless wiring `{forbidden}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2
            .contains("- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。")
            && check2.contains("Hello World 维持 5 行最小调用路径")
            && check2.contains("Hello World (Default Path)")
            && check2.contains("menu_api_dx_paradox_keeps_minimal_path_simple_and_docs_visible"),
        "check2.md should record DX paradox completion evidence and regression test reference."
    );
}

#[test]
fn menu_composition_api_prefers_typed_item_specs_over_parallel_arrays() {
    let mod_source = load_component_source("src/mod.rs");
    for required in [
        "pub struct MenuItemSpec",
        "pub label: String,",
        "pub kind: MenuItemKind,",
        "pub is_disabled: bool,",
        "pub fn action(label: impl Into<String>) -> Self",
    ] {
        assert!(
            mod_source.contains(required),
            "menu module should expose typed item spec contract via `{required}`."
        );
    }

    let logic_source = load_component_source("src/logic.rs");
    for required in [
        "pub struct MenuItemsInput",
        "pub struct MenuItemsOutput",
        "pub fn normalize_menu_items(input: MenuItemsInput) -> MenuItemsOutput",
        "if !input.item_specs.is_empty()",
        "items.push(spec.label);",
        "item_kinds.push(spec.kind);",
        "MenuItemsOutput {\n            has_item_specs: true,",
    ] {
        assert!(
            logic_source.contains(required),
            "menu logic should map typed specs to item semantics via `{required}`."
        );
    }

    let menu_view = load_component_source("src/view.rs");
    for required in [
        "#[prop(optional)] item_specs: Vec<MenuItemSpec>",
        "let normalized_items = logic::normalize_menu_items(logic::MenuItemsInput {",
        "data-items-source=has_item_specs",
        ".then_some(\"item-spec\")",
        ".or(Some(\"legacy-arrays\"))",
    ] {
        assert!(
            menu_view.contains(required),
            "menu view should use typed item spec path and source markers via `{required}`."
        );
    }

    let docs_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/collections.rs");
    let menu_docs_block = docs_source
        .split("pub(super) fn menu() -> AnyView {")
        .nth(1)
        .and_then(|tail| {
            tail.split("pub(super) fn menu_trigger() -> AnyView {")
                .next()
        })
        .expect("collections.rs should contain menu() section");

    for required in [
        "title=\"Hello World (Default Path)\"",
        "item_specs=hello_item_specs",
        "最小默认路径：仅 `id_base + item_specs + on_action`",
        "item_specs=vec![MenuItemSpec::action(\"New file\"), MenuItemSpec::action(\"Share with team\")]",
    ] {
        assert!(
            menu_docs_block.contains(required),
            "menu docs default path should recommend typed item specs via `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");

    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。")
                && check.contains("默认推荐 `item_specs: Vec<MenuItemSpec>`")
                && check.contains("`items + item_kinds + disabled_indices`")
                && check.contains(
                    "menu_composition_api_prefers_typed_item_specs_over_parallel_arrays"
                ),
            "checklist should record typed-spec composition completion and regression reference."
        );
    }
}

#[test]
fn menu_macro_micro_drag_state_machine_is_not_applicable_and_documented() {
    for rel_path in [
        "src/view.rs",
        "src/motion.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "Dragging",
            "Action::DragEnd",
            "is_dragging",
            "drag_start",
            "drag_end",
            "on:drag",
            "on:dragstart",
            "on:dragend",
            "set_pointer_capture",
            "release_pointer_capture",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not define drag macro/micro state token `{forbidden}` for menu."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。")
            && check2.contains("本组件判定：N/A（Menu 当前不包含拖拽手势，不存在 `Dragging` / `Action::DragEnd` 状态轴；高频输入仅为 headless 菜单导航的 pointer/focus 语义）。")
            && check2.contains("menu_macro_micro_drag_state_machine_is_not_applicable_and_documented"),
        "check2.md should document Macro/Micro Duality as N/A with a regression test link."
    );
}

#[test]
fn menu_two_pass_geometry_rendering_is_not_implemented_in_component_layer() {
    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/motion.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "get_bounding_client_rect",
            "getBoundingClientRect",
            "Intent",
            "Rectification",
            "measure_geometry",
            "rectification",
            "layout_effect",
            "requestAnimationFrame",
            "raf",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not implement component-local geometry two-pass token `{forbidden}`."
            );
        }
    }

    let menu_view = load_component_source("src/view.rs");
    let menu_motion = load_component_source("src/motion.rs");
    assert!(
        menu_view.contains("crate::menu::motion::attach_motion("),
        "menu view should mount geometry-sensitive highlight behavior through menu motion contract."
    );
    assert!(
        menu_motion.contains("attach_active_highlight_motion("),
        "menu motion contract should delegate highlight geometry handling to shared visual primitive."
    );

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。")
            && check2.contains("本组件判定：N/A（`menu` 组件层未自实现 overlay 几何测量闭环；若存在高亮条位置测量，已委托给共享 `ui-visual-primitive` 动效能力，不在 `menu/logic.rs` 重建 `Measure -> Rectification` 状态机）。")
            && check2.contains("menu_two_pass_geometry_rendering_is_not_implemented_in_component_layer"),
        "check2.md should document Two-Pass Rendering as N/A with a regression test link."
    );
}

#[test]
fn menu_registration_protocol_is_not_applicable_and_order_is_vector_driven() {
    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "RegistrationContext",
            "Register",
            "Unregister",
            "items_order",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not claim dynamic registration token `{forbidden}` for menu."
            );
        }
    }

    let menu_view = load_component_source("src/view.rs");
    for required in [
        "items.iter().cloned().enumerate()",
        "data-index=index",
        "let item = use_menu_item(",
        "disabled_indices.contains(&index)",
    ] {
        assert!(
            menu_view.contains(required),
            "menu view should drive item order via stable vector index path `{required}`."
        );
    }
    for forbidden in ["disabled_indices.iter()", "for index in disabled_indices"] {
        assert!(
            !menu_view.contains(forbidden),
            "menu view should not derive navigation order from HashSet iteration `{forbidden}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。")
            && check2.contains("本组件判定：N/A（`Menu` 当前子项来源是 `Vec`/`Arc<[String]>` 与 `MenuItemSpec`，不采用动态挂载子树与 `RegistrationContext` 协议）。")
            && check2.contains("menu_registration_protocol_is_not_applicable_and_order_is_vector_driven"),
        "check2.md should document Registration Protocol N/A with regression test evidence."
    );
}

#[test]
fn menu_slot_projection_uses_lazy_presence_and_not_keep_alive() {
    for rel_path in [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for required in [
            "let presence = use_presence(open);",
            "<Show when=move || presence.is_present.get()>",
            "on_exit_complete=presence.finish_exit",
        ] {
            assert!(
                source.contains(required),
                "{rel_path} should implement lazy slot projection with presence contract `{required}`."
            );
        }
    }

    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "KeepAlive",
            "NotifyHidden",
            "data-projection=\"keep-alive\"",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not implement keep-alive slot projection token `{forbidden}`."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。")
            && check2.contains("本组件判定：采用 `Lazy` 投影（`open=false` 时通过 `use_presence + Show` 卸载 overlay 内容），未采用 `KeepAlive` 常驻隐藏树。")
            && check2.contains("`KeepAlive/NotifyHidden` 判定：N/A（当前实现无 KeepAlive 隐藏常驻分支，因此不存在 `NotifyHidden` 生命周期通知需求）。")
            && check2.contains("menu_slot_projection_uses_lazy_presence_and_not_keep_alive"),
        "check2.md should document Slot Projection completion and regression evidence."
    );
}

#[test]
fn menu_env_streams_are_not_used_and_no_raw_env_event_flood_exists() {
    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "ResizeObserver",
            "IntersectionObserver",
            "match_media",
            "matchMedia",
            "BreakpointChanged",
            "on:resize",
            "on:scroll",
            "on:visibilitychange",
            "use_resize_observer",
            "use_intersection_observer",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not consume raw env stream token `{forbidden}` in menu."
            );
        }
    }

    let interaction_sources = [
        load_component_source("src/view.rs"),
        load_component_source("src/action_menu/view.rs"),
        load_component_source("src/context_menu/view.rs"),
        load_component_source("src/dropdown/view.rs"),
        load_component_source("src/dropdown_menu/view.rs"),
        load_component_source("src/menubar/view.rs"),
        load_component_source("src/trigger/view.rs"),
    ]
    .join("\n");
    for required in [
        "use_menu(",
        "use_presence(open)",
        "on:keydown=",
        "on:pointermove=",
    ] {
        assert!(
            interaction_sources.contains(required),
            "menu interaction should stay user-input driven and still include `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。")
            && check2.contains("本组件判定：N/A（`Menu` 现有交互仅依赖显式用户输入与受控 props，不消费 `Resize/Theme/Intersection` 环境流）。")
            && check2.contains("menu_env_streams_are_not_used_and_no_raw_env_event_flood_exists"),
        "check2.md should document Env Streams completion with N/A reasoning and test reference."
    );
}

#[test]
fn menu_event_light_cone_is_not_applicable_for_large_batch_collection_semantics() {
    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "Context Bus",
            "ContextBus",
            "SelectionState::All",
            "select_all",
            "batch_select",
            "bulk_action",
            "prop drilling",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not implement large-collection event-light-cone token `{forbidden}`."
            );
        }
    }

    let interaction_sources = [
        load_component_source("src/view.rs"),
        load_component_source("src/action_menu/view.rs"),
        load_component_source("src/context_menu/view.rs"),
        load_component_source("src/dropdown/view.rs"),
        load_component_source("src/dropdown_menu/view.rs"),
        load_component_source("src/menubar/view.rs"),
        load_component_source("src/trigger/view.rs"),
    ]
    .join("\n");
    for required in [
        "on_action: Callback<usize>",
        "on_action=on_action_wrapped",
        "let item = use_menu_item(",
    ] {
        assert!(
            interaction_sources.contains(required),
            "menu interaction should stay index-event driven and include `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。")
            && check2.contains("本组件判定：N/A（`Menu` 不属于 `Table/Grid` 大型集合批量操作组件，无 `select-all`/批处理状态压缩需求）。")
            && check2.contains("menu_event_light_cone_is_not_applicable_for_large_batch_collection_semantics"),
        "check2.md should document Event Light Cone N/A reasoning and regression evidence."
    );
}

#[test]
fn menu_causality_bus_trace_id_is_not_applicable_for_current_event_model() {
    for rel_path in [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "TraceId",
            "trace_id",
            "Causality Bus",
            "causality_bus",
            "publish(",
            "subscribe(",
            "broadcast(",
            "event_bus",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not implement causality-bus token `{forbidden}` for current menu model."
            );
        }
    }

    let interaction_sources = [
        load_component_source("src/action_menu/view.rs"),
        load_component_source("src/context_menu/view.rs"),
        load_component_source("src/dropdown/view.rs"),
        load_component_source("src/dropdown_menu/view.rs"),
        load_component_source("src/menubar/view.rs"),
        load_component_source("src/navigation_menu/view.rs"),
        load_component_source("src/trigger/view.rs"),
    ]
    .join("\n");
    for required in [
        "on_action: Callback<usize>",
        "on_open_change: Option<Callback<bool>>",
        "on_selected_id_change: Option<Callback<Option<String>>>",
    ] {
        assert!(
            interaction_sources.contains(required),
            "menu event model should remain direct-callback based and include `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。")
            && check2.contains("本组件判定：N/A（`Menu` 当前无复杂派生总线与广播订阅链路，仅本地输入事件驱动 open/selection/on_action 回调）。")
            && check2.contains("menu_causality_bus_trace_id_is_not_applicable_for_current_event_model"),
        "check2.md should document Causality Bus N/A reasoning and regression evidence."
    );
}

#[test]
fn menu_overlay_focus_restoration_uses_global_focus_manager_stack() {
    for rel_path in [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("<Popover"),
            "{rel_path} should render overlay content through shared Popover."
        );
        for forbidden in [
            "use_focus_trap(",
            "FocusTrapOptions",
            "RestorePolicy::",
            "previous_focus",
            "restore_target",
            "document.body()",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not implement private focus-restore logic token `{forbidden}`."
            );
        }
    }

    let popover_view = load_workspace_source("components/popover/src/view.rs");
    for required in [
        "let focus_trap = use_focus_trap(",
        "FocusTrapOptions::enabled(panel_ref).with_scope_id(\"popover\").with_restore_policy(",
        "RestorePolicy::FallbackTo(",
        ".with_fallback_selector(",
    ] {
        assert!(
            popover_view.contains(required),
            "shared popover should own focus restore policy contract `{required}`."
        );
    }

    let headless_focus_trap = load_workspace_source("crates/ui-headless/src/focus_trap.rs");
    for required in [
        "FOCUS_MANAGER_STACK",
        "fn focus_manager_push_trap(",
        "fn focus_manager_pop_trap(",
        "RestorePolicy::Selector",
        "RestorePolicy::FallbackTo",
        "if let Some(body) = document.body() {",
    ] {
        assert!(
            headless_focus_trap.contains(required),
            "ui-headless focus trap should keep global focus stack contract `{required}`."
        );
    }
    for forbidden in ["RestorePolicy::NodeRef", "Option<NodeRef"] {
        assert!(
            !headless_focus_trap.contains(forbidden),
            "focus restore policy should not expose NodeRef-based restoration `{forbidden}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。")
            && check2.contains("`components/menu/src/*/view.rs` 的 overlay 统一经 `Popover` 装配")
            && check2.contains("`components/popover/src/view.rs` 使用 `use_focus_trap + RestorePolicy::FallbackTo`")
            && check2.contains("`crates/ui-headless/src/focus_trap.rs` 维护 `FOCUS_MANAGER_STACK`")
            && check2.contains("menu_overlay_focus_restoration_uses_global_focus_manager_stack"),
        "check2.md should record Focus Stack & GC completion evidence and regression reference."
    );
}

#[test]
fn menu_escape_hatches_foreign_zone_is_not_applicable_and_no_third_party_instance_leak() {
    for rel_path in [
        "src/lib.rs",
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/action_menu/logic.rs",
        "src/action_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/logic.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/logic.rs",
        "src/navigation_menu/view.rs",
        "src/trigger/logic.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "ECharts",
            "echarts",
            "Mapbox",
            "Leaflet",
            "OpenLayers",
            "google.maps",
            "YieldControl",
            "CleanupForeign",
            "Foreign Zone",
            "foreign_zone",
            "chart_instance",
            "map_instance",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not embed third-party imperative integration token `{forbidden}` in menu."
            );
        }
    }

    let public_api = [
        load_component_source("src/lib.rs"),
        load_component_source("src/mod.rs"),
    ]
    .join("\n");
    for forbidden in [
        "pub struct Echarts",
        "pub struct Mapbox",
        "pub enum ForeignZone",
        "pub type ForeignZone",
        "pub type JsValue",
        "pub fn set_chart",
        "pub fn set_map",
    ] {
        assert!(
            !public_api.contains(forbidden),
            "menu public API should not expose imperative third-party instance token `{forbidden}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。")
            && check2.contains("本组件判定：N/A（`menu` 当前未集成 ECharts/Map 等命令式第三方库，不存在 `Foreign Zone` 边界治理需求）。")
            && check2.contains("menu_escape_hatches_foreign_zone_is_not_applicable_and_no_third_party_instance_leak"),
        "check2.md should record Escape Hatches N/A reasoning and regression reference."
    );
}

#[test]
fn menu_hydration_discontinuity_ids_are_deterministic_without_time_or_random_sources() {
    for rel_path in [
        "src/lib.rs",
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/action_menu/logic.rs",
        "src/action_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/logic.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/logic.rs",
        "src/navigation_menu/view.rs",
        "src/trigger/logic.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "now()",
            "Instant::now",
            "SystemTime::now",
            "UNIX_EPOCH",
            "Uuid",
            "uuid::",
            "rand::",
            "thread_rng",
            "random(",
            "Math::random",
            "Date::now",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not initialize IDs from non-deterministic source `{forbidden}`."
            );
        }
    }

    let deterministic_id_sources = [
        load_component_source("src/action_menu/logic.rs"),
        load_component_source("src/context_menu/logic.rs"),
        load_component_source("src/dropdown_menu/logic.rs"),
        load_component_source("src/trigger/logic.rs"),
        load_component_source("src/menubar/logic.rs"),
        load_component_source("src/navigation_menu/logic.rs"),
    ]
    .join("\n");
    for required in ["normalize_id_base(", "resolve_ids(", "resolve_menus("] {
        assert!(
            deterministic_id_sources.contains(required),
            "menu logic should keep deterministic id derivation contract `{required}`."
        );
    }

    let menu_primitives = load_workspace_source("crates/ui-state-primitives/src/menu.rs");
    for required in [
        "pub fn normalize_id_base(id_base: String, default_id_base: &str) -> String {",
        "pub fn resolve_id_pair(id_base: &str) -> (String, String) {",
    ] {
        assert!(
            menu_primitives.contains(required),
            "menu primitives should keep deterministic id helpers `{required}`."
        );
    }
    for forbidden in [
        "now()",
        "Instant::now",
        "SystemTime::now",
        "Uuid",
        "uuid::",
        "rand::",
    ] {
        assert!(
            !menu_primitives.contains(forbidden),
            "menu primitive id helpers should not depend on `{forbidden}`."
        );
    }

    let action_menu_primitives =
        load_workspace_source("crates/ui-state-primitives/src/action_menu.rs");
    for required in [
        "pub fn normalize_id_base(id_base: String) -> String {",
        "pub fn resolve_id_pair(id_base: &str) -> (String, String) {",
    ] {
        assert!(
            action_menu_primitives.contains(required),
            "action-menu primitives should keep deterministic id helpers `{required}`."
        );
    }

    let id_provider = load_workspace_source("crates/ui-headless/src/id_provider.rs");
    for required in [
        "pub struct UiIdProvider {",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
    ] {
        assert!(
            id_provider.contains(required),
            "ui-headless should keep deterministic IdProvider injection hook `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    assert!(
        check2.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。")
            && check2.contains("`menu` 家族 ID 由显式 `id_base` 与纯函数派生")
            && check2.contains("`crates/ui-headless/src/id_provider.rs`")
            && check2.contains(
                "menu_hydration_discontinuity_ids_are_deterministic_without_time_or_random_sources"
            ),
        "check2.md should record Hydration Discontinuity completion evidence and regression reference."
    );
}

#[test]
fn menu_platform_compile_contract_covers_web_ssr_wasm_and_keeps_non_wasm_browser_free() {
    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。")
                && check.contains("cargo check -p ui-menu")
                && check.contains("cargo check -p ui-menu --target wasm32-unknown-unknown")
                && check
                    .contains("cargo check -p ui-headless --no-default-features --features ssr")
                && check.contains("menu_platform_compile_contract_covers_web_ssr_wasm_and_keeps_non_wasm_browser_free"),
            "checklist should record platform compile-only commands and regression reference."
        );
    }

    let navigation_view = load_component_source("src/navigation_menu/view.rs");
    let menubar_view = load_component_source("src/menubar/view.rs");
    let context_motion = load_component_source("src/context_menu/motion.rs");
    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            navigation_view.contains(required),
            "navigation_menu view should keep explicit platform branch `{required}`."
        );
        assert!(
            menubar_view.contains(required),
            "menubar view should keep explicit platform branch `{required}`."
        );
        assert!(
            context_motion.contains(required),
            "context_menu motion should keep explicit platform branch `{required}`."
        );
    }

    let menu_sources = [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/context_menu/motion.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
        "src/motion.rs",
    ];
    for rel_path in menu_sources {
        let source = load_component_source(rel_path);
        for forbidden in [
            "web_sys::",
            "web-sys",
            "wasm_bindgen",
            "js_sys::",
            "window()",
            "document()",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should keep non-wasm compile path browser-free token `{forbidden}`."
            );
        }
    }
}

#[test]
fn menu_ui_headless_web_ssr_feature_mutex_is_compile_error_guarded() {
    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。")
                && check.contains("cargo check -p ui-headless --no-default-features --features web")
                && check.contains("cargo check -p ui-headless --no-default-features --features ssr")
                && check.contains("cargo check -p ui-headless --no-default-features --features web,ssr")
                && check.contains("menu_ui_headless_web_ssr_feature_mutex_is_compile_error_guarded"),
            "checklist should record ui-headless web/ssr mutex guard evidence and regression reference."
        );
    }

    let headless_lib = load_workspace_source("crates/ui-headless/src/lib.rs");
    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless must keep compile-time mutex guard `{required}`."
        );
    }

    let headless_cargo = load_workspace_source("crates/ui-headless/Cargo.toml");
    for required in [
        "[features]",
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_cargo.contains(required),
            "ui-headless feature model should keep `{required}`."
        );
    }

    let menu_cargo = load_component_source("Cargo.toml");
    assert!(
        menu_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "menu should depend on ui-headless without overriding web/ssr features."
    );

    let ui_components_cargo = load_workspace_source("crates/ui-components/Cargo.toml");
    assert!(
        ui_components_cargo.contains("ui-headless = { path = \"../ui-headless\" }"),
        "ui-components should depend on ui-headless without overriding web/ssr features."
    );
}

#[test]
fn menu_ui_motion_non_wasm_noop_stub_contract_is_preserved() {
    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。")
                && check.contains("`components/popover/src/motion.rs`")
                && check.contains("`components/menu/src/context_menu/motion.rs`")
                && check.contains("menu_ui_motion_non_wasm_noop_stub_contract_is_preserved"),
            "checklist should record ui-motion non-wasm noop/stub evidence and regression reference."
        );
    }

    let ui_motion_lib = load_workspace_source("crates/ui-motion/src/lib.rs");
    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep non-wasm noop/stub contract `{required}`."
        );
    }

    let popover_motion = load_workspace_source("components/popover/src/motion.rs");
    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(required),
            "popover motion should keep non-wasm predictable fallback `{required}`."
        );
    }

    let context_menu_motion = load_component_source("src/context_menu/motion.rs");
    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            context_menu_motion.contains(required),
            "context_menu motion should keep non-wasm noop attach branch `{required}`."
        );
    }
    for forbidden in ["panic!(", "unwrap()", "expect("] {
        assert!(
            !context_menu_motion.contains(forbidden),
            "context_menu motion non-wasm path should avoid panic-prone token `{forbidden}`."
        );
    }

    for rel_path in [
        "src/motion.rs",
        "src/action_menu/motion.rs",
        "src/context_menu/motion.rs",
        "src/dropdown/motion.rs",
        "src/dropdown_menu/motion.rs",
        "src/navigation_menu/motion.rs",
        "src/trigger/motion.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in ["SpringAnimator::new", "requestAnimationFrame", "web_sys::"] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not re-implement runtime motion engine token `{forbidden}`."
            );
        }
    }
}

#[test]
fn menu_reduced_motion_ssr_wasm_branches_preserve_semantic_contract() {
    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。")
                && check.contains("ui_motion::web::prefers_reduced_motion()")
                && check.contains("components/menu/src/context_menu/motion.rs")
                && check
                    .contains("menu_reduced_motion_ssr_wasm_branches_preserve_semantic_contract"),
            "checklist should record reduced-motion/SSR/wasm branch evidence and regression reference."
        );
    }

    let popover_motion = load_workspace_source("components/popover/src/motion.rs");
    for required in [
        "if ui_motion::web::prefers_reduced_motion() {",
        "if !open {",
        "on_exit_complete.run(());",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn attach_motion(",
        "if !is_open.get() {",
    ] {
        assert!(
            popover_motion.contains(required),
            "popover motion should keep reduced-motion + non-wasm fallback contract `{required}`."
        );
    }

    let context_menu_motion = load_component_source("src/context_menu/motion.rs");
    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "crate::popover::motion::attach_motion(",
        "std::hint::black_box(sanitize_motion(motion));",
    ] {
        assert!(
            context_menu_motion.contains(required),
            "context_menu motion should keep wasm-enhanced / non-wasm-noop split `{required}`."
        );
    }

    let semantic_views_without_platform_split = [
        "src/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/trigger/view.rs",
    ];
    for rel_path in semantic_views_without_platform_split {
        let source = load_component_source(rel_path);
        for required in ["data-state=", "on:keydown="] {
            assert!(
                source.contains(required),
                "{rel_path} should keep core semantics marker `{required}`."
            );
        }
        for forbidden in [
            "#[cfg(feature = \"ssr\")]",
            "#[cfg(not(feature = \"ssr\"))]",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not split semantic contract by ssr feature `{forbidden}`."
            );
        }
    }

    let menubar_view = load_component_source("src/menubar/view.rs");
    let navigation_view = load_component_source("src/navigation_menu/view.rs");
    for required in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "on:keydown=",
        "data-state=",
    ] {
        assert!(
            menubar_view.contains(required),
            "menubar should keep platform helper split while preserving semantics `{required}`."
        );
        assert!(
            navigation_view.contains(required),
            "navigation_menu should keep platform helper split while preserving semantics `{required}`."
        );
    }
}

#[test]
fn menu_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let motion_source = load_component_source("src/motion.rs");
    let view_source = load_component_source("src/view.rs");
    let ui_motion_spring_source = load_workspace_source("crates/ui-motion/src/spring.rs");
    let active_highlight_source =
        load_workspace_source("crates/ui-visual-primitive/src/active_highlight.rs");
    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "pub struct MenuMotion {",
        "pub fn sanitize_motion(motion: MenuMotion) -> MenuMotion {",
        "stiffness",
        "damping",
        "mass",
        "precision",
        "pub fn attach_motion(",
        "attach_active_highlight_motion(",
        "sanitize_motion(motion).highlight",
    ] {
        assert!(
            motion_source.contains(needle),
            "menu motion contract should keep marker `{needle}`."
        );
    }

    assert!(
        view_source.contains("crate::menu::motion::attach_motion("),
        "menu view should mount highlight animation via motion.rs attach contract."
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
        "web_sys::",
        "window()",
        "document()",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "menu motion bridge should not hard-bind browser runtime marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(script_needle),
        "contract hygiene script should include `{script_needle}`."
    );

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "menu_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2.contains(needle),
            "menu check2 should keep motion-contract evidence marker `{needle}`."
        );
    }
}

#[test]
fn menu_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_workspace_source("apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_workspace_source("apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_workspace_source("apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_workspace_source("e2e/tests/docs_app_components_coverage.spec.mjs");
    let perf_script_source = load_workspace_source("scripts/check-ui-components-performance.sh");
    let todo_source = load_workspace_source("docs/plan/TODO.md");

    for needle in [
        "\"menu\" => UiPerfBudget {",
        "\"menu-trigger\" => UiPerfBudget {",
        "\"dropdown-menu\" => UiPerfBudget {",
        "\"action-menu\" => UiPerfBudget {",
        "\"context-menu\" => UiPerfBudget {",
        "\"menubar\" => UiPerfBudget {",
        "\"navigation-menu\" => UiPerfBudget {",
        "max_update_ms: Some(",
        "max_heap_kb: Some(",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep menu perf budget marker `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"ActionMenu\", \"action-menu\", \"Actions\", actions::action_menu),",
        "component_doc!(\"Menu\", \"menu\", \"Collections\", collections::menu),",
        "\"menu-trigger\",",
        "\"dropdown-menu\",",
        "\"context-menu\",",
        "\"menubar\",",
        "\"navigation-menu\",",
    ] {
        assert!(
            pages_source.contains(needle),
            "menu docs catalog should keep perf-tracked route marker `{needle}`."
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
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should keep repeatable perf threshold marker `{needle}`."
        );
    }

    for needle in [
        "const perfProbe = page.locator('[data-slot=\"ui-perf-probe\"]').first();",
        "toHaveAttribute(\"data-perf-mount-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-budget-ms\", /[0-9]/);",
        "toHaveAttribute(\"data-perf-observability\", /mount/);",
        "not.toHaveAttribute(\"data-perf-violation\", \"true\");",
    ] {
        assert!(
            coverage_source.contains(needle),
            "coverage e2e should keep perf regression guard `{needle}`."
        );
    }

    let menu_view = load_component_source("src/view.rs");
    let action_menu_view = load_component_source("src/action_menu/view.rs");
    let context_menu_view = load_component_source("src/context_menu/view.rs");
    let menubar_view = load_component_source("src/menubar/view.rs");
    let navigation_view = load_component_source("src/navigation_menu/view.rs");
    let dropdown_menu_view = load_component_source("src/dropdown_menu/view.rs");

    for (source, needles, rel_path) in [
        (
            &menu_view,
            vec![
                "data-items-source=has_item_specs",
                "data-motion-source=motion_source",
            ],
            "src/view.rs",
        ),
        (
            &action_menu_view,
            vec![
                "data-open-source=move || root_state.get().open_source_attr",
                "data-motion-source=move || root_state.get().motion_source_attr",
            ],
            "src/action_menu/view.rs",
        ),
        (
            &context_menu_view,
            vec![
                "data-open-source=move || root_state.get().open_source_attr",
                "data-motion-source=move || root_state.get().motion_source_attr",
                "data-ui-source=move || root_state.get().open_source_attr",
            ],
            "src/context_menu/view.rs",
        ),
        (
            &menubar_view,
            vec![
                "data-open-index-source=move || root_state.get().open_index_source_attr",
                "data-motion-source=move || root_state.get().motion_source_attr",
            ],
            "src/menubar/view.rs",
        ),
        (
            &navigation_view,
            vec![
                "data-selected-id-source=move || root_state.get().selected_id_source_attr",
                "data-motion-source=move || root_state.get().motion_source_attr",
            ],
            "src/navigation_menu/view.rs",
        ),
        (
            &dropdown_menu_view,
            vec![
                "data-state=move || logic::resolve_root_state_attr(open.get(), state.is_trigger_disabled)",
                "data-motion-source=if motion == DropdownMenuMotion::default() {",
            ],
            "src/dropdown_menu/view.rs",
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "{rel_path} should keep perf attribution marker `{needle}`."
            );
        }
    }

    for needle in [
        "cargo test -p ui-menu menu_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should keep blocker command `{needle}`."
        );
    }

    for needle in ["render_count", "mount-only 等价证据"] {
        assert!(
            todo_source.contains(needle),
            "TODO follow-up plan should keep performance marker `{needle}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        for needle in [
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
            "component_page_perf_budget",
            "menu/menu-trigger/dropdown-menu/action-menu/context-menu/menubar/navigation-menu",
            "data-perf-violation != true",
            "render_count",
            "cargo test -p ui-menu menu_performance_governance_contract_is_budgeted_traceable_and_blocking",
            "menu_performance_governance_contract_is_budgeted_traceable_and_blocking",
        ] {
            assert!(
                check.contains(needle),
                "menu checklist should keep performance governance evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn menu_view_macro_complexity_is_split_into_semantic_subblocks() {
    let menu_view = load_component_source("src/view.rs");
    for needle in [
        "fn render_menu_item(",
        "render_menu_item(&menu, aria.active_index, index, label, kind, is_disabled)",
        "let item = use_menu_item(",
        "data-slot=\"menu-item\"",
    ] {
        assert!(
            menu_view.contains(needle),
            "menu view should split item subtree into semantic helper via `{needle}`."
        );
    }

    let menu_view_macro_count = menu_view.matches("view! {").count();
    assert!(
        menu_view_macro_count <= 2,
        "menu view macro complexity regression: expected <= 2 `view!` blocks, found {menu_view_macro_count}.",
    );

    let menu_component_macro_count = menu_view.matches("#[component]").count();
    assert_eq!(
        menu_component_macro_count, 1,
        "menu view should keep exactly one public component entry; found {menu_component_macro_count}.",
    );

    let menubar_view = load_component_source("src/menubar/view.rs");
    for needle in [
        "let render_menu = move |index: usize| {",
        "children=render_menu",
    ] {
        assert!(
            menubar_view.contains(needle),
            "menubar view should keep semantic subrender split token `{needle}`."
        );
    }
    let menubar_macro_count = menubar_view.matches("view! {").count();
    assert!(
        menubar_macro_count <= 3,
        "menubar view macro complexity regression: expected <= 3 `view!` blocks, found {menubar_macro_count}.",
    );

    let navigation_view = load_component_source("src/navigation_menu/view.rs");
    for needle in [
        "let render_item = move |index: usize| {",
        "children=render_item",
    ] {
        assert!(
            navigation_view.contains(needle),
            "navigation_menu view should keep semantic subrender split token `{needle}`."
        );
    }
    let navigation_macro_count = navigation_view.matches("view! {").count();
    assert!(
        navigation_macro_count <= 3,
        "navigation_menu view macro complexity regression: expected <= 3 `view!` blocks, found {navigation_macro_count}.",
    );

    for rel_path in [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        let count = source.matches("view! {").count();
        assert!(
            count <= 2,
            "{rel_path} should keep bounded macro expansion (<= 2 `view!` blocks), found {count}.",
        );
    }

    let script_source = load_workspace_source("scripts/check-ui-components-view-macro.sh");
    let script_needle =
        "cargo test -p ui-menu menu_view_macro_complexity_is_split_into_semantic_subblocks";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include menu complexity regression command."
    );

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        for needle in [
            "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
            "render_menu_item(",
            "children=render_menu",
            "children=render_item",
            "scripts/check-ui-components-view-macro.sh",
            "menu_view_macro_complexity_is_split_into_semantic_subblocks",
        ] {
            assert!(
                check.contains(needle),
                "menu checklist should keep view-macro governance evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn menu_view_functional_split_prefers_plain_functions_over_local_components() {
    let menu_view = load_component_source("src/view.rs");
    for needle in [
        "fn render_menu_item(",
        ") -> impl IntoView {",
        "render_menu_item(&menu, aria.active_index, index, label, kind, is_disabled)",
    ] {
        assert!(
            menu_view.contains(needle),
            "menu view should keep function-first split marker `{needle}`."
        );
    }
    for forbidden in [
        "#[component]\nfn render_menu_item(",
        "#[component]\r\nfn render_menu_item(",
    ] {
        assert!(
            !menu_view.contains(forbidden),
            "menu view helper should stay plain function, not local component `{forbidden}`."
        );
    }

    let menu_item_view = load_component_source("src/item/view.rs");
    for needle in [
        "fn render_submenu_indicator(has_submenu: bool) -> impl IntoView {",
        "{move || render_submenu_indicator(state.get().has_submenu)}",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu item view should keep function-first split marker `{needle}`."
        );
    }
    for forbidden in [
        "#[component]\nfn render_submenu_indicator(",
        "#[component]\r\nfn render_submenu_indicator(",
    ] {
        assert!(
            !menu_item_view.contains(forbidden),
            "menu item helper should stay plain function, not local component `{forbidden}`."
        );
    }

    for rel_path in [
        "src/view.rs",
        "src/item/view.rs",
        "src/section/view.rs",
        "src/trigger/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
    ] {
        let source = load_component_source(rel_path);
        let component_count = source.matches("#[component]").count();
        assert_eq!(
            component_count, 1,
            "{rel_path} should expose exactly one public component entry; found {component_count}."
        );
        for forbidden in ["#[component]\nfn render_", "#[component]\r\nfn render_"] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not promote local render helper to component `{forbidden}`."
            );
        }
    }

    let script_source = load_workspace_source("scripts/check-ui-components-view-macro.sh");
    let script_needle = "cargo test -p ui-menu menu_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include menu function-first split test target."
    );

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        for needle in [
            "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
            "render_menu_item(",
            "render_submenu_indicator(",
            "scripts/check-ui-components-view-macro.sh",
            "menu_view_functional_split_prefers_plain_functions_over_local_components",
        ] {
            assert!(
                check.contains(needle),
                "menu checklist should keep function-first split evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn menu_static_fragments_are_constantized_with_stable_semantics() {
    let menu_view = load_component_source("src/view.rs");
    for needle in [
        "const CHECKBOX_INDICATOR_MARK: &str = \"✓\";",
        "const RADIO_INDICATOR_MARK: &str = \"●\";",
        "then_some(CHECKBOX_INDICATOR_MARK)",
        "then_some(RADIO_INDICATOR_MARK)",
    ] {
        assert!(
            menu_view.contains(needle),
            "menu view should keep static indicator constantization marker `{needle}`."
        );
    }
    assert_eq!(
        menu_view.matches("\"✓\"").count(),
        1,
        "checkbox marker should be centralized as a single static literal source."
    );
    assert_eq!(
        menu_view.matches("\"●\"").count(),
        1,
        "radio marker should be centralized as a single static literal source."
    );

    let menu_item_view = load_component_source("src/item/view.rs");
    for needle in [
        "const SUBMENU_INDICATOR_SLOT: &str = \"menu-item-submenu-indicator\";",
        "const SUBMENU_INDICATOR_MARK: &str = \"›\";",
        "let marker = has_submenu.then_some(SUBMENU_INDICATOR_MARK);",
        "data-slot=SUBMENU_INDICATOR_SLOT",
        "aria-hidden=\"true\"",
        "data-visible=data_visible",
    ] {
        assert!(
            menu_item_view.contains(needle),
            "menu-item view should keep static submenu fragment constantization marker `{needle}`."
        );
    }
    assert_eq!(
        menu_item_view.matches("\"›\"").count(),
        1,
        "submenu marker should be centralized as a single static literal source."
    );

    let script_source = load_workspace_source("scripts/check-ui-components-view-macro.sh");
    let script_needle =
        "cargo test -p ui-menu menu_static_fragments_are_constantized_with_stable_semantics";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include menu static fragment constantization test target."
    );

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        for needle in [
            "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
            "CHECKBOX_INDICATOR_MARK",
            "RADIO_INDICATOR_MARK",
            "SUBMENU_INDICATOR_MARK",
            "SUBMENU_INDICATOR_SLOT",
            "scripts/check-ui-components-view-macro.sh",
            "menu_static_fragments_are_constantized_with_stable_semantics",
        ] {
            assert!(
                check.contains(needle),
                "menu checklist should keep static fragment constantization evidence token `{needle}`."
            );
        }
    }
}

#[test]
fn menu_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    let mut rust_sources = Vec::new();
    collect_paths_with_extension(&component_dir().join("src"), "rs", &mut rust_sources);
    assert!(
        !rust_sources.is_empty(),
        "menu component should expose Rust sources for inner_html injection-surface audit."
    );

    for source_path in rust_sources {
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {source_path:?}: {e}"));
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "menu source `{source_path:?}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    for rel_path in [
        "apps/docs-app/src/pages/components/pages/actions.rs",
        "apps/docs-app/src/pages/components/pages/collections.rs",
        "apps/docs-app/src/pages/components/pages/collections_extra.rs",
        "apps/docs-app/src/pages/components/pages/collections_command.rs",
    ] {
        let source = load_workspace_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html",
            "dangerously_set_inner_html",
            "<script",
            "javascript:",
        ] {
            assert!(
                !source.contains(forbidden),
                "menu docs example source `{rel_path}` must not contain raw-html injection token `{forbidden}`."
            );
        }
    }

    let docs_shell_source = load_workspace_source("apps/docs-app/src/pages/components/shell.rs");
    for required in [
        "const DROPDOWN_MENU_README_MD: &str =",
        "components/menu/src/dropdown_menu/README.md",
        "fn component_readme_markdown(slug: &str) -> Option<&'static str> {",
        "\"dropdown-menu\" => Some(DROPDOWN_MENU_README_MD),",
        "_ => None,",
        "map(crate::markdown::markdown_to_html)",
        "<div data-slot=\"component-readme\" inner_html=html></div>",
    ] {
        assert!(
            docs_shell_source.contains(required),
            "docs shell should keep trusted include_str! whitelist contract marker `{required}`."
        );
    }

    let script_source = load_workspace_source("scripts/check-ui-components-inner-html.sh");
    let script_needle =
        "cargo test -p ui-menu menu_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(script_needle),
        "inner-html gate script should include menu contract command `{script_needle}`."
    );

    for rel_path in ["check2.md", "src/check2.md", "src/action_menu/check2.md"] {
        let check = load_component_source(rel_path);
        for required in [
            "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
            "零注入面",
            "component_readme_markdown(slug)",
            "DROPDOWN_MENU_README_MD",
            "_ => None",
            "menu_inner_html_usage_is_forbidden_in_component_and_docs_examples",
            "scripts/check-ui-components-inner-html.sh",
            "cargo test -p ui-menu menu_inner_html_usage_is_forbidden_in_component_and_docs_examples",
        ] {
            assert!(
                check.contains(required),
                "{rel_path} should keep inner_html governance evidence marker `{required}`."
            );
        }
    }
}

#[test]
fn menu_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let menu_cargo = load_component_source("Cargo.toml");
    let ui_components_cargo = load_workspace_source("crates/ui-components/Cargo.toml");
    let ui_components_lib = load_workspace_source("crates/ui-components/src/lib.rs");
    let docs_app_lib = load_workspace_source("apps/docs-app/src/lib.rs");
    let debug_overlay = load_workspace_source("apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_workspace_source("crates/ui-headless/src/trace.rs");
    let controllable_state_source =
        load_workspace_source("crates/ui-headless/src/controllable_state.rs");
    let action_menu_view = load_component_source("src/action_menu/view.rs");
    let context_menu_view = load_component_source("src/context_menu/view.rs");
    let dropdown_view = load_component_source("src/dropdown/view.rs");
    let dropdown_menu_view = load_component_source("src/dropdown_menu/view.rs");
    let menu_trigger_view = load_component_source("src/trigger/view.rs");
    let menu_view = load_component_source("src/view.rs");
    let menu_item_view = load_component_source("src/item/view.rs");
    let actions_docs = load_workspace_source("apps/docs-app/src/pages/components/pages/actions.rs");
    let collections_docs =
        load_workspace_source("apps/docs-app/src/pages/components/pages/collections.rs");
    let wasm_debug_script = load_workspace_source("scripts/check-ui-components-wasm-debug.sh");

    for required in ["[features]", "default = []"] {
        assert!(
            menu_cargo.contains(required),
            "menu crate feature boundary should include `{required}`."
        );
    }

    for forbidden in ["wasm-debug", "wasm_debug", "dep:tracing", "menu-wasm-debug"] {
        assert!(
            !menu_cargo.contains(forbidden),
            "menu crate should not leak component-local wasm-debug feature `{forbidden}`."
        );
    }

    for required in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui-components shared wasm-debug graph should include `{required}`."
        );
    }

    for forbidden in [
        "menu-wasm-debug =",
        "menu_wasm_debug =",
        "component-menu\", \"dep:tracing",
    ] {
        assert!(
            !ui_components_cargo.contains(forbidden),
            "ui-components should not define menu-local wasm-debug feature `{forbidden}`."
        );
    }

    for required in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui-components root should keep global wasm-debug isolation marker `{required}`."
        );
    }

    for required in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_app_lib.contains(required),
            "docs-app should keep dev-only debug overlay entry `{required}`."
        );
    }

    for required in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay.contains(required),
            "debug overlay should keep visualized replay marker `{required}`."
        );
    }

    for required in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(required),
            "ui-headless trace should keep typed timestamp/source marker `{required}`."
        );
    }

    for required in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            controllable_state_source.contains(required),
            "controllable state should emit traced open-change marker `{required}`."
        );
    }

    let combined_views = format!(
        "{action_menu_view}\n{context_menu_view}\n{dropdown_view}\n{dropdown_menu_view}\n{menu_trigger_view}\n{menu_view}\n{menu_item_view}"
    );
    for required in [
        "use_controllable_open_state_traced(",
        "\"action-menu\"",
        "\"context-menu\"",
        "\"dropdown\"",
        "\"dropdown-menu\"",
        "\"menu-trigger\"",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-open-source=move || root_state.get().open_source_attr",
        "data-open-change-source=move || root_state.get().open_change_source_attr",
        "data-controlled=",
        "data-uncontrolled=",
        "on:keydown=on_key_down",
        "on:pointermove=move |_| {",
    ] {
        assert!(
            combined_views.contains(required),
            "menu family should keep reproducible interaction/state marker `{required}`."
        );
    }

    for required in [
        "title=\"State + Source Markers\"",
        "Inspect data-id-source / data-aria-label-source / data-disabled-indices-source / data-item-kinds-source / data-open-source / data-open-change-source / data-motion-source in DevTools.",
        "\"open: \"",
        "\" · last action: \"",
    ] {
        assert!(
            actions_docs.contains(required),
            "actions docs should keep ActionMenu replay path marker `{required}`."
        );
    }

    for required in [
        "title=\"Interactive Playground (Display / Config / Code / CSS Test)\"",
        "data-slot=\"menu-trigger-workbench-display\"",
        "\"display: current config vs baseline\"",
        "\"open: \"",
    ] {
        assert!(
            collections_docs.contains(required),
            "collections docs should keep MenuTrigger/Dropdown replay path marker `{required}`."
        );
    }

    let mut rust_sources = Vec::new();
    collect_paths_with_extension(&component_dir().join("src"), "rs", &mut rust_sources);
    for source_path in rust_sources {
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {source_path:?}: {e}"));
        for forbidden in [
            "use_ui_trace(",
            "provide_ui_trace(",
            "trace.emit(",
            "debug_overlay",
            "request_replay",
            "wasm_debug_proxy!",
            "observability::",
            "#[prop(optional)] debug",
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
        ] {
            assert!(
                !source.contains(forbidden),
                "menu runtime/public source `{source_path:?}` should not leak wasm-debug internals `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui-menu menu_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        wasm_debug_script.contains(script_needle),
        "wasm-debug gate script should include menu contract command `{script_needle}`."
    );

    for rel_path in ["check2.md", "src/check2.md", "src/action_menu/check2.md"] {
        let check = load_component_source(rel_path);
        for required in [
            "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
            "use_controllable_open_state_traced(\"action-menu\"",
            "provide_ui_trace(debug_overlay_enabled)",
            "menu_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
            "scripts/check-ui-components-wasm-debug.sh",
            "cargo test -p ui-menu menu_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                check.contains(required),
                "{rel_path} should keep wasm-debug governance evidence marker `{required}`."
            );
        }
    }
}

#[test]
fn menu_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench() {
    let playground_source = load_workspace_source("apps/docs-app/src/playground.rs");
    let collections_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/collections.rs");
    let actions_source =
        load_workspace_source("apps/docs-app/src/pages/components/pages/actions.rs");
    let dx_script_source = load_workspace_source("scripts/check-ui-components-dx.sh");

    let menu_trigger_section = extract_section(&collections_source, "pub(super) fn menu_trigger()");
    let dropdown_menu_section =
        extract_section(&collections_source, "pub(super) fn dropdown_menu()");
    let action_menu_section = extract_section(&actions_source, "pub(super) fn action_menu()");

    for required in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "data-playground-scope=scope_id.clone()",
        "class_name=\"playground__panel playground__controls\".to_string()",
        "data-slot=\"playground-test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(required),
            "shared Playground should keep DX hot-reload/isolated-canvas marker `{required}`."
        );
    }

    for required in [
        "title=\"Interactive Playground (Display / Config / Code / CSS Test)\"",
        "test_css_source=workbench_test_css_source",
        "test_source_path=\"crates/ui-components/src/menu/trigger/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"menu-trigger-workbench-display\"",
        "\"display: current config vs baseline\"",
        "set_workbench_open_raw.set(next)",
        "\"open: \"",
    ] {
        assert!(
            menu_trigger_section.contains(required),
            "menu_trigger docs section should keep DX workbench marker `{required}`."
        );
    }

    for required in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + Code + CSS Test:",
        "test_css_source=interactive_test_css",
        "test_source_path=\"crates/ui-components/src/menu/dropdown_menu/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "Switch checked=interactive_controlled set_checked=set_interactive_controlled",
        "set_interactive_open_raw.set(next)",
        "\" · open: \"",
    ] {
        assert!(
            dropdown_menu_section.contains(required),
            "dropdown_menu docs section should keep DX workbench marker `{required}`."
        );
    }

    for required in [
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "set_marker_open_raw.set(true)",
        "set_marker_open_raw.set(false)",
        "\"open: \"",
        "\" · last action: \"",
    ] {
        assert!(
            action_menu_section.contains(required),
            "action_menu docs section should keep context-preserving replay marker `{required}`."
        );
    }

    for section in [
        &menu_trigger_section,
        &dropdown_menu_section,
        &action_menu_section,
    ] {
        for forbidden in [
            "Persist workbench state",
            "localStorage",
            "sessionStorage",
            "save_",
            "load_",
        ] {
            assert!(
                !section.contains(forbidden),
                "menu DX scope keeps optional persisted state as N/A; found `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui-menu menu_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench";
    assert!(
        dx_script_source.contains(script_needle),
        "dx gate script should include menu DX contract command `{script_needle}`."
    );

    for rel_path in ["check2.md", "src/check2.md", "src/action_menu/check2.md"] {
        let check = load_component_source(rel_path);
        for required in [
            "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
            "test_css_source=workbench_test_css_source",
            "test_css_source=interactive_test_css",
            "State + Source Markers",
            "可选状态保留：N/A",
            "menu_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench",
            "scripts/check-ui-components-dx.sh",
            "cargo test -p ui-menu menu_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                check.contains(required),
                "{rel_path} should keep DX governance evidence marker `{required}`."
            );
        }
    }
}

#[test]
fn menu_a11y_i18n_l10n_contract_is_wired_without_view_hardcoded_copy() {
    let menu_view = load_component_source("src/view.rs");
    for required in [
        "let aria = use_menu(MenuOptions {",
        "role=aria.attrs.role",
        "aria-label=aria_label.get_value()",
        "aria-labelledby=aria_labelledby.get_value()",
        "on:keydown=on_key_down",
    ] {
        assert!(
            menu_view.contains(required),
            "menu view should mount headless role/aria/keyboard semantics via `{required}`."
        );
    }

    let action_view = load_component_source("src/action_menu/view.rs");
    for required in [
        "use_ui_i18n();",
        "strings::<CommonStrings>()",
        "locale_attrs(lang, dir)",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "ui_headless::aria_controls_when_open(",
        "on:keydown=on_key_down",
        "aria_label: Option<String>",
    ] {
        assert!(
            action_view.contains(required),
            "action_menu should expose i18n/l10n + shared a11y wiring via `{required}`."
        );
    }
    assert!(
        action_view.contains(
            "fallback_aria_label: common.action_menu_trigger_aria_label.as_ref().into(),"
        ),
        "action_menu should source fallback trigger aria label from i18n bundle before logic fallback."
    );

    let context_view = load_component_source("src/context_menu/view.rs");
    for required in [
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "ui_headless::aria_controls_when_open(",
        "aria-haspopup=\"menu\"",
        "aria-expanded=move || logic::resolve_aria_expanded(open.get())",
        "aria-controls=aria_controls",
        "on:keydown=on_key_down",
    ] {
        assert!(
            context_view.contains(required),
            "context_menu should mount locale + aria/keyboard semantics via `{required}`."
        );
    }

    let dropdown_view = load_component_source("src/dropdown/view.rs");
    for required in [
        "lang=lang_attr.clone()",
        "dir=dir_attr",
        "ui_headless::aria_controls_when_open(",
        "on:keydown=on_key_down",
    ] {
        assert!(
            dropdown_view.contains(required),
            "dropdown should pass lang/dir and shared a11y helper via `{required}`."
        );
    }

    let dropdown_menu_view = load_component_source("src/dropdown_menu/view.rs");
    for required in [
        "ui_headless::aria_controls_when_open(",
        "on:keydown=on_key_down",
    ] {
        assert!(
            dropdown_menu_view.contains(required),
            "dropdown_menu should keep headless shared a11y + keyboard contract via `{required}`."
        );
    }

    let trigger_view = load_component_source("src/trigger/view.rs");
    for required in [
        "ui_headless::aria_controls_when_open(",
        "on:keydown=on_key_down",
    ] {
        assert!(
            trigger_view.contains(required),
            "menu_trigger should keep headless shared a11y + keyboard contract via `{required}`."
        );
    }

    let headless_a11y = load_workspace_source("crates/ui-headless/src/a11y.rs");
    assert!(
        headless_a11y.contains("pub fn aria_controls_when_open("),
        "ui-headless should provide shared a11y helper `aria_controls_when_open`."
    );

    let context_logic = load_component_source("src/context_menu/logic.rs");
    let trigger_logic = load_component_source("src/trigger/logic.rs");
    let navigation_logic = load_component_source("src/navigation_menu/logic.rs");
    for required in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Open context menu\";",
        "pub fn resolve_trigger_aria_label(value: Option<String>) -> (String, bool) {",
    ] {
        assert!(
            context_logic.contains(required),
            "context_menu logic should keep aria-label fallback contract via `{required}`."
        );
    }
    assert!(
        trigger_logic.contains(
            "menu_state::resolve_aria_label_with_fallback(value, \"Open menu\", \"Open menu\")"
        ),
        "menu_trigger logic should own fallback aria-label path, not view."
    );
    assert!(
        navigation_logic.contains("pub const DEFAULT_ARIA_LABEL: &str = \"Main navigation\";")
            && navigation_logic.contains(
                "menu_state::resolve_aria_label_with_fallback(value, DEFAULT_ARIA_LABEL, DEFAULT_ARIA_LABEL)"
            ),
        "navigation_menu logic should own fallback aria-label path, not view."
    );

    for rel_path in [
        "src/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
        "src/section/view.rs",
        "src/item/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for forbidden in [
            "Open menu",
            "Open context menu",
            "Main navigation",
            "Menu section",
            "Menu item",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not hardcode user-visible fallback copy `{forbidden}` in view."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains(
                "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
            ) && check.contains("`Menu` 家族语义契约由 headless 输出并在各 `view.rs` 挂载")
                && check.contains("use_ui_i18n().strings::<CommonStrings>()")
                && check.contains("`lang` / `dir`")
                && check.contains("ui_headless::aria_controls_when_open")
                && check
                    .contains("menu_a11y_i18n_l10n_contract_is_wired_without_view_hardcoded_copy"),
            "checklist should record a11y+i18n/l10n completion evidence and regression test reference."
        );
    }
}

#[test]
fn menu_state_markers_are_observable_searchable_and_enumerated() {
    let root_views = [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
        "src/trigger/view.rs",
    ];
    for rel_path in root_views {
        let source = load_component_source(rel_path);
        for required in [
            "data-state=",
            "data-open=",
            "data-closed=",
            "data-disabled=",
            "data-controlled=",
            "data-uncontrolled=",
            "on:keydown=",
        ] {
            assert!(
                source.contains(required),
                "{rel_path} should expose observable/searchable state marker `{required}`."
            );
        }
    }

    let menu_view = load_component_source("src/view.rs");
    for required in [
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-items-source=has_item_specs",
        "data-focused=move || {",
        "data-checked=move || {",
        "data-kind=item.attrs.role",
        "data-disabled=if is_disabled { Some(\"true\") } else { None }",
    ] {
        assert!(
            menu_view.contains(required),
            "menu base view should expose searchable semantics via `{required}`."
        );
    }

    let item_view = load_component_source("src/item/view.rs");
    for required in [
        "data-state=move || state.get().data_state_attr",
        "data-checkable=move || state.get().is_checkable.then_some(\"true\")",
        "data-checked=move || state.get().is_checked.then_some(\"true\")",
        "data-focused=move || state.get().is_focused.then_some(\"true\")",
        "data-aria-source=move || state.get().aria_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "aria-checked=move || logic::resolve_aria_checked(kind)",
    ] {
        assert!(
            item_view.contains(required),
            "menu item should expose stable data/aria markers via `{required}`."
        );
    }

    let navigation_view = load_component_source("src/navigation_menu/view.rs");
    for required in [
        "data-selection-mode=move || root_state.get().selection_mode_attr",
        "data-selected-id-source=move || root_state.get().selected_id_source_attr",
        "data-default-selected-id-source=move || root_state.get().default_selected_id_source_attr",
        "data-selected-id-change-source=move || root_state.get().selected_id_change_source_attr",
        "data-focused-index=move || root_state.get().focused_index",
        "data-selected-index=move || root_state.get().selected_index",
        "data-selected=move || (selected_index.get() == Some(index)).then_some(\"true\")",
    ] {
        assert!(
            navigation_view.contains(required),
            "navigation_menu should expose source and selection markers via `{required}`."
        );
    }

    let source_markers = [
        load_component_source("src/action_menu/view.rs"),
        load_component_source("src/context_menu/view.rs"),
        load_component_source("src/menubar/view.rs"),
        load_component_source("src/navigation_menu/view.rs"),
    ]
    .join("\n");
    for required in [
        "data-id-source=",
        "data-class-source=",
        "data-open-source=",
        "data-default-open-source=",
        "data-open-change-source=",
        "data-motion-source=",
    ] {
        assert!(
            source_markers.contains(required),
            "menu variants should expose source markers for automation via `{required}`."
        );
    }

    for rel_path in [
        "src/action_menu/logic.rs",
        "src/context_menu/logic.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/logic.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("fn source_attr(is_custom: bool) -> &'static str {")
                && source.contains("if is_custom { \"custom\" } else { \"default\" }"),
            "{rel_path} should keep source marker values in closed set `custom/default`."
        );
    }

    let trigger_logic = load_component_source("src/trigger/logic.rs");
    assert!(
        trigger_logic.contains("pub fn resolve_root_state_attr(is_open: bool, trigger_disabled: bool) -> &'static str {")
            && trigger_logic.contains("\"open\"")
            && trigger_logic.contains("\"disabled\"")
            && trigger_logic.contains("\"closed\""),
        "menu_trigger root state marker should stay in closed set `open/disabled/closed`."
    );

    let menubar_logic = load_component_source("src/menubar/logic.rs");
    assert!(
        menubar_logic.contains("pub fn resolve_menu_state_attr(is_open: bool, is_trigger_disabled: bool) -> &'static str {")
            && menubar_logic.contains("\"open\"")
            && menubar_logic.contains("\"disabled\"")
            && menubar_logic.contains("\"closed\"")
            && menubar_logic.contains("pub fn resolve_aria_expanded(is_open: bool) -> &'static str {")
            && menubar_logic.contains("if is_open { \"true\" } else { \"false\" }"),
        "menubar state/aria markers should stay in closed sets."
    );

    let item_logic = load_component_source("src/item/logic.rs");
    for required in [
        "MenuItemKind::Action => \"action\"",
        "MenuItemKind::Checkbox { .. } => \"checkbox\"",
        "MenuItemKind::Radio { .. } => \"radio\"",
        "if input.disabled {",
        "\"focused-checked\"",
        "\"focused\"",
        "\"checked\"",
        "\"idle\"",
    ] {
        assert!(
            item_logic.contains(required),
            "menu item logic should keep kind/state marker values enumerable via `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。")
                && check.contains("`open/closed/disabled/selected/focused`")
                && check.contains("`data-controlled` / `data-uncontrolled`")
                && check.contains("`data-*-source`")
                && check.contains("`custom/default`")
                && check.contains("menu_state_markers_are_observable_searchable_and_enumerated"),
            "checklist should record marker observability/searchability completion and regression reference."
        );
    }
}

#[test]
fn menu_styles_depend_on_explicit_state_markers_not_fragile_dom_structure() {
    let style_files = [
        "src/styles.rs",
        "src/action_menu/styles.rs",
        "src/context_menu/styles.rs",
        "src/dropdown/styles.rs",
        "src/dropdown_menu/styles.rs",
        "src/item/styles.rs",
        "src/menubar/styles.rs",
        "src/navigation_menu/styles.rs",
        "src/section/styles.rs",
        "src/trigger/styles.rs",
    ];
    let mut all_styles = String::new();
    for rel_path in style_files {
        let source = load_component_source(rel_path);
        all_styles.push_str(&source);
        all_styles.push('\n');

        assert!(
            source.contains("[data-"),
            "{rel_path} should branch styles with explicit data markers."
        );
        for forbidden in [
            ":nth-child",
            ":nth-of-type",
            ":first-child",
            ":last-child",
            ":only-child",
            ":has(",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not use fragile structural selector `{forbidden}`."
            );
        }

        for line in source.lines() {
            let line = line.trim();
            if line.starts_with(".ui-") {
                let depth = line.matches(" .ui-").count();
                assert!(
                    depth <= 1,
                    "{rel_path} should not use deep descendant selector guessing state: `{line}`."
                );
            }
        }
    }

    for required in [
        ".ui-action-menu[data-state=\"open\"]",
        ".ui-context-menu[data-open-mode=\"controlled\"]",
        ".ui-dropdown[data-keep-open-on-action=\"true\"]",
        ".ui-navigation-menu[data-selection-mode=\"controlled\"]",
        ".ui-menubar[data-open-mode=\"controlled\"]",
        ".ui-menu-item[data-focused=\"true\"]",
        ".ui-menu-section[data-sticky-heading=\"true\"]",
    ] {
        assert!(
            all_styles.contains(required),
            "styles should map visual state from explicit marker `{required}`."
        );
    }

    for rel_path in [
        "src/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/item/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
        "src/section/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            !source.contains("style="),
            "{rel_path} should not push business style logic into inline style attribute."
        );
        for forbidden in ["set_property(", ".style()", ".style("] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not mutate runtime style via `{forbidden}`."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。")
                && check.contains("`:nth-child` / `:nth-of-type`")
                && check.contains("`style=` 内联业务样式")
                && check.contains("视觉切换直接映射到标记")
                && check.contains(
                    "menu_styles_depend_on_explicit_state_markers_not_fragile_dom_structure"
                ),
            "checklist should record explicit-style-state completion evidence and regression reference."
        );
    }
}

#[test]
fn menu_semantic_contract_tests_cover_interaction_matrix_without_snapshot_dependency() {
    let semantics_source = load_component_source("test/semantics.rs");
    for required in [
        "fn menu_view_mounts_headless_contracts()",
        "fn menu_a11y_i18n_l10n_contract_is_wired_without_view_hardcoded_copy()",
        "fn menu_state_markers_are_observable_searchable_and_enumerated()",
        "fn menu_open_axis_keeps_open_default_open_on_open_change_and_is_open_bridge()",
        "fn controllable_axes_use_controllable_primitives_and_expose_mode_markers()",
        "fn menu_boolean_props_expose_is_prefixed_names_with_disabled_alias_compatibility()",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantics suite should include contract-focused test `{required}`."
        );
    }

    for required in [
        "role=aria.attrs.role",
        "aria-label=aria_label.get_value()",
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-state=",
        "data-open=",
        "data-closed=",
        "data-controlled=",
        "data-uncontrolled=",
        "data-*-source",
        "on:keydown=on_key_down",
        "on:pointermove=",
    ] {
        assert!(
            semantics_source.contains(required),
            "semantics suite should assert marker/interaction contract token `{required}`."
        );
    }

    for forbidden in [
        "assert_snapshot!",
        "assert_debug_snapshot!",
        "insta::",
        "to_match_snapshot",
        "snapshot",
    ] {
        if forbidden == "snapshot" {
            continue;
        }
        assert!(
            !semantics_source.contains(forbidden),
            "semantic contract tests should not depend on visual snapshot token `{forbidden}`."
        );
    }

    let component_sources = [
        "src/view.rs",
        "src/logic.rs",
        "src/action_menu/view.rs",
        "src/action_menu/logic.rs",
        "src/context_menu/view.rs",
        "src/context_menu/logic.rs",
        "src/dropdown/view.rs",
        "src/dropdown/logic.rs",
        "src/dropdown_menu/view.rs",
        "src/dropdown_menu/logic.rs",
        "src/item/view.rs",
        "src/item/logic.rs",
        "src/menubar/view.rs",
        "src/menubar/logic.rs",
        "src/navigation_menu/view.rs",
        "src/navigation_menu/logic.rs",
        "src/section/view.rs",
        "src/section/logic.rs",
        "src/trigger/view.rs",
        "src/trigger/logic.rs",
    ];
    for rel_path in component_sources {
        let source = load_component_source(rel_path);
        for forbidden in [
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
            "#[cfg(feature = \"ssr\")]",
            "cfg!(target_arch = \"wasm32\")",
            "cfg!(feature = \"ssr\")",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not have platform-divergent semantic branch token `{forbidden}`."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。")
                && check.contains("role/aria/data-state/source markers")
                && check.contains("受控/非受控、disabled、键盘路径、指针路径")
                && check.contains("SSR/wasm 差异判定：N/A")
                && check.contains("不依赖视觉 snapshot 断言")
                && check.contains(
                    "menu_semantic_contract_tests_cover_interaction_matrix_without_snapshot_dependency"
                ),
            "checklist should record semantic-contract testing completion and regression reference."
        );
    }
}

#[test]
fn menu_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement() {
    let menu_view = load_component_source("src/view.rs");
    let item_view = load_component_source("src/item/view.rs");
    let semantics_source = load_component_source("test/semantics.rs");
    let perf_script_source = load_workspace_source("scripts/check-ui-components-performance.sh");
    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");

    for marker in [
        "role=aria.attrs.role",
        "aria-activedescendant=move || aria.attrs.aria_activedescendant.get()",
        "data-state=",
        "data-controlled=",
        "data-uncontrolled=",
        "data-items-source=has_item_specs",
    ] {
        assert!(
            menu_view.contains(marker),
            "menu semantics/perf matrix should keep aria/data marker `{marker}`."
        );
    }

    let interaction_sources = format!("{menu_view}\n{item_view}");
    for marker in [
        "on:keydown=on_key_down",
        "if aria.handlers.on_key_down.run(ev.key())",
        "on:pointermove=",
        "data-focused=move || {",
    ] {
        assert!(
            interaction_sources.contains(marker),
            "menu semantics/perf matrix should keep focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "menu_semantic_contract_tests_cover_interaction_matrix_without_snapshot_dependency",
        "menu_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "render_count",
    ] {
        assert!(
            semantics_source.contains(marker),
            "menu semantics/perf matrix should keep coverage marker `{marker}`."
        );
    }

    let perf_script_needle = "cargo test -p ui-menu menu_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement";
    assert!(
        perf_script_source.contains(perf_script_needle),
        "performance gate script should include `{perf_script_needle}`."
    );

    for check in [&check2, &src_check2] {
        for marker in [
            "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
            "N/A（精确 `render_count` 自动计数）：当前仓库仍在 `docs/plan/TODO.md` 跟踪“建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据”",
            "menu_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        ] {
            assert!(
                check.contains(marker),
                "menu checklist should pin semantics+performance completion marker `{marker}`."
            );
        }
    }
}

#[test]
fn menu_component_file_roles_remain_layered_and_boundary_safe() {
    let mod_source = load_component_source("src/mod.rs");
    for required in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use motion::MenuMotion;",
        "pub use view::Menu;",
        "pub struct MenuItemSpec {",
    ] {
        assert!(
            mod_source.contains(required),
            "mod.rs should keep export boundary and stable API via `{required}`."
        );
    }
    for forbidden in ["view! {", "NodeRef<", "web_sys::", "use_menu("] {
        assert!(
            !mod_source.contains(forbidden),
            "mod.rs should not host runtime implementation token `{forbidden}`."
        );
    }

    let logic_source = load_component_source("src/logic.rs");
    for required in [
        "pub struct MenuNormalizeInput",
        "pub fn normalize_props(input: MenuNormalizeInput) -> MenuNormalizedProps",
        "pub fn normalize_menu_items(input: MenuItemsInput) -> MenuItemsOutput",
        "pub fn resolve_state(",
    ] {
        assert!(
            logic_source.contains(required),
            "logic.rs should keep normalization/derivation responsibility via `{required}`."
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "NodeRef<",
        "web_sys::",
        "on:keydown",
        "on:pointermove",
        "style=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "logic.rs should not host DOM/render/style-side implementation token `{forbidden}`."
        );
    }

    let styles_source = load_component_source("src/styles.rs");
    assert!(
        styles_source.contains("pub const CSS: &str = r#\""),
        "styles.rs should expose static CSS contract constant."
    );
    for required in ["var(--ui-", ".ui-menu[data-motion-source=\"custom\"]"] {
        assert!(
            styles_source.contains(required),
            "styles.rs should remain token-first static CSS via `{required}`."
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "NodeRef<",
        "Callback<",
        "on:keydown",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "styles.rs should not contain view/runtime logic token `{forbidden}`."
        );
    }

    let view_source = load_component_source("src/view.rs");
    for required in [
        "#[component]",
        "view! {",
        "let aria = use_menu(MenuOptions {",
        "let item = use_menu_item(",
        "crate::menu::motion::attach_motion(",
        "logic::normalize_props(",
        "logic::normalize_menu_items(",
    ] {
        assert!(
            view_source.contains(required),
            "view.rs should render structure and mount headless contracts via `{required}`."
        );
    }
    for forbidden in ["pub const CSS", "sanitize_spring(", "SpringConfig {"] {
        assert!(
            !view_source.contains(forbidden),
            "view.rs should not own styles/motion engine token `{forbidden}`."
        );
    }

    let motion_source = load_component_source("src/motion.rs");
    for required in [
        "pub struct MenuMotion {",
        "ui_motion::spring::SpringConfig",
        "pub fn sanitize_motion(motion: MenuMotion) -> MenuMotion",
        "pub fn attach_motion(",
        "attach_active_highlight_motion(",
    ] {
        assert!(
            motion_source.contains(required),
            "motion.rs should map component motion contract via `{required}`."
        );
    }
    for forbidden in [
        "#[component]",
        "view! {",
        "on:keydown",
        "requestAnimationFrame",
        "web_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "motion.rs should not implement view/runtime engine token `{forbidden}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。")
                && check.contains("`mod.rs` 保持最小导出边界")
                && check.contains("`logic.rs` 不包含 DOM/事件绑定")
                && check.contains("`styles.rs` 仅提供静态 CSS 常量")
                && check.contains("`view.rs` 负责结构渲染与 headless 挂载")
                && check.contains("`motion.rs` 仅做动效契约映射")
                && check.contains("menu_component_file_roles_remain_layered_and_boundary_safe"),
            "checklist should record component-file role completion and regression reference."
        );
    }
}

#[test]
fn menu_spec_rs_is_not_introduced_for_simple_component_and_is_documented() {
    let spec_path = component_dir().join("src/spec.rs");
    assert!(
        !spec_path.exists(),
        "menu should not introduce spec.rs unless external schema contract complexity requires it."
    );

    let mod_source = load_component_source("src/mod.rs");
    for forbidden in ["mod spec;", "pub mod spec;"] {
        assert!(
            !mod_source.contains(forbidden),
            "menu mod.rs should not expose spec module token `{forbidden}` for this simple component."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。")
                && check.contains("不引入 `spec.rs`")
                && check.contains(
                    "menu_spec_rs_is_not_introduced_for_simple_component_and_is_documented"
                ),
            "checklist should record that menu keeps spec.rs absent and binds this regression test."
        );
    }
}

#[test]
fn menu_token_first_static_styles_are_aggregated_through_uiroot_without_utility_css_leakage() {
    let style_files = [
        "src/styles.rs",
        "src/action_menu/styles.rs",
        "src/context_menu/styles.rs",
        "src/dropdown/styles.rs",
        "src/dropdown_menu/styles.rs",
        "src/item/styles.rs",
        "src/menubar/styles.rs",
        "src/navigation_menu/styles.rs",
        "src/section/styles.rs",
        "src/trigger/styles.rs",
    ];
    for rel_path in style_files {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("pub const CSS: &str = r#\""),
            "{rel_path} should keep static CSS contract in styles.rs."
        );
        assert!(
            source.contains("var(--ui-"),
            "{rel_path} should consume theme tokens through var(--ui-*)."
        );
    }

    let root_styles = load_component_source("src/styles.rs");
    for required in [
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "gap: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "padding: var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
    ] {
        assert!(
            root_styles.contains(required),
            "menu root styles should keep spacing/radius tokenized via `{required}`."
        );
    }

    let item_styles = load_component_source("src/item/styles.rs");
    for required in [
        "gap: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "padding: var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "padding-inline-start: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "padding-inline-end: var(--ui-space-md, var(--ui-fallback-space-md));",
    ] {
        assert!(
            item_styles.contains(required),
            "menu item styles should consume tokenized spacing/radius via `{required}`."
        );
    }

    let section_styles = load_component_source("src/section/styles.rs");
    for required in [
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "padding-inline: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "padding-block: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "margin-inline: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "padding-bottom: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "outline-offset: var(--ui-space-3xs, var(--ui-fallback-space-3xs));",
    ] {
        assert!(
            section_styles.contains(required),
            "menu section styles should consume tokenized spacing via `{required}`."
        );
    }

    let ui_components_css = load_workspace_source("crates/ui-components/src/css.rs");
    for required in [
        "out.push_str(crate::menu::styles::CSS);",
        "out.push_str(crate::menu::item::styles::CSS);",
        "out.push_str(crate::menu::section::styles::CSS);",
        "out.push_str(crate::action_menu::styles::CSS);",
        "out.push_str(crate::context_menu::styles::CSS);",
        "out.push_str(crate::dropdown::styles::CSS);",
        "out.push_str(crate::dropdown_menu::styles::CSS);",
        "out.push_str(crate::menu_trigger::styles::CSS);",
        "out.push_str(crate::menubar::styles::CSS);",
        "out.push_str(crate::navigation_menu::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui-components css aggregator should include menu family styles via `{required}`."
        );
    }

    let ui_root = load_workspace_source("crates/ui-components/src/root.rs");
    for required in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should gate component CSS injection via `{required}`."
        );
    }

    let menu_sources = [
        "src/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/item/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
        "src/section/view.rs",
        "src/trigger/view.rs",
        "src/styles.rs",
        "src/item/styles.rs",
        "src/section/styles.rs",
    ];
    for rel_path in menu_sources {
        let source = load_component_source(rel_path);
        for forbidden in [
            " class=\"flex",
            " class=\"grid",
            " class=\"p-",
            " class=\"m-",
            " class=\"rounded-",
            "tailwind",
            "tw-",
            "stylist::",
            "style!(",
            "css!(",
            "styled(",
            "emotion",
        ] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not leak Utility-First/CSS-in-Rust token `{forbidden}`."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。")
                && check.contains("`crates/ui-components/src/css.rs`")
                && check.contains("`inject_components_css=true`")
                && check.contains("`var(--ui-space-*)` 与 `var(--ui-radius-*)`")
                && check.contains("menu_token_first_static_styles_are_aggregated_through_uiroot_without_utility_css_leakage"),
            "checklist should record token-first style contract completion evidence and regression reference."
        );
    }
}

#[test]
fn menu_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let style_files = [
        "src/styles.rs",
        "src/action_menu/styles.rs",
        "src/context_menu/styles.rs",
        "src/dropdown/styles.rs",
        "src/dropdown_menu/styles.rs",
        "src/item/styles.rs",
        "src/menubar/styles.rs",
        "src/navigation_menu/styles.rs",
        "src/section/styles.rs",
        "src/trigger/styles.rs",
    ];
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");
    let theme_css_source = load_workspace_source("crates/ui-theme/src/css.rs");

    for rel_path in style_files {
        let source = load_component_source(rel_path);
        let css_body = source
            .split_once("r#\"")
            .and_then(|(_, tail)| tail.rsplit_once("\"#;").map(|(css, _)| css))
            .unwrap_or(source.as_str());
        assert!(
            css_body.contains("var(--ui-"),
            "{rel_path} should consume ui-theme variables."
        );
        for line in css_body.lines().filter(|line| line.contains("var(--ui-")) {
            assert!(
                line.contains("var(--ui-fallback-"),
                "{rel_path} should use two-level fallback chain; offending line: `{line}`."
            );
        }
        assert!(
            !css_body.contains('#'),
            "{rel_path} should not keep hardcoded hex color terminals in CSS body."
        );
        for token in css_body
            .split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '.' || ch == '-' || ch == '%'))
        {
            let has_digit = token.chars().any(|ch| ch.is_ascii_digit());
            if has_digit && (token.ends_with("px") || token.ends_with("rem")) {
                panic!("{rel_path} should not keep bare size terminal `{token}` in CSS body.");
            }
        }
    }

    let all_styles = [
        load_component_source("src/styles.rs"),
        load_component_source("src/action_menu/styles.rs"),
        load_component_source("src/context_menu/styles.rs"),
        load_component_source("src/dropdown/styles.rs"),
        load_component_source("src/dropdown_menu/styles.rs"),
        load_component_source("src/item/styles.rs"),
        load_component_source("src/menubar/styles.rs"),
        load_component_source("src/navigation_menu/styles.rs"),
        load_component_source("src/section/styles.rs"),
        load_component_source("src/trigger/styles.rs"),
    ]
    .join("\n");
    for required in [
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-drop-zone-min-height, var(--ui-fallback-drop-zone-min-height))",
    ] {
        assert!(
            all_styles.contains(required),
            "menu styles should keep defensive fallback chain marker `{required}`."
        );
    }

    for required in [
        "--ui-fallback-space-2xs",
        "--ui-fallback-radius-lg",
        "--ui-fallback-border-width",
        "--ui-fallback-focus-ring",
        "--ui-fallback-overlay-panel-min-width",
        "--ui-fallback-drop-zone-min-height",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme should remain SSOT source for fallback variable `{required}`."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    let check2 = load_component_source("check2.md");
    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "components/menu/src/*/styles.rs",
        "crates/ui-theme/src/css.rs",
        "menu_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "scripts/check-ui-components-contract-hygiene.sh",
        "cargo test -p ui-menu menu_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "menu check2 defensive-variable evidence should include `{required}`."
        );
    }
}

#[test]
fn menu_cascade_layer_contract_is_aggregated_in_ui_layer_and_runtime_style_is_css_variable_only() {
    let css_source = load_workspace_source("crates/ui-components/src/css.rs");
    let script_source = load_workspace_source("scripts/check-ui-components-contract-hygiene.sh");
    let check2 = load_component_source("check2.md");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-menu\")]",
        "out.push_str(crate::menu::styles::CSS);",
        "#[cfg(feature = \"component-menu_item\")]",
        "out.push_str(crate::menu::item::styles::CSS);",
        "#[cfg(feature = \"component-menu_section\")]",
        "out.push_str(crate::menu::section::styles::CSS);",
        "#[cfg(feature = \"component-dropdown\")]",
        "out.push_str(crate::dropdown::styles::CSS);",
        "#[cfg(feature = \"component-dropdown_menu\")]",
        "out.push_str(crate::dropdown_menu::styles::CSS);",
        "#[cfg(feature = \"component-context_menu\")]",
        "out.push_str(crate::context_menu::styles::CSS);",
        "#[cfg(feature = \"component-action_menu\")]",
        "out.push_str(crate::action_menu::styles::CSS);",
        "#[cfg(feature = \"component-menu_trigger\")]",
        "out.push_str(crate::menu_trigger::styles::CSS);",
        "#[cfg(feature = \"component-menubar\")]",
        "out.push_str(crate::menubar::styles::CSS);",
        "#[cfg(feature = \"component-navigation_menu\")]",
        "out.push_str(crate::navigation_menu::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_source.contains(required),
            "ui-components css aggregation should keep cascade-layer marker `{required}`."
        );
    }

    for rel_path in [
        "src/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/item/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
        "src/section/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            !source.contains("style="),
            "{rel_path} should not use plain inline style assignments."
        );
        for line in source.lines().filter(|line| line.contains("style:")) {
            assert!(
                line.contains("style:--"),
                "{rel_path} runtime style adjustments must be css-variable-only; offending line: `{line}`."
            );
        }
        for forbidden in ["set_property(", ".style(", ".style()"] {
            assert!(
                !source.contains(forbidden),
                "{rel_path} should not mutate runtime style via `{forbidden}`."
            );
        }
    }

    let script_needle = "cargo test -p ui-menu menu_cascade_layer_contract_is_aggregated_in_ui_layer_and_runtime_style_is_css_variable_only";
    assert!(
        script_source.contains(script_needle),
        "contract-hygiene script should include `{script_needle}`."
    );

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "crates/ui-components/src/css.rs",
        "out.push_str(\"\\n@layer ui {\\n\")",
        "style= / style:top",
        "menu_cascade_layer_contract_is_aggregated_in_ui_layer_and_runtime_style_is_css_variable_only",
        "scripts/check-ui-components-contract-hygiene.sh",
        "cargo test -p ui-menu menu_cascade_layer_contract_is_aggregated_in_ui_layer_and_runtime_style_is_css_variable_only",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "menu check2 cascade-layer evidence should include `{required}`."
        );
    }
}

#[test]
fn menu_visual_desire_baseline_is_documented_with_theme_page_and_snapshot_contract() {
    let item_styles = load_component_source("src/item/styles.rs");
    for required in [
        "font-weight: 500;",
        ".ui-menu-item[data-focused=\"true\"]",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 82%",
        "var(--ui-fg, var(--ui-fallback-fg)) 92%",
    ] {
        assert!(
            item_styles.contains(required),
            "menu item styles should expose hierarchy/contrast feedback via `{required}`."
        );
    }

    let menubar_styles = load_component_source("src/menubar/styles.rs");
    for required in [
        ".ui-menubar__trigger:hover:not(:disabled)",
        ".ui-menubar__trigger:focus-visible",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted)) 78%",
    ] {
        assert!(
            menubar_styles.contains(required),
            "menubar styles should keep modern interaction/contrast feedback via `{required}`."
        );
    }

    let navigation_styles = load_component_source("src/navigation_menu/styles.rs");
    for required in [
        ".ui-navigation-menu__item:hover:not([data-disabled=\"true\"])",
        ".ui-navigation-menu__item:focus-visible",
        "font-size: var(--ui-font-size-100, var(--ui-fallback-font-size-100));",
        "var(--ui-fg, var(--ui-fallback-fg)) 70%",
    ] {
        assert!(
            navigation_styles.contains(required),
            "navigation menu styles should keep modern interaction/contrast feedback via `{required}`."
        );
    }

    for source in [&item_styles, &menubar_styles, &navigation_styles] {
        assert!(
            !source.contains("bootstrap"),
            "visual baseline should not regress to bootstrap-like fallback styling."
        );
    }

    let theme_baseline_page =
        load_workspace_source("apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    for required in [
        "slug=\"theme-visual-baseline\"",
        "Default Theme Visual Baseline",
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
    ] {
        assert!(
            theme_baseline_page.contains(required),
            "docs-app should provide visual baseline contract via `{required}`."
        );
    }

    let theme_visual_e2e =
        load_workspace_source("e2e/tests/docs_app_theme_visual_baseline.spec.mjs");
    for required in [
        "toHaveScreenshot(",
        "docs-app-theme-visual-baseline-page.png",
        "docs-app-theme-visual-baseline-button.png",
        "docs-app-theme-visual-baseline-input.png",
        "docs-app-theme-visual-baseline-overlay.png",
    ] {
        assert!(
            theme_visual_e2e.contains(required),
            "e2e baseline should lock visual snapshots via `{required}`."
        );
    }

    let heroui_strategy = load_workspace_source("docs/spec/heroui-parameter-design-strategy.md");
    for required in [
        "接近 HeroUI 的参数设计规范",
        "一次性把所有组件都重写为 HeroUI 完全同构 API。",
    ] {
        assert!(
            heroui_strategy.contains(required),
            "HeroUI alignment strategy should keep quality-alignment but avoid API cloning via `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。")
                && check.contains("theme_visual_baseline.rs")
                && check.contains("docs_app_theme_visual_baseline.spec.mjs")
                && check.contains("menu_visual_desire_baseline_is_documented_with_theme_page_and_snapshot_contract"),
            "checklist should record visual desire completion and regression reference."
        );
    }
}

#[test]
fn menu_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let ui_components_cargo = load_workspace_source("crates/ui-components/Cargo.toml");
    for required in [
        "component-menu = [\"component-active_highlight\"]",
        "component-menu_item = [\"component-menu\"]",
        "component-menu_section = [\"component-menu\"]",
        "component-menu_trigger = [",
        "component-menubar = []",
        "component-navigation_menu = [",
        "web-demo-components = [",
        "all-components = [",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui-components feature graph should register tree-shakable menu features via `{required}`."
        );
    }

    let ui_components_lib = load_workspace_source("crates/ui-components/src/lib.rs");
    for required in [
        "#[cfg(feature = \"component-menu\")]",
        "#[path = \"../../../components/menu/src/mod.rs\"]",
        "pub mod menu;",
        "#[cfg(feature = \"component-menu_trigger\")]",
        "pub mod menu_trigger;",
        "#[cfg(feature = \"component-menubar\")]",
        "pub mod menubar;",
        "#[cfg(feature = \"component-navigation_menu\")]",
        "pub mod navigation_menu;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "lib.rs should gate menu-family modules by features via `{required}`."
        );
    }

    let ui_components_css = load_workspace_source("crates/ui-components/src/css.rs");
    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "#[cfg(feature = \"component-menu\")]",
        "out.push_str(crate::menu::styles::CSS);",
        "#[cfg(feature = \"component-menu_item\")]",
        "out.push_str(crate::menu::item::styles::CSS);",
        "#[cfg(feature = \"component-menu_section\")]",
        "out.push_str(crate::menu::section::styles::CSS);",
        "#[cfg(feature = \"component-menu_trigger\")]",
        "out.push_str(crate::menu_trigger::styles::CSS);",
        "#[cfg(feature = \"component-menubar\")]",
        "out.push_str(crate::menubar::styles::CSS);",
        "#[cfg(feature = \"component-navigation_menu\")]",
        "out.push_str(crate::navigation_menu::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "css.rs should keep menu-family style aggregation feature-gated via `{required}`."
        );
    }

    let web_demo_cargo = load_workspace_source("apps/web-demo/Cargo.toml");
    for required in [
        "ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
        "ui-layout = { path = \"../../crates/ui-layout\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }",
    ] {
        assert!(
            web_demo_cargo.contains(required),
            "web-demo should depend on scoped feature bundles via `{required}`."
        );
    }
    assert!(
        !web_demo_cargo.contains("all-components"),
        "web-demo must not pull all-components directly."
    );

    let tree_shaking_script = load_workspace_source("scripts/check-ui-components-tree-shaking.sh");
    for required in [
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MIN_FEATURES\"",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(required),
            "tree-shaking CI script should enforce feature-pruning and budget guard via `{required}`."
        );
    }

    let tree_shaking_budget = load_workspace_source("scripts/tree_shaking_budget.env");
    for required in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            tree_shaking_budget.contains(required),
            "tree-shaking budget file should define `{required}`."
        );
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。")
                && check.contains("component-menu,inject-css")
                && check.contains("web-demo-components")
                && check.contains("scripts/tree_shaking_budget.env")
                && check.contains("menu_tree_shaking_contract_is_feature_gated_and_budget_guarded"),
            "checklist should record tree-shaking completion evidence and regression reference."
        );
    }
}

#[test]
fn menu_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2 = load_component_source("check2.md");
    let tree_shaking_script = load_workspace_source("scripts/check-ui-components-tree-shaking.sh");

    for required in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "component-menu,inject-css",
        "crates/ui-components/Cargo.toml",
        "crates/ui-components/src/lib.rs",
        "crates/ui-components/src/css.rs",
        "menu_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "menu_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "scripts/check-ui-components-tree-shaking.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include tree-shaking feature-pruning evidence marker `{required}`."
        );
    }

    for needle in [
        "MENU_MIN_FEATURES=\"component-menu,inject-css\"",
        "cargo test -p ui-menu menu_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "cargo test -p ui-menu menu_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$MENU_MIN_FEATURES\"",
        "if ! grep -q 'feature \"component-menu\" (command-line)' <<<\"$MENU_TREE_OUTPUT\";",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$MENU_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$MENU_TREE_OUTPUT\";",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$MENU_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should include menu feature-pruning guard `{needle}`."
        );
    }
}

#[test]
fn menu_type_system_and_semantic_markers_form_machine_readable_contract() {
    let mod_source = load_component_source("src/mod.rs");
    for required in [
        "pub struct MenuItemSpec {",
        "pub label: String,",
        "pub kind: MenuItemKind,",
        "pub is_disabled: bool,",
    ] {
        assert!(
            mod_source.contains(required),
            "menu module should expose typed item spec contract via `{required}`."
        );
    }

    for (logic_path, required_tokens) in [
        (
            "src/action_menu/logic.rs",
            vec![
                "pub enum ActionMenuDisabledState",
                "pub enum ActionMenuActionMode",
                "pub fn normalize_menu_items(input: ActionMenuItemsInput) -> ActionMenuItemsOutput",
                "fn source_attr(is_custom: bool) -> &'static str {",
            ],
        ),
        (
            "src/context_menu/logic.rs",
            vec![
                "pub enum ContextMenuDisabledState",
                "pub enum ContextMenuActionMode",
                "pub fn normalize_open_state(input: ContextMenuOpenStateInput) -> ContextMenuOpenState",
                "fn source_attr(is_custom: bool) -> &'static str {",
            ],
        ),
        (
            "src/dropdown_menu/logic.rs",
            vec![
                "pub enum DropdownMenuActionMode",
                "pub fn normalize_open_state(input: DropdownMenuOpenStateInput) -> DropdownMenuOpenState",
            ],
        ),
        (
            "src/trigger/logic.rs",
            vec![
                "pub enum MenuTriggerActionMode",
                "pub fn normalize_open_state(input: MenuTriggerOpenStateInput) -> MenuTriggerOpenState",
            ],
        ),
        (
            "src/menubar/logic.rs",
            vec![
                "pub enum MenubarActionMode",
                "pub fn normalize_close_on_action(input: MenubarActionModeInput) -> MenubarActionMode",
                "fn source_attr(is_custom: bool) -> &'static str {",
            ],
        ),
        (
            "src/section/logic.rs",
            vec![
                "pub enum MenuSectionHeadingTone",
                "pub fn normalize_props(input: MenuSectionNormalizeInput) -> MenuSectionNormalizedProps",
            ],
        ),
        (
            "src/navigation_menu/logic.rs",
            vec![
                "pub enum NavigationSelectionTarget",
                "pub fn resolve_key_decision(",
                "fn source_attr(is_custom: bool) -> &'static str {",
            ],
        ),
        (
            "src/item/logic.rs",
            vec![
                "MenuItemKind::Action => \"action\"",
                "MenuItemKind::Checkbox { .. } => \"checkbox\"",
                "MenuItemKind::Radio { .. } => \"radio\"",
                "\"focused-checked\"",
                "\"focused\"",
                "\"checked\"",
                "\"idle\"",
            ],
        ),
    ] {
        let source = load_component_source(logic_path);
        for token in required_tokens {
            assert!(
                source.contains(token),
                "{logic_path} should keep type-constrained normalization/marker contract via `{token}`."
            );
        }
    }

    let source_markers = [
        load_component_source("src/action_menu/logic.rs"),
        load_component_source("src/context_menu/logic.rs"),
        load_component_source("src/menubar/logic.rs"),
        load_component_source("src/navigation_menu/logic.rs"),
    ]
    .join("\n");
    assert!(
        source_markers.contains("if is_custom { \"custom\" } else { \"default\" }"),
        "source markers should stay in the closed set `custom/default`."
    );

    for rel_path in [
        "src/view.rs",
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/menubar/view.rs",
        "src/navigation_menu/view.rs",
        "src/item/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        for marker in [
            "data-state=",
            "data-controlled=",
            "data-uncontrolled=",
            "data-",
            "aria-",
        ] {
            assert!(
                source.contains(marker),
                "{rel_path} should expose machine-readable semantic marker `{marker}`."
            );
        }
    }

    let check2 = fs::read_to_string(component_dir().join("check2.md"))
        .expect("menu check2.md should be readable");
    let src_check2 = fs::read_to_string(component_dir().join("src/check2.md"))
        .expect("menu src/check2.md should be readable");
    for check in [&check2, &src_check2] {
        assert!(
            check.contains(
                "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"
            ) && check.contains("`MenuItemSpec { label, kind: MenuItemKind, is_disabled }`")
                && check.contains("`state_attr` / `source_attr`（`custom/default`）")
                && check.contains(
                    "menu_type_system_and_semantic_markers_form_machine_readable_contract"
                ),
            "checklist should record type-system + semantic-marker completion and regression reference."
        );
    }
}

#[test]
fn menu_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let engineering_script = load_workspace_source("scripts/check-ui-components-engineering.sh");
    let cargo_source = load_component_source("Cargo.toml");
    let shared_trace_source = load_workspace_source("crates/ui-headless/src/trace.rs");
    let shared_controllable_state_source =
        load_workspace_source("crates/ui-headless/src/controllable_state.rs");

    for protocol_path in [
        "src/protocol.rs",
        "src/action_menu/protocol.rs",
        "src/context_menu/protocol.rs",
        "src/dropdown/protocol.rs",
        "src/dropdown_menu/protocol.rs",
        "src/item/protocol.rs",
        "src/menubar/protocol.rs",
        "src/navigation_menu/protocol.rs",
        "src/section/protocol.rs",
        "src/trigger/protocol.rs",
    ] {
        let source = load_component_source(protocol_path);
        for required in [
            "use serde::{Deserialize, Serialize};",
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
            "#[serde(rename_all = \"snake_case\")]",
            "pub enum ",
            "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
            "pub struct ",
            "#[serde(default)]",
            "pub schema_version:",
            "#[cfg(test)]",
            "#[path = ",
        ] {
            assert!(
                source.contains(required),
                "{protocol_path} should keep serde protocol contract marker `{required}`."
            );
        }
        for forbidden in [
            "serde_json::",
            "from_json(",
            "to_json_result(",
            "SchemaError",
            "migrate_v1_to_v2(",
        ] {
            assert!(
                !source.contains(forbidden),
                "{protocol_path} should avoid ad-hoc serde/migration token `{forbidden}`."
            );
        }
    }

    for required in [
        "pub struct UiTraceEvent {",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            shared_trace_source.contains(required),
            "shared trace should keep canonical marker `{required}`."
        );
    }
    for required in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            shared_controllable_state_source.contains(required),
            "shared controllable-state trace path should keep marker `{required}`."
        );
    }

    for rel_path in [
        "src/action_menu/view.rs",
        "src/context_menu/view.rs",
        "src/dropdown/view.rs",
        "src/dropdown_menu/view.rs",
        "src/trigger/view.rs",
    ] {
        let source = load_component_source(rel_path);
        assert!(
            source.contains("use_controllable_open_state_traced("),
            "{rel_path} should reuse headless traced open-state helper."
        );
    }

    let mut rust_sources = Vec::new();
    collect_paths_with_extension(&component_dir().join("src"), "rs", &mut rust_sources);
    for source_path in rust_sources {
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {source_path:?}: {e}"));
        for forbidden in [
            "tracing::span!(",
            "tracing::event!(",
            "#[tracing::instrument]",
            "target: \"ui_menu::",
            "const MENU_TRACE_TARGET",
            "tokio::",
            "tokio ",
            "async_std::",
            "async-std",
            "runtime::Handle",
            "smol::",
            "spawn_blocking(",
        ] {
            assert!(
                !source.contains(forbidden),
                "menu source `{source_path:?}` should not leak engineering boundary token `{forbidden}`."
            );
        }
    }

    for forbidden in [
        "tokio =",
        "async-std =",
        "async_std =",
        "dep:tracing",
        "wasm-debug",
        "menu-wasm-debug",
        "menu_wasm_debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "components/menu Cargo contract should not include `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-menu menu_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        engineering_script.contains(script_needle),
        "engineering script should include `{script_needle}`."
    );

    for rel_path in ["check2.md", "src/check2.md", "src/action_menu/check2.md"] {
        let check = load_component_source(rel_path);
        for required in [
            "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
            "components/menu/src/protocol.rs",
            "components/menu/src/*/protocol.rs",
            "use_controllable_open_state_traced",
            "components/menu/Cargo.toml",
            "menu_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
            "scripts/check-ui-components-engineering.sh",
            "cargo test -p ui-menu menu_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries",
            "Invalid cross-device link (os error 18)",
        ] {
            assert!(
                check.contains(required),
                "{rel_path} should keep engineering contract evidence marker `{required}`."
            );
        }
    }
}

#[test]
fn menu_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let mut rust_sources = Vec::new();
    collect_paths_with_extension(&component_dir().join("src"), "rs", &mut rust_sources);

    for source_path in rust_sources {
        let source = fs::read_to_string(&source_path)
            .unwrap_or_else(|e| panic!("read_to_string failed for {source_path:?}: {e}"));
        for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "menu non-test source `{source_path:?}` should forbid rust-hygiene anti-pattern `{forbidden}`."
            );
        }
    }
}

#[test]
fn menu_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic = load_component_source("src/logic.rs");
    let view = load_component_source("src/view.rs");
    let styles = load_component_source("src/styles.rs");
    let motion = load_component_source("src/motion.rs");
    let protocol = load_component_source("src/protocol.rs");
    let mod_source = load_component_source("src/mod.rs");

    for required in [
        "use std::borrow::Cow;",
        "[Cow::Borrowed(BASE_CLASS_NAME), Cow::Owned(class_name)]",
        ".map(|class_name| class_name.as_ref())",
        ".unwrap_or_else(|| BASE_CLASS_NAME.into());",
    ] {
        assert!(
            logic.contains(required),
            "menu logic should keep Cow-based string hotspot mitigation marker `{required}`."
        );
    }

    let combined = format!("{logic}\n{view}\n{styles}\n{motion}\n{protocol}\n{mod_source}");
    for forbidden in [
        "BASE_CLASS_NAME.to_string()",
        "String::from(BASE_CLASS_NAME)",
        "format!(\"{BASE_CLASS_NAME}",
    ] {
        assert!(
            !combined.contains(forbidden),
            "menu string hotspot contract should avoid `{forbidden}` in core files."
        );
    }
}

#[test]
fn menu_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = load_workspace_source("scripts/check-rust-hygiene.sh");
    let engineering_script = load_workspace_source("scripts/check-ui-components-engineering.sh");

    for required in [
        r#"'\.(unwrap|unwrap_err|expect)\s*\('"#,
        r#"'^[[:space:]]*let[[:space:]]+_[[:space:]]*='"#,
        "string clone hotspots (prefer Cow<'static, str>)",
        "failed: enforce no unwrap/expect, no let _ = swallowing, and no string clone churn",
    ] {
        assert!(
            rust_hygiene_script.contains(required),
            "rust-hygiene script should enforce `{required}`."
        );
    }

    for needle in [
        "cargo test -p ui-menu menu_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-menu menu_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-menu menu_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering script should include menu rust-hygiene command `{needle}`."
        );
    }
}

#[test]
fn menu_check2_marks_rust_hygiene_contract_complete() {
    let check2 = load_component_source("check2.md");
    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "Cow<'static, str>",
        "./scripts/check-rust-hygiene.sh",
        "components/menu/test/semantics.rs::menu_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "components/menu/test/semantics.rs::menu_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "components/menu/test/semantics.rs::menu_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-components-engineering.sh",
    ] {
        assert!(
            check2.contains(required),
            "check2.md should include rust-hygiene evidence marker `{required}`."
        );
    }
}
