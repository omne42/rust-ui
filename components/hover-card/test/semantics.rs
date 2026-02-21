use std::fs;
use std::path::{Path, PathBuf};

fn component_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn load_source(rel_path: &str) -> String {
    let path = component_root().join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path:?}: {err}"))
}

fn path_exists(rel_path: &str) -> bool {
    component_root().join(rel_path).exists()
}

#[test]
fn component_contract_files_exist() {
    let root = component_root();
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "test/logic.rs",
        "test/motion.rs",
        "test/protocol.rs",
        "test/semantics.rs",
    ] {
        assert!(
            root.join(rel_path).exists(),
            "required hover-card contract file should exist: {rel_path}"
        );
    }
}

#[test]
fn spec_rs_is_not_introduced_for_simple_hover_card_component() {
    let root = component_root();
    let mod_source = load_source("src/mod.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    assert!(
        !root.join("src/spec.rs").exists(),
        "hover-card should not introduce `spec.rs` without stable schema/config hard requirement",
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "hover-card module should not expose spec layer marker `{forbidden}`",
        );
    }

    for forbidden in ["HoverCardSpec", "spec::", ".render()"] {
        assert!(
            !docs_source.contains(forbidden),
            "hover-card docs should not force spec-builder usage via `{forbidden}`",
        );
    }
}

#[test]
fn tree_shaking_contract_is_feature_gated_through_ui_components_package_mode() {
    let cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let web_demo_cargo = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "component-hover_card = [\"dep:ui-hover-card\"]",
        "ui-hover-card = { path = \"../../components/hover-card\", optional = true }",
        "default-features = false, features = [\"inject-css\", \"web-demo-components\"]",
    ] {
        assert!(
            cargo_source.contains(needle) || web_demo_cargo.contains(needle),
            "tree-shaking contract should include feature/dependency marker `{needle}`",
        );
    }

    let lib_lines: Vec<&str> = lib_source.lines().collect();
    assert!(
        lib_lines.windows(2).any(|window| {
            window[0].trim() == "#[cfg(feature = \"component-hover_card\")]"
                && window[1].trim() == "pub use ui_hover_card as hover_card;"
        }),
        "hover-card export should stay gated by `component-hover_card` in lib.rs",
    );
    assert!(
        lib_lines.windows(2).any(|window| {
            window[0].trim() == "#[cfg(feature = \"all-components\")]"
                && window[1].trim() == "pub use all_components::*;"
        }),
        "`all_components` re-export should stay cfg-gated and never become unconditional",
    );

    let css_lines: Vec<&str> = css_source.lines().collect();
    assert!(
        css_lines.windows(2).any(|window| {
            window[0].trim() == "#[cfg(feature = \"component-hover_card\")]"
                && window[1].trim() == "out.push_str(crate::hover_card::styles::CSS);"
        }),
        "hover-card css aggregation should stay gated by `component-hover_card`",
    );
}

#[test]
fn hover_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget_locally() {
    let script_source = load_source("../../scripts/check-ui-components-tree-shaking.sh");
    let budget_source = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "HOVER_CARD_MIN_FEATURES=\"component-hover_card,inject-css\"",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "HOVER_CARD_TREE_OUTPUT",
        "if grep -q 'all-components' <<<\"$HOVER_CARD_TREE_OUTPUT\";",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features",
        "cargo tree -e features -i ui-components -p web-demo",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features",
        "cargo build -p ui-components --target wasm32-unknown-unknown --release --no-default-features --features",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\";",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "if ! grep -q 'web-demo-components' <<<\"$WEB_DEMO_TREE_OUTPUT\";",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            script_source.contains(needle),
            "tree-shaking check script should include `{needle}`",
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES=",
        "TREE_SHAKING_MAX_RATIO_PERCENT=",
    ] {
        assert!(
            budget_source.contains(needle),
            "tree-shaking budget file should define `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_tree_shaking_feature_pruning_contract_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "hover-card check2 should mark tree-shaking first-class ability item complete",
    );
    assert!(
        source.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "hover-card check2 should mark tree-shaking feature-pruning checklist item complete",
    );

    for needle in [
        "tree_shaking_contract_is_feature_gated_through_ui_components_package_mode",
        "hover_card_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "hover_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget_locally",
        "hover_card_check2_marks_tree_shaking_feature_pruning_contract_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_tree_shaking_contract_stays_feature_gated_in_package_and_demo_modes",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-hover_card,inject-css",
        "cargo tree -e features -i ui-components -p web-demo",
        "scripts/check-ui-components-tree-shaking.sh",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 tree-shaking section should reference `{needle}`",
        );
    }
}

#[test]
fn file_responsibilities_are_enforced_across_component_modules() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::HoverCardMotion;",
        "pub use view::HoverCard;",
    ] {
        assert!(
            mod_source.contains(needle),
            "hover-card mod.rs should keep minimal export boundary marker `{needle}`",
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod motion;",
        "#[component]",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "hover-card mod.rs should not carry implementation detail `{forbidden}`",
        );
    }

    for needle in [
        "pub fn normalize_delay_state(input: DelayStateInput) -> DelayState {",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState {",
        "pub fn normalize_part_states(input: PartStatesInput) -> PartStates {",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card logic.rs should expose normalization marker `{needle}`",
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef<",
        "set_property(",
        "use_hover_card_trigger",
        "use_popover_position",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "hover-card logic.rs should not contain view/headless/dom detail `{forbidden}`",
        );
    }

    for needle in ["pub const CSS: &str", ".ui-hover-card", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "hover-card styles.rs should keep static token-first css marker `{needle}`",
        );
    }

    for forbidden in [
        "#[component]",
        "use_hover_card_trigger",
        "Signal<",
        "on:pointerenter",
        "on:keydown",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "hover-card styles.rs should not contain runtime logic marker `{forbidden}`",
        );
    }

    for needle in [
        "#[component]",
        "view! {",
        "use_hover_card_trigger",
        "use_popover_position",
        "motion::attach_motion(",
        "logic::normalize_part_states(logic::PartStatesInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view.rs should keep render/headless assembly marker `{needle}`",
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator::new(",
        "pub const CSS: &str",
        "hover_card_state::resolve_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view.rs should not inline engine/state-primitive internals `{forbidden}`",
        );
    }

    for needle in [
        "pub struct HoverCardMotion",
        "pub fn sanitize_motion(motion: HoverCardMotion) -> HoverCardMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "hover-card motion.rs should keep motion contract marker `{needle}`",
        );
    }

    for forbidden in ["view! {", "use_hover_card_trigger", "data-slot="] {
        assert!(
            !motion_source.contains(forbidden),
            "hover-card motion.rs should not contain render/semantic mounting detail `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_component_file_responsibilities_remain_scoped_locally() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "hover-card component directory should include required file `{required}`"
        );
    }

    for forbidden in ["src/render.rs", "src/spec.rs"] {
        assert!(
            !path_exists(forbidden),
            "hover-card simple component should not introduce `{forbidden}`"
        );
    }

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::HoverCardMotion;",
        "pub use view::HoverCard;",
    ] {
        assert!(
            mod_source.contains(needle),
            "hover-card mod.rs should keep minimal stable export marker `{needle}`"
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod view;",
        "pub mod motion;",
        "mod spec;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "hover-card mod.rs should not leak implementation marker `{forbidden}`"
        );
    }

    for needle in [
        "pub fn normalize_delay_state(input: DelayStateInput) -> DelayState {",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState {",
        "pub fn normalize_part_states(input: PartStatesInput) -> PartStates {",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card logic.rs should keep normalization marker `{needle}`"
        );
    }

    for forbidden in [
        "view! {",
        "NodeRef<",
        "set_property(",
        "use_hover_card_trigger",
        "use_popover_position",
        "ui_state_primitives::controlled::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "hover-card logic.rs should avoid non-logic concern `{forbidden}`"
        );
    }

    for needle in ["pub const CSS: &str", ".ui-hover-card", "var(--ui-"] {
        assert!(
            styles_source.contains(needle),
            "hover-card styles.rs should keep static token-first marker `{needle}`"
        );
    }

    for forbidden in [
        "#[component]",
        "use_hover_card_trigger",
        "Signal<",
        "on:pointerenter",
        "on:keydown",
        "background: #",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "hover-card styles.rs should avoid runtime/business marker `{forbidden}`"
        );
    }

    for needle in [
        "#[component]",
        "view! {",
        "use_hover_card_trigger",
        "use_popover_position",
        "motion::attach_motion(",
        "logic::normalize_part_states(logic::PartStatesInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view.rs should keep render/headless assembly marker `{needle}`"
        );
    }

    for forbidden in [
        "ui_motion::spring::SpringAnimator::new(",
        "pub const CSS: &str",
        "hover_card_state::resolve_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view.rs should avoid engine/state primitive internals `{forbidden}`"
        );
    }

    for needle in [
        "pub struct HoverCardMotion",
        "pub fn sanitize_motion(motion: HoverCardMotion) -> HoverCardMotion",
        "pub fn attach_motion(",
        "ui_motion::spring::SpringAnimator::new(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "hover-card motion.rs should keep motion contract marker `{needle}`"
        );
    }

    for forbidden in ["view! {", "use_hover_card_trigger", "data-slot="] {
        assert!(
            !motion_source.contains(forbidden),
            "hover-card motion.rs should avoid render/semantic mounting marker `{forbidden}`"
        );
    }
}

#[test]
fn hover_card_component_files_check_script_covers_scoped_responsibility_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_component_file_responsibilities_remain_scoped";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`"
    );
}

#[test]
fn hover_card_check2_marks_component_file_responsibility_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 组件目录标准文件落点正确。"),
        "hover-card check2 should mark component file-responsibility gate complete."
    );

    for required in [
        "`<component>/mod.rs`：最小稳定导出面，存在且无过度导出。",
        "`<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。",
        "`<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。",
        "`<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。",
        "`<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。",
        "`<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。",
    ] {
        assert!(
            source.contains(required),
            "hover-card check2 component-file section should retain rule `{required}`"
        );
    }

    for needle in [
        "components/hover-card/src/mod.rs",
        "components/hover-card/src/logic.rs",
        "components/hover-card/src/styles.rs",
        "components/hover-card/src/view.rs",
        "components/hover-card/src/motion.rs",
        "components/hover-card/src/render.rs`（不存在）",
        "components/hover-card/src/spec.rs`（不存在）",
        "components/hover-card/test/semantics.rs::hover_card_component_file_responsibilities_remain_scoped_locally",
        "components/hover-card/test/semantics.rs::hover_card_component_files_check_script_covers_scoped_responsibility_contract_locally",
        "components/hover-card/test/semantics.rs::hover_card_check2_marks_component_file_responsibility_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_component_file_responsibilities_remain_scoped",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_component_files_check_script_covers_scoped_responsibility_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_component_file_responsibility_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_component_file_responsibilities_remain_scoped",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 component-file section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_file_placement_discipline_is_strict_for_component_scope_locally() {
    for required in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        assert!(
            path_exists(required),
            "hover-card file placement should include required file `{required}`"
        );
    }

    {
        let forbidden = "src/render.rs";
        assert!(
            !path_exists(forbidden),
            "hover-card file placement should forbid `{forbidden}`"
        );
    }

    assert!(
        !path_exists("src/spec.rs"),
        "hover-card is a simple component; `src/spec.rs` should remain absent in this scope",
    );

    assert!(
        path_exists("src/protocol.rs"),
        "hover-card keeps `src/protocol.rs` as sidecar contract file and should not regress",
    );
}

#[test]
fn hover_card_component_files_script_covers_file_placement_discipline_locally() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_file_placement_discipline_is_strict_for_component_scope";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`"
    );
}

#[test]
fn hover_card_check2_marks_file_placement_discipline_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "hover-card check2 should mark file-placement discipline gate complete."
    );

    for needle in [
        "components/hover-card/src/mod.rs",
        "components/hover-card/src/logic.rs",
        "components/hover-card/src/styles.rs",
        "components/hover-card/src/view.rs",
        "components/hover-card/src/motion.rs",
        "components/hover-card/src/render.rs`（不存在）",
        "components/hover-card/src/spec.rs`（不存在）",
        "components/hover-card/src/protocol.rs`（sidecar 保留）",
        "components/hover-card/test/semantics.rs::hover_card_file_placement_discipline_is_strict_for_component_scope_locally",
        "components/hover-card/test/semantics.rs::hover_card_component_files_script_covers_file_placement_discipline_locally",
        "components/hover-card/test/semantics.rs::hover_card_check2_marks_file_placement_discipline_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_file_placement_discipline_is_strict_for_component_scope",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_component_files_script_covers_file_placement_discipline",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_file_placement_discipline_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_file_placement_discipline_is_strict_for_component_scope",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 file-placement section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component_locally() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    assert!(
        !path_exists("src/spec.rs"),
        "hover-card is a simple component; `src/spec.rs` should remain absent for Hyper-Structure Builder gate",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "pub use spec::",
        "HoverCardSpec",
        "spec::",
        "Spec::new(",
        ".render()",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "hover-card simple scope should not expose Hyper-Structure Builder marker `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_component_files_script_covers_hyper_structure_builder_na_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`"
    );
}

