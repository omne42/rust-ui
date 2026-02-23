use std::fs;
use std::path::Path;

fn resolve_source_path(rel_path: &str) -> Option<std::path::PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let mut candidates = vec![manifest_dir.join(rel_path)];

    if let Some(component_rel_path) = rel_path.strip_prefix("../../components/") {
        let direct = workspace_dir.join("components").join(component_rel_path);
        candidates.push(direct.clone());

        let parts: Vec<&str> = component_rel_path.split('/').collect();
        if parts.len() > 3 && parts.get(1) == Some(&"src") && parts.get(2) == parts.first() {
            let collapsed = workspace_dir
                .join("components")
                .join(parts[0])
                .join("src")
                .join(parts[3..].join("/"));
            candidates.push(collapsed);
        }
    }

    if let Some(src_rel_path) = rel_path.strip_prefix("src/") {
        let segments: Vec<&str> = src_rel_path.split('/').collect();
        let components_root = workspace_dir.join("components");

        if let Ok(entries) = fs::read_dir(&components_root) {
            let component_dirs: Vec<String> = entries
                .flatten()
                .filter_map(|entry| {
                    let path = entry.path();
                    path.is_dir()
                        .then(|| entry.file_name().to_string_lossy().to_string())
                })
                .collect();

            for component_dir in component_dirs {
                for start in 0..segments.len() {
                    for end in start..segments.len() {
                        let name = segments[start..=end]
                            .iter()
                            .map(|segment| segment.replace('_', "-"))
                            .collect::<Vec<_>>()
                            .join("-");

                        if name != component_dir {
                            continue;
                        }

                        if end + 1 >= segments.len() {
                            candidates
                                .push(components_root.join(&component_dir).join("src/mod.rs"));
                            candidates
                                .push(components_root.join(&component_dir).join("src/check2.md"));
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                            continue;
                        }

                        let suffix = segments[end + 1..].join("/");
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("src")
                                .join(&suffix),
                        );
                        candidates.push(
                            components_root
                                .join(&component_dir)
                                .join("test")
                                .join(&suffix),
                        );

                        if suffix == "check2.md" {
                            candidates.push(components_root.join(&component_dir).join("check2.md"));
                        }
                    }
                }
            }
        }
    }

    candidates.into_iter().find(|path| path.exists())
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_source_path(rel_path)
        .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR")).join(rel_path));

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}
#[test]
fn toast_does_not_expose_logic_module() {
    let source = load_source("../../components/toast/src/toast/mod.rs");

    assert!(
        !source.contains("pub mod logic"),
        "Toast's `logic` module should stay private to avoid leaking store internals into the public API."
    );
}

#[test]
fn toast_module_exposes_slot_and_part_state_contracts() {
    let source = load_source("../../components/toast/src/toast/mod.rs");

    for needle in [
        "pub enum ToastSlot",
        "pub struct ToastPartStateInput",
        "pub struct ToastPartState",
        "pub enum ToastViewportSlot",
        "pub enum ToastStoreSource",
        "pub struct ToastViewportStateInput",
        "pub struct ToastViewportState",
        "DEFAULT_VIEWPORT_PORTAL",
        "DEFAULT_VIEWPORT_MAX_TOASTS",
    ] {
        assert!(
            source.contains(needle),
            "toast::mod should include `{needle}` for stable toast contracts."
        );
    }
}

#[test]
fn toast_is_publicly_exported_from_toast_module_and_crate_root() {
    let toast_mod = load_source("../../components/toast/src/toast/mod.rs");
    let crate_root = load_source("src/lib.rs");

    assert!(
        toast_mod.contains("pub use view::{Toast, ToastViewport};"),
        "toast::mod should re-export both Toast and ToastViewport."
    );
    assert!(
        crate_root.contains("Toast, ToastMotion"),
        "crate root should expose Toast together with toast types."
    );
}

#[test]
fn toast_logic_models_state_and_source_contracts() {
    let source = load_source("../../components/toast/src/toast/logic.rs");

    for needle in [
        "pub const DEFAULT_TITLE: &str = \"Notification\";",
        "pub const DEFAULT_VIEWPORT_PORTAL: bool = true;",
        "pub const DEFAULT_VIEWPORT_MAX_TOASTS: usize = toast_state::DEFAULT_MAX_TOASTS;",
        "pub fn toast_state_attr(is_open: bool)",
        "pub fn description_attr(has_description: bool)",
        "pub fn close_mode_attr(has_on_close: bool)",
        "pub fn viewport_state_attr(portal: bool)",
        "pub fn viewport_queue_attr(max_toasts: usize)",
        "pub fn normalize_viewport_max_toasts(max_toasts: usize) -> usize",
        "pub fn resolve_state(input: ToastPartStateInput) -> ToastPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: ToastPartState)",
        "pub fn resolve_viewport_state(input: ToastViewportStateInput) -> ToastViewportState",
        "pub fn compose_viewport_class_name(",
        "pub fn resolve_open_state_config(",
    ] {
        assert!(
            source.contains(needle),
            "Toast logic should include `{needle}` for centralized source-state derivation."
        );
    }
}

