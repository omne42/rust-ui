use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    if let Some(suffix) = rel_path.strip_prefix("src/tag/") {
        workspace_dir.join("components/tag/src").join(suffix)
    } else if rel_path == "src/lib.rs" {
        workspace_dir.join("crates/ui/src/lib.rs")
    } else if rel_path == "src/css.rs" {
        workspace_dir.join("crates/ui/src/css.rs")
    } else if rel_path == "Cargo.toml" {
        workspace_dir.join("crates/ui/Cargo.toml")
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-state-primitives/") {
        workspace_dir
            .join("crates/ui-state-primitives")
            .join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-headless/") {
        workspace_dir.join("crates/ui-headless").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../ui-motion/") {
        workspace_dir.join("crates/ui-motion").join(suffix)
    } else if let Some(suffix) = rel_path.strip_prefix("../../") {
        workspace_dir.join(suffix)
    } else {
        manifest_dir.join(rel_path)
    }
}

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
fn path_exists(rel_path: &str) -> bool {
    resolve_source_path(rel_path).is_some()
}
#[test]
fn tag_group_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/tag/group/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "TagGroup internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn tag_group_component_layers_follow_file_responsibility_contract() {
    let mod_source = load_source("src/tag/group/mod.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let styles_source = load_source("src/tag/group/styles.rs");
    let view_source = load_source("src/tag/group/view.rs");

    for forbidden in ["pub mod logic", "pub mod view"] {
        assert!(
            !mod_source.contains(forbidden),
            "TagGroup mod.rs should keep minimal exports and avoid `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "<div", "on:click", "web_sys::"] {
        assert!(
            !logic_source.contains(forbidden),
            "TagGroup logic.rs should stay pure normalization/derivation and avoid `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:click", "Callback::new(", "use "] {
        assert!(
            !styles_source.contains(forbidden),
            "TagGroup styles.rs should remain static CSS and avoid runtime token `{forbidden}`."
        );
    }

    for forbidden in [
        "disabled || tag.disabled",
        "has_remove_callback && !is_disabled",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TagGroup view.rs should not reconstruct item-state rules via `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_does_not_introduce_component_spec_module() {
    let spec_path = resolve_path("src/tag/group/spec.rs");
    assert!(
        !spec_path.exists(),
        "TagGroup is a simple component and should not introduce `src/tag/group/spec.rs`."
    );
}

#[test]
fn tag_group_uses_logic_state_model() {
    let view_source = load_source("src/tag/group/view.rs");
    let logic_source = load_source("src/tag/group/logic.rs");

    for needle in [
        "pub use ui_state_primitives::tag_group::{",
        "TagGroupState",
        "TagGroupItemState",
        "TagGroupItemStateInput",
        "resolve_state",
        "resolve_item_state",
        "merge_describedby_ids",
        "pub fn normalize_group_input(",
        "pub fn resolve_group_state(",
        "pub fn resolve_group_item_state(",
        "pub struct TagGroupRootStateInput",
        "pub struct TagGroupRenderableItemStateInput",
        "DEFAULT_ID_BASE",
        "DEFAULT_ARIA_LABEL",
        "pub enum TagGroupValueSource",
        "pub enum TagGroupPresenceSource",
    ] {
        assert!(
            logic_source.contains(needle),
            "TagGroup logic should include `{needle}` for primitive-backed state derivation."
        );
    }

    for needle in [
        "let state = Memo::new(move |_|",
        "resolve_group_state(",
        "TagGroupRootStateInput {",
        "has_remove_callback",
        "normalize_group_input(",
        "resolve_group_item_state(TagGroupRenderableItemStateInput",
        "ui_headless::{A11yDirection, OnPress, locale_attrs}",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-id-base-source=id_base_source",
        "data-aria-label-source=aria_label_source",
        "data-class-source=class_name_source",
        "data-lang-source=lang_source",
        "data-disabled-source=item_state.disabled_source_attr",
        "data-removable-source=item_state.removable_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should derive root/item state via logic wrappers; missing `{needle}`."
        );
    }

    for needle in [
        "unwrap_or_else(|| \"tag-group\".to_string())",
        "unwrap_or_else(|| \"Tags\".to_string())",
        "disabled || tag.disabled",
        "has_remove_callback && !is_disabled",
    ] {
        assert!(
            !view_source.contains(needle),
            "TagGroup view should not own default fallback logic; found `{needle}`."
        );
    }
}

#[test]
fn tag_group_supports_baseline_aria_contract() {
    let source = load_source("src/tag/group/view.rs");

    for attr in [
        "aria-labelledby",
        "aria-describedby",
        "aria-invalid",
        "aria-required",
        "lang=locale.lang.clone()",
        "dir=locale.dir",
        "data-slot=\"tag-group\"",
        "const TAG_GROUP_LIST_SLOT: &str = \"tag-group-list\";",
        "const TAG_GROUP_ITEM_SLOT: &str = \"tag-group-item\";",
        "const TAG_GROUP_DESCRIPTION_SLOT: &str = \"tag-group-description\";",
        "const TAG_GROUP_ERROR_SLOT: &str = \"tag-group-error\";",
        "data-slot=TAG_GROUP_LIST_SLOT",
        "data-slot=TAG_GROUP_ITEM_SLOT",
        "data-slot=TAG_GROUP_DESCRIPTION_SLOT",
        "data-slot=TAG_GROUP_ERROR_SLOT",
    ] {
        assert!(
            source.contains(attr),
            "TagGroup should provide `{attr}` for baseline-style semantics."
        );
    }
}

#[test]
fn tag_group_emits_root_and_item_state_attrs() {
    let source = load_source("src/tag/group/view.rs");

    for attr in [
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-has-items=move || state.get().has_items.then_some(\"true\")",
        "data-count=move || state.get().item_count.to_string()",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-has-disabled-tags=move || state.get().has_disabled_tags.then_some(\"true\")",
        "data-has-removable-tags=move || state.get().has_removable_tags.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-id-base-source=id_base_source",
        "data-aria-label-source=aria_label_source",
        "data-class-source=class_name_source",
        "data-lang-source=lang_source",
        "data-index=index",
        "data-tag-id=tag_id_for_attr",
        "data-disabled=item_state.is_disabled.then_some(\"true\")",
        "data-removable=item_state.is_removable.then_some(\"true\")",
        "data-disabled-source=item_state.disabled_source_attr",
        "data-removable-source=item_state.removable_source_attr",
    ] {
        assert!(
            source.contains(attr),
            "TagGroup should expose `{attr}` for baseline-style styling and regression checks."
        );
    }
}

#[test]
fn tag_group_merges_external_and_internal_describedby_ids() {
    let source = load_source("src/tag/group/view.rs");
    assert!(
        source.contains("merge_describedby_ids"),
        "TagGroup should merge external aria-describedby with description/error ids."
    );
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn tag_group_composes_tag_primitive_instead_of_chip() {
    let source = load_source("src/tag/group/view.rs");

    for needle in [
        "tag::{Tag as TagPrimitive, TagSize, TagVariant}",
        "<TagPrimitive",
        "variant=variant",
        "size=size",
        "is_removable=true",
        "on_remove=on_remove",
    ] {
        assert!(
            source.contains(needle),
            "TagGroup should compose Tag primitive contracts; missing `{needle}`."
        );
    }

    assert!(
        !source.contains("<Chip"),
        "TagGroup should no longer compose Chip directly after Tag alignment."
    );
}

#[test]
fn tag_group_styles_include_description_and_error_states() {
    let source = load_source("src/tag/group/styles.rs");

    for selector in [
        ".ui-tag-group__description",
        ".ui-tag-group__error",
        ".ui-tag-group[data-invalid=\"true\"]",
        ".ui-tag-group__item[data-disabled=\"true\"]",
        ".ui-tag-group__item[data-disabled-source=\"group\"]",
        ".ui-tag-group__item[data-removable-source=\"unsupported\"]",
        ".ui-tag-group[data-id-base-source=\"custom\"]",
        ".ui-tag-group[data-aria-label-source=\"custom\"]",
        ".ui-tag-group[data-class-source=\"custom\"]",
        ".ui-tag-group[data-lang-source=\"provided\"]",
    ] {
        assert!(
            source.contains(selector),
            "TagGroup styles should include `{selector}`"
        );
    }

    for token_var in [
        "var(--ui-space-",
        "var(--ui-fg",
        "var(--ui-danger",
        "var(--ui-radius-",
    ] {
        assert!(
            source.contains(token_var),
            "TagGroup styles should stay token-first and include `{token_var}`."
        );
    }

    for forbidden in [":nth-child", ":nth-of-type", ":first-child", ":last-child"] {
        assert!(
            !source.contains(forbidden),
            "TagGroup styles should not guess state with brittle selector `{forbidden}`."
        );
    }

    for forbidden in ["view! {", "on:click", "Callback::new(", "@apply "] {
        assert!(
            !source.contains(forbidden),
            "TagGroup styles should remain static CSS and avoid `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let styles_source = load_source("src/tag/group/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for needle in [
        "var(--ui-space-sm, var(--ui-fallback-space-sm))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-button-size-s-font-size, var(--ui-fallback-button-size-s-font-size))",
        "var(--ui-button-size-s-line-height, var(--ui-fallback-button-size-s-line-height))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-danger-fg, var(--ui-fallback-danger-fg))",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-danger, var(--ui-fallback-danger))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
    ] {
        assert!(
            styles_source.contains(needle),
            "TagGroup styles should keep defensive fallback-chain marker `{needle}`."
        );
    }

    for needle in [
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-button-size-s-font-size:",
        "--ui-fallback-button-size-s-line-height:",
        "--ui-fallback-font-size-100:",
        "--ui-fallback-line-height-100:",
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-danger-fg:",
        "--ui-fallback-border-width:",
        "--ui-fallback-danger:",
        "--ui-fallback-radius-sm:",
    ] {
        assert!(
            theme_css_source.contains(needle),
            "ui-theme fallback SSOT should include `{needle}`."
        );
    }

    for forbidden in [
        "var(--ui-button-size-s-font-size, 13px)",
        "var(--ui-button-size-s-line-height, 18px)",
        "var(--ui-font-size-100, 12px)",
        "var(--ui-line-height-100, 16px)",
        "outline: 1px solid var(--ui-danger)",
        "outline-offset: 4px",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "TagGroup styles should not keep bare fallback literal `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_cascade_layer_and_runtime_style_contract_is_enforced() {
    let checklist_source = load_source("src/tag/check2.md");
    let view_source = load_source("src/tag/group/view.rs");
    let ui_css_source = load_source("src/css.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "crates/ui/src/css.rs",
        "components/tag/src/view.rs` 与 `components/tag/src/group/view.rs` 均未使用普通 inline style",
        "tag_cascade_layer_and_runtime_style_contract_is_enforced",
        "tag_group_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            checklist_source.contains(needle),
            "tag/check2.md should keep cascade-layer contract token `{needle}`."
        );
    }

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-tag_group\")]",
        "out.push_str(crate::tag::group::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_css_source.contains(needle),
            "ui css aggregation should keep cascade-layer fragment `{needle}`."
        );
    }

    assert!(
        ui_root_source.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should keep injecting component css aggregated by @layer ui."
    );

    for forbidden in ["style=", "style:top", "style:left", "style:transform"] {
        assert!(
            !view_source.contains(forbidden),
            "tag/group/view.rs should avoid plain inline style token `{forbidden}`."
        );
    }

    for needle in [
        "tag-group css is aggregated in @layer ui and runtime style is css-variable-only",
        "tag_group_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_group_tree_shaking_feature_wiring_is_component_scoped() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");

    for needle in [
        "component-tag = [\"dep:ui-tag\"]",
        "component-tag_group = [\"component-tag\", \"ui-tag/tag-group\"]",
        "#[cfg(feature = \"component-tag\")]\npub use ui_tag as tag;",
        "#[cfg(feature = \"component-tag\")]\n    out.push_str(crate::tag::styles::CSS);",
        "#[cfg(feature = \"component-tag_group\")]\n    out.push_str(crate::tag::group::styles::CSS);",
    ] {
        assert!(
            cargo_source.contains(needle)
                || lib_source.contains(needle)
                || css_source.contains(needle),
            "TagGroup tree-shaking contract should include `{needle}`."
        );
    }
}