#[test]
fn hover_card_check2_marks_hyper_structure_builder_contract_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "hover-card check2 should mark hyper-structure-builder gate complete."
    );

    for needle in [
        "N/A：`hover-card` 为简单组件",
        "components/hover-card/src/spec.rs",
        "HoverCardSpec",
        "spec::",
        ".render()",
        "components/hover-card/test/semantics.rs::hover_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component_locally",
        "components/hover-card/test/semantics.rs::hover_card_component_files_script_covers_hyper_structure_builder_na_contract_locally",
        "components/hover-card/test/semantics.rs::hover_card_check2_marks_hyper_structure_builder_contract_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_component_files_script_covers_hyper_structure_builder_na_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_hyper_structure_builder_contract_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_hyper_structure_builder_spec_is_not_applicable_for_simple_component",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 hyper-structure-builder section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_context_compression_manifest_and_rbi_projection_are_present_and_current_locally() {
    for required_file in ["src/Component.toml", "src/hover_card.rbi"] {
        assert!(
            path_exists(required_file),
            "hover-card context-compression artifact should exist: `{required_file}`"
        );
    }

    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/hover_card.rbi");
    let view_source = load_source("src/view.rs");

    for needle in [
        "schema_version = \"1\"",
        "name = \"HoverCard\"",
        "crate = \"ui-hover-card\"",
        "rbi = \"hover_card.rbi\"",
        "name = \"content\"",
        "name = \"children\"",
        "name = \"is_disabled\"",
        "name = \"disabled\"",
        "name = \"placement\"",
        "name = \"is_open\"",
        "name = \"open\"",
        "name = \"default_open\"",
        "name = \"on_open_change\"",
        "name = \"open_delay_ms\"",
        "name = \"close_delay_ms\"",
        "name = \"motion\"",
        "name = \"class_name\"",
        "name = \"id\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "hover-card Component.toml should include context-compression marker `{needle}`"
        );
    }

    for needle in [
        "pub use crate::motion::HoverCardMotion;",
        "pub use ui_headless::PopoverPlacement;",
        "pub use ui_state_primitives::hover_card::{HoverCardPartState, HoverCardPartStateInput, HoverCardSlot};",
        "pub const DEFAULT_OPEN_DELAY_MS: u64;",
        "pub const DEFAULT_CLOSE_DELAY_MS: u64;",
        "pub struct HoverCardMotion {",
        "pub enum HoverCardComponentSchemaVersion {",
        "pub struct HoverCardComponentSpec {",
        "pub fn HoverCard(",
        "content: leptos::children::ViewFn",
        "children: leptos::children::Children",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "open_delay_ms: Option<u64>",
        "close_delay_ms: Option<u64>",
        "motion: crate::HoverCardMotion",
        "class_name: Option<String>",
        "id: Option<String>",
        "lang: Option<String>",
        "dir: Option<String>",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi_source.contains(needle),
            "hover-card RBI projection should keep signature marker `{needle}`"
        );
    }

    for needle in [
        "pub fn HoverCard(",
        "#[prop(into)] content: ViewFn,",
        "children: Children,",
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "#[prop(optional)] open_delay_ms: Option<u64>",
        "#[prop(optional)] close_delay_ms: Option<u64>",
        "#[prop(optional)] motion: HoverCardMotion",
        "#[prop(optional, into)] class_name: Option<String>",
        "#[prop(optional, into)] id: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view signature should include `{needle}` for manifest/rbi drift detection"
        );
    }
}

#[test]
fn hover_card_component_files_check_script_covers_context_compression_manifest_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_context_compression_manifest_and_rbi_projection_are_present_and_current";

    assert!(
        script_source.contains(needle),
        "component-files check script should enforce `{needle}`"
    );
}

#[test]
fn hover_card_check2_marks_context_compression_manifest_and_rbi_contract_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "hover-card check2 should mark context-compression manifest/rbi gate complete."
    );

    for needle in [
        "components/hover-card/src/Component.toml",
        "components/hover-card/src/hover_card.rbi",
        "components/hover-card/test/semantics.rs::hover_card_context_compression_manifest_and_rbi_projection_are_present_and_current_locally",
        "components/hover-card/test/semantics.rs::hover_card_component_files_check_script_covers_context_compression_manifest_contract_locally",
        "components/hover-card/test/semantics.rs::hover_card_check2_marks_context_compression_manifest_and_rbi_contract_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_component_files_check_script_covers_context_compression_manifest_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_context_compression_manifest_and_rbi_contract_complete",
        "scripts/check-ui-components-component-files.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 context-compression section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_documents_agent_contract_schema_governance_rules_locally() {
    let checklist_source = load_source("check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
        "hover_card_agent_contract_is_schema_typed_and_machine_readable_locally",
        "hover_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing_locally",
        "hover_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally",
        "scripts/check-ui-components-contract-hygiene.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "hover-card checklist should keep Agent Contract governance rule `{required}`"
        );
    }
}

#[test]
fn hover_card_agent_contract_is_schema_typed_and_machine_readable_locally() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/hover_card.rbi");

    for needle in [
        "pub const HOVER_CARD_AGENT_SCHEMA: &str = \"ui.hover_card.agent-contract\";",
        "pub enum HoverCardAgentSchemaVersion",
        "pub enum HoverCardAgentIntent",
        "pub enum HoverCardAgentAction",
        "pub enum HoverCardAgentState",
        "pub enum HoverCardAgentSource",
        "pub enum HoverCardAgentConfigPolicy",
        "pub enum HoverCardAgentOutputStatus",
        "pub struct HoverCardAgentCapabilities",
        "pub struct HoverCardAgentContractInput",
        "pub struct HoverCardAgentContract",
        "pub fn resolve_agent_contract(input: HoverCardAgentContractInput) -> HoverCardAgentContract",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card logic should keep typed agent contract marker `{needle}`"
        );
    }

    for needle in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::HoverCardAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-config-policy=move || agent_contract.get().config_policy.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-capability-open=move || agent_contract.get().capabilities.can_open.then_some(\"true\")",
        "data-ui-capability-close=move || agent_contract.get().capabilities.can_close.then_some(\"true\")",
        "data-ui-capability-panel=move || agent_contract.get().capabilities.has_panel.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view should mount schemaized agent marker `{needle}`"
        );
    }

    for needle in [
        "name = \"agent-contract-markers\"",
        "name = \"agent_contract_schema_markers\"",
        "name = \"agent_contract_whitelist_render_policy\"",
        "schema = \"ui.hover_card.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-config-policy\"",
        "attr = \"data-ui-output-status\"",
        "name = \"render_path\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "hover-card manifest should include agent-contract marker `{needle}`"
        );
    }

    for needle in [
        "pub const HOVER_CARD_AGENT_SCHEMA: &str;",
        "pub enum HoverCardAgentSchemaVersion {",
        "pub enum HoverCardAgentIntent {",
        "pub enum HoverCardAgentAction {",
        "pub enum HoverCardAgentState {",
        "pub enum HoverCardAgentSource {",
        "pub enum HoverCardAgentConfigPolicy {",
        "pub enum HoverCardAgentOutputStatus {",
        "pub struct HoverCardAgentContractInput {",
        "pub struct HoverCardAgentContract {",
        "pub fn resolve_agent_contract(",
    ] {
        assert!(
            rbi_source.contains(needle),
            "hover-card rbi should project typed agent contract marker `{needle}`"
        );
    }
}

#[test]
fn hover_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing_locally()
 {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");

    for typed_source in [
        "schema_name: HOVER_CARD_AGENT_SCHEMA,",
        "schema_version: HoverCardAgentSchemaVersion::V1,",
        "intent: HoverCardAgentIntent::OverlayHint,",
        "HoverCardAgentAction::Open",
        "HoverCardAgentAction::Close",
        "HoverCardAgentState::Open",
        "HoverCardAgentState::Closed",
        "HoverCardAgentSource::Controlled",
        "HoverCardAgentSource::Uncontrolled",
        "config_policy: HoverCardAgentConfigPolicy::Whitelist,",
        "output_status: HoverCardAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(typed_source),
            "hover-card agent fields should stay type-derived via `{typed_source}`"
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
            "hover-card agent contract should avoid free-form schema splicing `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let styles_source = load_source("src/styles.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let manifest_source = load_source("src/Component.toml");
    let combined =
        format!("{view_source}\n{logic_source}\n{styles_source}\n{mod_source}\n{motion_source}");

    for needle in [
        "name = \"render_path\"",
        "typed_agent_contract_from_logic::resolve_agent_contract",
        "typed_render_mount_from_view::HoverCard",
        "\"inner_html\"",
        "\"dangerously_set_inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "hover-card manifest whitelist should include `{needle}`"
        );
    }

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
            "hover-card render path should stay whitelist-safe without `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_contract_hygiene_script_covers_agent_contract_schema_guards_locally() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene script should enforce `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_marks_agent_contract_schema_governance_complete_locally() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "hover_card_check2_documents_agent_contract_schema_governance_rules_locally",
        "hover_card_agent_contract_is_schema_typed_and_machine_readable_locally",
        "hover_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing_locally",
        "hover_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally",
        "hover_card_contract_hygiene_script_covers_agent_contract_schema_guards_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_documents_agent_contract_schema_governance_rules",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_agent_contract_is_schema_typed_and_machine_readable",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should keep Agent Contract governance marker `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes_locally() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "N/A：`HoverCard` 不是 LLM 正文渲染组件",
    ] {
        assert!(
            check2_source.contains(required),
            "hover-card check2 should keep streaming-definition marker `{required}`"
        );
    }

    for forbidden in [
        "AiRenderMode",
        "AiOutputStatus",
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
        "project_streaming_",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "hover-card runtime path should not embed LLM streaming protocol marker `{forbidden}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`"
    );
}

#[test]
fn hover_card_streaming_script_covers_two_mode_definition_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        script_source.contains(needle),
        "streaming check script should enforce `{needle}`"
    );
}

#[test]
fn hover_card_check2_marks_streaming_two_mode_definition_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "hover-card check2 should mark streaming two-mode definition gate complete."
    );

    for needle in [
        "hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes_locally",
        "hover_card_streaming_script_covers_two_mode_definition_contract_locally",
        "hover_card_check2_marks_streaming_two_mode_definition_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_streaming_script_covers_two_mode_definition_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_streaming_two_mode_definition_complete",
        "scripts/check-ui-components-streaming.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 streaming section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_documents_snapshot_as_default_baseline_capability_locally() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
        "N/A：`HoverCard` 不直接渲染 LLM 正文",
        "hover_card_check2_documents_snapshot_as_default_baseline_capability_locally",
        "scripts/check-ui-components-streaming.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should keep snapshot-baseline marker `{needle}`"
        );
    }
}

#[test]
fn hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably_locally() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");
    let combined = format!("{view_source}\n{logic_source}");

    for needle in [
        "let delay_state = logic::normalize_delay_state(logic::DelayStateInput {",
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "let part_states = logic::normalize_part_states(logic::PartStatesInput {",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
        "data-open=move || open_signal.get().then_some(\"true\")",
        "data-closed=move || (!open_signal.get()).then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card snapshot baseline should keep stable complete-result render marker `{needle}`"
        );
    }

    for forbidden in [
        "data-ui-stream-support",
        "data-ui-stream-fallback",
        "data-ui-stream-mode",
        "data-stream-mode",
        "data-stream-fallback",
        "project_streaming_",
        "use_ai_space_state",
    ] {
        assert!(
            !combined.contains(forbidden),
            "hover-card snapshot baseline should avoid streaming-only protocol marker `{forbidden}`",
        );
    }

    for needle in [
        "hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably_locally",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 snapshot section should reference `{needle}`"
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        script_source.contains(script_needle),
        "streaming check script should include `{script_needle}`"
    );
}

#[test]
fn hover_card_streaming_script_covers_snapshot_baseline_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_marks_snapshot_baseline_capability_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "hover-card check2 should mark snapshot baseline gate complete."
    );

    for needle in [
        "hover_card_check2_documents_snapshot_as_default_baseline_capability_locally",
        "hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably_locally",
        "hover_card_streaming_script_covers_snapshot_baseline_contract_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_documents_snapshot_as_default_baseline_capability",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_streaming_script_covers_snapshot_baseline_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_snapshot_baseline_capability_complete",
        "scripts/check-ui-components-streaming.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 snapshot section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_documents_streaming_required_optional_classification_rules_locally() {
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "N/A：`HoverCard` 归类为 `Streaming Optional`",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should keep required/optional classification marker `{needle}`"
        );
    }

    for script_needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(script_needle),
            "streaming check script should enforce `{script_needle}`"
        );
    }
}

#[test]
fn hover_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous_locally() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "role=\"tooltip\"",
        "aria-keyshortcuts=aria_keyshortcuts",
        "data-open=move || open_signal.get().then_some(\"true\")",
        "data-closed=move || (!open_signal.get()).then_some(\"true\")",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card optional-streaming scope should keep semantic continuity marker `{needle}`",
        );
    }

    for needle in [
        "pub enum HoverCardAgentOutputStatus",
        "HoverCardAgentOutputStatus::Verified",
        "output_status: HoverCardAgentOutputStatus",
    ] {
        assert!(
            logic_source.contains(needle) || view_source.contains(needle),
            "hover-card optional-streaming scope should expose explicit output-status marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer_locally()
 {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let mod_source = load_source("src/mod.rs");
    let motion_source = load_source("src/motion.rs");
    let combined = format!("{view_source}\n{logic_source}\n{mod_source}\n{motion_source}");

    for forbidden in [
        "on_retry",
        "retry(",
        "reconnect",
        "backoff",
        "resume_stream",
        "validate_stream",
        "stream_error",
        "network_error",
        "disconnect",
    ] {
        assert!(
            !combined.contains(forbidden),
            "hover-card should keep validation/retry/resilience policy outside component layer; found `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_streaming_script_covers_required_optional_classification_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(needle),
            "streaming check script should enforce `{needle}`"
        );
    }
}

#[test]
fn hover_card_check2_marks_streaming_required_optional_classification_complete_locally() {
    let source = load_source("check2.md");

    for needle in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "hover_card_check2_documents_streaming_required_optional_classification_rules_locally",
        "hover_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous_locally",
        "hover_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer_locally",
        "hover_card_streaming_script_covers_required_optional_classification_contract_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_documents_streaming_required_optional_classification_rules",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_streaming_script_covers_required_optional_classification_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_streaming_required_optional_classification_complete",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 should keep required/optional classification evidence marker `{needle}`"
        );
    }
}

#[test]
fn public_api_boundary_is_minimal_and_hides_dom_details() {
    let source = load_source("src/mod.rs");

    for needle in [
        "mod logic;",
        "mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::HoverCardMotion;",
        "pub use view::HoverCard;",
        "pub use ui_state_primitives::hover_card::{",
    ] {
        assert!(
            source.contains(needle),
            "hover-card module boundary should include `{needle}`",
        );
    }

    for forbidden in [
        "pub mod logic;",
        "pub mod motion;",
        "pub mod view;",
        "web_sys",
        "HtmlElement",
        "NodeRef<",
    ] {
        assert!(
            !source.contains(forbidden),
            "hover-card public API should not expose `{forbidden}`",
        );
    }
}

