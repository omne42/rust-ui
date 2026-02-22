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
fn icon_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/icon/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Icon internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn icon_uses_logic_state_model() {
    let logic_source = load_source("src/icon/logic.rs");
    let view_source = load_source("src/icon/view.rs");

    for needle in [
        "pub enum IconSize",
        "pub enum IconTone",
        "pub fn normalize_optional_text(",
        "pub fn normalize_aria_label(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
        "aria_source_attr",
        "class_source_attr",
    ] {
        assert!(
            logic_source.contains(needle),
            "Icon logic should include `{needle}` for centralized state derivation."
        );
    }

    for needle in [
        "logic::normalize_optional_text(class_name)",
        "logic::normalize_aria_label(aria_label)",
        "logic::resolve_state(IconStateInput {",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "Icon view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn icon_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/icon/view.rs");

    for attr in [
        "data-slot=\"icon\"",
        "data-slot=\"icon-glyph\"",
        "data-size=state.size_attr",
        "data-tone=state.tone_attr",
        "data-state=state.data_state_attr",
        "data-disabled=state.is_disabled.then_some(\"true\")",
        "data-decorative=state.is_decorative.then_some(\"true\")",
        "data-has-label=state.has_accessible_name.then_some(\"true\")",
        "data-aria-source=state.aria_source_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "Icon should expose `{attr}` for baseline-style styling and state inspection."
        );
    }
}

#[test]
fn icon_agent_contract_schema_is_typed_and_mounted_across_views() {
    let protocol_source = load_source("src/icon/protocol.rs");

    for needle in [
        "pub const ICON_AGENT_SCHEMA: &str = \"ui.icon.agent-contract.v1\";",
        "pub enum IconAgentIntent",
        "pub enum IconAgentAction",
        "pub enum IconAgentState",
        "pub enum IconAgentSource",
        "pub enum IconStreamingRequirement",
        "pub enum IconOutputMode",
        "pub enum IconOutputStatus",
        "pub struct IconOutputDataAttrs",
        "pub fn resolve_agent_data_attrs(",
        "pub const fn resolve_output_data_attrs()",
        "pub fn from_state_attr(",
        "pub fn from_source_attr(",
    ] {
        assert!(
            protocol_source.contains(needle),
            "icon protocol should include typed agent-contract primitive `{needle}`."
        );
    }

    for rel_path in [
        "src/icon/view.rs",
        "src/icon/icons/view.rs",
        "src/icon/set/view.rs",
        "src/icon/ui/view.rs",
        "src/icon/workflow/view.rs",
    ] {
        let view_source = load_source(rel_path);

        for needle in [
            "protocol::resolve_agent_data_attrs(",
            "data-ui-schema=agent_data.schema_name",
            "data-ui-schema-version=agent_data.schema_version.as_attr()",
            "data-ui-intent=agent_data.intent.as_attr()",
            "data-ui-action=agent_data.action.as_attr()",
            "data-ui-state=agent_data.state.as_attr()",
            "data-ui-source=agent_data.source.as_attr()",
            "data-ui-streaming=output_data.streaming.as_attr()",
            "data-ui-streaming-fallback=output_data.fallback.as_attr()",
            "data-ui-output-mode=output_data.mode.as_attr()",
            "data-ui-output-status=output_data.status.as_attr()",
        ] {
            assert!(
                view_source.contains(needle),
                "{rel_path} should mount typed agent-contract marker `{needle}`.",
            );
        }
    }
}