#[test]
fn tag_group_platform_guards_keep_non_wasm_paths_browser_object_free() {
    let mod_source = load_source("src/tag/group/mod.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let styles_source = load_source("src/tag/group/styles.rs");
    let view_source = load_source("src/tag/group/view.rs");

    for forbidden in ["web_sys", "js_sys", "wasm_bindgen", "leptos::web_sys"] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "non-wasm TagGroup files should stay browser-object free; found `{forbidden}`."
        );
    }

    for forbidden in ["target_arch = \"wasm32\"", "cfg(target_arch"] {
        assert!(
            !logic_source.contains(forbidden)
                && !styles_source.contains(forbidden)
                && !view_source.contains(forbidden),
            "TagGroup should keep platform-agnostic component files and avoid ad-hoc platform branch `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_platform_check_script_covers_native_ssr_and_wasm_compile_paths() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "cargo check -p ui --no-default-features --features component-tag_group,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-tag_group,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable",
        "components/tag/src/group/view.rs",
        "components/tag/src/group/logic.rs",
    ] {
        assert!(
            script_source.contains(needle),
            "platform check script should include `{needle}`."
        );
    }
}

#[test]
fn tag_group_ui_headless_feature_mutex_compile_error_guard_is_present() {
    let headless_source = load_source("../ui-headless/src/lib.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_source.contains(needle),
            "ui-headless should keep feature mutex compile guard `{needle}`."
        );
    }
}