#[test]
fn toast_store_state_primitives_are_sourced_from_ui_state_primitives() {
    let source = load_source("../../components/toast/src/toast/logic.rs");
    let primitive = load_source("../ui-state-primitives/src/toast.rs");

    for needle in [
        "use ui_state_primitives::toast as toast_state;",
        "toast_state::ToastState::from_records(",
        "toast_state::normalize_max_toasts(",
    ] {
        assert!(
            source.contains(needle),
            "Toast logic should consume state primitive boundary via `{needle}`."
        );
    }

    for needle in [
        "pub struct ToastState<T>",
        "pub fn normalize_max_toasts(max_toasts: usize) -> usize",
    ] {
        assert!(
            primitive.contains(needle),
            "Toast state primitive module should include `{needle}`."
        );
    }
}

#[test]
fn toast_view_uses_logic_state_contracts() {
    let source = load_source("../../components/toast/src/toast/view.rs");

    for needle in [
        "logic::resolve_state(ToastPartStateInput {",
        "slot: ToastSlot::Root",
        "logic::compose_class_name(class_name.get_value(), state.get())",
        "data-slot=move || state.get().slot_attr",
        "data-state=move || state.get().state_attr",
        "data-variant=move || state.get().variant_attr",
        "data-description=move || state.get().description_attr",
        "data-open=move || state.get().open_attr",
        "data-close-mode=move || state.get().close_mode_attr",
        "data-id-source=move || state.get().id_source_attr",
        "data-description-source=move || state.get().description_source_attr",
        "data-class-source=move || state.get().class_source_attr",
        "data-motion-source=move || state.get().motion_source_attr",
        "data-close-source=move || state.get().close_source_attr",
        "data-exit-source=move || state.get().exit_source_attr",
        "data-control-mode=control_mode_attr",
        "data-open-source=open_source_attr",
        "data-default-open-source=default_open_source_attr",
        "data-open-change-source=open_change_source_attr",
        "data-custom-id=move || state.get().has_custom_id.then_some(\"true\")",
        "data-custom-description=move || state.get().has_custom_description.then_some(\"true\")",
        "data-custom-class=move || state.get().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || state.get().has_custom_motion.then_some(\"true\")",
        "data-custom-close=move || state.get().has_custom_on_close.then_some(\"true\")",
        "data-custom-exit=move || state.get().has_custom_on_exit_complete.then_some(\"true\")",
        "use_controllable_open_state_traced(",
        "logic::resolve_open_state_config(",
        "use crate::close_button::{CloseButton, CloseButtonSize, CloseButtonVariant};",
        "common.close_aria_label",
        "locale_attrs(logic::normalize_optional_text(lang), dir)",
    ] {
        assert!(
            source.contains(needle),
            "Toast view should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn toast_viewport_uses_logic_state_contracts() {
    let source = load_source("../../components/toast/src/toast/view.rs");

    for needle in [
        "logic::resolve_viewport_state(ToastViewportStateInput {",
        "slot: ToastViewportSlot::Root",
        "logic::compose_viewport_class_name(class_name, viewport_state)",
        "if let Some(provided_store) = store",
        "ToastStoreSource::Provided",
        "ToastStoreSource::Context",
        "ToastStoreSource::Local",
        "<Portal>",
        "data-ui-overlay-portal",
        "data-slot=move || viewport_state.get_value().slot_attr",
        "data-state=move || viewport_state.get_value().state_attr",
        "data-queue=move || viewport_state.get_value().queue_attr",
        "data-portal=move || viewport_state.get_value().portal_attr",
        "data-max-toasts=move || viewport_state.get_value().max_toasts.to_string()",
        "data-portal-source=move || viewport_state.get_value().portal_source_attr",
        "data-max-toasts-source=move || viewport_state.get_value().max_toasts_source_attr",
        "data-class-source=move || viewport_state.get_value().class_source_attr",
        "data-motion-source=move || viewport_state.get_value().motion_source_attr",
        "data-store-source=move || viewport_state.get_value().store_source_attr",
        "data-custom-portal=move || viewport_state.get_value().has_custom_portal.then_some(\"true\")",
        "data-custom-max-toasts=move || viewport_state.get_value().has_custom_max_toasts.then_some(\"true\")",
        "data-custom-class=move || viewport_state.get_value().has_custom_class_name.then_some(\"true\")",
        "data-custom-motion=move || viewport_state.get_value().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "ToastViewport should include `{needle}` for stable marker contracts."
        );
    }
}

#[test]
fn toast_has_baseline_style_accessibility_semantics() {
    let source = load_source("../../components/toast/src/toast/view.rs");

    for needle in [
        "live_region_attrs(LiveRegionPriority::Polite)",
        "live_region_attrs(LiveRegionPriority::Assertive)",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-atomic=\"true\"",
        "aria_label=close_aria_label",
        "if ev.key() == \"Escape\"",
    ] {
        assert!(
            source.contains(needle),
            "Toast should include `{needle}` for baseline-style accessibility semantics."
        );
    }
}

#[test]
fn toast_view_disallows_inner_html_injection_paths() {
    let source = load_source("../../components/toast/src/toast/view.rs");

    for needle in ["inner_html", "innerHTML"] {
        assert!(
            !source.contains(needle),
            "Toast view must not use `{needle}`; untrusted html injection paths are forbidden."
        );
    }

    assert!(
        source.contains("const TOAST_CLOSE_GLYPH: &str = \"×\";"),
        "Toast close glyph should remain a trusted compile-time constant."
    );
}

#[test]
fn toast_wasm_debug_capability_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let toast_mod_source = load_source("../../components/toast/src/toast/mod.rs");
    let toast_logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let toast_motion_source = load_source("../../components/toast/src/toast/motion.rs");
    let toast_styles_source = load_source("../../components/toast/src/toast/styles.rs");
    let toast_view_source = load_source("../../components/toast/src/toast/view.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let wasm_debug_script = load_source("../../scripts/check-ui-wasm-debug.sh");

    for needle in ["macro_rules! wasm_debug_proxy"] {
        assert!(
            crate_root_source.contains(needle),
            "ui should keep wasm debug capability isolated via `{needle}`."
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep explicit wasm-debug opt-in marker `{needle}`."
        );
    }

    assert!(
        !cargo_source.contains("toast-wasm-debug"),
        "Toast should not expose a dedicated wasm-debug feature because debug timeline/replay comes from global ui-trace overlay."
    );

    let toast_combined = format!(
        "{toast_mod_source}\n{toast_logic_source}\n{toast_motion_source}\n{toast_styles_source}\n{toast_view_source}"
    );
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !toast_combined.contains(forbidden),
            "Toast production contract should not leak wasm-debug internals `{forbidden}`."
        );
    }

    for marker in [
        "data-control-mode=control_mode_attr",
        "data-open-source=open_source_attr",
        "data-default-open-source=default_open_source_attr",
        "data-open-change-source=open_change_source_attr",
    ] {
        assert!(
            toast_view_source.contains(marker),
            "Toast should expose stable source markers for debug attribution via `{marker}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(\"toast\",",
        "request_open_change.run(false);",
        "if ev.key() == \"Escape\" {",
        "<CloseButton",
        "aria_label=close_aria_label",
        "on_press=close_toast",
    ] {
        assert!(
            toast_view_source.contains(needle),
            "Toast interaction chain should remain reproducible/replayable via `{needle}`."
        );
    }

    for needle in [
        "let debug_overlay_enabled = cfg!(debug_assertions);",
        "provide_ui_trace(debug_overlay_enabled);",
        "<Show when=move || debug_overlay_enabled>",
        "<debug_overlay::UiDebugOverlay enabled=true />",
    ] {
        assert!(
            docs_lib_source.contains(needle),
            "docs-app should keep wasm dev visual-entry gate `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`."
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "OpenChange {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamped/source event marker `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features inject-css,button-wasm-debug",
        "cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_wasm_debug_capability_stays_feature_isolated_and_non_polluting",
    ] {
        assert!(
            wasm_debug_script.contains(needle),
            "wasm-debug check script should keep feature-isolated verification marker `{needle}`."
        );
    }
}

#[test]
fn toast_does_not_define_spec_module_for_simple_component_scope() {
    let source = load_source("../../components/toast/src/toast/mod.rs");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let toast_spec = manifest_dir.join("../../components/toast/src/toast/spec.rs");

    for forbidden in ["mod spec;", "pub mod spec;", "use crate::toast::spec"] {
        assert!(
            !source.contains(forbidden),
            "Toast simple component scope should not introduce spec module wiring: `{forbidden}`."
        );
    }

    assert!(
        !toast_spec.exists(),
        "Toast should not define `../../components/toast/src/toast/spec.rs` without explicit schema-contract need."
    );
}

#[test]
fn toast_engineering_contract_is_spec_free_tracing_aligned_and_runtime_agnostic() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let toast_mod_source = load_source("../../components/toast/src/toast/mod.rs");
    let toast_logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let toast_motion_source = load_source("../../components/toast/src/toast/motion.rs");
    let toast_view_source = load_source("../../components/toast/src/toast/view.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    assert!(
        cargo_source.contains("component-toast = [\"component-close_button\", \"dep:ui-toast\"]"),
        "Toast feature should stay lightweight and avoid implicit engineering dependency fan-out."
    );
    for forbidden in [
        "component-toast = [\"dep:serde\"",
        "component-toast = [\"dep:serde_json\"",
        "component-toast = [\"dep:tracing\"",
        "component-toast = [\"dep:tokio\"",
        "component-toast = [\"dep:async-std\"",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "Toast feature should not pin serde/tracing/runtime deps directly: `{forbidden}`."
        );
    }

    assert!(
        !manifest_dir
            .join("../../components/toast/src/toast/spec.rs")
            .exists(),
        "Toast scope should keep spec/config serde migration path as N/A without local spec.rs."
    );
    for forbidden in ["mod spec;", "pub mod spec;", "use crate::toast::spec"] {
        assert!(
            !toast_mod_source.contains(forbidden),
            "Toast module boundary should stay spec-free for current scope: `{forbidden}`."
        );
    }

    let toast_combined = format!(
        "{toast_mod_source}\n{toast_logic_source}\n{toast_motion_source}\n{toast_view_source}"
    );
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "schema_version",
        "tokio::",
        "async_std::",
        "async-std::",
        "Runtime",
        "JoinHandle",
        "#[tokio::main]",
        "async fn ",
    ] {
        assert!(
            !toast_combined.contains(forbidden),
            "Toast implementation should not leak spec serialization/runtime details into component API: `{forbidden}`."
        );
    }

    for forbidden in [
        "pub use toast::{Toast, ToastViewport,",
        "tokio",
        "async_std",
        "serde",
    ] {
        assert!(
            !crate_root_source.contains(forbidden),
            "ui crate root should not leak runtime/spec details through toast public exports: `{forbidden}`."
        );
    }

    for needle in [
        "use_controllable_open_state_traced(\"toast\", controlled_open, default_open, on_open_change);",
        "pub enum UiTraceEventKind {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
    ] {
        assert!(
            toast_view_source.contains(needle) || trace_source.contains(needle),
            "Toast tracing should stay aligned with shared ui-headless trace semantics via `{needle}`."
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "info_span!(",
        "debug_span!(",
    ] {
        assert!(
            !toast_combined.contains(forbidden),
            "Toast should not introduce ad-hoc tracing vocabulary outside shared contracts: `{forbidden}`."
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toast_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for needle in [
        "#[cfg(feature = \"component-toast\")]\npub use ui_toast::toast;",
        "pub use root::UiRoot;",
        "pub use toast::{",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui lib entry should keep stable export/gate marker `{needle}`."
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
            "ui lib entry should not expose internal platform/details marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-toast\")]\n    out.push_str(crate::toast::styles::CSS);",
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

    for forbidden in ["#[component]", "pub fn Toast(", "ui-toast"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain generic shared utility, not component-business implementation: `{forbidden}`."
        );
    }

    assert!(
        manifest_dir
            .join("../ui-visual-primitive/src/active_highlight.rs")
            .exists(),
        "ui should keep shared `../ui-visual-primitive/src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
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
        "- [x] `ui` 固定入口文件落点正确。",
        "`crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn toast_component_directory_standard_files_follow_responsibility_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mod_path = manifest_dir.join("../../components/toast/src/toast/mod.rs");
    let logic_path = manifest_dir.join("../../components/toast/src/toast/logic.rs");
    let styles_path = manifest_dir.join("../../components/toast/src/toast/styles.rs");
    let view_path = manifest_dir.join("../../components/toast/src/toast/view.rs");
    let motion_path = manifest_dir.join("../../components/toast/src/toast/motion.rs");
    let spec_path = manifest_dir.join("../../components/toast/src/toast/spec.rs");
    let render_path = manifest_dir.join("../../components/toast/src/toast/render.rs");

    assert!(
        mod_path.exists(),
        "Toast should keep `../../components/toast/src/toast/mod.rs`."
    );
    assert!(
        logic_path.exists(),
        "Toast should keep `../../components/toast/src/toast/logic.rs`."
    );
    assert!(
        styles_path.exists(),
        "Toast should keep `../../components/toast/src/toast/styles.rs`."
    );
    assert!(
        view_path.exists(),
        "Toast should keep `../../components/toast/src/toast/view.rs`."
    );
    assert!(
        motion_path.exists(),
        "Toast should keep `../../components/toast/src/toast/motion.rs`."
    );
    assert!(
        !spec_path.exists(),
        "Toast should not define `../../components/toast/src/toast/spec.rs` for current simple component scope."
    );
    assert!(
        !render_path.exists(),
        "Toast should not drift to `../../components/toast/src/toast/render.rs`; rendering must stay in view.rs."
    );

    let mod_source = load_source("../../components/toast/src/toast/mod.rs");
    let logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let styles_source = load_source("../../components/toast/src/toast/styles.rs");
    let view_source = load_source("../../components/toast/src/toast/view.rs");
    let motion_source = load_source("../../components/toast/src/toast/motion.rs");
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod motion;",
        "pub mod styles;",
        "pub use logic::{",
        "pub use motion::ToastMotion;",
        "pub use view::{Toast, ToastViewport};",
    ] {
        assert!(
            mod_source.contains(needle),
            "Toast mod.rs should keep minimal/stable boundary marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "mod render;",
        "pub mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "Toast mod.rs should not leak implementation details via `{forbidden}`."
        );
    }

    for needle in [
        "use ui_state_primitives::toast as toast_state;",
        "pub fn resolve_state(input: ToastPartStateInput) -> ToastPartState",
        "pub fn resolve_viewport_state(input: ToastViewportStateInput) -> ToastViewportState",
        "pub fn resolve_open_state_config(",
    ] {
        assert!(
            logic_source.contains(needle),
            "Toast logic.rs should keep normalization/derivation marker `{needle}`."
        );
    }

    for forbidden in [
        "web_sys::",
        "HtmlElement",
        "NodeRef<html::",
        "view! {",
        "on:click=",
        "on:keydown=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "Toast logic.rs should avoid DOM/render/event mounting detail `{forbidden}`."
        );
    }

    for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "Toast styles.rs should keep token-first static style marker `{needle}`."
        );
    }

    for forbidden in [
        "fn resolve_state(",
        "use_controllable_open_state_traced(",
        "on:click=",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "Toast styles.rs should not contain logic/view behavior marker `{forbidden}`."
        );
    }

    for needle in [
        "#[component]",
        "pub fn Toast(",
        "pub fn ToastViewport(",
        "logic::resolve_state(ToastPartStateInput {",
        "logic::resolve_viewport_state(ToastViewportStateInput {",
        "use_controllable_open_state_traced(\"toast\",",
        "<CloseButton",
        "live_region_attrs(",
        "locale_attrs(",
    ] {
        assert!(
            view_source.contains(needle),
            "Toast view.rs should keep structure/headless mounting marker `{needle}`."
        );
    }

    for forbidden in [
        "pub struct ToastMotion",
        "impl Default for ToastMotion",
        "SpringAnimator::new(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "Toast view.rs should not own motion-engine implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "pub struct ToastMotion",
        "pub fn sanitize_motion(motion: ToastMotion) -> ToastMotion",
        "pub fn attach_motion(",
        "ui_motion::",
    ] {
        assert!(
            motion_source.contains(needle),
            "Toast motion.rs should keep motion contract/attach marker `{needle}`."
        );
    }

    for forbidden in ["#[component]", "pub fn Toast(", "pub fn ToastViewport("] {
        assert!(
            !motion_source.contains(forbidden),
            "Toast motion.rs should not include component rendering marker `{forbidden}`."
        );
    }

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
            "Toast checklist should keep component-file governance rule `{required}`."
        );
    }
}