#[test]
fn logic_layer_stays_as_state_primitive_adapter() {
    let source = load_source("src/logic.rs");

    for needle in [
        "use ui_state_primitives::hover_card as hover_card_state;",
        "pub const DEFAULT_OPEN_DELAY_MS: u64 = hover_card_state::DEFAULT_OPEN_DELAY_MS;",
        "pub const DEFAULT_CLOSE_DELAY_MS: u64 = hover_card_state::DEFAULT_CLOSE_DELAY_MS;",
        "pub struct DelayStateInput {",
        "pub struct DelayState {",
        "pub fn normalize_delay_state(input: DelayStateInput) -> DelayState {",
        "pub fn is_custom_motion(motion: HoverCardMotion) -> bool {",
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {",
        "pub struct OpenStateInput {",
        "pub struct OpenState {",
        "pub fn normalize_open_state(input: OpenStateInput) -> OpenState {",
        "pub struct PartStatesInput {",
        "pub struct PartStates {",
        "pub fn normalize_part_states(input: PartStatesInput) -> PartStates {",
        "pub fn resolve_part_state(input: HoverCardPartStateInput) -> HoverCardPartState {",
    ] {
        assert!(
            source.contains(needle),
            "hover-card logic should keep primitive-mapping contract `{needle}`",
        );
    }

    for forbidden in [
        "ui_headless::",
        "web_sys::",
        "NodeRef<",
        "KeyboardEvent",
        "set_attribute(",
    ] {
        assert!(
            !source.contains(forbidden),
            "hover-card logic should not carry view/headless contract detail `{forbidden}`",
        );
    }
}

#[test]
fn state_primitives_are_the_single_state_source() {
    let source = load_source("src/logic.rs");

    for needle in [
        "use ui_state_primitives::hover_card as hover_card_state;",
        "hover_card_state::state_attr_for_open(is_open)",
        "hover_card_state::normalize_optional_text(value)",
        "hover_card_state::resolve_id(custom_id, fallback_id)",
        "hover_card_state::has_custom_delays(open_delay_ms, close_delay_ms)",
        "hover_card_state::resolve_state(input)",
        "hover_card_state::compose_class_name(base_class_name, state)",
        "hover_card_state::compose_panel_vars(top_px, left_px, anchor_width_px)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card should consume state primitives through `{needle}`",
        );
    }

    for forbidden in ["store::", "app_state::", "global_state::", "use_context::<"] {
        assert!(
            !source.contains(forbidden),
            "hover-card logic should not bind business store contract via `{forbidden}`",
        );
    }
}

#[test]
fn async_interaction_contract_is_not_applicable_for_hover_card() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in [
        "is_loading",
        "aria-busy",
        "retry",
        "on_retry",
        "use_async_action",
        "error_message",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "hover-card should not carry async protocol marker `{forbidden}` when async is N/A",
        );
    }
}

#[test]
fn api_keeps_basic_usage_simple_without_state_object_wiring() {
    let view_source = load_source("src/view.rs");
    let signature_start = view_source
        .find("pub fn HoverCard(")
        .unwrap_or_else(|| panic!("HoverCard signature should exist"));
    let signature_end = view_source[signature_start..]
        .find(") -> impl IntoView {")
        .map(|offset| signature_start + offset)
        .unwrap_or_else(|| panic!("HoverCard signature should end with impl IntoView"));
    let signature = &view_source[signature_start..signature_end];

    for needle in ["#[prop(into)] content: ViewFn", "children: Children"] {
        assert!(
            signature.contains(needle),
            "hover-card simple API should keep `{needle}` as baseline inputs",
        );
    }

    for forbidden in ["state:", "state =", "state="] {
        assert!(
            !signature.contains(forbidden),
            "hover-card should not require internal state object wiring via `{forbidden}`",
        );
    }
}

#[test]
fn docs_include_short_hello_world_default_path() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    assert!(
        docs_source.contains("<Playground title=\"HoverCard\" code_signal=code>"),
        "hover-card docs should include a primary hello-world playground",
    );

    let snippet_start = docs_source
        .find("<HoverCard content=move || view!{ <div>...</div> }>")
        .unwrap_or_else(|| panic!("hover-card docs should include hello-world source snippet"));
    let snippet_end = docs_source[snippet_start..]
        .find("</HoverCard>")
        .map(|offset| snippet_start + offset + "</HoverCard>".len())
        .unwrap_or_else(|| panic!("hover-card hello-world snippet should terminate"));
    let snippet = &docs_source[snippet_start..snippet_end];
    let non_empty_lines = snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        non_empty_lines <= 5,
        "hover-card hello-world snippet should stay within 5 non-empty lines, got {non_empty_lines}"
    );

    for forbidden in ["default_open", "on_open_change", "open=", "state="] {
        assert!(
            !snippet.contains(forbidden),
            "hover-card hello-world snippet should keep default path and avoid `{forbidden}`",
        );
    }
}

#[test]
fn default_theme_visual_desire_contract_is_documented_with_baseline_anchor() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    let styles_source = load_source("src/styles.rs");

    for needle in [
        "title=\"HoverCard\"",
        "description=\"Hover/focus triggered card with open/close delays.\"",
        "data-visual-baseline=\"hover-card-default-theme\"",
        "class=\"docs-stack\"",
        "class=\"ui-muted\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs should keep visual baseline marker `{needle}`",
        );
    }

    for needle in [
        ".ui-hover-card__trigger:focus-visible",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "opacity: var(--ui-hover-card-opacity, 0);",
        "var(--ui-overlay-enter-offset-y,",
        "var(--ui-fallback-overlay-enter-offset-y)",
        "var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale))",
    ] {
        assert!(
            styles_source.contains(needle),
            "hover-card styles should preserve visible interaction feedback contract `{needle}`",
        );
    }

    for forbidden in [
        "class=\"btn",
        "btn btn-",
        "class=\"form-control",
        "class=\"panel",
        "class=\"card-header",
        "class=\"glyphicon",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "hover-card docs should avoid legacy bootstrap-like visual contract marker `{forbidden}`",
        );
    }
}

#[test]
fn composite_parent_item_api_is_not_applicable_for_hover_card() {
    let view_source = load_source("src/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    let signature_start = view_source
        .find("pub fn HoverCard(")
        .unwrap_or_else(|| panic!("HoverCard signature should exist"));
    let signature_end = view_source[signature_start..]
        .find(") -> impl IntoView {")
        .map(|offset| signature_start + offset)
        .unwrap_or_else(|| panic!("HoverCard signature should end with impl IntoView"));
    let signature = &view_source[signature_start..signature_end];

    assert!(
        signature.contains("children: Children"),
        "hover-card should expose explicit child composition entrypoint",
    );

    for forbidden in ["labels:", "titles:", "panels:", "item_specs:", "ItemSpec"] {
        assert!(
            !signature.contains(forbidden),
            "hover-card should not expose collection sugar via `{forbidden}`",
        );
    }

    for forbidden in ["labels=", "titles=", "panels=", "item_specs="] {
        assert!(
            !docs_source.contains(forbidden),
            "hover-card docs should not recommend collection array binding via `{forbidden}`",
        );
    }
}

#[test]
fn macro_micro_drag_state_machine_is_not_applicable_for_hover_card() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");

    for forbidden in ["Dragging", "Action::DragEnd", "on:pointermove", "on:drag"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "hover-card should keep macro/micro drag state machine as N/A; found `{forbidden}`",
        );
    }
}

#[test]
fn two_pass_geometry_rendering_contract_is_wired_with_idempotent_convergence_guard() {
    let view_source = load_source("src/view.rs");
    let headless_source = load_source("../../crates/ui-headless/src/popover_position.rs");

    for needle in [
        "use_popover_position",
        "PopoverPositionOptions {",
        "placement,",
        "logic::compose_panel_vars(",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view should keep two-pass geometry wiring marker `{needle}`",
        );
    }

    for needle in [
        "compute_popover_position(",
        "should_update_scalar(",
        "POSITION_EPSILON_PX",
        "if should_update_scalar(anchor_width_px.get_untracked(), computed.anchor_width) {",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
        "if should_update_scalar(left_px.get_untracked(), computed.left) {",
    ] {
        assert!(
            headless_source.contains(needle),
            "headless popover positioning should keep rectification/idempotent guard marker `{needle}`",
        );
    }
}

#[test]
fn registration_protocol_is_not_applicable_for_hover_card() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "hover-card should keep collection registration protocol as N/A; found `{forbidden}`",
        );
    }
}

#[test]
fn slot_projection_strategy_is_not_applicable_for_hover_card() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");

    for forbidden in ["Lazy", "KeepAlive", "Eager", "NotifyHidden"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "hover-card should keep slot projection strategy as N/A; found `{forbidden}`",
        );
    }
}

#[test]
fn env_streams_are_sampled_in_headless_and_not_flooded_in_view() {
    let view_source = load_source("src/view.rs");
    let headless_source = load_source("../../crates/ui-headless/src/popover_position.rs");

    for needle in [
        "use_popover_position",
        "request_animation_frame",
        "raf_pending",
        "ResizeObserver",
        "add_event_listener_with_callback(\"resize\"",
        "add_event_listener_with_callback_and_bool(\"scroll\"",
        "should_update_scalar(",
    ] {
        assert!(
            view_source.contains("use_popover_position") && headless_source.contains(needle),
            "hover-card env stream contract should include sampled headless marker `{needle}`",
        );
    }

    for forbidden in [
        "on:scroll=",
        "on:resize=",
        "BreakpointChanged",
        "IntersectionObserver",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should not flood raw env events via `{forbidden}`",
        );
    }
}

#[test]
fn event_light_cone_is_not_applicable_for_hover_card() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");

    for forbidden in [
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "Table",
        "Grid",
        "prop drilling",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "hover-card should keep event light cone as N/A; found `{forbidden}`",
        );
    }
}

#[test]
fn causality_bus_is_not_applicable_for_hover_card() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "Causality Bus",
        "broadcast",
        "subscriber",
    ] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !headless_source.contains(forbidden),
            "hover-card should keep causality bus as N/A; found `{forbidden}`",
        );
    }
}

#[test]
fn focus_stack_gc_is_not_applicable_for_non_modal_hover_card_overlay() {
    let view_source = load_source("src/view.rs");
    let headless_hover_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");

    for forbidden in [
        "use_focus_trap(",
        "FocusTrapOptions",
        "RestorePolicy",
        "focus_manager_push_trap",
        "focus_manager_pop_trap",
        "restore_focus_chain",
        "document.body",
        "previous_focus",
        "restore_target",
    ] {
        assert!(
            !view_source.contains(forbidden) && !headless_hover_source.contains(forbidden),
            "hover-card should keep focus-stack restore protocol as N/A; found `{forbidden}`",
        );
    }

    for needle in [
        "pub fn use_hover_card_focus_a11y(options: HoverCardFocusA11yOptions) -> HoverCardFocusA11y",
        "let focus_target = StoredValue::new_local(None::<leptos::web_sys::Element>);",
        "set_attribute(\"aria-describedby\", &id)",
        "remove_attribute(\"aria-describedby\")",
    ] {
        assert!(
            headless_hover_source.contains(needle),
            "hover-card should limit focus handling to aria-describedby wiring via `{needle}`",
        );
    }

    for needle in [
        "pub enum RestorePolicy {",
        "Selector(String)",
        "FallbackTo(String)",
        "FOCUS_MANAGER_STACK",
        "focus_manager_push_trap(",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(needle),
            "global overlay focus stack should stay implemented in headless focus_trap via `{needle}`",
        );
    }
}

#[test]
fn escape_hatches_foreign_zone_is_not_applicable_for_hover_card() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let state_source = load_source("../../crates/ui-state-primitives/src/hover_card.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for forbidden in [
        "ECharts",
        "echarts",
        "mapbox",
        "leaflet",
        "google.maps",
        "amap",
        "YieldControl",
        "CleanupForeign",
        "Foreign Zone",
        "foreign_zone",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !headless_source.contains(forbidden)
                && !state_source.contains(forbidden)
                && !docs_source.contains(forbidden),
            "hover-card should keep foreign-zone escape hatch contract as N/A; found `{forbidden}`",
        );
    }

    for forbidden in [
        "wasm_bindgen::JsValue",
        "js_sys::Object",
        "HtmlCanvasElement",
        "HtmlIFrameElement",
        "pub struct HoverCardForeign",
        "pub type HoverCardForeign",
    ] {
        assert!(
            !mod_source.contains(forbidden) && !view_source.contains(forbidden),
            "hover-card public api should not expose imperative foreign instance handle `{forbidden}`",
        );
    }

    for needle in [
        "pub use view::HoverCard;",
        "use ui_state_primitives::hover_card as hover_card_state;",
    ] {
        assert!(
            mod_source.contains(needle) || logic_source.contains(needle),
            "hover-card should keep API/state contracts pure without foreign-instance pollution via `{needle}`",
        );
    }
}

#[test]
fn hydration_discontinuity_uses_seeded_id_provider_instead_of_runtime_time_or_random() {
    let view_source = load_source("src/view.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");

    for needle in [
        "use_ui_id_provider",
        "let generated_id = use_ui_id_provider()",
        "id_provider.next_prefixed_id(\"ui-hover-card\")",
        "let (id, has_custom_id) = logic::resolve_id(id, generated_id);",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card hydration id contract should include `{needle}`",
        );
    }

    for forbidden in [
        "thread_local!",
        "fn next_id()",
        "SystemTime::now",
        "Instant::now",
        "Uuid::new_v4",
        "rand::",
        "js_sys::Date::now",
        "crypto.getRandomValues",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card should not use non-deterministic id source `{forbidden}` in view",
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should provide deterministic id seed contract via `{needle}`",
        );
    }

    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
        "pub fn next_prefixed_id(self, prefix: &str) -> String",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "ui-headless id provider contract should expose `{needle}`",
        );
    }
}