#[test]
fn tag_group_platform_script_enforces_ui_headless_web_ssr_mutex() {
    let script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            script_source.contains(needle),
            "platform guard script should enforce ui-headless feature mutex via `{needle}`."
        );
    }
}

#[test]
fn tag_group_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion()
{
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");
    let non_wasm_stub_test = load_source("../../crates/ui-motion/tests/non_wasm_stub.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");
    let view_source = load_source("src/tag/group/view.rs");
    let logic_source = load_source("src/tag/group/logic.rs");

    for needle in [
        "Compile on non-wasm targets (no-op stubs) to support SSR/tooling builds.",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm no-op/stub contract `{needle}`."
        );
    }

    for needle in [
        "#![cfg(not(target_arch = \"wasm32\"))]",
        "fn non_wasm_web_backend_prefers_reduced_motion()",
        "fn non_wasm_web_backend_animate_is_safe_noop()",
    ] {
        assert!(
            non_wasm_stub_test.contains(needle),
            "ui-motion non-wasm stub test should include `{needle}`."
        );
    }

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "platform script should keep ui-motion compile/stub guard `{needle}`."
        );
    }

    for forbidden in ["ui_motion", "attach_motion", "motion::"] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "TagGroup has no motion contract and should avoid motion binding token `{forbidden}`."
        );
    }

    let motion_path = resolve_path("src/tag/group/motion.rs");
    assert!(
        !motion_path.exists(),
        "TagGroup currently has no component motion contract and should not require `src/tag/group/motion.rs`."
    );
}

#[test]
fn tag_group_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable() {
    let logic_source = load_source("src/tag/group/logic.rs");
    let view_source = load_source("src/tag/group/view.rs");
    let styles_source = load_source("src/tag/group/styles.rs");
    let platform_script_source = load_source("../../scripts/check-ui-platforms.sh");

    for needle in [
        "cargo check -p ui --no-default-features --features component-tag_group,inject-css",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-tag_group,inject-css",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            platform_script_source.contains(needle),
            "TagGroup platform compile-only coverage should include `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"tag-group\"",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-id-base-source=id_base_source",
        "data-disabled-source=item_state.disabled_source_attr",
        "data-removable-source=item_state.removable_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup should keep hydration-stable semantic marker `{needle}` across SSR/wasm paths."
        );
    }

    for forbidden in [
        "target_arch = \"wasm32\"",
        "cfg(target_arch",
        "prefers_reduced_motion",
        "ui_motion",
        "attach_motion",
        "animation:",
        "transition:",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !styles_source.contains(forbidden),
            "TagGroup has no motion contract; reduced-motion/platform split token `{forbidden}` should be absent."
        );
    }
}

#[test]
fn tag_group_machine_readable_marker_values_are_closed_sets() {
    let primitive_source = load_source("../../crates/ui-state-primitives/src/tag_group.rs");
    let logic_source = load_source("src/tag/group/logic.rs");

    for needle in [
        "pub enum TagGroupItemDisabledSource",
        "\"none\"",
        "\"group\"",
        "\"item\"",
        "\"group-and-item\"",
        "pub enum TagGroupItemRemovableSource",
        "\"removable\"",
        "\"disabled\"",
        "\"unsupported\"",
    ] {
        assert!(
            primitive_source.contains(needle),
            "TagGroup primitive should keep closed marker value token `{needle}`."
        );
    }

    for needle in [
        "pub enum TagGroupValueSource",
        "\"default\"",
        "\"custom\"",
        "pub enum TagGroupPresenceSource",
        "\"missing\"",
        "\"provided\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "TagGroup logic should keep closed source marker token `{needle}`."
        );
    }
}

#[test]
fn tag_group_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "pub(super) fn tag_group() -> AnyView",
        "title=\"TagGroup\"",
        "slug=\"tag-group\"",
        "description=\"Tag list with removable chips, validation semantics, and baseline-style root state attrs.\"",
        "title=\"Hello World\"",
        "title=\"Removable + State\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Empty\"",
        "code_signal=hello_world_code",
        "code_signal=code",
        "code_signal=states_code",
        "code_signal=disabled_empty_code",
        "test_source_path=\"components/tag/src/group/view.rs\".to_string()",
        "<TagGroup",
        "on_remove=on_remove_removable",
        "is_invalid=validation_invalid",
        "is_disabled=true",
    ] {
        assert!(
            source.contains(needle),
            "collections docs page should include `{needle}` for tag-group coverage.",
        );
    }
}

#[test]
fn tag_group_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");

    for needle in [
        "Tag::new(\"tag-rust\", \"Rust\")",
        "Tag::new(\"tag-leptos\", \"Leptos\")",
        "Tag::disabled(\"tag-a11y\", \"Accessibility\")",
        "on_remove=on_remove_removable",
        "label=\"Framework tags\".to_string()",
        "description=\"Remove any non-disabled tag\".to_string()",
        "\"count: \"",
        "\"has disabled tags: \"",
        "Tag::new(\"tag-required\", \"Required\")",
        "Tag::new(\"tag-baseline\", \"Baseline\")",
        "on_remove=on_remove_validation",
        "error=\"At least one tag is required\".to_string()",
        "is_required=validation_required",
        "\"invalid: \"",
        "tags=disabled_tags",
        "label=\"Disabled tags\".to_string()",
        "tags=empty_tags",
        "label=\"Empty tags\".to_string()",
        "\"disabled: true\"",
        "\"empty: true\"",
    ] {
        assert!(
            source.contains(needle),
            "tag-group docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn tag_group_performance_governance_budget_is_defined_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-performance.sh");
    let view_source = load_source("src/tag/group/view.rs");

    for needle in [
        "\"tag-group\" => UiPerfBudget {",
        "max_mount_ms: 34.0,",
        "max_update_ms: Some(12.0),",
        "max_heap_kb: Some(640.0),",
    ] {
        assert!(
            shell_source.contains(needle),
            "TagGroup docs page should keep performance budget contract token `{needle}`."
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

    for needle in ["\"TagGroup\",", "\"tag-group\",", "collections::tag_group"] {
        assert!(
            pages_source.contains(needle),
            "TagGroup docs page should remain in coverage traversal via `{needle}`."
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
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance governance should keep render_count follow-up marker `{needle}`."
        );
    }

    for needle in [
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-class-source=class_name_source",
        "data-disabled-source=item_state.disabled_source_attr",
        "data-removable-source=item_state.removable_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should expose attribution marker `{needle}` for perf triage."
        );
    }

    let script_needle = "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_performance_governance_budget_is_defined_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`."
    );
}