#[test]
fn toast_agent_contract_schema_is_typed_traceable_and_whitelisted() {
    let logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let view_source = load_source("../../components/toast/src/toast/view.rs");
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for needle in [
        "pub enum ToastAgentIntent",
        "pub enum ToastAgentActionModel",
        "pub struct ToastAgentContract",
        "pub fn toast_agent_contract() -> ToastAgentContract",
        "pub fn toast_viewport_agent_contract() -> ToastAgentContract",
        "schema_attr: \"ui.toast.v1\"",
        "schema_attr: \"ui.toast.viewport.v1\"",
        "ToastAgentIntent::NotificationItem.as_attr()",
        "ToastAgentIntent::NotificationViewport.as_attr()",
        "ToastAgentActionModel::DismissClose.as_attr()",
        "ToastAgentActionModel::QueueDismissRemove.as_attr()",
        "state_axis_attr: \"state|variant|description|close-mode|open\"",
        "state_axis_attr: \"state|queue|portal|max-toasts\"",
        "source_axis_attr: \"id|description|class|motion|close|exit|open\"",
        "source_axis_attr: \"portal|max-toasts|class|motion|store\"",
        "let agent_contract = logic::toast_agent_contract();",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "let viewport_agent_contract = logic::toast_viewport_agent_contract();",
        "data-ui-schema=viewport_agent_contract.schema_attr",
        "data-ui-intent=viewport_agent_contract.intent_attr",
        "data-ui-action-model=viewport_agent_contract.action_model_attr",
        "data-ui-state-axis=viewport_agent_contract.state_axis_attr",
        "data-ui-source-axis=viewport_agent_contract.source_axis_attr",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "Toast agent contract should include typed schema marker `{needle}`."
        );
    }

    for forbidden in [
        "format!(\"ui.toast",
        "String::from(\"ui.toast",
        "let ui_schema =",
        "inner_html=",
        "set_inner_html(",
        "dangerously_set_inner_html",
        "<script",
        "eval(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "Toast agent contract render path should stay whitelisted and reject script injection token `{forbidden}`."
        );
    }

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep agent-contract governance rule `{required}`."
        );
    }
}