#[test]
fn ssr_cross_platform_contract_covers_web_ssr_wasm_and_non_wasm_source_guards() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_hover_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let headless_popover_source = load_source("../../crates/ui-headless/src/popover_position.rs");
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let platforms_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-components",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-components --no-default-features --features component-hover_card,inject-css",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-hover_card,inject-css",
    ] {
        assert!(
            platforms_script_source.contains(needle),
            "platform compile-only evidence script should include `{needle}`",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            motion_source.contains(needle),
            "hover-card motion should keep explicit platform cfg branch `{needle}`",
        );
    }

    for needle in [
        "#[cfg(all(feature = \"web\", target_arch = \"wasm32\"))]",
        "#[cfg(not(all(feature = \"web\", target_arch = \"wasm32\")))]",
    ] {
        assert!(
            headless_popover_source.contains(needle),
            "headless popover should keep explicit web/ssr cfg branch `{needle}`",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`",
        );
    }

    assert!(
        headless_lib_source.contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless should keep web/ssr mutex compile guard contract",
    );

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "wasm_bindgen::",
        "window(",
        "document(",
    ] {
        assert!(
            !logic_source.contains(forbidden) && !view_source.contains(forbidden),
            "hover-card non-wasm component path should not reference browser object `{forbidden}`",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            headless_hover_source.contains(needle),
            "headless hover-card contract should keep explicit platform cfg branch `{needle}`",
        );
    }
}

#[test]
fn ui_headless_web_ssr_mutex_contract_is_enforced_for_hover_card_dependencies() {
    let view_source = load_source("src/view.rs");
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let platforms_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    assert!(
        headless_lib_source
            .contains("compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")"),
        "ui-headless should enforce web/ssr mutex via compile_error contract",
    );

    for needle in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
        "cargo check -p ui-headless --no-default-features --features web,ssr",
        "expected ui-headless web+ssr to fail",
        "mutually exclusive",
    ] {
        assert!(
            platforms_script_source.contains(needle),
            "platform guard script should include ui-headless mutex verification marker `{needle}`",
        );
    }

    for needle in [
        "use_hover_card_trigger",
        "use_hover_card_focus_a11y",
        "use_hover_card_dismiss",
        "use_popover_position",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card should consume ui-headless contract via `{needle}` without bypassing feature boundaries",
        );
    }
}

#[test]
fn ui_motion_non_wasm_noop_stub_contract_is_enforced_for_hover_card() {
    let motion_source = load_source("src/motion.rs");
    let motion_lib_source = load_source("../../crates/ui-motion/src/lib.rs");
    let platforms_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    for needle in [
        "cargo check -p ui-motion",
        "cargo check -p ui-motion --target wasm32-unknown-unknown",
        "cargo test -p ui-motion --test non_wasm_stub",
    ] {
        assert!(
            platforms_script_source.contains(needle),
            "platform script should include ui-motion compile/stub verification marker `{needle}`",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib_source.contains(needle),
            "ui-motion non-wasm stub baseline should include `{needle}`",
        );
    }

    let non_wasm_start = motion_source
        .find("#[cfg(not(target_arch = \"wasm32\"))]")
        .unwrap_or_else(|| panic!("hover-card motion should define non-wasm attach branch"));
    let non_wasm_end = motion_source[non_wasm_start..]
        .find("\n#[cfg(test)]")
        .map(|offset| non_wasm_start + offset)
        .unwrap_or_else(|| panic!("hover-card motion non-wasm branch should end before cfg(test)"));
    let non_wasm_branch = &motion_source[non_wasm_start..non_wasm_end];

    for needle in [
        "pub fn attach_motion(",
        "_node_ref: leptos::prelude::NodeRef<leptos::html::Div>",
        "_placement: leptos::prelude::Signal<PopoverPlacement>",
        "std::hint::black_box(sanitize_motion(motion));",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            non_wasm_branch.contains(needle),
            "hover-card non-wasm motion fallback should include `{needle}`",
        );
    }

    for forbidden in [
        "unchecked_into",
        "set_property(",
        "SpringAnimator::new(",
        "panic!(",
        "unwrap(",
        "expect(",
    ] {
        assert!(
            !non_wasm_branch.contains(forbidden),
            "hover-card non-wasm motion fallback should avoid runtime-dependent marker `{forbidden}`",
        );
    }
}

#[test]
fn reduced_motion_ssr_wasm_branches_keep_semantics_consistent_locally() {
    let view_source = load_source("src/view.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_hover_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let platforms_script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    assert!(
        platforms_script_source.contains(
            "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_reduced_motion_ssr_wasm_branches_keep_semantics_consistent"
        ),
        "platform script should include hover-card reduced-motion/ssr/wasm contract command",
    );

    for needle in [
        "role=\"tooltip\"",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card semantic output should keep stable marker `{needle}` across platform branches",
        );
    }

    assert!(
        !view_source.contains("#[cfg("),
        "hover-card view should not split semantic markup by target cfg",
    );

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if ui_motion::web::prefers_reduced_motion() {",
        "if !open {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            motion_source.contains(needle),
            "hover-card motion should include reduced-motion/wasm/non-wasm branch marker `{needle}`",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            headless_hover_source.contains(needle),
            "headless hover-card should keep platform cfg marker `{needle}`",
        );
    }
}

#[test]
fn a11y_i18n_l10n_contract_is_wired_without_hardcoded_copy() {
    let view_source = load_source("src/view.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");

    for needle in [
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional, into)] dir: Option<String>",
        "let lang = StoredValue::new(logic::normalize_optional_text(lang));",
        "let dir = StoredValue::new(logic::normalize_optional_text(dir));",
        "lang=move || lang.with_value(|value| value.clone())",
        "dir=move || dir.with_value(|value| value.clone())",
        "role=\"tooltip\"",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
        "use_hover_card_focus_a11y(HoverCardFocusA11yOptions {",
        "use_hover_card_dismiss(HoverCardDismissOptions {",
        "#[prop(into)] content: ViewFn",
        "{move || content.with_value(|content| content.run())}",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card a11y/i18n contract should include `{needle}`",
        );
    }

    for forbidden in ["view! { \"", "view!{\""] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should not hardcode user-visible copy via `{forbidden}`",
        );
    }

    for needle in [
        "pub fn use_hover_card_focus_a11y(options: HoverCardFocusA11yOptions) -> HoverCardFocusA11y",
        "pub fn use_hover_card_dismiss(options: HoverCardDismissOptions) -> HoverCardDismissA11y",
    ] {
        assert!(
            headless_source.contains(needle),
            "hover-card should delegate shared a11y helpers to headless via `{needle}`",
        );
    }
}

#[test]
fn state_markers_are_observable_queryable_and_enum_bounded() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "data-slot=root_state.slot_attr",
        "data-slot=trigger_state.slot_attr",
        "data-slot=panel_state.slot_attr",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open=move || open_signal.get().then_some(\"true\")",
        "data-closed=move || (!open_signal.get()).then_some(\"true\")",
        "data-disabled=root_state.is_disabled.then_some(\"true\")",
        "data-enabled=(!root_state.is_disabled).then_some(\"true\")",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
        "data-class-source=root_state.class_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-delay-source=root_state.delay_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card observable state marker contract should include `{needle}`",
        );
    }

    for needle in [
        "pub fn open_mode_attr(is_controlled: bool) -> &'static str",
        "pub fn open_value_source_attr(is_controlled: bool) -> &'static str",
        "pub const fn open_intent_source_attr() -> &'static str",
        "\"controlled\"",
        "\"uncontrolled\"",
        "\"external\"",
        "\"default\"",
        "\"interaction\"",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card source marker values should stay enum-bounded via `{needle}`",
        );
    }
}

#[test]
fn semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let semantics_source = load_source("test/semantics.rs");

    for needle in [
        "role=\"tooltip\"",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card semantics should expose role/aria/state/source marker `{needle}`",
        );
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "pub fn open_mode_attr(is_controlled: bool) -> &'static str",
        "pub fn open_value_source_attr(is_controlled: bool) -> &'static str",
        "\"controlled\"",
        "\"uncontrolled\"",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "hover-card semantics matrix should include controlled/uncontrolled branch marker `{needle}`",
        );
    }

    for needle in [
        "#[prop(optional, into)] is_disabled: Option<bool>",
        "data-disabled=root_state.is_disabled.then_some(\"true\")",
        "data-disabled=trigger_state.is_disabled.then_some(\"true\")",
        "data-disabled=panel_state.is_disabled.then_some(\"true\")",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card semantics matrix should include disabled branch marker `{needle}`",
        );
    }

    for needle in [
        "on:keydown=move |ev| on_trigger_key_down.run(ev)",
        "on:keydown=move |ev| on_panel_key_down.run(ev)",
        "should_dismiss_on_escape",
    ] {
        assert!(
            view_source.contains(needle) || headless_source.contains(needle),
            "hover-card semantics matrix should include keyboard path marker `{needle}`",
        );
    }

    for needle in [
        "on:pointerenter=move |_| on_trigger_pointer_enter.run(())",
        "on:pointerleave=move |_| on_trigger_pointer_leave.run(())",
        "on:pointerenter=move |_| on_panel_pointer_enter.run(())",
        "on:pointerleave=move |_| on_panel_pointer_leave.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card semantics matrix should include pointer path marker `{needle}`",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "ui_motion::web::prefers_reduced_motion()",
    ] {
        assert!(
            motion_source.contains(needle),
            "hover-card semantics matrix should include wasm/ssr branch marker `{needle}`",
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !semantics_source.contains(&snapshot_macro) && !semantics_source.contains(&insta_snapshot),
        "hover-card semantic tests should not be replaced by visual snapshot-only assertions",
    );
}

#[test]
fn hover_card_check2_documents_semantics_first_testing_rules_locally() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。",
        "断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。",
        "新增/变更语义字段必须同步补测试，否则不得打勾。",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 semantics-first section should include `{needle}`",
        );
    }
}

#[test]
fn hover_card_semantics_suite_is_contract_first_not_snapshot_only_locally() {
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");
    let aggregated_source = load_source("../../components/hover-card/test/hover_card_semantics.rs");

    for needle in [
        "role=\"tooltip\"",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
        "on:keydown=move |ev| on_trigger_key_down.run(ev)",
        "on:keydown=move |ev| on_panel_key_down.run(ev)",
        "on:pointerenter=move |_| on_trigger_pointer_enter.run(())",
        "on:pointerleave=move |_| on_trigger_pointer_leave.run(())",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card semantics-first suite should keep semantic marker `{needle}` in view contract",
        );
    }

    for needle in [
        "fn semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency()",
        "fn hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()",
        "fn hover_card_semantics_suite_is_contract_first_not_snapshot_only_locally()",
        "fn hover_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks_locally()",
    ] {
        assert!(
            semantics_source.contains(needle),
            "hover-card local semantics suite should include `{needle}`",
        );
    }

    assert!(
        aggregated_source.contains(
            "fn hover_card_semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency()",
        ),
        "aggregated hover-card semantics suite should retain semantic contract matrix guard",
    );

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !semantics_source.contains(&snapshot_macro) && !semantics_source.contains(&insta_snapshot),
        "hover-card semantics-first suite must not degrade to snapshot-only assertions",
    );
}

#[test]
fn hover_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks_locally() {
    let view_source = load_source("src/view.rs");
    let semantics_source = load_source("test/semantics.rs");

    for marker in [
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
        "data-delay-source=root_state.delay_source_attr",
        "data-id-source=root_state.id_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-class-source=root_state.class_source_attr",
        "data-disabled=root_state.is_disabled.then_some(\"true\")",
        "role=\"tooltip\"",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
    ] {
        assert!(
            view_source.contains(marker),
            "hover-card view semantic marker `{marker}` should exist before test-matrix coverage assertion",
        );
        assert!(
            semantics_source.contains(marker),
            "hover-card semantics suite should cover changed semantic marker `{marker}`",
        );
    }
}

#[test]
fn hover_card_contract_hygiene_script_covers_semantics_first_contract_guards_locally() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_semantics_first_testing_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_semantics_suite_is_contract_first_not_snapshot_only",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
    ] {
        assert!(
            script_source.contains(needle),
            "contract-hygiene check script should enforce `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_semantics_first_testing_complete_locally() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "components/hover-card/test/semantics.rs::hover_card_check2_documents_semantics_first_testing_rules_locally",
        "components/hover-card/test/semantics.rs::hover_card_semantics_suite_is_contract_first_not_snapshot_only_locally",
        "components/hover-card/test/semantics.rs::hover_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks_locally",
        "components/hover-card/test/semantics.rs::hover_card_contract_hygiene_script_covers_semantics_first_contract_guards_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_documents_semantics_first_testing_rules",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_semantics_suite_is_contract_first_not_snapshot_only",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_contract_hygiene_script_covers_semantics_first_contract_guards",
        "scripts/check-ui-components-contract-hygiene.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 semantics-first completion section should reference `{needle}`",
        );
    }
}

#[test]
fn view_layer_mounts_headless_contract_without_rebuilding_it() {
    let source = load_source("src/view.rs");

    for needle in [
        "use_hover_card_dismiss",
        "use_hover_card_focus_a11y",
        "use_hover_card_trigger",
        "use_popover_position",
        "let part_states = logic::normalize_part_states(logic::PartStatesInput {",
        "motion::attach_motion(",
        "data-slot=root_state.slot_attr",
        "data-slot=trigger_state.slot_attr",
        "data-slot=panel_state.slot_attr",
    ] {
        assert!(
            source.contains(needle),
            "hover-card view should include semantic assembly marker `{needle}`",
        );
    }

    for forbidden in [
        "should_dismiss_on_escape",
        "set_attribute(\"aria-describedby\"",
        "remove_attribute(\"aria-describedby\"",
        "motion != HoverCardMotion::default()",
        "logic::resolve_part_state(HoverCardPartStateInput {",
    ] {
        assert!(
            !source.contains(forbidden),
            "hover-card view should not reimplement headless semantics via `{forbidden}`",
        );
    }
}