#[test]
fn tag_group_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/tag/group/view.rs");
    let script_source = load_source("../../scripts/check-ui-view-macro.sh");

    for needle in [
        "fn render_group_label(label: Option<String>, label_id: String) -> AnyView",
        "fn render_group_description(description: Option<String>, description_id: String) -> AnyView",
        "fn render_group_error(error: Option<String>, error_id: String, invalid: Signal<bool>) -> AnyView",
        "fn render_tag_node(",
        "fn render_tag_group_item(",
        "fn render_tag_group_items(",
        "let label_view = render_group_label(label.get_value(), label_id.clone());",
        "let description_view =",
        "render_group_description(description.get_value(), description_id.clone());",
        "let error_view = render_group_error(error.get_value(), error_id.clone(), invalid);",
        "render_tag_group_items(",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should keep macro complexity split marker `{needle}`."
        );
    }

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "TagGroup should keep a single public component boundary."
    );

    let script_needle = "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`."
    );
}

#[test]
fn tag_group_view_functional_split_prefers_plain_functions_over_local_components() {
    let view_source = load_source("src/tag/group/view.rs");

    for needle in [
        "fn render_group_label(label: Option<String>, label_id: String) -> AnyView",
        "fn render_group_description(description: Option<String>, description_id: String) -> AnyView",
        "fn render_group_error(error: Option<String>, error_id: String, invalid: Signal<bool>) -> AnyView",
        "fn render_tag_group_item(",
        "fn render_tag_group_items(",
        "pub fn TagGroup(",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should keep function-first split marker `{needle}`."
        );
    }

    for forbidden in [
        "#[component]\nfn render_group_label(",
        "#[component]\nfn render_group_description(",
        "#[component]\nfn render_group_error(",
        "#[component]\nfn render_tag_group_item(",
        "#[component]\nfn render_tag_group_items(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "TagGroup local fragments should stay plain functions, not extra components `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_static_fragments_are_constantized_with_stable_semantics() {
    let view_source = load_source("src/tag/group/view.rs");

    for needle in [
        "const TAG_GROUP_LABEL_CLASS: &str = \"ui-tag-group__label\";",
        "const TAG_GROUP_LABEL_SLOT: &str = \"tag-group-label\";",
        "const TAG_GROUP_LIST_CLASS: &str = \"ui-tag-group__list\";",
        "const TAG_GROUP_LIST_SLOT: &str = \"tag-group-list\";",
        "const TAG_GROUP_ITEM_CLASS: &str = \"ui-tag-group__item\";",
        "const TAG_GROUP_ITEM_SLOT: &str = \"tag-group-item\";",
        "const TAG_GROUP_DESCRIPTION_CLASS: &str = \"ui-tag-group__description\";",
        "const TAG_GROUP_DESCRIPTION_SLOT: &str = \"tag-group-description\";",
        "const TAG_GROUP_ERROR_CLASS: &str = \"ui-tag-group__error\";",
        "const TAG_GROUP_ERROR_SLOT: &str = \"tag-group-error\";",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should keep static fragment constants via `{needle}`."
        );
    }
}

#[test]
fn tag_group_inner_html_usage_is_forbidden_in_component_and_docs_examples() {
    for rel_path in [
        "src/tag/group/mod.rs",
        "src/tag/group/logic.rs",
        "src/tag/group/styles.rs",
        "src/tag/group/view.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
            assert!(
                !source.contains(forbidden),
                "TagGroup component source `{rel_path}` must not inject raw html; found `{forbidden}`."
            );
        }
    }

    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    for forbidden in ["inner_html", "set_inner_html", "dangerously_set_inner_html"] {
        assert!(
            !docs_source.contains(forbidden),
            "TagGroup docs examples must not demonstrate raw html injection token `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_inner_html_check_script_covers_security_contract() {
    let script_source = load_source("../../scripts/check-ui-inner-html.sh");

    let needle = "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_inner_html_usage_is_forbidden_in_component_and_docs_examples";
    assert!(
        script_source.contains(needle),
        "inner-html check script should enforce TagGroup contract marker `{needle}`."
    );
}

#[test]
fn tag_group_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../ui-headless/src/trace.rs");
    let tag_group_view_source = load_source("src/tag/group/view.rs");
    let tag_group_logic_source = load_source("src/tag/group/logic.rs");

    assert!(
        cargo_source.contains("button-wasm-debug = [\"component-button\", \"dep:tracing\"]"),
        "wasm debug capability should stay feature-gated via `button-wasm-debug`."
    );

    let all_components_start = cargo_source
        .find("all-components = [")
        .expect("all-components feature list should exist");
    let all_components_end = cargo_source[all_components_start..]
        .find("\n\ncomponent-accordion")
        .map(|offset| all_components_start + offset)
        .expect("all-components list should end before component feature declarations");
    let all_components_block = &cargo_source[all_components_start..all_components_end];
    assert!(
        !all_components_block.contains("button-wasm-debug"),
        "wasm debug feature must not be pulled into all-components production path."
    );

    for needle in [
        "data-debug-source=source.clone()",
        "data-debug-before=before_attr",
        "data-debug-after=after_attr",
        "data-debug-timestamp-ms=format!(\"{:.0}\", event.timestamp_ms)",
        "data-slot=\"button-debug-replay\"",
        "request_replay.run(event.source)",
    ] {
        assert!(
            button_view_source.contains(needle),
            "shared button wasm debug path should keep trace/replay marker `{needle}`."
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
            "docs debug visual entry should keep `{needle}`."
        );
    }

    for needle in [
        "events.push(event);",
        ".into_iter()",
        ".take(40)",
        "let ts_ms = event.ts_ms;",
        "UiTraceEventKind::Note",
        "UiTraceEventKind::Inspect",
    ] {
        assert!(
            trace_source.contains(needle) || debug_overlay_source.contains(needle),
            "global trace timeline/replay evidence should keep marker `{needle}`."
        );
    }

    for needle in [
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-class-source=class_name_source",
        "data-disabled-source=item_state.disabled_source_attr",
        "data-removable-source=item_state.removable_source_attr",
    ] {
        assert!(
            tag_group_view_source.contains(needle),
            "TagGroup should keep machine-readable state/source marker `{needle}` for debug attribution."
        );
    }

    for forbidden in [
        "button-wasm-debug",
        "wasm_debug",
        "render_debug_panel(",
        "data-debug-source",
        "request_replay.run(",
    ] {
        assert!(
            !tag_group_view_source.contains(forbidden)
                && !tag_group_logic_source.contains(forbidden),
            "TagGroup should not duplicate shared wasm debug runtime token `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_wasm_debug_check_script_covers_shared_contract() {
    let script_source = load_source("../../scripts/check-ui-wasm-debug.sh");

    let needle = "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_wasm_debug_contract_reuses_global_debug_trace_and_keeps_feature_isolated";
    assert!(
        script_source.contains(needle),
        "wasm debug check script should enforce `{needle}`."
    );
}

#[test]
fn tag_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/tag/group/mod.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let view_source = load_source("src/tag/group/view.rs");
    let styles_source = load_source("src/tag/group/styles.rs");
    let checklist_source = load_source("src/tag/check2.md");

    assert!(
        !path_exists("src/tag/group/spec.rs"),
        "TagGroup should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-tag_group = [\"component-tag\", \"ui-tag/tag-group\"]"),
        "TagGroup feature should stay lightweight without serde/spec dependency fan-out."
    );
    assert!(
        !cargo_source.contains("component-tag_group = [\"dep:serde\"")
            && !cargo_source.contains("component-tag_group = [\"dep:serde_json\""),
        "TagGroup should not opt into serde/spec migration dependencies without explicit schema contract."
    );

    let combined = format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}");
    for forbidden in [
        "serde::",
        "serde_json::",
        "Serialize",
        "Deserialize",
        "from_json(",
        "to_json_result(",
        "SchemaError",
        "spec::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "TagGroup engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
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
            "Tag checklist should keep engineering governance rule `{required}`."
        );
    }
}