#[test]
fn toast_streaming_definition_is_llm_output_only_with_two_modes() {
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep two-mode LLM-only streaming definition marker `{required}`."
        );
    }
}

#[test]
fn toast_check2_documents_snapshot_as_default_baseline_capability() {
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep snapshot-baseline marker `{required}`."
        );
    }
}

#[test]
fn toast_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let view_source = load_source("../../components/toast/src/toast/view.rs");
    let logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let mod_source = load_source("../../components/toast/src/toast/mod.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}");

    for marker in [
        "pub fn Toast(",
        "pub fn ToastViewport(",
        "title: String,",
        "#[prop(optional, into)] description: Option<String>,",
        "let title = logic::normalize_title(title);",
        "let description = logic::normalize_description(description);",
        "description.unwrap_or_default(),",
        "if let Some(provided_store) = store",
        "ToastStoreSource::Provided",
        "ToastStoreSource::Context",
        "ToastStoreSource::Local",
    ] {
        assert!(
            view_source.contains(marker),
            "Toast snapshot-baseline render path should include `{marker}`."
        );
    }

    for marker in [
        "pub fn normalize_title(value: String) -> String",
        "pub fn normalize_description(value: Option<String>) -> Option<String>",
        "pub fn resolve_viewport_state(input: ToastViewportStateInput) -> ToastViewportState",
    ] {
        assert!(
            logic_source.contains(marker),
            "Toast logic should keep complete-result normalization marker `{marker}`."
        );
    }

    let toast_docs_start = docs_source
        .find("pub(super) fn toast() -> AnyView")
        .expect("toast docs section should exist");
    let toast_docs_end = docs_source
        .find("pub(super) fn toast_viewport() -> AnyView")
        .expect("toast_viewport docs section should exist after toast");
    let toast_docs = &docs_source[toast_docs_start..toast_docs_end];

    for marker in [
        "<Playground title=\"Basic Toast + Escape/Close\" code_signal=code_basic>",
        "<Playground title=\"State + Source Markers\" code_signal=code_danger>",
        "<Toast",
    ] {
        assert!(
            toast_docs.contains(marker),
            "Toast docs should keep snapshot-usable complete configuration marker `{marker}`."
        );
    }

    for marker in [
        "pub(super) fn toast_viewport() -> AnyView",
        "<ToastViewport />",
    ] {
        assert!(
            docs_source.contains(marker),
            "Toast docs should keep viewport snapshot baseline marker `{marker}`."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            !combined.contains(forbidden) && !toast_docs.contains(forbidden),
            "Toast snapshot-baseline scope should not mount streaming protocol token `{forbidden}`."
        );
    }
}