#[test]
fn api_naming_contract_uses_is_prefix_with_compat_alias() {
    let source = load_source("src/view.rs");

    for needle in [
        "#[prop(optional, into)] is_disabled: Option<bool>",
        "#[prop(optional, into)] disabled: Option<bool>",
        "let is_disabled = logic::resolve_is_disabled(is_disabled, disabled);",
    ] {
        assert!(
            source.contains(needle),
            "hover-card API naming contract should include `{needle}`",
        );
    }

    assert!(
        !source.contains("#[prop(optional)] disabled: bool"),
        "hover-card should not expose legacy boolean naming as the primary contract.",
    );
}

#[test]
fn controlled_uncontrolled_open_triplet_is_wired() {
    let source = load_source("src/view.rs");
    let headless_source = load_source("../../crates/ui-headless/src/hover_card.rs");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "open,",
        "default_open,",
        "on_open_change,",
        "open: normalized_open_state.open,",
        "default_open: normalized_open_state.default_open,",
        "on_open_change: normalized_open_state.on_open_change,",
    ] {
        assert!(
            source.contains(needle),
            "hover-card controlled/uncontrolled API should include `{needle}`",
        );
    }

    for needle in [
        "pub open: Option<Signal<bool>>",
        "pub default_open: Option<bool>",
        "pub on_open_change: Option<Callback<bool>>",
        "use_controllable_open_state_traced(\"hover_card\", open, default_open, on_open_change)",
    ] {
        assert!(
            headless_source.contains(needle),
            "hover-card headless trigger should include `{needle}`",
        );
    }
}

#[test]
fn default_delay_values_are_normalized_only_in_logic() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "#[prop(optional)] open_delay_ms: Option<u64>",
        "#[prop(optional)] close_delay_ms: Option<u64>",
        "let delay_state = logic::normalize_delay_state(logic::DelayStateInput {",
        "let open_delay_ms = delay_state.open_delay_ms;",
        "let close_delay_ms = delay_state.close_delay_ms;",
        "let has_custom_delays = delay_state.has_custom_delays;",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card delay normalization should include `{needle}` in view wiring",
        );
    }

    for forbidden in [
        "#[prop(optional, default = logic::DEFAULT_OPEN_DELAY_MS)] open_delay_ms: u64",
        "#[prop(optional, default = logic::DEFAULT_CLOSE_DELAY_MS)] close_delay_ms: u64",
        "logic::has_custom_delays(open_delay_ms, close_delay_ms)",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should not define delay defaults via `{forbidden}`",
        );
    }
}

#[test]
fn state_normalization_is_centralized_in_logic_layer() {
    let view_source = load_source("src/view.rs");

    for needle in [
        "let delay_state = logic::normalize_delay_state(logic::DelayStateInput {",
        "let normalized_open_state = logic::normalize_open_state(logic::OpenStateInput {",
        "let part_states = logic::normalize_part_states(logic::PartStatesInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view should consume centralized state normalization via `{needle}`",
        );
    }

    for forbidden in [
        "motion != HoverCardMotion::default()",
        "let root_state = logic::resolve_part_state(HoverCardPartStateInput {",
        "let trigger_state = logic::resolve_part_state(HoverCardPartStateInput {",
        "let panel_state = logic::resolve_part_state(HoverCardPartStateInput {",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should not keep distributed state derivation via `{forbidden}`",
        );
    }
}

#[test]
fn discrete_state_axes_are_enum_backed() {
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "#[prop(optional)] placement: PopoverPlacement",
        "slot: HoverCardSlot::Root",
        "slot: HoverCardSlot::Trigger",
        "slot: HoverCardSlot::Panel",
    ] {
        assert!(
            view_source.contains(needle) || logic_source.contains(needle),
            "hover-card discrete state axis should stay enum-backed via `{needle}`",
        );
    }

    for forbidden in [
        "variant: Option<String>",
        "size: Option<String>",
        "mode: Option<String>",
        "status: Option<String>",
        "placement: Option<String>",
    ] {
        assert!(
            !view_source.contains(forbidden) && !logic_source.contains(forbidden),
            "hover-card should not use free-form string discrete state via `{forbidden}`",
        );
    }
}

#[test]
fn styles_layer_stays_token_first() {
    let source = load_source("src/styles.rs");

    for needle in [
        "pub const CSS: &str",
        "--ui-overlay-panel-min-width,",
        "var(--ui-fallback-overlay-panel-min-width)",
        "--ui-overlay-viewport-inset,",
        "var(--ui-fallback-overlay-viewport-inset)",
        "var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index))",
        ".ui-hover-card__panel[data-state=\"panel\"]",
    ] {
        assert!(
            source.contains(needle),
            "hover-card styles should consume theme/token contract via `{needle}`",
        );
    }
}

#[test]
fn token_first_static_style_contract_excludes_utility_and_css_in_rust_pollution() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let motion_source = load_source("src/motion.rs");

    for needle in [
        "padding: var(--ui-space-md, var(--ui-fallback-space-md));",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border))",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
    ] {
        assert!(
            styles_source.contains(needle),
            "hover-card token-first visual style should consume theme variable `{needle}`",
        );
    }

    for forbidden in [
        "color: #",
        "background: #",
        "border-color: #",
        "rgb(",
        "rgba(",
        "hsl(",
        "hsla(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "hover-card styles should not introduce hardcoded color literal marker `{forbidden}`",
        );
    }

    for forbidden in [
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
        "class=\"text-",
        "class=\"bg-",
        "class=\"rounded-",
        "class=\"shadow-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card component contract should not embed utility-first class marker `{forbidden}`",
        );
    }

    for forbidden in [
        "stylist::",
        "stylex::",
        "emotion",
        "stitches",
        "linaria",
        "goober",
        "css!(",
    ] {
        assert!(
            !mod_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden),
            "hover-card component modules should not depend on css-in-rust default path `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals_locally() {
    let styles_source = load_source("src/styles.rs");
    let theme_css_source = load_source("../../crates/ui-theme/src/css.rs");

    for required in [
        "var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-focus-ring, var(--ui-fallback-focus-ring))",
        "var(--ui-radius-md, var(--ui-fallback-radius-md))",
        "--ui-overlay-panel-min-width,",
        "var(--ui-fallback-overlay-panel-min-width)",
        "--ui-overlay-viewport-inset,",
        "var(--ui-fallback-overlay-viewport-inset)",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index))",
        "var(--ui-overlay-enter-scale, var(--ui-fallback-overlay-enter-scale))",
        "var(--ui-overlay-enter-offset-y,",
        "var(--ui-fallback-overlay-enter-offset-y)",
        "var(--ui-fallback-min-inline-size-none)",
    ] {
        assert!(
            styles_source.contains(required),
            "hover-card styles should keep defensive fallback chain marker `{required}`",
        );
    }

    for required in [
        "--ui-fallback-disabled-opacity:",
        "--ui-fallback-border-width:",
        "--ui-fallback-focus-ring:",
        "--ui-fallback-radius-md:",
        "--ui-fallback-overlay-panel-min-width:",
        "--ui-fallback-overlay-viewport-inset:",
        "--ui-fallback-space-md:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-border:",
        "--ui-fallback-bg:",
        "--ui-fallback-fg:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-overlay-z-index:",
        "--ui-fallback-overlay-enter-scale:",
        "--ui-fallback-overlay-enter-offset-y:",
        "--ui-fallback-min-inline-size-none:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css should provide fallback terminal `{required}`",
        );
    }

    for forbidden in [
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width, 260px))",
        "var(--ui-overlay-viewport-inset, 16px)",
        "var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index, 1000))",
        "var(--ui-overlay-enter-offset-y, 8px)",
        "var(--ui-overlay-enter-scale, 0.98)",
        "padding: var(--ui-space-md);",
        "border-radius: var(--ui-radius-lg);",
        "border: 1px solid var(--ui-border);",
        "background: var(--ui-bg);",
        "color: var(--ui-fg);",
        "box-shadow: var(--ui-shadow-md);",
        "outline: 3px solid var(--ui-focus-ring);",
        "background: #",
        "color: #",
        "border-color: #",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "hover-card styles should avoid raw terminal token `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_defensive_variables_check_script_covers_style_fallback_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`",
    );
}

#[test]
fn hover_card_check2_marks_defensive_variables_contract_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "hover-card check2 should mark defensive-variables gate complete.",
    );

    for needle in [
        "hover_card_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals_locally",
        "hover_card_defensive_variables_check_script_covers_style_fallback_contract_locally",
        "scripts/check-ui-components-contract-hygiene.sh",
        "components/hover-card/src/styles.rs",
        "crates/ui-theme/src/css.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 defensive-variables section should reference `{needle}`",
        );
    }
}

#[test]
fn styles_depend_on_explicit_state_markers_and_runtime_css_vars_only() {
    let styles_source = load_source("src/styles.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        ".ui-hover-card[data-state=\"open\"]",
        ".ui-hover-card[data-open=\"true\"]",
        ".ui-hover-card[data-state=\"closed\"]",
        ".ui-hover-card[data-closed=\"true\"]",
        ".ui-hover-card[data-class-source=\"custom\"]",
        ".ui-hover-card[data-motion-source=\"custom\"]",
        ".ui-hover-card[data-delay-source=\"custom\"]",
        ".ui-hover-card[data-id-source=\"custom\"]",
        ".ui-hover-card__trigger[data-state=\"trigger\"]",
        ".ui-hover-card__panel[data-state=\"panel\"]",
    ] {
        assert!(
            styles_source.contains(needle),
            "hover-card styles should key state branches with explicit marker `{needle}`",
        );
    }

    for forbidden in [
        ":nth-child",
        ":nth-of-type",
        ":first-child",
        ":last-child",
        ".ui-hover-card .",
        ".ui-hover-card__panel .",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "hover-card styles should not guess state from brittle structure selector `{forbidden}`",
        );
    }

    assert!(
        view_source.contains("style=panel_vars"),
        "hover-card runtime style should stay on css-var payload via `style=panel_vars`",
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"transform:",
        "style=move || format!(\"top:",
        "style=move || format!(\"left:",
        "style=move || format!(\"transform:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should not inline business style branch via `{forbidden}`",
        );
    }

    for needle in [
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String",
        "\"--ui-hover-card-top:",
        "\"--ui-hover-card-left:",
        "\"--ui-hover-card-anchor-width:",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card runtime style contract should stay css-variable-only via `{needle}`",
        );
    }
}

#[test]
fn hover_card_cascade_layer_and_runtime_style_contract_is_enforced_locally() {
    let css_entry_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let view_source = load_source("src/view.rs");
    let logic_source = load_source("src/logic.rs");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-hover_card\")]",
        "out.push_str(crate::hover_card::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "hover-card css should be aggregated in @layer ui via `{needle}`",
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized css injection contract `{needle}`",
        );
    }

    assert!(
        view_source.contains("style=panel_vars"),
        "hover-card runtime style should stay css-var-only via `style=panel_vars`",
    );

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"transform:",
        "style=move || format!(\"top:",
        "style=move || format!(\"left:",
        "style=move || format!(\"transform:",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should not use plain inline style branch `{forbidden}`",
        );
    }

    for needle in [
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String",
        "\"--ui-hover-card-top:",
        "\"--ui-hover-card-left:",
        "\"--ui-hover-card-anchor-width:",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card runtime style payload should stay custom-property only via `{needle}`",
        );
    }
}

#[test]
fn hover_card_cascade_layer_check_script_covers_runtime_css_variable_only_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`",
    );
}

#[test]
fn hover_card_check2_marks_cascade_layer_runtime_style_contract_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "hover-card check2 should mark cascade-layer gate complete.",
    );

    for needle in [
        "hover_card_cascade_layer_and_runtime_style_contract_is_enforced_locally",
        "hover_card_cascade_layer_check_script_covers_runtime_css_variable_only_contract_locally",
        "scripts/check-ui-components-contract-hygiene.sh",
        "crates/ui-components/src/css.rs",
        "crates/ui-components/src/root.rs",
        "components/hover-card/src/view.rs",
        "components/hover-card/src/logic.rs",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 cascade-layer section should reference `{needle}`",
        );
    }
}

#[test]
fn motion_layer_maps_semantics_and_keeps_non_wasm_stub() {
    let source = load_source("src/motion.rs");

    for needle in [
        "pub struct HoverCardMotion {",
        "pub fn sanitize_motion(motion: HoverCardMotion) -> HoverCardMotion",
        "pub fn attach_motion(",
        "default_overlay_layout_tokens",
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            source.contains(needle),
            "hover-card motion should preserve contract/runtime guard `{needle}`",
        );
    }
}

#[test]
fn motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally() {
    let motion_source = load_source("src/motion.rs");
    let motion_test_source = load_source("test/motion.rs");
    let view_source = load_source("src/view.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for needle in [
        "pub struct HoverCardMotion {",
        "pub spring: ui_motion::spring::SpringConfig,",
        "fn sanitize_spring(value: ui_motion::spring::SpringConfig)",
        "stiffness:",
        "damping:",
        "pub fn sanitize_motion(motion: HoverCardMotion) -> HoverCardMotion",
        "#[cfg(target_arch = \"wasm32\")]",
        "pub fn attach_motion(",
        "let motion = StoredValue::new(sanitize_motion(motion));",
        "if ui_motion::web::prefers_reduced_motion() {",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "std::hint::black_box(sanitize_motion(motion));",
        "if !is_open.get() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            motion_source.contains(needle),
            "hover-card motion contract should include `{needle}`",
        );
    }

    for needle in [
        "fn default_motion_uses_slide_spring_contract()",
        "fn supports_custom_motion_contract()",
        "stiffness: 320.0",
        "damping: 28.0",
        "fn sanitize_motion_falls_back_for_invalid_values()",
    ] {
        assert!(
            motion_test_source.contains(needle),
            "hover-card motion regression should include `{needle}`",
        );
    }

    assert!(
        view_source.contains("motion::attach_motion("),
        "hover-card view should mount motion contract through `attach_motion`",
    );

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            ui_motion_source.contains(needle),
            "ui-motion should keep non-wasm reduced-motion/no-op marker `{needle}`",
        );
    }
}

#[test]
fn motion_contract_platform_script_covers_guard_locally() {
    let source = load_source("../../scripts/check-ui-components-platforms.sh");

    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        source.contains(needle),
        "platform check script should enforce `{needle}`",
    );
}

#[test]
fn hover_card_check2_marks_motion_contractualization_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "hover-card check2 should mark motion contractualization gate complete.",
    );

    for needle in [
        "HoverCardMotion` + `sanitize_motion` + `attach_motion`",
        "supports_custom_motion_contract",
        "stiffness: 320.0",
        "damping: 28.0",
        "if ui_motion::web::prefers_reduced_motion() {",
        "std::hint::black_box(sanitize_motion(motion));",
        "hover_card_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "scripts/check-ui-components-platforms.sh",
        "components/hover-card/src/motion.rs",
        "components/hover-card/src/view.rs",
        "crates/ui-motion/src/lib.rs",
        "components/hover-card/test/semantics.rs::motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 motion section should reference `{needle}`",
        );
    }
}