#[test]
fn tag_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events() {
    let cargo_source = load_source("Cargo.toml");
    let button_view_source = load_source("src/button/view.rs");
    let combined = [
        load_source("src/tag/group/mod.rs"),
        load_source("src/tag/group/logic.rs"),
        load_source("src/tag/group/view.rs"),
        load_source("src/tag/group/styles.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`."
        );
    }

    assert!(
        !cargo_source.contains("tag-group-wasm-debug"),
        "TagGroup should not define component-local tracing feature when no local debug event/replay contract exists."
    );

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::tag_group::",
        "const TAG_GROUP_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "TagGroup should avoid ad-hoc tracing semantic drift token `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/tag/group/mod.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let view_source = load_source("src/tag/group/view.rs");
    let styles_source = load_source("src/tag/group/styles.rs");

    let sources = [&mod_source, &logic_source, &view_source, &styles_source];
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
                "TagGroup engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    assert!(
        !mod_source.contains("web_sys"),
        "TagGroup public module boundary should not leak web_sys types."
    );
}

#[test]
fn tag_group_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_group_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present() {
    for forbidden in ["src/overlay_open.rs", "src/presence.rs", "src/a11y.rs"] {
        assert!(
            !path_exists(forbidden),
            "ui forbidden entrypoint file should not exist: `{forbidden}`."
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`."
        );
    }

    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`."
        );
    }
}

#[test]
fn tag_group_entrypoints_check_script_covers_fixed_entrypoint_contract() {
    let script_source = load_source("../../scripts/check-ui-entrypoints.sh");

    let needle = "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_ui_components_forbidden_entrypoint_files_are_absent_and_headless_paths_are_present";
    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`."
    );
}

#[test]
fn tag_group_component_directory_has_standard_file_layout() {
    for required in [
        "src/tag/group/mod.rs",
        "src/tag/group/logic.rs",
        "src/tag/group/styles.rs",
        "src/tag/group/view.rs",
    ] {
        assert!(
            path_exists(required),
            "tag-group component directory should include required file `{required}`."
        );
    }

    assert!(
        !path_exists("src/tag/group/render.rs"),
        "tag-group component should not drift into `render.rs`; keep rendering in `view.rs`."
    );
    assert!(
        !path_exists("src/tag/group/motion.rs"),
        "TagGroup currently has no motion contract; keep `src/tag/group/motion.rs` absent until a reusable semantic motion mapping exists."
    );
    assert!(
        !path_exists("src/tag/group/spec.rs"),
        "TagGroup is a simple component and should not introduce `src/tag/group/spec.rs`."
    );
}

#[test]
fn tag_group_mod_rs_keeps_minimal_stable_exports() {
    let mod_source = load_source("src/tag/group/mod.rs");

    for needle in [
        "mod logic;",
        "mod view;",
        "pub mod styles;",
        "pub use view::TagGroup;",
        "pub use logic::Tag;",
    ] {
        assert!(
            mod_source.contains(needle),
            "tag/group/mod.rs should include stable export marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "web_sys",
        "leptos::web_sys",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "tag/group/mod.rs should not leak internal/platform marker `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_component_file_responsibilities_remain_scoped() {
    let logic_source = load_source("src/tag/group/logic.rs");
    let styles_source = load_source("src/tag/group/styles.rs");
    let view_source = load_source("src/tag/group/view.rs");

    for forbidden in [
        "view!",
        "on:click",
        "NodeRef<",
        "web_sys",
        "leptos::html",
        "role=",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "tag/group/logic.rs should stay normalization-only; found `{forbidden}`."
        );
    }

    for required in ["var(--ui-", "pub const CSS: &str ="] {
        assert!(
            styles_source.contains(required),
            "tag/group/styles.rs should keep token-first CSS marker `{required}`."
        );
    }

    for forbidden in ["#[component]", "use leptos::", "on:click=", "view!"] {
        assert!(
            !styles_source.contains(forbidden),
            "tag/group/styles.rs should stay static style contract; found `{forbidden}`."
        );
    }

    for required in [
        "#[component]",
        "pub fn TagGroup(",
        "normalize_group_input(",
        "resolve_group_state(",
        "resolve_group_item_state(TagGroupRenderableItemStateInput",
        "ui_headless::{A11yDirection, OnPress, locale_attrs}",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "tag/group/view.rs should keep rendering + semantics mount marker `{required}`."
        );
    }
}