#[test]
fn toast_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "归类为 `Streaming Optional` 且当前实现为 `fallback=snapshot`",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn toast_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("../../components/toast/src/toast/view.rs");
    let logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

    for required in [
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "aria-atomic=\"true\"",
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action-model=agent_contract.action_model_attr",
        "data-state=move || state.get().state_attr",
        "data-control-mode=control_mode_attr",
        "data-open-source=open_source_attr",
        "data-ui-schema=viewport_agent_contract.schema_attr",
        "data-store-source=move || viewport_state.get_value().store_source_attr",
    ] {
        assert!(
            combined.contains(required),
            "Toast should keep continuous role/aria/data semantics via `{required}` in snapshot-only optional-streaming scope."
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-ui-output-status",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Toast should not mount fake streaming status field `{forbidden}` when stream protocol is N/A."
        );
    }
}

#[test]
fn toast_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("../../components/toast/src/toast/view.rs");
    let logic_source = load_source("../../components/toast/src/toast/logic.rs");
    let mod_source = load_source("../../components/toast/src/toast/mod.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}");
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for forbidden in [
        "retry",
        "on_retry",
        "backoff",
        "reconnect",
        "recovery_policy",
        "stream_validator",
    ] {
        assert!(
            !combined.contains(forbidden),
            "Toast component layer should keep streaming validation/retry/resilience policy outside component implementation: `{forbidden}`."
        );
    }

    assert!(
        checklist_source.contains("数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。"),
        "Toast checklist should keep boundary statement for streaming validation/retry/resilience responsibilities."
    );
}