#[test]
fn icon_snapshot_mode_is_baseline_and_streaming_protocol_is_not_exposed() {
    let manifest_source = load_source("../../components/icon/src/Component.toml");
    let rbi_source = load_source("../../components/icon/src/icon.rbi");

    for needle in [
        "schema_version = \"1\"",
        "name = \"Icon\"",
        "crate = \"ui-icon\"",
        "rbi = \"icon.rbi\"",
        "name = \"snapshot_rendering\"",
        "name = \"streaming_optional_with_snapshot_fallback\"",
        "enabled = true",
        "streaming = \"optional\"",
        "fallback = \"snapshot\"",
        "output_mode = \"snapshot\"",
        "output_status = \"verified\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "icon Component.toml should include snapshot baseline marker `{needle}`."
        );
    }

    for needle in [
        "pub fn Icon(",
        "pub fn Icons(",
        "pub fn Iconset(",
        "pub fn IconsUi(",
        "pub fn IconsWorkflow(",
    ] {
        assert!(
            rbi_source.contains(needle),
            "icon.rbi should project complete snapshot-callable entry `{needle}`."
        );
    }

    for forbidden in [
        "streaming: bool",
        "output_mode: ",
        "chunk: ",
        "token: ",
        "delta: ",
    ] {
        assert!(
            !rbi_source.contains(forbidden),
            "icon.rbi should not expose streaming-only protocol field `{forbidden}` in snapshot baseline path.",
        );
    }
}

#[test]
fn icon_version_upgrade_migration_is_not_required_in_current_change_set() {
    let manifest_source = load_source("../../components/icon/src/Component.toml");
    let protocol_source = load_source("src/icon/protocol.rs");
    let rbi_source = load_source("../../components/icon/src/icon.rbi");

    assert!(
        manifest_source.contains("schema_version = \"1\""),
        "icon manifest should remain on schema v1 when no breaking upgrade is introduced.",
    );
    assert!(
        protocol_source.contains("pub enum IconComponentSchemaVersion")
            && protocol_source.contains("V1"),
        "icon protocol should still expose only v1 schema in this change set.",
    );

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "deprecation_window",
        "deprecated_since",
    ] {
        assert!(
            !protocol_source.contains(forbidden),
            "icon protocol should not introduce breaking-upgrade migration hook `{forbidden}` without explicit version upgrade.",
        );
        assert!(
            !manifest_source.contains(forbidden),
            "icon manifest should not require migration registry marker `{forbidden}` in non-breaking change set.",
        );
        assert!(
            !rbi_source.contains(forbidden),
            "icon RBI should not expose migration symbol `{forbidden}` in non-breaking change set.",
        );
    }
}

#[test]
fn icon_styles_include_tone_size_and_state_markers() {
    let source = load_source("src/icon/styles.rs");

    for selector in [
        ".ui-icon--size-sm",
        ".ui-icon[data-size=\"md\"]",
        ".ui-icon--tone-default",
        ".ui-icon[data-tone=\"accent\"]",
        ".ui-icon--disabled",
        ".ui-icon[data-disabled=\"true\"]",
        ".ui-icon--decorative",
        ".ui-icon[data-decorative=\"true\"]",
        ".ui-icon--custom-class",
        ".ui-icon[data-custom-class=\"true\"]",
    ] {
        assert!(
            source.contains(selector),
            "Icon styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn icon_supports_accessibility_role_and_label_contract() {
    let source = load_source("src/icon/view.rs");

    for needle in [
        "role=(!state.is_decorative).then_some(\"img\")",
        "aria-label=state.has_accessible_name.then_some(aria_label)",
        "aria-hidden=state.is_decorative.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Icon should include `{needle}` for baseline-style accessibility contracts."
        );
    }
}

#[test]
fn icon_semantics_contract_covers_aria_and_data_markers_without_visual_snapshot_dependency() {
    let icon_view = load_source("src/icon/view.rs");

    for needle in [
        "role=(!state.is_decorative).then_some(\"img\")",
        "aria-label=state.has_accessible_name.then_some(aria_label)",
        "aria-hidden=state.is_decorative.then_some(\"true\")",
    ] {
        assert!(
            icon_view.contains(needle),
            "icon root semantics should include `{needle}`."
        );
    }

    for rel_path in [
        "src/icon/view.rs",
        "src/icon/icons/view.rs",
        "src/icon/set/view.rs",
        "src/icon/ui/view.rs",
        "src/icon/workflow/view.rs",
    ] {
        let source = load_source(rel_path);
        for needle in [
            "data-slot=",
            "data-state=",
            "data-ui-schema=",
            "data-ui-source=",
        ] {
            assert!(
                source.contains(needle),
                "{rel_path} should expose semantic marker `{needle}`.",
            );
        }
    }
}

#[test]
fn icon_focus_flow_not_applicable_and_render_path_stays_static() {
    for rel_path in [
        "src/icon/view.rs",
        "src/icon/icons/view.rs",
        "src/icon/set/view.rs",
        "src/icon/ui/view.rs",
        "src/icon/workflow/view.rs",
    ] {
        let source = load_source(rel_path);

        for forbidden_focus in ["tabindex=", "on:focus=", "on:blur=", "on:keydown="] {
            assert!(
                !source.contains(forbidden_focus),
                "{rel_path} should stay non-focusable for icon semantics; found `{forbidden_focus}`.",
            );
        }

        for forbidden_reactive in [
            "create_signal(",
            "create_rw_signal(",
            "create_memo(",
            "create_effect(",
        ] {
            assert!(
                !source.contains(forbidden_reactive),
                "{rel_path} should not introduce reactive render loops; found `{forbidden_reactive}`.",
            );
        }
    }
}

#[test]
fn icon_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "pub(super) fn icon() -> AnyView",
        "title=\"Icon\"",
        "slug=\"icon\"",
        "description=\"baseline-style icon primitive with centralized size/tone/accessibility/source state contracts and stable slot/data markers.\"",
        "<Playground title=\"Size + Tone Matrix\" code_signal=matrix_code>",
        "<Playground title=\"Accessible + Disabled + Custom Class\" code_signal=states_code>",
        "<Icon",
    ] {
        assert!(
            source.contains(needle),
            "display_extra icon docs should include `{needle}` for primary playground coverage.",
        );
    }
}