#[test]
fn tag_group_component_files_check_script_covers_directory_contract() {
    let script_source = load_source("../../scripts/check-ui-component-files.sh");

    for needle in [
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_component_directory_has_standard_file_layout",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_mod_rs_keeps_minimal_stable_exports",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_component_file_responsibilities_remain_scoped",
    ] {
        assert!(
            script_source.contains(needle),
            "component-files check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_group_agent_contract_is_schema_typed_and_machine_readable() {
    let view_source = load_source("src/tag/group/view.rs");
    let logic_source = load_source("src/tag/group/logic.rs");

    for needle in [
        "pub enum TagGroupAgentSchemaVersion",
        "pub enum TagGroupAgentIntent",
        "pub enum TagGroupAgentAction",
        "pub enum TagGroupAgentStateAxis",
        "pub enum TagGroupAgentSource",
        "pub enum TagGroupAgentOutputStatus",
        "pub struct TagGroupAgentContract",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            logic_source.contains(needle),
            "TagGroup agent contract typing should include `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-capability-remove=move || {",
        "data-ui-capability-validate=move || {",
        "data-ui-capability-disable=move || {",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should mount schemaized agent contract field `{needle}`."
        );
    }
}

#[test]
fn tag_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/tag/group/view.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let styles_source = load_source("src/tag/group/styles.rs");
    let mod_source = load_source("src/tag/group/mod.rs");
    let combined = format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}");

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
            "TagGroup Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_snapshot_baseline_and_streaming_fallback_contract_are_explicit() {
    let view_source = load_source("src/tag/group/view.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let check2_source = load_source("src/tag/check2.md");

    for needle in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup view should expose explicit snapshot/fallback marker `{needle}`."
        );
    }

    for needle in [
        "TagGroupAgentStreamSupport::Unsupported",
        "TagGroupAgentStreamFallback::FullSnapshot",
    ] {
        assert!(
            logic_source.contains(needle),
            "TagGroup logic should model stream N/A/fallback contract via `{needle}`."
        );
    }

    for needle in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [ ] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
    ] {
        assert!(
            check2_source.contains(needle),
            "tag/check2.md should pin streaming baseline marker `{needle}`."
        );
    }
}

#[test]
fn tag_group_check2_documents_streaming_required_optional_classification_rules() {
    let checklist_source = load_source("src/tag/check2.md");

    for required in [
        "- [ ] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "Tag/TagGroup 归类为 `Streaming Optional`；当前实现使用 `data-ui-stream-support=\\\"unsupported\\\" + data-ui-stream-fallback=\\\"full-snapshot\\\"`，并输出 `data-ui-output-status`。",
    ] {
        assert!(
            checklist_source.contains(required),
            "tag/check2.md should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn tag_group_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let view_source = load_source("src/tag/group/view.rs");

    for required in [
        "role=\"group\"",
        "aria-label=move || {",
        "aria-labelledby=move || aria_labelledby.get_value()",
        "aria-describedby=move || group_aria_describedby.get()",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "aria-required=move || state.get().is_required.then_some(\"true\")",
        "data-slot=\"tag-group\"",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-ui-stream-mode=\"snapshot\"",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "TagGroup should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope."
        );
    }
}

#[test]
fn tag_group_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let view_source = load_source("src/tag/group/view.rs");
    let logic_source = load_source("src/tag/group/logic.rs");
    let combined = format!("{view_source}\n{logic_source}");

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
            "TagGroup should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_streaming_check_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_snapshot_baseline_and_streaming_fallback_contract_are_explicit",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_group_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_cascade_layer_and_runtime_style_contract_is_enforced",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_group_check2_marks_defensive_variables_contract_complete() {
    let checklist_source = load_source("src/tag/check2.md");

    for needle in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "components/tag/src/styles.rs` 与 `components/tag/src/group/styles.rs`",
        "tag_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
        "tag_group_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals",
    ] {
        assert!(
            checklist_source.contains(needle),
            "tag/check2.md should keep defensive-variable completion evidence `{needle}`."
        );
    }
}

#[test]
fn tag_group_check2_marks_cascade_layer_contract_complete() {
    let checklist_source = load_source("src/tag/check2.md");

    for needle in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "crates/ui/src/css.rs",
        "components/tag/src/view.rs` 与 `components/tag/src/group/view.rs` 均未使用普通 inline style",
        "tag_cascade_layer_and_runtime_style_contract_is_enforced",
        "tag_group_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            checklist_source.contains(needle),
            "tag/check2.md should keep cascade-layer completion evidence `{needle}`."
        );
    }
}

#[test]
fn tag_group_check2_marks_motion_contract_complete() {
    let checklist_source = load_source("src/tag/check2.md");

    for needle in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "N/A（`Tag/TagGroup` 当前无组件级 motion contract）",
        "tag_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion",
        "tag_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable",
        "tag_group_motion_contract_uses_ui_motion_non_wasm_stub_and_keeps_component_safe_without_motion",
        "tag_group_reduced_motion_ssr_wasm_contract_is_n_a_but_semantics_stay_platform_stable",
        "scripts/check-ui-platforms.sh` / `scripts/check-ui-layout-platforms.sh`",
    ] {
        assert!(
            checklist_source.contains(needle),
            "tag/check2.md should keep motion-contract completion evidence `{needle}`."
        );
    }
}

#[test]
fn tag_group_check2_documents_semantics_first_testing_rules() {
    let checklist_source = load_source("src/tag/check2.md");

    for required in [
        "- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            checklist_source.contains(required),
            "tag/check2.md should keep semantics-first testing rule `{required}`."
        );
    }
}

#[test]
fn tag_group_semantics_suite_is_contract_first_not_snapshot_only() {
    let semantics_source = load_source("tests/tag_group/semantics.rs");

    for required in [
        "tag_group_supports_baseline_aria_contract",
        "tag_group_emits_root_and_item_state_attrs",
        "tag_group_machine_readable_marker_values_are_closed_sets",
        "tag_group_agent_contract_is_schema_typed_and_machine_readable",
        "tag_group_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
    ] {
        assert!(
            semantics_source.contains(required),
            "TagGroup semantic test suite should assert contract marker `{required}`."
        );
    }

    let forbidden_terms = [
        ["assert", "_snapshot"].concat(),
        ["insta", "::"].concat(),
        ["image", "_snapshot"].concat(),
    ];

    for forbidden in forbidden_terms {
        assert!(
            !semantics_source.contains(forbidden.as_str()),
            "TagGroup semantic test suite should not rely on visual snapshot assertion `{forbidden}` as primary signal."
        );
    }
}