#[test]
fn toast_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("../../components/toast/src/toast/check2.md");

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "Toast checklist should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn toast_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/toast/semantics.rs");

    for required in [
        "toast_view_uses_logic_state_contracts",
        "toast_viewport_uses_logic_state_contracts",
        "toast_has_baseline_style_accessibility_semantics",
        "toast_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "data-state=move || state.get().state_attr",
        "data-open-source=open_source_attr",
    ] {
        assert!(
            semantics_source.contains(required),
            "Toast semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["toMatch", "Snapshot"].concat(),
        ["image", "_snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "Toast semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn toast_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("../../components/toast/src/toast/view.rs");
    let semantics_source = load_source("tests/toast/semantics.rs");

    for marker in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-state=move || state.get().state_attr",
        "data-open-source=open_source_attr",
        "data-id-source=move || state.get().id_source_attr",
        "role=live_region.role",
        "aria-live=live_region.aria_live",
        "data-ui-schema=viewport_agent_contract.schema_attr",
        "data-store-source=move || viewport_state.get_value().store_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "Toast view should expose semantic marker `{marker}`."
        );
        assert!(
            semantics_source.contains(marker),
            "Toast semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn toast_styles_include_state_and_source_marker_contracts() {
    let source = load_source("../../components/toast/src/toast/styles.rs");

    for selector in [
        ".ui-toast[data-motion-source=\"custom\"]",
        ".ui-toast[data-custom-motion=\"true\"]",
        ".ui-toast[data-id-source=\"custom\"]",
        ".ui-toast[data-custom-id=\"true\"]",
        ".ui-toast[data-description-source=\"custom\"]",
        ".ui-toast[data-custom-description=\"true\"]",
        ".ui-toast[data-close-source=\"custom\"]",
        ".ui-toast[data-custom-close=\"true\"]",
        ".ui-toast[data-exit-source=\"custom\"]",
        ".ui-toast[data-custom-exit=\"true\"]",
        ".ui-toast[data-close-mode=\"noop\"] .ui-toast__close",
        ".ui-toast[data-variant=\"accent\"]",
        ".ui-toast[data-variant=\"danger\"]",
        ".ui-toast-viewport[data-motion-source=\"custom\"]",
        ".ui-toast-viewport[data-custom-motion=\"true\"]",
        ".ui-toast-viewport[data-store-source=\"provided\"]",
        ".ui-toast-viewport[data-store-source=\"context\"]",
        ".ui-toast-viewport[data-store-source=\"local\"]",
        ".ui-toast-viewport[data-state=\"inline\"]",
        ".ui-toast-viewport[data-queue=\"single\"]",
    ] {
        assert!(
            source.contains(selector),
            "Toast styles should include `{selector}` as stable state/source selectors."
        );
    }
}

#[test]
fn toast_styles_consume_ui_theme_tokens_for_overlay_layout() {
    let source = load_source("../../components/toast/src/toast/styles.rs");

    for needle in [
        "--ui-overlay-viewport-inset",
        "--ui-overlay-panel-min-width",
        "--ui-overlay-z-index",
        "--ui-space-lg",
        "--ui-space-2xs",
        "--ui-font-size-100",
    ] {
        assert!(
            source.contains(needle),
            "toast styles should consume ui-theme token `{needle}`."
        );
    }
}

#[test]
fn toast_docs_page_contains_state_source_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "pub(super) fn toast() -> AnyView",
        "title=\"Toast\"",
        "slug=\"toast\"",
        "State + Source Markers",
        "data-id-source",
        "data-description-source",
        "data-close-source",
        "data-exit-source",
        "data-motion-source",
        "<Toast",
    ] {
        assert!(
            source.contains(needle),
            "toast docs page should contain `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toast_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("../../components/toast/src/toast/motion.rs");
    let view_source = load_source("../../components/toast/src/toast/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: ToastMotion) -> ToastMotion",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig) -> ui_motion::spring::SpringConfig",
        "fn sanitize_number(value: f64, fallback: f64) -> f64",
        "fn sanitize_motion_falls_back_for_invalid_values()",
        "drop(sanitize_motion(motion));",
    ] {
        assert!(
            motion_source.contains(needle),
            "Toast motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    for needle in [
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "let motion = crate::toast::motion::sanitize_motion(motion);",
    ] {
        assert!(
            motion_source.contains(needle) || view_source.contains(needle),
            "Toast should include `{needle}` to sanitize motion at component and runtime boundaries.",
        );
    }
}

#[test]
fn toast_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "let danger_motion = ToastMotion {",
        "initial_y_px: 18.0",
        "initial_scale: 0.96",
        "title=\"State + Source Markers\"",
        "id=\"docs-toast-danger\".to_string()",
        "class_name=\"docs-toast-custom\".to_string()",
        "motion=danger_motion",
        "variant=ToastVariant::Danger",
        "Inspect data-id-source / data-description-source / data-close-source / data-exit-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "toast docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn toast_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }

    for needle in [
        "pub(super) fn toast() -> AnyView",
        "<Playground title=\"Basic Toast + Escape/Close\" code_signal=code_basic>",
        "<Playground title=\"State + Source Markers\" code_signal=code_danger>",
    ] {
        assert!(
            docs_source.contains(needle),
            "Toast docs should mount reusable Playground hot-reload path via `{needle}`."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn toast_dx_workbench_uses_interactive_playground_and_marks_persist_state_na() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

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

    let toast_docs_start = docs_source
        .find("pub(super) fn toast() -> AnyView")
        .expect("toast docs section should exist");
    let toast_docs_end = docs_source
        .find("pub(super) fn toast_viewport() -> AnyView")
        .expect("toast_viewport docs section should exist after toast");
    let toast_docs = &docs_source[toast_docs_start..toast_docs_end];

    for needle in [
        "let (open_default_raw, set_open_default_raw) = signal(true);",
        "let (open_danger_raw, set_open_danger_raw) = signal(true);",
        "Re-open basic toast",
        "Re-open danger toast",
        "\"open: \" {move || open_default_raw.get().to_string()}",
        "\"open: \" {move || open_danger_raw.get().to_string()}",
    ] {
        assert!(
            toast_docs.contains(needle),
            "Toast docs should keep interactive context-preserving playground marker `{needle}`."
        );
    }

    for forbidden in [
        "TOAST_WORKBENCH_STORAGE_KEY",
        "load_toast_workbench_state(",
        "save_toast_workbench_state(",
        "clear_toast_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !toast_docs.contains(forbidden),
            "Toast docs should keep optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent."
        );
    }
}

#[test]
fn toast_dx_check_script_keeps_shared_playground_contract_gate() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "[dx] contract: playground css hot-reload path",
        "cargo test -p ui --test button_semantics button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test action_button_semantics action_button_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test well_semantics --no-default-features --features component-well,inject-css well_dx_non_interactive_scope_keeps_isolated_canvas_and_marks_persist_state_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should keep shared playground contract gate `{needle}`."
        );
    }
}

#[test]
fn toast_docs_page_covers_primary_playgrounds() {
    toast_docs_page_contains_state_source_playground();
}

#[test]
fn toast_docs_playgrounds_lock_state_matrix_contract_values() {
    toast_docs_custom_motion_playground_locks_contract_values();
}