#[test]
fn icon_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Size + Tone Matrix\"",
        "size=IconSize::Sm",
        "tone=IconTone::Default",
        "size=IconSize::Md",
        "tone=IconTone::Muted",
        "size=IconSize::Lg",
        "tone=IconTone::Accent",
        "tone=IconTone::Danger",
        "is_decorative=true",
        "title=\"Accessible + Disabled + Custom Class\"",
        "is_decorative=false",
        "aria_label=\"Sync successful\".to_string()",
        "is_disabled=true",
        "class_name=\"docs-icon-custom\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icon docs playgrounds should contain `{needle}`."
        );
    }
}

#[test]
fn icon_docs_api_names_and_defaults_are_synced_with_logic_contract() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    let view_source = load_source("src/icon/view.rs");
    let logic_source = load_source("src/icon/logic.rs");

    for needle in [
        "title=\"Hello World (Default Path)\"",
        "<Icon>\"✓\"</Icon>",
        "title=\"Size + Tone Matrix\"",
        "title=\"Accessible + Disabled + Custom Class\"",
        "size=IconSize::Sm",
        "tone=IconTone::Default",
        "is_disabled=true",
        "is_decorative=true",
        "is_decorative=false",
    ] {
        assert!(
            docs_source.contains(needle),
            "icon docs should stay synced with public API marker `{needle}`.",
        );
    }

    for needle in [
        "#[prop(optional)] size: IconSize",
        "#[prop(optional)] tone: IconTone",
        "#[prop(optional)] is_disabled: bool",
        "#[prop(default = true)] is_decorative: bool",
    ] {
        assert!(
            view_source.contains(needle),
            "icon view API contract should include `{needle}`.",
        );
    }

    for needle in [
        "pub enum IconSize",
        "#[default]\n    Md",
        "pub enum IconTone",
        "#[default]\n    Default",
        "pub const DEFAULT_ARIA_LABEL: &str = \"Icon\";",
    ] {
        assert!(
            logic_source.contains(needle),
            "icon logic default contract should include `{needle}`.",
        );
    }
}