#[test]
fn tag_group_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let view_source = load_source("src/tag/group/view.rs");
    let semantics_source = load_source("tests/tag_group/semantics.rs");
    let semantics_source_normalized = semantics_source.replace("\\\"", "\"");

    for marker in [
        "role=\"group\"",
        "aria-label=move || {",
        "aria-labelledby=move || aria_labelledby.get_value()",
        "aria-describedby=move || group_aria_describedby.get()",
        "aria-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "aria-required=move || state.get().is_required.then_some(\"true\")",
        "data-empty=move || state.get().is_empty.then_some(\"true\")",
        "data-disabled=move || state.get().is_disabled.then_some(\"true\")",
        "data-invalid=move || state.get().is_invalid.then_some(\"true\")",
        "data-required=move || state.get().is_required.then_some(\"true\")",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(marker),
            "TagGroup view should keep semantic marker `{marker}`."
        );
        assert!(
            semantics_source_normalized.contains(marker),
            "TagGroup semantic marker `{marker}` changed without matching semantics assertion update."
        );
    }
}

#[test]
fn tag_group_check2_documents_e2e_selector_and_repeatable_flow_rules() {
    let check2_source = load_source("src/tag/check2.md");

    for needle in [
        "- [ ] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "- [ ] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
    ] {
        assert!(
            check2_source.contains(needle),
            "tag/check2.md should keep e2e selector/repeatable-flow rule `{needle}`."
        );
    }
}

#[test]
fn tag_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_tag_group_contract.spec.mjs");

    for needle in [
        "body:not(:has(#boot))",
        "[data-component=\"tag-group\"] [data-slot=\"tag-group\"]",
        "toHaveAttribute(\"role\", \"group\")",
        "toHaveAttribute(\"data-ui-schema\", \"ui.tag-group.agent-contract\")",
        "toHaveAttribute(\"data-ui-stream-support\", \"unsupported\")",
        "toHaveAttribute(\"data-ui-stream-fallback\", \"full-snapshot\")",
        "toHaveAttribute(\"data-ui-stream-mode\", \"snapshot\")",
        "toHaveAttribute(\"data-count\", /[0-9]+/)",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TagGroup e2e selector/stable-wait contract should include `{needle}`."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e_source.contains(forbidden),
            "TagGroup e2e should avoid unstable fixed-delay wait `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_tag_group_contract.spec.mjs");

    for needle in [
        "docs-app tag-group key flow is repeatable with semantic breakpoints",
        "page.keyboard.press(\"Enter\")",
        "toHaveAttribute(\"data-count\", \"2\")",
        "toHaveAttribute(\"data-ui-action\", \"remove-pointer\")",
        "toHaveAttribute(\"data-ui-source\", \"remove-pointer\")",
        "toHaveAttribute(\"data-ui-output-status\", \"submittable\")",
        "await page.reload();",
        "toHaveAttribute(\"data-count\", \"3\")",
        "toHaveAttribute(\"data-ui-action\", \"initialize\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TagGroup e2e repeatable-flow contract should include `{needle}`."
        );
    }

    for forbidden in ["toHaveScreenshot(", "toMatchSnapshot(", "waitForTimeout("] {
        assert!(
            !e2e_source.contains(forbidden),
            "TagGroup e2e key flow should avoid non-semantic/flaky token `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_e2e_check_script_covers_selector_and_key_flow_contracts() {
    let script_source = load_source("../../components/tag/scripts/check-ui-e2e-tag.sh");

    for needle in [
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
    ] {
        assert!(
            script_source.contains(needle),
            "tag-group e2e check script should enforce `{needle}`."
        );
    }
}

#[test]
fn tag_group_docs_examples_sync_with_logic_api_names_and_default_matrix() {
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let view_source = load_source("src/tag/group/view.rs");

    for needle in [
        "pub(super) fn tag_group() -> AnyView",
        "title=\"TagGroup\"",
        "slug=\"tag-group\"",
        "title=\"Hello World\"",
        "<TagGroupItem slot:item",
        "title=\"Removable + State\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Empty\"",
        "Tag::new(\"tag-rust\", \"Rust\")",
        "Tag::new(\"tag-leptos\", \"Leptos\")",
        "Tag::disabled(\"tag-a11y\", \"Accessibility\")",
        "on_remove=on_remove_removable",
        "is_invalid=validation_invalid",
        "is_required=validation_required",
    ] {
        assert!(
            docs_source.contains(needle),
            "TagGroup docs examples should keep matrix/API marker `{needle}`."
        );
    }

    for needle in [
        "#[slot]",
        "pub struct TagGroupItem {",
        "#[prop(optional)] item: Vec<TagGroupItem>,",
        "#[prop(optional, into)] item_specs: Option<Signal<Vec<TagGroupItemSpec>>>",
        "#[prop(optional, into)] tags: Option<Signal<Vec<Tag>>>",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] on_remove: Option<Callback<Tag>>",
        "#[prop(optional)] variant: TagVariant,",
        "#[prop(optional)] size: TagSize,",
        "#[prop(optional, into)] label: Option<String>,",
        "#[prop(optional, into)] description: Option<String>,",
        "#[prop(optional, into)] error: Option<String>,",
        "#[prop(optional, into)] is_invalid: Option<Signal<bool>>,",
        "#[prop(optional, into)] is_required: Option<Signal<bool>>,",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup public API should keep marker `{needle}` for docs/runtime sync."
        );
    }
}

#[test]
fn tag_group_docs_entry_exists_and_is_beginner_friendly_default_then_advanced() {
    let check2_source = load_source("src/tag/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let section_start = docs_source
        .find("pub(super) fn tag_group() -> AnyView")
        .expect("TagGroup docs section should exist");
    let tag_group_section = &docs_source[section_start..];

    for needle in [
        "- [ ] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep documentation-as-product marker `{needle}`."
        );
    }

    for needle in [
        "title=\"TagGroup\"",
        "slug=\"tag-group\"",
        "title=\"Hello World\"",
        "<TagGroupItem slot:item",
        "title=\"Removable + State\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Empty\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TagGroup docs entry should include beginner-to-advanced marker `{needle}`."
        );
    }

    let hello_pos = tag_group_section
        .find("title=\"Hello World\"")
        .expect("TagGroup docs should include Hello World playground");
    let removable_pos = tag_group_section
        .find("title=\"Removable + State\"")
        .expect("TagGroup docs should include removable playground");
    let validation_pos = tag_group_section
        .find("title=\"Validation + Required\"")
        .expect("TagGroup docs should include validation playground");
    let advanced_pos = tag_group_section
        .find("title=\"Disabled + Empty\"")
        .expect("TagGroup docs should include advanced playground");
    assert!(
        hello_pos < removable_pos
            && removable_pos < validation_pos
            && validation_pos < advanced_pos,
        "TagGroup docs should keep default path before advanced controls."
    );
}