#[test]
fn hover_card_ui_components_fixed_entry_files_follow_layered_boundaries_locally() {
    let lib_source = load_source("../../crates/ui-components/src/lib.rs");
    let css_source = load_source("../../crates/ui-components/src/css.rs");
    let root_source = load_source("../../crates/ui-components/src/root.rs");
    let active_highlight_source =
        load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let presence_source = load_source("../../crates/ui-headless/src/presence.rs");
    let a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "mod css;",
        "pub mod root;",
        "pub use root::UiRoot;",
        "#[cfg(feature = \"component-hover_card\")]",
        "pub use ui_hover_card as hover_card;",
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String)",
        "css::push_components_css(out);",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep marker `{needle}`"
        );
    }

    for forbidden in [
        "pub mod css;",
        "leptos::web_sys",
        "web_sys::",
        "wasm_bindgen",
    ] {
        assert!(
            !lib_source.contains(forbidden),
            "ui-components lib entry should not leak platform/internal marker `{forbidden}`"
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-hover_card\")]",
        "out.push_str(crate::hover_card::styles::CSS);",
        "#[cfg(feature = \"component-active_highlight\")]",
        "out.push_str(ui_visual_primitive::active_highlight::CSS);",
        "#[cfg(not(feature = \"inject-css\"))]",
        "pub fn push_components_css(_out: &mut String) {}",
    ] {
        assert!(
            css_source.contains(needle),
            "ui-components css registry should keep feature-gated marker `{needle}`"
        );
    }

    for needle in [
        "use ui_headless::{UiI18n, provide_ui_i18n, provide_ui_id_provider};",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if let Some(overrides) = semantic_overrides.get_value() {",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
        "data-theme-scheme",
        "data-theme-color",
        "data-theme-system",
        "data-theme-scale",
    ] {
        assert!(
            root_source.contains(needle),
            "UiRoot should keep centralized theme/i18n marker `{needle}`"
        );
    }

    for needle in [
        "pub const CSS: &str =",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight_source.contains(needle),
            "active_highlight shared primitive should keep marker `{needle}`"
        );
    }

    for forbidden in [
        "HoverCard",
        "Button",
        "Accordion",
        "aria-",
        "data-slot",
        "on:click",
    ] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should stay generic and avoid component business marker `{forbidden}`"
        );
    }

    for forbidden in [
        "../../crates/ui-components/src/overlay_open.rs",
        "../../crates/ui-components/src/presence.rs",
        "../../crates/ui-components/src/a11y.rs",
    ] {
        assert!(
            !path_exists(forbidden),
            "ui-components forbidden entrypoint file should not exist: `{forbidden}`"
        );
    }

    for required in [
        "../../crates/ui-headless/src/controllable_state.rs",
        "../../crates/ui-headless/src/presence.rs",
        "../../crates/ui-headless/src/a11y.rs",
    ] {
        assert!(
            path_exists(required),
            "ui-headless canonical primitive file should exist: `{required}`"
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "pub fn use_presence(",
        "pub fn aria_controls_when_open(",
    ] {
        assert!(
            controllable_state_source.contains(needle)
                || presence_source.contains(needle)
                || a11y_source.contains(needle),
            "headless canonical primitive files should keep marker `{needle}`"
        );
    }
}

#[test]
fn hover_card_entrypoints_check_script_covers_fixed_entrypoint_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-entrypoints.sh");
    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_ui_components_fixed_entry_files_follow_layered_boundaries";

    assert!(
        script_source.contains(needle),
        "entrypoints check script should enforce `{needle}`"
    );
}

#[test]
fn hover_card_check2_marks_ui_components_fixed_entry_files_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] `ui-components` 固定入口文件落点正确。"),
        "hover-card check2 should mark fixed-entrypoint gate complete."
    );

    for required in [
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            source.contains(required),
            "hover-card check2 fixed-entrypoint section should retain rule `{required}`"
        );
    }

    for needle in [
        "components/hover-card/test/semantics.rs::hover_card_ui_components_fixed_entry_files_follow_layered_boundaries_locally",
        "components/hover-card/test/semantics.rs::hover_card_entrypoints_check_script_covers_fixed_entrypoint_contract_locally",
        "components/hover-card/test/semantics.rs::hover_card_check2_marks_ui_components_fixed_entry_files_complete_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_ui_components_fixed_entry_files_follow_layered_boundaries",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_entrypoints_check_script_covers_fixed_entrypoint_contract",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_check2_marks_ui_components_fixed_entry_files_complete",
        "scripts/check-ui-components-entrypoints.sh",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_ui_components_fixed_entry_files_follow_layered_boundaries",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 fixed-entrypoint section should reference `{needle}`"
        );
    }
}

#[test]
fn hover_card_performance_governance_budget_is_defined_traceable_and_blocking_locally() {
    let check2_source = load_source("check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let view_source = load_source("src/view.rs");
    let button_check2_source = load_source("../../components/button/check2.md");
    let input_check2_source = load_source("../../components/text-input/src/input/check2.md");

    for needle in [
        "\"hover-card\" => UiPerfBudget {",
        "max_mount_ms: 30.0,",
        "max_update_ms: Some(10.0),",
        "max_heap_kb: Some(512.0),",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep hover-card perf budget marker `{needle}`",
        );
    }

    for needle in [
        "component_doc!(\"HoverCard\", \"hover-card\", \"Overlays\", overlays::hover_card)",
        "\"hover-card\"",
        "overlays::hover_card",
    ] {
        assert!(
            pages_source.contains(needle),
            "hover-card docs page should remain in traversal coverage via `{needle}`",
        );
    }

    for needle in [
        "data-perf-mount-ms",
        "data-perf-budget-ms",
        "data-perf-budget-update-ms",
        "data-perf-budget-heap-kb",
        "data-perf-violation",
        "data-perf-observability",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should keep performance marker `{needle}`",
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
            "docs coverage e2e should keep perf guard `{needle}`",
        );
    }

    for needle in ["use_ui_trace()", "trace.emit("] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep trace attribution marker `{needle}`",
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view should expose perf attribution marker `{needle}`",
        );
    }

    for forbidden in [
        "on:mousemove=",
        "on:pointermove=",
        "on:touchmove=",
        "set_interval(",
        "spawn_local(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should avoid high-frequency flood marker `{forbidden}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_performance_governance_contract_is_budgeted_traceable_and_blocking";
    assert!(
        script_source.contains(script_needle),
        "performance gate script should include `{script_needle}`",
    );

    for needle in [
        "性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "UiPerfBudget{ mount=30ms, update=10ms, heap=512KB }",
        "component_page_perf_budget + UiPerfProbe",
        "use_ui_trace()/trace.emit",
        "hover_card_performance_governance_budget_is_defined_traceable_and_blocking_locally",
        "hover_card_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "渲染次数预算为 `1`",
        "mount-only + trace 等价证据过渡",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 performance evidence should include `{needle}`",
        );
    }

    for needle in [
        "性能治理：关键路径有预算",
        "渲染次数预算为 `1`",
        "render_count",
    ] {
        assert!(
            button_check2_source.contains(needle) && input_check2_source.contains(needle),
            "Button/Input shared baseline should include `{needle}`",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "todo plan should keep render_count follow-up marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally()
 {
    let local_semantics = load_source("test/semantics.rs");
    let aggregated_semantics =
        load_source("../../components/hover-card/test/hover_card_semantics.rs");
    let view_source = load_source("src/view.rs");
    let headless_hover_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency()",
        "fn hover_card_performance_governance_budget_is_defined_traceable_and_blocking_locally()",
        "fn hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally(",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "hover-card local semantics/performance suite should include `{required_test}`",
        );
    }

    for required_test in [
        "fn hover_card_semantic_contract_tests_cover_state_and_interaction_matrix_without_snapshot_dependency()",
        "fn hover_card_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement(",
    ] {
        assert!(
            aggregated_semantics.contains(required_test),
            "hover-card aggregated semantics/performance suite should include `{required_test}`",
        );
    }

    for marker in [
        "role=\"tooltip\"",
        "aria-keyshortcuts=dismiss_a11y.attrs.aria_keyshortcuts",
        "data-state=move || logic::state_attr_for_open(open_signal.get())",
        "data-open=move || open_signal.get().then_some(\"true\")",
        "data-closed=move || (!open_signal.get()).then_some(\"true\")",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "hover-card view should expose aria/data semantic marker `{marker}`",
        );
    }

    for marker in [
        "use_hover_card_focus_a11y(HoverCardFocusA11yOptions {",
        "on:focusin=move |ev| on_trigger_focus_in.run(ev)",
        "on:focusout=move |ev| on_trigger_focus_out.run(ev)",
        "on:focusin=move |_| on_panel_focus_in.run(())",
        "on:focusout=move |_| on_panel_focus_out.run(())",
        "manages_aria_describedby: true",
        "set_attribute(\"aria-describedby\", &id)",
        "remove_attribute(\"aria-describedby\")",
    ] {
        assert!(
            view_source.contains(marker) || headless_hover_source.contains(marker),
            "hover-card focus-flow contract should include `{marker}`",
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !local_semantics.contains(&snapshot_macro) && !local_semantics.contains(&insta_snapshot),
        "hover-card semantic/performance regression should not degrade to snapshot-only checks",
    );

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_semantics_and_performance_script_covers_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_semantics_and_performance_regression_contract_complete_locally() {
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "hover-card check2 semantic/performance section should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_view_macro_complexity_is_split_into_semantic_subrenders_locally() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "fn render_trigger_view(input: TriggerViewInput) -> impl IntoView {",
        "fn render_panel_view(input: PanelViewInput) -> impl IntoView {",
        "render_trigger_view(TriggerViewInput {",
        "render_panel_view(PanelViewInput {",
        "data-slot=trigger_state.slot_attr",
        "data-slot=panel_state.slot_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view should keep semantic subview split marker `{needle}`",
        );
    }

    let view_macro_count = view_source.matches("view! {").count();
    assert!(
        view_macro_count >= 3,
        "hover-card should split view macros into semantic blocks (expected >=3, found {view_macro_count})",
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "components/hover-card/test/semantics.rs::hover_card_view_macro_complexity_is_split_into_semantic_subrenders_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_view_macro_complexity_is_split_into_semantic_subrenders",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should include view-macro governance marker `{needle}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_view_macro_complexity_is_split_into_semantic_subrenders";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`",
    );
}

#[test]
fn hover_card_view_functional_split_prefers_plain_functions_over_local_components() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for needle in [
        "fn render_trigger_view(input: TriggerViewInput) -> impl IntoView {",
        "fn render_panel_view(input: PanelViewInput) -> impl IntoView {",
        "render_trigger_view(TriggerViewInput {",
        "render_panel_view(PanelViewInput {",
        "struct TriggerViewInput {",
        "struct PanelViewInput {",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card should keep function-first split marker `{needle}`",
        );
    }

    let component_attr_count = view_source.matches("#[component]").count();
    assert_eq!(
        component_attr_count, 1,
        "hover-card should keep only root `#[component]`; found {component_attr_count}",
    );

    for forbidden in [
        "let render_trigger_view = move || {",
        "let render_panel_view = move || {",
        "#[component]\nfn render_trigger_view(",
        "#[component]\nfn render_panel_view(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card view should avoid non-function split marker `{forbidden}`",
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "components/hover-card/test/semantics.rs::hover_card_view_functional_split_prefers_plain_functions_over_local_components",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_view_functional_split_prefers_plain_functions_over_local_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should include function-first governance marker `{needle}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_view_functional_split_prefers_plain_functions_over_local_components";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`",
    );
}