#[test]
fn icon_docs_workbench_exposes_display_config_code_and_css_test_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "code_signal=workbench_code",
        "test_css_source=workbench_test_css",
        "test_source_path=\"components/icon/src/styles.rs\".to_string()",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"icon-workbench-controls\"",
        "display: baseline vs configured vs disabled contrast",
    ] {
        assert!(
            source.contains(needle),
            "icon workbench should contain `{needle}`.",
        );
    }
}

#[test]
fn icon_docs_are_copy_paste_ready_with_controlled_contrast_and_stream_snapshot_contract() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");

    for needle in [
        "title=\"Hello World (Default Path)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for Icon)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "let icon_code_imports =",
        "use leptos::prelude::*;\\nuse ui::{Icon, IconSize, IconTone};",
        "code_imports=icon_code_imports.clone()",
        "requires `ui` dependency in Cargo.toml",
        "streaming is optional and falls back to snapshot",
        "aria_label=\"Mapped from upstream app state\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "icon docs copy-paste/streaming/contrast contract should contain `{needle}`.",
        );
    }
}

#[test]
fn icon_docs_source_path_points_to_real_component_file() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/display_extra.rs");
    assert!(
        source.contains("test_source_path=\"components/icon/src/styles.rs\".to_string()"),
        "icon docs should point workbench source path to components/icon/src/styles.rs",
    );

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));
    let source_path = workspace_dir.join("components/icon/src/styles.rs");
    assert!(
        source_path.exists(),
        "icon docs source path should resolve to a real file: {source_path:?}",
    );
}

#[test]
fn icon_readme_covers_workbench_display_config_code_css_test_sections() {
    let source = load_source("src/icon/README.md");

    for needle in [
        "# Icon",
        "Docs Playground（展示 / Config / Code / CSS Test）",
        "展示",
        "Config",
        "Code",
        "CSS Test",
        "对比场景",
        "Workbench (Display + Config + Code + CSS Test)",
    ] {
        assert!(
            source.contains(needle),
            "icon README should contain `{needle}`.",
        );
    }
}

#[test]
fn icon_readme_is_beginner_friendly_with_default_path_before_advanced_sections() {
    let source = load_source("src/icon/README.md");

    for needle in [
        "## 先用起来（Hello World）",
        "不需要先理解分层架构，先用默认 API：",
        "use ui::Icon;",
        "<Icon>\"✓\"</Icon>",
        "## 常见用法（默认路径优先）",
        "## 进阶（需要时再看）",
        "### Architecture Layers",
        "### API (Table)",
    ] {
        assert!(
            source.contains(needle),
            "icon README should include beginner-friendly section `{needle}`.",
        );
    }

    let hello_index = source
        .find("## 先用起来（Hello World）")
        .expect("README should contain beginner hello section");
    let common_index = source
        .find("## 常见用法（默认路径优先）")
        .expect("README should contain common usage section");
    let advanced_index = source
        .find("## 进阶（需要时再看）")
        .expect("README should contain advanced section");
    let architecture_index = source
        .find("### Architecture Layers")
        .expect("README should contain architecture section");

    assert!(
        hello_index < common_index && common_index < advanced_index,
        "README should keep default path before advanced content."
    );
    assert!(
        advanced_index < architecture_index,
        "Architecture details should stay in advanced section, after default path."
    );
}

#[test]
fn icon_heroui_strategy_doc_and_docs_entry_are_synced() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let readme_source = load_source("src/icon/README.md");

    for needle in [
        "### Icon 同步记录（2026-02-21）",
        "`Icon` 参数主轴保持 `size/tone/is_disabled/is_decorative/aria_label/class_name/lang/dir/slot`",
        "component_doc!(\"Icon\", \"icon\", \"Display\", display_extra::icon)",
        "`#/components/icon` 可索引访问",
        "components/icon/src/styles.rs",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy doc should contain icon sync marker `{needle}`.",
        );
    }

    assert!(
        pages_source
            .contains("component_doc!(\"Icon\", \"icon\", \"Display\", display_extra::icon)"),
        "docs-app catalog should keep icon page indexable from component list.",
    );
    assert!(
        readme_source.contains("# Icon"),
        "icon component should keep README entry as equivalent documentation portal.",
    );
}