#[test]
fn tag_group_docs_app_provides_interactive_playground_with_live_props_and_state_preview() {
    let check2_source = load_source("src/tag/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "- [ ] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep interactive-playground marker `{needle}`."
        );
    }

    for needle in [
        "let (removable_tags, set_removable_tags) = signal",
        "let on_remove_removable = Callback::new",
        "let removable_count = Signal::derive",
        "let validation_invalid = Signal::derive",
        "title=\"Hello World\"",
        "title=\"Removable + State\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Empty\"",
        "count:",
        "invalid:",
    ] {
        assert!(
            docs_source.contains(needle),
            "TagGroup docs interactive playground should include `{needle}`."
        );
    }

    for needle in [
        "pub fn Playground(",
        "#[prop(optional, into)] code_signal: Option<Signal<String>>",
        "children: Children,",
        "let resolved_code = Signal::derive(move || {",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground runtime should keep live-preview marker `{needle}`."
        );
    }
}

#[test]
fn tag_group_dx_playground_supports_css_hot_reload_without_wasm_rebuild() {
    let source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`."
        );
    }
}

#[test]
fn tag_group_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections/tag_group.rs");

    for needle in [
        "title=\"Workbench (All API + Actual Config)\"",
        "test_config_signal=workbench_actual_config",
        "data-slot=\"tag-group-workbench-controls\"",
        "data-slot=\"tag-group-workbench-display\"",
        "data-slot=\"tag-group-workbench-feedback\"",
        "let (workbench_remove_count, set_workbench_remove_count) = signal(0_u32);",
        "let (workbench_last_removed, set_workbench_last_removed) = signal(None::<String>);",
        "\"on_remove count: \" {move || workbench_remove_count.get()}",
        "\" · last removed: \"",
        "\" · remaining: \" {move || workbench_tags.get().len()}",
    ] {
        assert!(
            source.contains(needle),
            "TagGroup workbench should keep DX context marker `{needle}`."
        );
    }

    for forbidden in [
        "TAG_GROUP_WORKBENCH_STORAGE_KEY",
        "local_storage",
        "Persist workbench state",
    ] {
        assert!(
            !source.contains(forbidden),
            "TagGroup workbench should keep optional persist-state as explicit N/A; found `{forbidden}`."
        );
    }
}

#[test]
fn tag_group_dx_check_script_covers_hot_reload_and_workbench_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui --test tag_group_semantics --no-default-features --features component-tag_group,inject-css tag_group_dx_workbench_keeps_context_and_isolated_canvas_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should enforce `{needle}`."
        );
    }
}

fn tag_group_docs_source_first_copy_paste_ready_with_imports_source_paths_and_sync() {
    let check2_source = load_source("src/tag/check2.md");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let e2e_source = load_source("../../e2e/tests/docs_app_tag_group_contract.spec.mjs");

    for needle in [
        "- [ ] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep source-first copy-ready marker `{needle}`."
        );
    }

    for needle in [
        "test_source_path=\"components/tag/src/group/view.rs\".to_string()",
        "title=\"Hello World\"",
        "title=\"Removable + State\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Empty\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TagGroup docs should keep source-first marker `{needle}`."
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground copy pipeline should keep marker `{needle}`."
        );
    }

    for needle in [
        "docs-app tag-group playground source is copy-paste ready",
        "toHaveAttribute(\"data-copyable\", \"true\")",
        "toHaveAttribute(\"aria-label\", /Copy to clipboard/i)",
        "toContainText(\"use leptos::prelude::*;\")",
        "toContainText(\"use ui::*;\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "TagGroup copy-flow e2e should keep marker `{needle}`."
        );
    }
}

#[test]
fn tag_group_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes() {
    let check2_source = load_source("src/tag/check2.md");
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source = load_source("../../apps/docs-app/src/pages/components/pages/collections.rs");
    let view_source = load_source("src/tag/group/view.rs");

    for needle in [
        "- [ ] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
    ] {
        assert!(
            check2_source.contains(needle),
            "Tag checklist should keep HeroUI/docs-sync marker `{needle}`."
        );
    }

    for needle in [
        "### TagGroup 同步记录（2026-02-17）",
        "`TagGroup` 维持 collection primitive 定位",
        "component_doc!(\"TagGroup\", \"tag-group\", \"Collections\", collections::tag_group)",
        "`#/components/tag-group` 可索引访问",
        "`Hello World`、`Removable + State`、`Validation + Required`、`Disabled + Empty`",
    ] {
        assert!(
            strategy_source.contains(needle),
            "HeroUI strategy should keep TagGroup sync marker `{needle}`."
        );
    }

    for needle in ["\"TagGroup\"", "\"tag-group\"", "collections::tag_group"] {
        assert!(
            pages_source.contains(needle),
            "docs catalog should expose TagGroup token `{needle}`."
        );
    }

    for needle in [
        "title=\"TagGroup\"",
        "slug=\"tag-group\"",
        "title=\"Hello World\"",
        "title=\"Removable + State\"",
        "title=\"Validation + Required\"",
        "title=\"Disabled + Empty\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "TagGroup docs page should keep indexed/example marker `{needle}`."
        );
    }

    for needle in [
        "#[slot]",
        "#[prop(optional)] item: Vec<TagGroupItem>,",
        "#[prop(optional, into)] item_specs: Option<Signal<Vec<TagGroupItemSpec>>>",
        "#[prop(optional, into)] tags: Option<Signal<Vec<Tag>>>",
        "#[prop(optional)] is_disabled: Option<bool>,",
        "#[prop(optional)] on_remove: Option<Callback<Tag>>",
        "#[prop(optional)] variant: TagVariant,",
        "#[prop(optional)] size: TagSize,",
        "#[prop(optional, into)] label: Option<String>,",
        "#[prop(optional, into)] description: Option<String>,",
        "#[prop(optional, into)] error: Option<String>,",
    ] {
        assert!(
            view_source.contains(needle),
            "TagGroup API should keep marker `{needle}` for docs/runtime sync."
        );
    }
}