#[test]
fn hover_card_static_fragments_are_constantized_or_absent_for_simple_layout_locally() {
    let check2_source = load_source("check2.md");
    let view_source = load_source("src/view.rs");
    let script_source = load_source("../../scripts/check-ui-components-view-macro.sh");

    for forbidden in [
        "inner_html=",
        "include_str!(",
        "<svg",
        "<footer",
        "lorem ipsum",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "hover-card simple layout should avoid heavyweight inline static fragment marker `{forbidden}`",
        );
    }

    assert!(
        view_source.contains("{move || content.with_value(|content| content.run())}"),
        "hover-card panel should keep external content slot instead of embedding large static content fragments",
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "components/hover-card/test/semantics.rs::hover_card_static_fragments_are_constantized_or_absent_for_simple_layout_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_static_fragments_are_constantized_or_absent_for_simple_layout",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should include static-fragment governance marker `{needle}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_static_fragments_are_constantized_or_absent_for_simple_layout";
    assert!(
        script_source.contains(script_needle),
        "view-macro gate script should include `{script_needle}`",
    );
}

#[test]
fn hover_card_inner_html_usage_is_explicitly_na_and_guarded_locally() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/styles.rs",
        "src/view.rs",
        "src/motion.rs",
    ] {
        let source = load_source(rel_path);
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
                "hover-card source `{rel_path}` must not contain raw-html injection token `{forbidden}`",
            );
        }
    }

    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    for forbidden in [
        "inner_html",
        "set_inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "hover-card docs examples must not contain raw-html injection token `{forbidden}`",
        );
    }

    let check2_source = load_source("check2.md");
    for needle in [
        "- [x] `inner_html` 使用约束：",
        "零注入面",
        "components/hover-card/test/semantics.rs::hover_card_inner_html_usage_is_explicitly_na_and_guarded_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_inner_html_usage_is_explicitly_na_and_guarded",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card checklist should keep inner_html security evidence `{needle}`",
        );
    }

    let script_source = load_source("../../scripts/check-ui-components-inner-html.sh");
    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_inner_html_usage_is_explicitly_na_and_guarded";
    assert!(
        script_source.contains(script_needle),
        "inner-html check script should enforce `{script_needle}`",
    );
}

#[test]
fn hover_card_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated_locally() {
    let cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let crate_root_source = load_source("../../crates/ui-components/src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_hover_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let view_source = load_source("src/view.rs");
    let docs_hover_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    let check2_source = load_source("check2.md");
    let script_source = load_source("../../scripts/check-ui-components-wasm-debug.sh");

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep shared wasm-debug marker `{needle}`",
        );
    }
    assert!(
        !cargo_source.contains("hover-card-wasm-debug")
            && !cargo_source.contains("hover_card-wasm-debug"),
        "hover-card should not add a component-local wasm-debug feature that pollutes production API surface",
    );

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components root should keep wasm-debug isolation marker `{needle}`",
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
            "docs-app should expose dev-only wasm debug entry via `{needle}`",
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep visual/temporal trace marker `{needle}`",
        );
    }

    for needle in [
        "pub struct UiTraceEvent {",
        "pub ts_ms: u64,",
        "pub component: &'static str,",
        "pub enum UiTraceEventKind {",
        "OpenChange {",
        "Inspect {",
        "pub fn emit(self, component: &'static str, kind: UiTraceEventKind)",
        "ts_ms: now_ms(),",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace should keep typed timestamp/source event marker `{needle}`",
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            controllable_state_source.contains(needle),
            "ui-headless controllable state should emit open-change trace event via `{needle}`",
        );
    }
    for needle in [
        "use_controllable_open_state_traced(\"hover_card\", open, default_open, on_open_change)",
        "data-open-mode=open_mode_attr",
        "data-open-value-source=open_value_source_attr",
        "data-open-intent-source=open_intent_source_attr",
    ] {
        assert!(
            headless_hover_source.contains(needle) || view_source.contains(needle),
            "hover-card should keep replayable state/source marker `{needle}`",
        );
    }

    for needle in [
        "title=\"State + Source Markers\"",
        "Inspect data-delay-source and data-id-source on root/trigger/panel.",
    ] {
        assert!(
            docs_hover_source.contains(needle),
            "hover-card docs playground should keep minimal replay path marker `{needle}`",
        );
    }

    for needle in [
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "components/hover-card/test/semantics.rs::hover_card_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 should include wasm-debug governance marker `{needle}`",
        );
    }

    let script_needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        script_source.contains(script_needle),
        "wasm-debug gate script should include `{script_needle}`",
    );
}

#[test]
fn hover_card_dx_playground_supports_css_hot_reload_without_wasm_rebuild_locally() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep CSS hot-reload contract marker `{needle}`",
        );
    }

    for needle in [
        "pub(super) fn hover_card() -> AnyView",
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_source_path=\"components/hover-card/src/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "\"Open interactive hover card\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs should mount reusable Playground hot-reload path via `{needle}`",
        );
    }
}

#[test]
fn hover_card_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na_locally()
 {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "<div data-playground-scope=scope_id.clone()>",
        "<Card class_name=\"playground__preview\".to_string()>",
        "<div class=\"playground__preview-stage\">{children()}</div>",
        "data-slot=\"playground-controls\"",
        "class_name=\"playground__panel playground__controls\".to_string()",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep isolated-canvas contract marker `{needle}`",
        );
    }

    for needle in [
        "let (interactive_open_raw, set_interactive_open_raw) = signal(false);",
        "let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());",
        "let open_interactive_hover_card: OnPress =",
        "let close_interactive_hover_card: OnPress =",
        "let on_interactive_open_change: Callback<bool> =",
        "\"open: \" {move || interactive_open_raw.get()}",
        "<HoverCard",
        "is_open=interactive_open",
        "on_open_change=on_interactive_open_change",
        "Inspect root markers in DevTools while keeping context.",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs should keep context-preserving interactive marker `{needle}`",
        );
    }

    for forbidden in [
        "HOVER_CARD_WORKBENCH_STORAGE_KEY",
        "load_hover_card_workbench_state(",
        "save_hover_card_workbench_state(",
        "clear_hover_card_workbench_state(",
        "Persist workbench state",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "hover-card keeps optional persisted workbench state as N/A for current scope; `{forbidden}` should remain absent",
        );
    }

    for required in [
        "DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。",
        "组件调试应尽量保持当前交互上下文，降低重复操作成本。",
        "复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。",
    ] {
        assert!(
            check2_source.contains(required),
            "hover-card checklist should keep DX governance rule `{required}`",
        );
    }
}

#[test]
fn hover_card_dx_check_script_covers_hot_reload_and_isolated_canvas_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_dx_playground_supports_css_hot_reload_without_wasm_rebuild",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na",
    ] {
        assert!(
            script_source.contains(needle),
            "dx check script should enforce `{needle}`",
        );
    }
}

#[test]
fn hover_card_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract_locally() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "const HOVER_CARD_DOC_IMPORTS: &str =",
        "use leptos::prelude::*;\\nuse ui_components::{Button, ButtonVariant, HoverCard, HoverCardMotion, OnPress};",
        "code_imports=HOVER_CARD_DOC_IMPORTS.to_string()",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "requested mode:",
        "requested output status:",
        "effective component status: data-ui-output-status=verified",
        "data-slot=\"hover-card-source-first\"",
        "data-slot=\"hover-card-source-paths\"",
        "component-hover_card",
        "inject-css",
        "compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs should keep copy-ready + streaming/snapshot contract `{needle}`",
        );
    }

    for needle in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground copy pipeline should keep import completion marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_dx_check_script_covers_docs_product_copy_paste_ready_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    let needle = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract";
    assert!(
        script_source.contains(needle),
        "dx check script should enforce `{needle}`",
    );
}

#[test]
fn hover_card_check2_marks_docs_product_copy_paste_ready_contract_complete_locally() {
    let source = load_source("check2.md");

    for needle in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "Hello World (Minimal Path)",
        "State Matrix",
        "Controlled vs Uncontrolled",
        "Streaming / Snapshot Contract",
        "HOVER_CARD_DOC_IMPORTS",
        "compose_copy_ready_code",
        "hover_card_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract",
        "hover_card_dx_check_script_covers_docs_product_copy_paste_ready_contract_locally",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 docs-product section should reference `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_documents_interactive_playground_rules_locally() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 interactive-playground section should include `{needle}`",
        );
    }
}

#[test]
fn hover_card_docs_app_provides_interactive_playground_for_props_state_and_preview_locally() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "title=\"Interactive Playground\"",
        "description=\"Display + Config + CSS Test: keep controlled-open context while tuning styles.\"",
        "code_signal=interactive_code",
        "test_css_source=interactive_test_css",
        "test_source_path=\"components/hover-card/src/styles.rs\".to_string()",
        "test_config_signal=interactive_config",
        "data-slot=\"hover-card-e2e-controls\"",
        "data-slot=\"hover-card-e2e-open\"",
        "data-slot=\"hover-card-e2e-close\"",
        "\"open: \" {move || interactive_open_raw.get()}",
        "data-slot=\"hover-card-e2e-canvas\"",
        "data-slot=\"hover-card-e2e-trigger\"",
        "is_open=interactive_open",
        "on_open_change=on_interactive_open_change",
        "HoverCardActualConfig {",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs interactive playground should include `{needle}`",
        );
    }
}

#[test]
fn hover_card_interactive_playground_reuses_repeatable_semantic_e2e_flow_locally() {
    let e2e_source = load_source("../../e2e/tests/docs_app_hover_card_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "docs-app hover-card key flow is repeatable and failure points are semantic",
        "docs-app hover-card high-risk paths cover focus keyboard and settled semantic breakpoints",
        "for (const cycle of [1, 2])",
        "hover-card key flow cycle ${cycle}",
        "await expectHoverCardReady(interactiveRoot, panel);",
        "await expectHoverCardSettledClosed(interactiveRoot, panel);",
        "toHaveAttribute(\"data-open-mode\", \"controlled\")",
        "toHaveAttribute(\"data-open-value-source\", \"controlled\")",
        "toHaveAttribute(\"data-open-intent-source\", \"interaction\")",
    ] {
        assert!(
            e2e_source.contains(needle),
            "hover-card interactive e2e flow should include `{needle}`",
        );
    }

    for needle in [
        "data-slot=\"hover-card-e2e-controls\"",
        "data-slot=\"hover-card-e2e-open\"",
        "data-slot=\"hover-card-e2e-close\"",
        "data-slot=\"hover-card-e2e-canvas\"",
        "data-slot=\"hover-card-e2e-trigger\"",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs should expose stable interactive anchor `{needle}` for repeatable e2e replay",
        );
    }
}

#[test]
fn hover_card_dx_check_script_covers_interactive_playground_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: hover-card interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include interactive-playground marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_interactive_playground_item_complete_locally() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains(
            "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"
        ),
        "hover-card check2 should mark interactive-playground item complete",
    );

    for needle in [
        "title=\"Interactive Playground\"",
        "data-slot=\"hover-card-e2e-controls\"",
        "data-slot=\"hover-card-e2e-open\"",
        "data-slot=\"hover-card-e2e-close\"",
        "HoverCardActualConfig {",
        "N/A：`HoverCard` 非 AI Spec 组件",
        "hover_card_check2_documents_interactive_playground_rules_locally",
        "hover_card_docs_app_provides_interactive_playground_for_props_state_and_preview_locally",
        "hover_card_interactive_playground_reuses_repeatable_semantic_e2e_flow_locally",
        "hover_card_dx_check_script_covers_interactive_playground_contract_locally",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 interactive-playground section should retain marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope_locally()
{
    let root = component_root();
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let check2_source = load_source("check2.md");

    assert!(
        !root.join("src/spec.rs").exists(),
        "hover-card should keep spec/schema boundary as N/A for simple component scope",
    );
    assert!(
        !mod_source.contains("pub mod protocol;"),
        "hover-card should not expose serde protocol from public API surface when spec/config input is N/A",
    );

    let combined =
        format!("{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}");
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
            "hover-card engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`",
        );
    }

    for required in [
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A：`HoverCard` 当前无 spec/config 输入",
        "若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。",
        "关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。",
        "异步边界不得把具体 runtime 类型暴露到组件公共接口。",
    ] {
        assert!(
            check2_source.contains(required),
            "hover-card checklist should keep engineering governance marker `{required}`",
        );
    }
}

#[test]
fn hover_card_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events_locally()
 {
    let cargo_source = load_source("../../crates/ui-components/Cargo.toml");
    let headless_hover_source = load_source("../../crates/ui-headless/src/hover_card.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let combined = [
        load_source("src/mod.rs"),
        load_source("src/logic.rs"),
        load_source("src/view.rs"),
        load_source("src/styles.rs"),
        load_source("src/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "use_controllable_open_state_traced(\"hover_card\", open, default_open, on_open_change)",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            cargo_source.contains(required)
                || headless_hover_source.contains(required)
                || controllable_state_source.contains(required),
            "hover-card tracing baseline should include canonical marker `{required}`",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui_components::hover_card::",
        "const HOVER_CARD_TRACE_TARGET",
    ] {
        assert!(
            !combined.contains(forbidden),
            "hover-card component layer should avoid ad-hoc tracing semantic drift token `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_engineering_contract_avoids_runtime_leaks_in_public_api_surface_locally() {
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");

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
                "hover-card engineering contract should not leak runtime marker `{forbidden}`",
            );
        }
    }

    for forbidden in ["web_sys", "HtmlElement", "NodeRef<"] {
        assert!(
            !mod_source.contains(forbidden),
            "hover-card public API boundary should not leak platform detail `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_engineering_check_script_covers_serde_tracing_and_runtime_boundaries_locally() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_version_deprecation_migration_is_na_without_major_breaking_upgrade",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`",
        );
    }
}

#[test]
fn hover_card_version_deprecation_migration_is_na_without_major_breaking_upgrade_locally() {
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/hover_card.rbi");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let protocol_source = load_source("src/protocol.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"HoverCard\"",
        "crate = \"ui-hover-card\"",
        "schema = \"ui.hover_card.agent-contract.v1\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "hover-card manifest should keep stable v1 schema marker `{needle}`",
        );
    }

    for needle in [
        "pub fn HoverCard(",
        "is_open: Option<leptos::prelude::Signal<bool>>",
        "default_open: Option<bool>",
        "on_open_change: Option<leptos::prelude::Callback<bool>>",
        "pub enum HoverCardComponentSchemaVersion {",
        "V1,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "hover-card RBI should keep stable public API marker `{needle}`",
        );
    }

    let combined = format!(
        "{mod_source}\n{logic_source}\n{view_source}\n{styles_source}\n{motion_source}\n{protocol_source}"
    );
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
                && !combined.contains(forbidden),
            "hover-card should not introduce major-version migration marker `{forbidden}` in current scope",
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `HoverCard` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "hover_card_version_deprecation_migration_is_na_without_major_breaking_upgrade_locally",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card/check2.md should keep version-migration governance marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_version_deprecation_migration_script_covers_engineering_gate_locally() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    let marker = "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`",
    );
}

#[test]
fn hover_card_rust_hygiene_non_test_source_forbids_unwrap_expect_and_ignored_bindings_locally() {
    let combined = [
        load_source("src/mod.rs"),
        load_source("src/logic.rs"),
        load_source("src/styles.rs"),
        load_source("src/view.rs"),
        load_source("src/motion.rs"),
        load_source("src/protocol.rs"),
    ]
    .join("\n");

    for forbidden in [".unwrap(", ".expect(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "hover-card non-test source should forbid rust-hygiene anti-pattern `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_rust_hygiene_id_fallback_path_converges_to_cow_locally() {
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let state_source = load_source("../../crates/ui-state-primitives/src/hover_card.rs");

    for needle in [
        "use std::borrow::Cow;",
        "pub fn resolve_id(custom_id: Option<String>, fallback_id: Cow<'static, str>) -> (String, bool)",
    ] {
        assert!(
            logic_source.contains(needle) || state_source.contains(needle),
            "hover-card id fallback contract should include `{needle}`",
        );
    }

    for needle in [
        "Cow::Owned(id_provider.next_prefixed_id(\"ui-hover-card\"))",
        "Cow::Borrowed(\"ui-hover-card\")",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view should keep Cow-based id fallback marker `{needle}`",
        );
    }

    assert!(
        !view_source.contains("\"ui-hover-card\".to_string()"),
        "hover-card view should not keep string-clone fallback path for generated id",
    );
}

#[test]
fn hover_card_check2_marks_rust_hygiene_gate_complete_locally() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"),
        "hover-card check2 should mark rust-hygiene gate complete",
    );

    for needle in [
        "components/hover-card/src/view.rs",
        "components/hover-card/src/logic.rs",
        "crates/ui-state-primitives/src/hover_card.rs",
        "components/hover-card/test/semantics.rs::hover_card_rust_hygiene_non_test_source_forbids_unwrap_expect_and_ignored_bindings_locally",
        "components/hover-card/test/semantics.rs::hover_card_rust_hygiene_id_fallback_path_converges_to_cow_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_rust_hygiene_non_test_source_forbids_unwrap_expect_and_ignored_bindings",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_rust_hygiene_id_fallback_path_converges_to_cow",
        "./scripts/check-rust-hygiene.sh",
        "PCRE2 is not available in this build of ripgrep",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 rust-hygiene section should reference `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_documents_e2e_selector_and_stable_wait_rules_locally() {
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(marker),
            "hover-card check2 should keep e2e selector stability rule `{marker}`",
        );
    }
}

#[test]
fn hover_card_e2e_selector_contract_uses_semantic_markers_and_settled_waits_locally() {
    let e2e_source = load_source("../../e2e/tests/docs_app_hover_card_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for marker in [
        "page.goto(\"/#/components/hover-card\")",
        "body:not(:has(#boot))",
        "waitForWasmReady(page)",
        "[data-component=\"hover-card\"]",
        "[data-slot=\"hover-card-e2e-controls\"]",
        "[data-slot=\"hover-card-e2e-open\"]",
        "[data-slot=\"hover-card-e2e-close\"]",
        "[data-slot=\"hover-card-e2e-canvas\"]",
        "[data-slot=\"hover-card-e2e-trigger\"]",
        "[data-slot=\"hover-card\"]",
        "[data-slot=\"hover-card-panel\"][id=\"docs-hover-card-interactive\"]",
        "toHaveAttribute(\"data-open-mode\", \"controlled\")",
        "toHaveAttribute(\"data-open-value-source\", \"controlled\")",
        "toHaveAttribute(\"data-open-intent-source\", \"interaction\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(marker),
            "hover-card e2e selector/stable-wait contract should include `{marker}`",
        );
    }

    for marker in [
        "data-slot=\"hover-card-e2e-controls\"",
        "data-slot=\"hover-card-e2e-open\"",
        "data-slot=\"hover-card-e2e-close\"",
        "data-slot=\"hover-card-e2e-canvas\"",
        "data-slot=\"hover-card-e2e-trigger\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "hover-card docs source should keep e2e semantic anchor `{marker}`",
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "setTimeout(",
        "sleep(",
        "getByText(",
        "toHaveScreenshot(",
        "toMatchSnapshot(",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "hover-card e2e contract should avoid flaky/snapshot selector token `{forbidden}`",
        );
    }
}

#[test]
fn hover_card_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths_locally() {
    let e2e_source = load_source("../../e2e/tests/docs_app_hover_card_contract.spec.mjs");

    for marker in [
        "async function expectHoverCardReady(root, panel)",
        "async function expectHoverCardSettledClosed(root, panel)",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"role\", \"tooltip\")",
        "toHaveAttribute(\"data-state\", \"panel\")",
        "await closeButton.click();",
        "await panel.press(\"Escape\");",
        "await expectHoverCardSettledClosed(interactiveRoot, panel);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "hover-card e2e ready/settled contract should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_e2e_check_script_covers_selector_and_settled_wait_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-hover-card.sh");

    for marker in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
    ] {
        assert!(
            script_source.contains(marker),
            "hover-card e2e check script should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_e2e_selector_stability_item_complete_locally() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "hover-card check2 should mark e2e selector stability item complete",
    );

    for marker in [
        "e2e/tests/docs_app_hover_card_contract.spec.mjs",
        "apps/docs-app/src/pages/components/pages/overlays_hover_card.rs",
        "components/hover-card/test/semantics.rs::hover_card_check2_documents_e2e_selector_and_stable_wait_rules_locally",
        "components/hover-card/test/semantics.rs::hover_card_e2e_selector_contract_uses_semantic_markers_and_settled_waits_locally",
        "components/hover-card/test/semantics.rs::hover_card_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths_locally",
        "components/hover-card/test/semantics.rs::hover_card_e2e_check_script_covers_selector_and_settled_wait_contract_locally",
        "components/hover-card/test/hover_card_semantics.rs::hover_card_e2e_selector_contract_uses_semantic_markers_and_settled_waits",
        "scripts/check-ui-components-e2e-hover-card.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "hover-card check2 e2e selector stability section should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_check2_documents_e2e_repeatable_key_flow_rules_locally() {
    let check2_source = load_source("check2.md");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(marker),
            "hover-card check2 should keep replayable e2e critical-flow rule `{marker}`",
        );
    }
}

#[test]
fn hover_card_e2e_key_flow_is_repeatable_and_failure_points_are_semantic_locally() {
    let e2e_source = load_source("../../e2e/tests/docs_app_hover_card_contract.spec.mjs");

    for marker in [
        "docs-app hover-card key flow is repeatable and failure points are semantic",
        "for (const cycle of [1, 2]) {",
        "hover-card key flow cycle ${cycle}",
        "await openButton.focus();",
        "await expect(openButton).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expectHoverCardReady(interactiveRoot, panel);",
        "toHaveAttribute(\"data-open-intent-source\", \"interaction\")",
        "await page.keyboard.press(\"Escape\");",
        "await expectHoverCardSettledClosed(interactiveRoot, panel);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "hover-card replayable e2e flow should include semantic breakpoint marker `{marker}`",
        );
    }
}

#[test]
fn hover_card_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints_locally() {
    let e2e_source = load_source("../../e2e/tests/docs_app_hover_card_contract.spec.mjs");

    for marker in [
        "docs-app hover-card high-risk paths cover focus keyboard and settled semantic breakpoints",
        "await triggerButton.hover();",
        "await triggerButton.focus();",
        "await expect(triggerButton).toBeFocused();",
        "await expectHoverCardReady(interactiveRoot, panel);",
        "await page.keyboard.press(\"Escape\");",
        "await closeButton.click();",
        "await expectHoverCardSettledClosed(interactiveRoot, panel);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "hover-card high-risk e2e path should include semantic breakpoint marker `{marker}`",
        );
    }
}

#[test]
fn hover_card_e2e_check_script_covers_replayable_flow_and_high_risk_path_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-hover-card.sh");

    for marker in [
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "hover-card e2e check script should include replay/high-risk marker `{marker}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_replayable_e2e_critical_flow_item_complete_locally() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "hover-card check2 should mark replayable e2e critical-flow item complete",
    );

    for marker in [
        "docs-app hover-card key flow is repeatable and failure points are semantic",
        "for (const cycle of [1, 2])",
        "await expect(openButton).toBeFocused()",
        "await expectHoverCardReady(interactiveRoot, panel)",
        "await expectHoverCardSettledClosed(interactiveRoot, panel)",
        "hover_card_e2e_key_flow_is_repeatable_and_failure_points_are_semantic_locally",
        "hover_card_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints_locally",
        "scripts/check-ui-components-e2e-hover-card.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "hover-card check2 replayable e2e critical-flow section should include `{marker}`",
        );
    }
}

#[test]
fn hover_card_check2_documents_docs_sync_and_state_matrix_rules_locally() {
    let check2_source = load_source("check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "hover-card check2 should keep docs-sync/state-matrix rule `{required}`",
        );
    }
}

#[test]
fn hover_card_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults_locally() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let check2_source = load_source("check2.md");

    for needle in [
        "pub const DEFAULT_OPEN_DELAY_MS: u64 = hover_card_state::DEFAULT_OPEN_DELAY_MS;",
        "pub const DEFAULT_CLOSE_DELAY_MS: u64 = hover_card_state::DEFAULT_CLOSE_DELAY_MS;",
        "pub struct DelayStateInput {",
        "pub open_delay_ms: Option<u64>,",
        "pub close_delay_ms: Option<u64>,",
        "let open_delay_ms = input.open_delay_ms.unwrap_or(DEFAULT_OPEN_DELAY_MS);",
        "let close_delay_ms = input.close_delay_ms.unwrap_or(DEFAULT_CLOSE_DELAY_MS);",
        "pub fn resolve_is_disabled(is_disabled: Option<bool>, disabled: Option<bool>) -> bool {",
        "is_disabled.or(disabled).unwrap_or(false)",
        "pub struct OpenStateInput {",
        "pub is_open: Option<Signal<bool>>",
        "pub default_open: Option<bool>",
        "pub on_open_change: Option<Callback<bool>>",
        "let open = input.is_open.or(input.open);",
    ] {
        assert!(
            logic_source.contains(needle),
            "hover-card API/default contract should keep marker `{needle}` for docs sync",
        );
    }

    for needle in [
        "#[prop(optional, into)] is_disabled: Option<bool>,",
        "#[prop(optional)] is_open: Option<Signal<bool>>,",
        "#[prop(optional)] default_open: Option<bool>,",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>,",
        "#[prop(optional)] open_delay_ms: Option<u64>,",
        "#[prop(optional)] close_delay_ms: Option<u64>,",
    ] {
        assert!(
            view_source.contains(needle),
            "hover-card view props should keep API marker `{needle}` for docs sync",
        );
    }

    for needle in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"hover-card-state-matrix\"",
        "data-slot=\"hover-card-controlled-uncontrolled\"",
        "open_delay_ms=220",
        "close_delay_ms=260",
        "is_disabled=true",
        "is_open=compare_controlled_open",
        "default_open=true",
        "on_open_change=on_compare_controlled_open_change",
        "on_open_change=on_compare_uncontrolled_open_change",
        "data-slot=\"hover-card-defaults-contract\"",
        "components/hover-card/src/logic.rs",
        "DEFAULT_OPEN_DELAY_MS (140)",
        "DEFAULT_CLOSE_DELAY_MS (180)",
        "resolve_is_disabled",
    ] {
        assert!(
            docs_source.contains(needle),
            "hover-card docs should keep synced example/matrix/default marker `{needle}`",
        );
    }

    for needle in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/overlays_hover_card.rs::hover_card",
        "hover_card_check2_documents_docs_sync_and_state_matrix_rules_locally",
        "hover_card_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults_locally",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(needle),
            "components/hover-card/check2.md should keep docs-sync evidence marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_dx_check_script_covers_docs_sync_and_state_matrix_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: hover-card docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include docs-sync/state-matrix marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_docs_sync_and_state_matrix_item_complete_locally() {
    let source = load_source("check2.md");

    assert!(
        source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "hover-card check2 should mark docs-sync/state-matrix checklist item complete",
    );

    for needle in [
        "apps/docs-app/src/pages/components/pages/overlays_hover_card.rs::hover_card",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"hover-card-defaults-contract\"",
        "DEFAULT_OPEN_DELAY_MS",
        "DEFAULT_CLOSE_DELAY_MS",
        "hover_card_check2_documents_docs_sync_and_state_matrix_rules_locally",
        "hover_card_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults_locally",
        "hover_card_dx_check_script_covers_docs_sync_and_state_matrix_contract_locally",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(needle),
            "hover-card check2 docs-sync/state-matrix section should reference `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_documents_documentation_as_product_rules_locally() {
    let check2_source = load_source("check2.md");

    for needle in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 documentation-as-product section should include `{needle}`",
        );
    }
}

#[test]
fn hover_card_documentation_entry_exists_with_beginner_first_progression_locally() {
    let readme_source = load_source("src/README.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_hover_card.rs");

    for needle in [
        "# HoverCard",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径：先用 `content + children`",
        "进阶控制：按需启用 `is_open + default_open + on_open_change`",
        "### Controlled Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(needle),
            "hover-card README should include beginner-first marker `{needle}`",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("hover-card README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("hover-card README should include beginner-first progression section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("hover-card README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Controlled Example（高级入口）")
        .expect("hover-card README should include controlled advanced section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "hover-card README should keep beginner-first progression order (hello -> beginner -> common -> advanced)",
    );

    for needle in [
        "component_doc!(\"HoverCard\", \"hover-card\", \"Overlays\", overlays::hover_card),",
        "pub(super) fn hover_card() -> AnyView",
        "title=\"HoverCard\"",
        "slug=\"hover-card\"",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            pages_source.contains(needle) || docs_source.contains(needle),
            "hover-card docs entry should include `{needle}`",
        );
    }
}

#[test]
fn hover_card_dx_check_script_covers_documentation_as_product_contract_locally() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for needle in [
        "echo \"[dx] contract: hover-card documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test hover_card_semantics --no-default-features --features component-hover_card,inject-css hover_card_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(needle),
            "DX check script should include documentation-as-product marker `{needle}`",
        );
    }
}

#[test]
fn hover_card_check2_marks_documentation_as_product_item_complete_locally() {
    let check2_source = load_source("check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "hover-card check2 should mark documentation-as-product item complete",
    );

    for needle in [
        "components/hover-card/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "hover_card_check2_documents_documentation_as_product_rules_locally",
        "hover_card_documentation_entry_exists_with_beginner_first_progression_locally",
        "hover_card_dx_check_script_covers_documentation_as_product_contract_locally",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "hover-card check2 documentation-as-product section should retain marker `{needle}`",
        );
    }
}
