use ui_test_support::source_contract;

fn load_source(path: &str) -> &'static str {
    match path {
        "mod" => include_str!("../src/mod.rs"),
        "logic" => include_str!("../src/logic.rs"),
        "view" => include_str!("../src/view.rs"),
        "styles" => include_str!("../src/styles.rs"),
        "check2" => include_str!("../check2.md"),
        "semantics_self" => include_str!("semantics.rs"),
        "ui_components_cargo" => include_str!("../../../crates/ui/Cargo.toml"),
        "ui_components_css" => include_str!("../../../crates/ui/src/css.rs"),
        "ui_components_lib" => include_str!("../../../crates/ui/src/lib.rs"),
        "ui_root" => include_str!("../../../crates/ui/src/root.rs"),
        "active_highlight" => {
            include_str!("../../../crates/ui-visual-primitive/src/active_highlight.rs")
        }
        "headless_a11y" => include_str!("../../../crates/ui-headless/src/a11y.rs"),
        "headless_presence" => include_str!("../../../crates/ui-headless/src/presence.rs"),
        "headless_controllable_state" => {
            include_str!("../../../crates/ui-headless/src/controllable_state.rs")
        }
        "check_script" => include_str!("../../../scripts/check.sh"),
        "rust_hygiene_script" => include_str!("../../../scripts/check-rust-hygiene.sh"),
        "keyboard_cargo" => include_str!("../Cargo.toml"),
        "web_demo_cargo" => include_str!("../../../apps/web-demo/Cargo.toml"),
        "ui_headless_lib" => include_str!("../../../crates/ui-headless/src/lib.rs"),
        "ui_motion_lib" => include_str!("../../../crates/ui-motion/src/lib.rs"),
        "ui_theme_css" => source_contract::source_from_file_relative(
            file!(),
            "../../../crates/ui-theme/src/css.rs",
        ),
        "primitive_keyboard" => include_str!("../../../crates/ui-state-primitives/src/keyboard.rs"),
        "headless_keyboard" => include_str!("../../../crates/ui-headless/src/keyboard.rs"),
        "logic_test" => include_str!("logic.rs"),
        "component_readme" => include_str!("../src/README.md"),
        "component_manifest" => include_str!("../src/Component.toml"),
        "component_rbi" => include_str!("../src/keyboard.rbi"),
        "docs_display_extra" => source_contract::source_from_file_relative(
            file!(),
            "../../../apps/docs-app/src/pages/components/pages/display_extra.rs",
        ),
        "docs_playground" => include_str!("../../../apps/docs-app/src/playground.rs"),
        "docs_pages_index" => include_str!("../../../apps/docs-app/src/pages/components/pages.rs"),
        "heroui_strategy" => include_str!("../../../docs/spec/heroui-parameter-design-strategy.md"),
        "e2e_keyboard_contract" => {
            include_str!("../../../e2e/tests/docs_app_keyboard_contract.spec.mjs")
        }
        "legacy_semantics" => {
            include_str!("../../../components/keyboard/test/keyboard_semantics.rs")
        }
        "protocol" => include_str!("../src/protocol.rs"),
        _ => panic!("unsupported source path: {path}"),
    }
}

fn snapshot_assertion_markers() -> [&'static str; 3] {
    [
        concat!("assert_", "snapshot"),
        concat!("to_match_", "snapshot"),
        concat!("insta", "::"),
    ]
}

fn has_hex_color_literal(source: &str) -> bool {
    let bytes = source.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'#' {
            let mut j = i + 1;
            let mut digits = 0;
            while j < bytes.len() && bytes[j].is_ascii_hexdigit() && digits < 8 {
                digits += 1;
                j += 1;
            }
            if matches!(digits, 3 | 4 | 6 | 8) {
                return true;
            }
        }
        i += 1;
    }
    false
}

#[test]
fn keyboard_semantics_tests_are_migrated_to_component_directory() {
    let module = load_source("mod");
    let legacy = load_source("legacy_semantics");

    assert!(
        module.contains("#[path = \"../test/semantics.rs\"]")
            && module.contains("mod semantics_tests;"),
        "keyboard should wire local semantics suite from `components/keyboard/src/mod.rs`."
    );
    assert!(
        legacy.contains("include!(\"../../../components/keyboard/test/semantics.rs\");")
            || legacy.contains("include!(\"../../components/keyboard/test/semantics.rs\");")
            || legacy.contains("include!(\"semantics.rs\");"),
        "legacy semantics entry should bridge to `components/keyboard/test/semantics.rs`."
    );
}

#[test]
fn keyboard_component_keeps_ui_components_layering_boundaries() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{DEFAULT_ARIA_LABEL, KeyboardTone};",
        "pub use view::Keyboard;",
    ] {
        assert!(
            module.contains(required),
            "keyboard module boundary should contain `{required}`."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;"] {
        assert!(
            !module.contains(forbidden),
            "keyboard internals should stay private (`{forbidden}`)."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "HtmlElement",
        "Element",
        "NodeRef",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !view.contains(forbidden)
                && !styles.contains(forbidden),
            "keyboard component surface should stay DOM-platform agnostic (`{forbidden}`)."
        );
    }
}

#[test]
fn keyboard_component_consumes_primitives_and_headless_contracts() {
    let logic = load_source("logic");
    let view = load_source("view");

    assert!(
        logic.contains("pub use ui_state_primitives::keyboard::{"),
        "keyboard logic should consume state primitives from `ui-state-primitives`."
    );

    for forbidden in [
        "pub struct KeyboardStateInput",
        "pub struct KeyboardState",
        "pub fn resolve_state(input: KeyboardStateInput)",
    ] {
        assert!(
            !logic.contains(forbidden),
            "keyboard logic should not reimplement primitive state contract (`{forbidden}`)."
        );
    }

    for required in [
        "use ui_headless::{A11yDirection, KeyboardOptions, use_keyboard};",
        "logic::normalize_root_state(KeyboardRootInput {",
        "use_keyboard(KeyboardOptions {",
        "data-slot=move || semantics.get().attrs.data_slot",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "aria-label=move || semantics.get().attrs.aria_label",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should mount typed headless semantic contract (`{required}`)."
        );
    }
}

#[test]
fn keyboard_api_naming_contract_uses_is_prefix_without_alias_drift() {
    let view = load_source("view");

    assert!(
        view.contains("#[prop(optional, into)] is_compact: Option<bool>,"),
        "keyboard boolean prop should use `is_*` naming."
    );

    for forbidden in ["#[prop(optional)] compact: bool,", "compact: bool,"] {
        assert!(
            !view.contains(forbidden),
            "keyboard API should not expose legacy/non-prefixed boolean alias (`{forbidden}`)."
        );
    }
}

#[test]
fn keyboard_defaults_are_normalized_in_logic_only() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub struct KeyboardRootInput",
        "pub struct KeyboardRootState",
        "pub fn normalize_root_state(input: KeyboardRootInput) -> KeyboardRootState",
        "let tone = input.tone.unwrap_or_default();",
        "let is_compact = input.is_compact.unwrap_or(false);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let class_name = normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic.contains(required),
            "keyboard logic should centralize default/priority normalization (`{required}`)."
        );
    }

    for required in [
        "logic::normalize_root_state(KeyboardRootInput {",
        "class=move || root_state.get().class_name.clone()",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should consume normalized root state only (`{required}`)."
        );
    }

    for forbidden in [
        "logic::normalize_aria_label(",
        "logic::normalize_optional_text(",
        "logic::resolve_state(",
        "unwrap_or(",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard view must not do fallback/default normalization (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。"),
        "keyboard checklist should mark default-source contract complete."
    );
}

#[test]
fn keyboard_state_normalization_is_centralized_in_logic_kernel() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for required in [
        "pub struct KeyboardRootInput",
        "pub struct KeyboardRootState",
        "pub fn normalize_root_state(input: KeyboardRootInput) -> KeyboardRootState",
        "let state = resolve_state(KeyboardStateInput {",
    ] {
        assert!(
            logic.contains(required),
            "keyboard state kernel should be typed and centralized in logic (`{required}`)."
        );
    }

    for required in [
        "logic::normalize_root_state(KeyboardRootInput {",
        "state: root_state.get().state,",
        "data-state=move || semantics.get().attrs.data_state",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-class-source=move || semantics.get().attrs.data_class_source",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should only mount derived semantic state markers (`{required}`)."
        );
    }

    for forbidden in [
        "KeyboardStateInput {",
        "logic::resolve_state(",
        "logic::normalize_aria_label(",
        "logic::normalize_optional_text(",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard view must not rebuild state machine rules (`{forbidden}`)."
        );
    }

    for required in [
        ".ui-keyboard[data-tone=\"default\"]",
        ".ui-keyboard[data-tone=\"muted\"]",
        ".ui-keyboard[data-compact=\"true\"]",
        ".ui-keyboard[data-custom-class=\"true\"]",
    ] {
        assert!(
            styles.contains(required),
            "keyboard styles should consume stable semantic markers only (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。"),
        "keyboard checklist should mark centralized state normalization complete."
    );
}

#[test]
fn keyboard_discrete_state_axes_are_type_constrained() {
    let primitive = load_source("primitive_keyboard");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub enum KeyboardTone",
        "Default,",
        "Muted,",
        "pub tone: KeyboardTone,",
    ] {
        assert!(
            primitive.contains(required),
            "keyboard discrete state should be modeled as enum in primitives (`{required}`)."
        );
    }

    assert!(
        view.contains("#[prop(optional)] tone: Option<KeyboardTone>,"),
        "keyboard tone prop should accept typed enum state."
    );

    for forbidden in [
        "tone: Option<String>",
        "tone: String",
        "tone: &'static str",
        "is_muted",
        "is_default",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard must not encode mutually exclusive states with strings/multi-bool flags (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。"),
        "keyboard checklist should mark discrete state type constraints complete."
    );
}

#[test]
fn keyboard_machine_readable_state_contract_is_type_constrained_and_traceable() {
    let primitive = load_source("primitive_keyboard");
    let headless = load_source("headless_keyboard");
    let logic = load_source("logic");
    let view = load_source("view");
    let logic_test = load_source("logic_test");
    let check2 = load_source("check2");

    for required in [
        "pub enum KeyboardTone",
        "pub struct KeyboardStateInput",
        "pub struct KeyboardState",
        "KeyboardTone::Default => \"default\"",
        "KeyboardTone::Muted => \"muted\"",
    ] {
        assert!(
            primitive.contains(required),
            "keyboard primitive type model should keep key input/state space machine-readable (`{required}`)."
        );
    }

    for required in [
        "pub fn normalize_root_state(input: KeyboardRootInput) -> KeyboardRootState",
        "let state = resolve_state(KeyboardStateInput {",
        "normalize_aria_label(input.aria_label)",
        "normalize_optional_text(input.class_name)",
    ] {
        assert!(
            logic.contains(required),
            "keyboard logic should centralize invalid/default input rectification (`{required}`)."
        );
    }

    for required in [
        "#[prop(optional)] tone: Option<KeyboardTone>,",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-class-source=move || semantics.get().attrs.data_class_source",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should publish stable semantic markers for automation (`{required}`)."
        );
    }

    for required in [
        "data_tone: options.state.tone_attr",
        "data_state: options.state.data_state_attr",
        "data_aria_source: options.state.aria_source_attr",
        "data_class_source: options.state.class_source_attr",
    ] {
        assert!(
            headless.contains(required),
            "headless mapping should keep marker derivation typed and centralized (`{required}`)."
        );
    }

    for forbidden in [
        "tone: Option<String>",
        "tone: String",
        "is_muted",
        "is_default",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not regress into string/bool-explosion state contracts (`{forbidden}`)."
        );
    }

    for required in [
        "fn keyboard_tone_contract_is_stable()",
        "fn resolve_state_tracks_tone_compact_and_sources()",
        "fn normalize_root_state_centralizes_default_priority_and_sources()",
    ] {
        assert!(
            logic_test.contains(required),
            "logic test suite should pinpoint state-contract regressions (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。"),
        "keyboard checklist should mark machine-readable typed state contract complete."
    );
    assert!(
        check2.contains("`Keyboard` 的离散输入由 `KeyboardTone` 枚举与 `KeyboardStateInput` 新类型承载；`logic.rs` 统一通过 `normalize_root_state -> resolve_state` 归一无效输入；`view.rs` 稳定输出 `data-tone/data-state/data-aria-source/data-class-source` 封闭语义域，`components/keyboard/test/logic.rs` 与 `components/keyboard/test/semantics.rs` 可直接定位类型或标记契约回归。"),
        "keyboard checklist should include concrete typed-marker evidence and test feedback loop."
    );
}

#[test]
fn keyboard_focus_stack_gc_is_not_applicable_and_overlay_focus_state_is_absent() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "NodeRef",
        "document.body",
        "FocusManager",
        "focus_stack",
        "FallbackTo",
        "fallback_to",
        "overlay",
        "Overlay",
        "restore_focus",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement overlay focus stack/restore path (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。"),
        "keyboard checklist should mark focus-stack contract complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 为单节点语义展示组件，不是层叠 `Overlay`，不存在焦点恢复栈、`NodeRef` 恢复目标或回落 `document.body` 的焦点管理路径。"),
        "keyboard checklist should include explicit N/A rationale for focus-stack contract."
    );
}

#[test]
fn keyboard_escape_hatches_foreign_zone_is_not_applicable_and_third_party_instance_is_absent() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "ECharts",
        "echarts",
        "Leaflet",
        "leaflet",
        "Mapbox",
        "mapbox",
        "google.maps",
        "ForeignZone",
        "Foreign Zone",
        "YieldControl",
        "CleanupForeign",
        "third_party_instance",
        "imperative_instance",
        "NodeRef",
        "web_sys::HtmlCanvasElement",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not embed imperative third-party escape-hatch integration (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。"),
        "keyboard checklist should mark escape-hatches contract complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 不集成 ECharts/Map 等命令式第三方库，无 `Foreign Zone`/`YieldControl`/`CleanupForeign` 接入路径；公共 API 也未暴露任何第三方实例句柄。"),
        "keyboard checklist should include explicit N/A rationale for escape-hatches contract."
    );
}

#[test]
fn keyboard_hydration_discontinuity_is_not_applicable_and_nondeterministic_init_is_absent() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "now()",
        "Instant::now",
        "SystemTime::now",
        "UNIX_EPOCH",
        "rand::",
        "thread_rng",
        "random()",
        "Uuid::",
        "uuid::",
        "IdProvider",
        "UiIdProvider",
        "create_unique_id",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not introduce non-deterministic init paths that break SSR/hydration parity (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。"),
        "keyboard checklist should mark hydration-discontinuity contract complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 不生成运行时 ID、无 hydration 期间的随机初始化路径，也不依赖 `now()`/UUID；该组件当前不需要 `IdProvider` 接入。"),
        "keyboard checklist should include explicit N/A rationale for hydration-discontinuity contract."
    );
}

#[test]
fn keyboard_cross_platform_contract_is_non_wasm_safe_and_ci_compile_only_is_wired() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let keyboard_cargo = load_source("keyboard_cargo");
    let check_script = load_source("check_script");
    let check2 = load_source("check2");

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "js_sys::",
        "window()",
        "document()",
        "navigator",
        "HtmlElement",
        "EventTarget",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard non-wasm path must stay browser-API free (`{forbidden}`)."
        );
    }

    assert!(
        keyboard_cargo.contains(
            "leptos = { version = \"0.8.15\", default-features = false, features = [\"csr\"] }"
        ),
        "keyboard crate should keep platform capability explicitly feature-managed in Cargo."
    );
    assert!(
        !keyboard_cargo.contains("web-sys"),
        "keyboard component crate should not directly depend on browser bindings."
    );

    for required in [
        "echo \"[check] ssr (compile-only)\"",
        "cargo check -p ui-headless --no-default-features --features ssr",
        "echo \"[check] wasm\"",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            check_script.contains(required),
            "repository gate should keep explicit ssr/wasm compile-only checks (`{required}`)."
        );
    }

    assert!(
        check2
            .contains("- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。"),
        "keyboard checklist should mark cross-platform contract complete."
    );
    assert!(
        check2.contains("当前环境执行 `cargo check -p ui-keyboard`、`cargo check -p ui-headless --no-default-features --features ssr` 与 `cargo check -p ui-keyboard --target wasm32-unknown-unknown` 均受 `Invalid cross-device link (os error 18)` 限制，compile-only 结果以 CI 门禁为准。"),
        "keyboard checklist should document local compile-only limitation and CI fallback evidence."
    );
}

#[test]
fn keyboard_headless_web_ssr_feature_mutex_contract_is_preserved() {
    let ui_headless_lib = load_source("ui_headless_lib");
    let keyboard_cargo = load_source("keyboard_cargo");
    let check_script = load_source("check_script");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            ui_headless_lib.contains(required),
            "ui-headless should keep compile-time mutex guard for web/ssr features (`{required}`)."
        );
    }

    assert!(
        keyboard_cargo.contains("ui-headless = { path = \"../../crates/ui-headless\" }"),
        "keyboard should consume ui-headless without overriding its feature-mutex contract."
    );

    for required in [
        "cargo check -p ui-headless --no-default-features --features ssr",
        "cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web",
    ] {
        assert!(
            check_script.contains(required),
            "repository gates should compile-check both headless feature paths (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。"),
        "keyboard checklist should mark headless web/ssr mutex item complete."
    );
    assert!(
        check2.contains("`crates/ui-headless/src/lib.rs` 已声明 `#[cfg(all(feature = \"web\", feature = \"ssr\"))] compile_error!(...)` 互斥保护；`Keyboard` 仅通过 `components/keyboard/Cargo.toml` 依赖 `ui-headless`，未覆写其 feature 互斥策略。仓库 `scripts/check.sh` 已分别覆盖 `ssr` 与 `web(wasm)` compile-only 路径。当前环境执行 `cargo check -p ui-headless --no-default-features --features web,ssr` 受 `Invalid cross-device link (os error 18)` 限制，互斥回归以该 `compile_error!` 源码断言与 CI 编译门禁共同兜底。"),
        "keyboard checklist should include concrete mutex evidence and local limitation note."
    );
}

#[test]
fn keyboard_ui_motion_non_wasm_noop_stub_contract_is_preserved() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let keyboard_cargo = load_source("keyboard_cargo");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop()",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should preserve non-wasm no-op/stub contract for SSR/tooling (`{required}`)."
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "attach_motion",
        "spring(",
        "keyframe",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not assume motion runtime handles in non-wasm paths (`{forbidden}`)."
        );
    }

    assert!(
        !keyboard_cargo.contains("ui-motion"),
        "keyboard should not pull ui-motion dependency for static non-animated rendering path."
    );

    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/keyboard/src/motion.rs");
    assert!(
        !motion_path.exists(),
        "keyboard should not add motion.rs when component has no animation state axis."
    );

    assert!(
        check2.contains("- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。"),
        "keyboard checklist should mark ui-motion non-wasm no-op/stub item complete."
    );
    assert!(
        check2.contains("`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = \"wasm32\"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(...)` no-op/stub，并带有 non-wasm 预测性测试。`Keyboard` 无 `motion.rs` 且未依赖 `ui-motion`，不存在动画句柄假设与 non-wasm panic 路径，测试/文档/静态分析不会被 motion 依赖阻塞。"),
        "keyboard checklist should include concrete no-op/stub evidence and keyboard non-motion boundary."
    );
}

#[test]
fn keyboard_reduced_motion_ssr_wasm_branches_are_not_applicable_and_semantics_stay_unified() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let headless = load_source("headless_keyboard");
    let check2 = load_source("check2");

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "prefers_reduced_motion",
        "attach_motion",
        "spring(",
        "keyframe",
        "cfg(target_arch = \"wasm32\")",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not split behavior via component-local motion/target branches (`{forbidden}`)."
        );
    }

    for required in [
        "data-slot=move || semantics.get().attrs.data_slot",
        "data-state=move || semantics.get().attrs.data_state",
        "aria-label=move || semantics.get().attrs.aria_label",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
    ] {
        assert!(
            view.contains(required),
            "keyboard should keep a single semantic contract output for SSR/wasm paths (`{required}`)."
        );
    }

    for required in [
        "let locale = locale_attrs(options.lang, options.dir);",
        "data_state: options.state.data_state_attr",
        "data_aria_source: options.state.aria_source_attr",
        "data_class_source: options.state.class_source_attr",
    ] {
        assert!(
            headless.contains(required),
            "headless keyboard contract should provide platform-agnostic semantic mapping (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。"),
        "keyboard checklist should mark reduced-motion/SSR/wasm branch item complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 无组件级动效与 `motion.rs`，不存在 `reduced-motion` 动画降级分支；组件在 SSR/wasm 上均输出同一 `<kbd>` 语义契约（`aria-label/lang/dir/data-*`），不依赖 wasm 专属增强路径，因此不存在跨分支语义分裂与 hydration 首帧错位来源。"),
        "keyboard checklist should include explicit N/A rationale for reduced-motion/SSR/wasm branch item."
    );
}

#[test]
fn keyboard_performance_governance_uses_component_equivalent_evidence_without_hot_paths() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    assert_eq!(
        view.matches("<kbd").count(),
        1,
        "keyboard view should keep a single-node render surface for predictable baseline cost."
    );
    assert_eq!(
        view.matches("Memo::new").count(),
        2,
        "keyboard should keep deterministic derived-state-only memoization footprint."
    );

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:pointer",
        "on:click",
        "requestAnimationFrame",
        "ResizeObserver",
        "IntersectionObserver",
        "spawn_local",
        "tokio::spawn",
        "set_interval",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not introduce hot-path update loops or observer floods (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains(
            "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。"
        ),
        "keyboard checklist should mark performance governance item complete."
    );
    assert!(
        check2.contains("N/A：该项以仓库级性能基线治理为主（含 `Button`/`Input` 的 `render_count` 预算）；`Keyboard` 非关键交互组件且无事件驱动更新循环。当前组件等价证据为：`view.rs` 仅渲染单 `<kbd>` 并通过两个 `Memo` 做确定性派生，无环境订阅、无逐帧动效、无异步任务路径；渲染计数自动化由仓库测试框架统一补齐时纳入。"),
        "keyboard checklist should include explicit N/A scope and equivalent profiling evidence."
    );
}

#[test]
fn keyboard_view_macro_complexity_stays_flat_and_single_node() {
    let view = load_source("view");
    let check2 = load_source("check2");

    assert_eq!(
        view.matches("view! {").count(),
        1,
        "keyboard should keep a single compact view! block."
    );
    assert_eq!(
        view.matches("<kbd").count(),
        1,
        "keyboard should keep a single semantic root node instead of deep nested layout trees."
    );
    assert_eq!(
        view.matches("</kbd>").count(),
        1,
        "keyboard root node should be closed once without duplicated repeated fragments."
    );

    for forbidden in [
        "<div",
        "<section",
        "<header",
        "<footer",
        "<ul",
        "<li",
        "fn render_",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard view should not regress into multi-layer container nesting/repeated macro blocks (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。"),
        "keyboard checklist should mark view-macro complexity item complete."
    );
    assert!(
        check2.contains("`Keyboard` 的 `view.rs` 仅含一个 `view!`，渲染单 `<kbd>` 结构，无多层容器嵌套与重复片段；当前宏展开体量稳定且无额外子块拆分需求。"),
        "keyboard checklist should include concrete macro-complexity evidence."
    );
}

#[test]
fn keyboard_prefers_functional_split_over_local_component_promotion_noise() {
    let view = load_source("view");
    let check2 = load_source("check2");

    assert_eq!(
        view.matches("#[component]").count(),
        1,
        "keyboard should keep exactly one component boundary for the public API."
    );
    assert!(
        view.contains("pub fn Keyboard("),
        "keyboard should expose a single semantic entry component."
    );

    for forbidden in [
        "#[component]\nfn ",
        "fn render_",
        "fn slot_",
        "fn section_",
        "pub fn KeyboardItem(",
        "pub fn KeyboardPart(",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard should not promote local UI fragments into extra component/function noise (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。"),
        "keyboard checklist should mark functional-split preference item complete."
    );
    assert!(
        check2.contains("`Keyboard` 当前仅保留一个公共入口 `#[component] fn Keyboard`，`view.rs` 无局部子组件升格与无意义片段抽象；在无重复静态片段与独立 props 需求前，保持单函数实现可读性更高、抽象噪音更低。"),
        "keyboard checklist should include concrete anti-noise component-splitting evidence."
    );
}

#[test]
fn keyboard_static_fragments_are_not_duplicated_and_static_assets_are_constantized() {
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for forbidden in [
        "<svg",
        "</svg>",
        "<footer",
        "</footer>",
        "<article",
        "</article>",
        "This is a long",
        "Lorem ipsum",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard view should not inline complex static fragments that should be templated/constantized (`{forbidden}`)."
        );
    }

    assert!(
        styles.contains("pub const CSS: &str = r#\""),
        "keyboard static style assets should stay centralized in styles.rs constant CSS."
    );
    assert!(
        view.contains("{children()}"),
        "keyboard content should stay externalized via children instead of hardcoded static text blocks."
    );
    assert!(
        view.contains("aria-label=move || semantics.get().attrs.aria_label"),
        "accessibility semantics should stay mounted after static-fragment simplification."
    );

    assert!(
        check2.contains("- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。"),
        "keyboard checklist should mark static-fragment-constantization item complete."
    );
    assert!(
        check2.contains("`Keyboard` 的 `view.rs` 不包含复杂 SVG/页脚/长静态文案，仅渲染语义 `<kbd>` 与外部 `children`；可常量化的静态资源集中在 `styles.rs` 的 `pub const CSS`，变更路径单一且不散落多处 `view!`。"),
        "keyboard checklist should include concrete static-asset centralization evidence."
    );
}

#[test]
fn keyboard_inner_html_constraint_is_enforced_with_no_injection_surface() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "inner_html",
        "innerHTML",
        "set_inner_html",
        "dangerously_set_inner_html",
        "html=move ||",
        "format!(\"<",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not introduce raw HTML injection surfaces (`{forbidden}`)."
        );
    }

    assert!(
        view.contains("{children()}"),
        "keyboard should render user-visible content through typed children, not raw HTML strings."
    );
    assert!(
        view.contains("aria-label=move || semantics.get().attrs.aria_label"),
        "keyboard should keep semantic a11y attributes mounted without inner_html shortcuts."
    );

    assert!(
        check2.contains("- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。"),
        "keyboard checklist should mark inner_html constraint item complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 不使用 `inner_html`，`view.rs` 仅渲染 `<kbd>{children()}</kbd>` 并挂载 headless 语义属性；不存在用户输入或远端内容注入 HTML 的路径。"),
        "keyboard checklist should include explicit N/A rationale for inner_html constraints."
    );
}

#[test]
fn keyboard_wasm_debug_requirements_are_na_and_debug_features_do_not_pollute_artifacts() {
    let logic = load_source("logic");
    let view = load_source("view");
    let keyboard_cargo = load_source("keyboard_cargo");
    let ui_components_cargo = load_source("ui_components_cargo");
    let check2 = load_source("check2");

    for forbidden in [
        "TraceId",
        "tracing::",
        "instrument(",
        "debug_panel",
        "debug_overlay",
        "event_replay",
        "replay_log",
        "state_transition_log",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not embed wasm debug tracing/replay machinery in component runtime (`{forbidden}`)."
        );
    }

    assert!(
        !keyboard_cargo.contains("tracing"),
        "keyboard crate should not pull tracing dependency by default."
    );
    assert!(
        !ui_components_cargo.contains("keyboard-wasm-debug")
            && !ui_components_cargo.contains("component-keyboard-wasm-debug"),
        "ui should not expose keyboard-specific wasm debug feature in production feature matrix."
    );
    assert!(
        ui_components_cargo.contains("accordion-wasm-debug")
            && ui_components_cargo.contains("button-wasm-debug"),
        "wasm debug capabilities should remain explicit opt-in features for interactive components."
    );

    assert!(
        check2.contains("- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。"),
        "keyboard checklist should mark wasm debug requirement item complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 为静态语义展示组件，无关键交互状态机与事件回放链路；组件实现未引入 `TraceId/tracing` 调试埋点。调试能力隔离由上层 `ui` 的 `*-wasm-debug` 特性治理，且当前不存在 `keyboard-wasm-debug` 特性，默认产物不携带组件调试开关。"),
        "keyboard checklist should include explicit N/A rationale and feature-isolation evidence for wasm debug requirement."
    );
}

#[test]
fn keyboard_dx_workbench_supports_fast_css_feedback_and_keeps_runtime_surface_clean() {
    let docs = load_source("docs_display_extra");
    let readme = load_source("component_readme");
    let view = load_source("view");
    let keyboard_cargo = load_source("keyboard_cargo");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "test_css_source=keyboard_test_css_source",
        "test_source_path=\"components/keyboard/src/styles.rs\".to_string()",
        "description=\"可调 tone/key/is_compact/aria/class，并在同一面板查看 code + config + scoped css test。\"",
        "let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0));",
        "let (workbench_is_compact, set_workbench_is_compact) = signal(false);",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs workbench should provide fast-feedback DX and in-session context (`{required}`)."
        );
    }

    for required in [
        "工作台区块：`Interactive Playground (展示 / Config / Code / CSS Test)`",
        "Interactive Playground 已接入 scoped CSS 测试面板：",
        "`test_source_path`: `components/keyboard/src/styles.rs`",
    ] {
        assert!(
            readme.contains(required),
            "keyboard README should keep DX entry path discoverable (`{required}`)."
        );
    }

    for forbidden in ["debug:", "trace:", "persist_state", "local_storage"] {
        assert!(
            !view.contains(forbidden) && !keyboard_cargo.contains("tracing"),
            "keyboard runtime surface should stay clean from debug/persistence pollution (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。"),
        "keyboard checklist should mark DX workbench item complete."
    );
    assert!(
        check2.contains("`Keyboard` 在 docs-app 提供 `Interactive Playground (展示 / Config / Code / CSS Test)` 工作台，`test_css_source + test_source_path` 支持 scoped CSS 快速验证，常见样式调整无需修改组件逻辑；交互上下文由 playground signals 持续保持。`Keyboard` 为轻量展示组件，不维护复杂工作流状态，持久化状态保留按复杂交互组件可选（N/A）。"),
        "keyboard checklist should include concrete DX evidence with scoped N/A for persistence."
    );
}

#[test]
fn keyboard_state_primitives_source_boundary_is_enforced() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "pub use ui_state_primitives::keyboard::{",
        "let state = resolve_state(KeyboardStateInput {",
        "logic::normalize_root_state(KeyboardRootInput {",
    ] {
        assert!(
            logic.contains(required) || view.contains(required),
            "keyboard should consume state primitives through layered mapping (`{required}`)."
        );
    }

    for forbidden in [
        "pub struct KeyboardStateInput",
        "pub struct KeyboardState",
        "use crate::store",
        "use super::store",
        "app_store",
        "global_store",
        "RwSignal<",
        "ReadSignal<",
        "WriteSignal<",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard component must not reimplement primitives or bind business stores (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。"),
        "keyboard checklist should mark primitive-source boundary complete."
    );
}

#[test]
fn keyboard_has_no_async_interaction_protocol_and_marks_na_reason() {
    let logic = load_source("logic");
    let view = load_source("view");
    let styles = load_source("styles");
    let check2 = load_source("check2");

    for forbidden in [
        "is_loading",
        "loading",
        "error",
        "retry",
        "aria-busy",
        "use_async_action",
        "spawn",
        "Future",
        "pending",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden) && !styles.contains(forbidden),
            "keyboard should not define component-local async loading/error protocol (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。"),
        "keyboard checklist should mark async protocol item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 为静态语义展示组件，无远程请求与异步状态轴，不涉及加载/失败/重试协议。"
        ),
        "keyboard checklist should include explicit N/A rationale for async protocol item."
    );
}

#[test]
fn keyboard_api_dx_paradox_keeps_simple_path_without_state_machine_wiring() {
    let view = load_source("view");
    let docs = load_source("docs_display_extra");
    let check2 = load_source("check2");

    for required in [
        "#[prop(optional)] tone: Option<KeyboardTone>,",
        "#[prop(optional, into)] is_compact: Option<bool>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "pub fn Keyboard(",
    ] {
        assert!(
            view.contains(required),
            "keyboard API should keep high-frequency usage optional and simple (`{required}`)."
        );
    }

    for forbidden in ["#[prop()] state:", "pub state:", "ui_state_primitives::"] {
        assert!(
            !view.contains(forbidden),
            "keyboard public component API should not require internal state-machine wiring (`{forbidden}`)."
        );
    }

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "title=\"Keyboard\"",
        "slug=\"keyboard\"",
        "<Keyboard>\"⌘K\"</Keyboard>",
        "<Keyboard tone=KeyboardTone::Muted>\"⌥⇧P\"</Keyboard>",
        "<Keyboard is_compact=true>\"Ctrl+K\"</Keyboard>",
    ] {
        assert!(
            docs.contains(required),
            "docs should provide direct minimal-to-advanced API path (`{required}`)."
        );
    }

    assert!(
        check2
            .contains("- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。"),
        "keyboard checklist should mark DX paradox item complete."
    );
}

#[test]
fn keyboard_non_composite_api_uses_single_node_contract_not_parallel_arrays() {
    let view = load_source("view");
    let check2 = load_source("check2");

    assert!(
        view.contains("children: Children,"),
        "keyboard should keep a single-node children contract."
    );

    for forbidden in [
        "labels:",
        "titles:",
        "panels:",
        "items:",
        "ItemSpec",
        "KeyboardItem",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard must not expose parallel-array or Parent/Item composite API (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。"),
        "keyboard checklist should mark composite-api item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 是单节点语义组件，不属于 `Parent/Item` 组合容器；公共 API 不存在 `labels + children`、`titles + panels` 等并行数组契约。"
        ),
        "keyboard checklist should include explicit N/A rationale for composite API item."
    );
}

#[test]
fn keyboard_macro_micro_drag_state_machine_is_not_applicable_and_not_implemented() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "pointermove",
        "mousemove",
        "touchmove",
        "requestAnimationFrame",
        "raf",
        "mod motion;",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement drag macro/micro state machine paths (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。"),
        "keyboard checklist should mark macro/micro dual state machine item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 为静态语义组件，不存在拖拽交互与逐帧物理循环，也无 `Action::DragEnd` 回流路径。"
        ),
        "keyboard checklist should include explicit N/A rationale for macro/micro dual state machine item."
    );
}

#[test]
fn keyboard_two_pass_geometry_rendering_is_not_applicable_and_absent() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "offsetWidth",
        "offsetHeight",
        "clientWidth",
        "clientHeight",
        "ResizeObserver",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement geometry two-pass measurement loops (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。"),
        "keyboard checklist should mark two-pass geometry rendering item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 不依赖几何测量，不读取 DOM 尺寸/位置信息，不存在 `Intent -> Measure -> Rectification` 收敛循环。"
        ),
        "keyboard checklist should include explicit N/A rationale for two-pass geometry rendering item."
    );
}

#[test]
fn keyboard_registration_protocol_is_not_applicable_and_not_present() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "KeyboardItem",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not expose dynamic collection registration protocol (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。"),
        "keyboard checklist should mark registration protocol item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 非动态集合容器，不存在子项注册/注销流程，也无 `items_order` 导航语义。"
        ),
        "keyboard checklist should include explicit N/A rationale for registration protocol item."
    );
}

#[test]
fn keyboard_slot_projection_policy_is_not_applicable_and_not_present() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "slot_projection",
        "projection_mode",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement slot projection lifecycle policy (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。"),
        "keyboard checklist should mark slot projection policy item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 不是容器投影组件，不承载子内容投影生命周期管理；不存在 `Lazy/KeepAlive/Eager` 策略切换与 `NotifyHidden` 隐藏通知链路。"
        ),
        "keyboard checklist should include explicit N/A rationale for slot projection policy item."
    );
}

#[test]
fn keyboard_env_streams_are_not_applicable_and_not_present() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "matchMedia",
        "match_media",
        "BreakpointChanged",
        "ThemeChanged",
        "IntersectionChanged",
        "debounce",
        "throttle",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement env stream sampling/dispatch pipeline (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。"),
        "keyboard checklist should mark env streams item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 为静态语义展示组件，不订阅 `Resize/Theme/Intersection` 环境流，也不存在采样/防抖后回推 `logic` 的高层 `Action` 管线。"
        ),
        "keyboard checklist should include explicit N/A rationale for env streams item."
    );
}

#[test]
fn keyboard_event_light_cone_is_not_applicable_and_not_present() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "ContextBus",
        "Context Bus",
        "Selector",
        "SelectionState::All",
        "SelectionState",
        "prop_drilling",
        "Table",
        "Grid",
        "bulk_select",
        "batch_select",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement event light cone bulk-collection protocol (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。"),
        "keyboard checklist should mark event light cone item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 非大型集合容器，不承担批量选择/批量广播语义；不存在 `Context Bus + Selector` 批处理通道与 `SelectionState::All` 状态压缩需求。"
        ),
        "keyboard checklist should include explicit N/A rationale for event light cone item."
    );
}

#[test]
fn keyboard_causality_bus_is_not_applicable_and_not_present() {
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "TraceId",
        "CausalityBus",
        "Causality Bus",
        "event_bus",
        "publish",
        "broadcast",
        "subscriber",
        "dispatch_command",
        "derived_command",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not implement causality bus trace chain (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。"),
        "keyboard checklist should mark causality bus item complete."
    );
    assert!(
        check2.contains(
            "N/A：`Keyboard` 为静态语义展示组件，不存在复杂派生命令总线与跨订阅者广播链路；无 `TraceId` 透传需求。"
        ),
        "keyboard checklist should include explicit N/A rationale for causality bus item."
    );
}

#[test]
fn keyboard_a11y_i18n_l10n_contracts_are_mounted_without_view_hardcoded_copy() {
    let primitive = load_source("primitive_keyboard");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "use ui_headless::{A11yDirection, KeyboardOptions, use_keyboard};",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "use_keyboard(KeyboardOptions {",
        "aria-label=move || semantics.get().attrs.aria_label",
        "lang=move || semantics.get().attrs.lang",
        "dir=move || semantics.get().attrs.dir",
    ] {
        assert!(
            view.contains(required),
            "keyboard should expose and mount typed a11y/i18n contract (`{required}`)."
        );
    }

    for required in [
        "normalize_aria_label(input.aria_label)",
        "normalize_optional_text(input.lang)",
    ] {
        assert!(
            logic.contains(required),
            "keyboard logic should normalize overridable a11y/i18n inputs (`{required}`)."
        );
    }

    assert!(
        primitive.contains("pub const DEFAULT_ARIA_LABEL: &str = \"Keyboard\";"),
        "keyboard fallback label should live in primitives rather than view."
    );
    assert!(
        !view.contains("\"Keyboard\""),
        "keyboard view should not hardcode user-visible fallback copy."
    );
    assert!(
        check2.contains(
            "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。"
        ),
        "keyboard checklist should mark a11y/i18n/l10n item complete."
    );
    assert!(
        check2.contains("`Keyboard` 通过 `use_keyboard(KeyboardOptions)` 挂载 headless 语义契约，输出 `aria-label/lang/dir` 与状态标记；文案入口为 `aria_label`（可覆盖）并在 primitives 提供默认回退，`view.rs` 不硬编码用户可见文本。"),
        "keyboard checklist should include concrete a11y/i18n evidence."
    );
}

#[test]
fn keyboard_state_markers_are_observable_searchable_and_enumerated() {
    let primitive = load_source("primitive_keyboard");
    let headless = load_source("headless_keyboard");
    let view = load_source("view");
    let readme = load_source("component_readme");
    let check2 = load_source("check2");

    for required in [
        "data-slot=move || semantics.get().attrs.data_slot",
        "data-tone=move || semantics.get().attrs.data_tone",
        "data-state=move || semantics.get().attrs.data_state",
        "data-compact=move || semantics.get().attrs.data_compact",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-custom-class=move || semantics.get().attrs.data_custom_class",
        "data-ui-schema=move || semantics.get().attrs.data_ui_schema",
        "data-ui-schema-version=move || semantics.get().attrs.data_ui_schema_version",
        "data-ui-intent=move || semantics.get().attrs.data_ui_intent",
        "data-ui-action=move || semantics.get().attrs.data_ui_action",
        "data-ui-state=move || semantics.get().attrs.data_ui_state",
        "data-ui-source=move || semantics.get().attrs.data_ui_source",
        "data-ui-output-status=move || semantics.get().attrs.data_ui_output_status",
        "aria-label=move || semantics.get().attrs.aria_label",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should expose stable observable semantic markers (`{required}`)."
        );
    }

    for required in [
        "data_slot: \"keyboard\"",
        "data_tone: options.state.tone_attr",
        "data_state: options.state.data_state_attr",
        "data_compact: options.state.is_compact.then_some(\"true\")",
        "data_aria_source: options.state.aria_source_attr",
        "data_class_source: options.state.class_source_attr",
        "data_custom_class: options.state.has_custom_class_name.then_some(\"true\")",
        "data_ui_schema: agent.data_ui_schema",
        "data_ui_schema_version: agent.data_ui_schema_version",
        "data_ui_intent: agent.data_ui_intent",
        "data_ui_action: agent.data_ui_action",
        "data_ui_state: agent.data_ui_state",
        "data_ui_source: agent.data_ui_source",
        "data_ui_output_status: agent.data_ui_output_status",
    ] {
        assert!(
            headless.contains(required),
            "keyboard headless contract should keep marker mapping centralized and typed (`{required}`)."
        );
    }

    for required in [
        "KeyboardTone::Default => \"default\"",
        "KeyboardTone::Muted => \"muted\"",
        "if input.has_custom_aria_label {\n        \"custom\"\n    } else {\n        \"default\"",
        "if input.has_custom_class_name {\n        \"custom\"\n    } else {\n        \"default\"",
        "if input.compact {\n        \"compact\"\n    } else if input.tone == KeyboardTone::Muted {\n        \"muted\"\n    } else {\n        \"default\"",
    ] {
        assert!(
            primitive.contains(required),
            "keyboard primitive should keep marker values as closed enumerations (`{required}`)."
        );
    }

    for required in [
        "data-tone=\"default|muted\"",
        "data-state=\"default|muted|compact\"",
        "data-aria-source=\"default|custom\"",
        "data-class-source=\"default|custom\"",
        "data-ui-schema=\"ui.keyboard.agent-contract/v1\"",
        "data-ui-state=\"default|muted|compact\"",
        "data-ui-source=\"default|custom\"",
        "data-ui-output-status=\"draft|verified|committable\"",
    ] {
        assert!(
            readme.contains(required),
            "keyboard docs should expose closed marker domains for deterministic selector usage (`{required}`)."
        );
    }

    assert!(
        check2.contains(
            "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。"
        ),
        "keyboard checklist should mark observable/searchable/verifiable state markers item complete."
    );
    assert!(
        check2.contains("`Keyboard` 在 `view.rs` 稳定挂载 `data-slot/data-tone/data-state/data-compact/data-aria-source/data-class-source/data-custom-class + aria-label`；来源标记由 primitives 的 `default|custom` 枚举与状态枚举（`default|muted|compact`）驱动，避免自由文本漂移。"),
        "keyboard checklist should include concrete marker observability evidence."
    );
}

#[test]
fn keyboard_styles_depend_on_explicit_semantic_state_markers() {
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        ".ui-keyboard--tone-default,\n.ui-keyboard[data-tone=\"default\"]",
        ".ui-keyboard--tone-muted,\n.ui-keyboard[data-tone=\"muted\"]",
        ".ui-keyboard--compact,\n.ui-keyboard[data-compact=\"true\"]",
        ".ui-keyboard--custom-class,\n.ui-keyboard[data-custom-class=\"true\"]",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-fg, var(--ui-fallback-fg))",
    ] {
        assert!(
            styles.contains(required),
            "keyboard styles should branch on explicit semantic markers/tokens (`{required}`)."
        );
    }

    for forbidden in [
        ":nth-child(",
        ":first-child",
        ":last-child",
        ":only-child",
        ":nth-of-type(",
        " style=",
        "\n  > ",
    ] {
        assert!(
            !styles.contains(forbidden) && !view.contains(forbidden),
            "keyboard should not rely on fragile DOM-structure selectors or inline style logic (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。"),
        "keyboard checklist should mark explicit-state style dependency item complete."
    );
    assert!(
        check2.contains("`Keyboard` 的 `styles.rs` 状态分支仅使用稳定 class 与 `data-tone/data-compact/data-custom-class`，未使用 `:nth-child` 或深层结构猜测；`view.rs` 未注入业务 inline style，视觉切换由语义标记直接驱动。"),
        "keyboard checklist should include concrete style-marker evidence."
    );
}

#[test]
fn keyboard_semantics_contract_tests_are_primary_and_snapshot_independent() {
    let logic = load_source("logic");
    let view = load_source("view");
    let semantics = load_source("semantics_self");
    let check2 = load_source("check2");

    for required in [
        "<kbd",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-class-source=move || semantics.get().attrs.data_class_source",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should expose semantic contract markers (`{required}`)."
        );
    }

    for required in [
        "fn keyboard_a11y_i18n_l10n_contracts_are_mounted_without_view_hardcoded_copy()",
        "fn keyboard_state_markers_are_observable_searchable_and_enumerated()",
        "fn keyboard_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet()",
        "fn keyboard_has_no_async_interaction_protocol_and_marks_na_reason()",
    ] {
        assert!(
            semantics.contains(required),
            "keyboard semantic suite should cover key contract and branch matrix paths (`{required}`)."
        );
    }

    for forbidden in snapshot_assertion_markers() {
        assert!(
            !semantics.contains(forbidden),
            "keyboard contract tests must not depend on visual snapshot assertions (`{forbidden}`)."
        );
    }

    for forbidden in [
        "on:keydown",
        "on:keyup",
        "on:pointer",
        "on:click",
        "disabled:",
        "web_sys::",
        "wasm_bindgen::",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "keyboard has no interactive/wasm-only branch and should keep N/A matrix explicit (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 测试验证“语义契约”而不只验证视觉快照。"),
        "keyboard checklist should mark semantic-contract-over-snapshot testing item complete."
    );
    assert!(
        check2.contains("`components/keyboard/test/semantics.rs` 以语义断言覆盖 `<kbd>/aria-label/data-state/data-aria-source/data-class-source` 等契约；测试不依赖视觉快照断言。`Keyboard` 无受控轴、disabled、键盘/指针交互与 wasm 专属分支，相关矩阵在本组件按 N/A 语义验证。"),
        "keyboard checklist should include concrete semantic-testing evidence."
    );
}

#[test]
fn keyboard_file_responsibilities_are_enforced_with_motion_na() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{DEFAULT_ARIA_LABEL, KeyboardTone};",
        "pub use view::Keyboard;",
    ] {
        assert!(
            module.contains(required),
            "keyboard mod.rs should keep minimal stable exports (`{required}`)."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "pub mod protocol;"] {
        assert!(
            !module.contains(forbidden),
            "keyboard mod.rs should not leak implementation details (`{forbidden}`)."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "NodeRef",
        "Element",
        ".ui-keyboard",
        "var(--ui-",
    ] {
        assert!(
            !logic.contains(forbidden),
            "keyboard logic.rs should stay in normalization/derivation boundary (`{forbidden}`)."
        );
    }

    for forbidden in [
        "KeyboardRootInput",
        "KeyboardRootState",
        "resolve_state(",
        "Memo::new",
        "view! {",
        "use_keyboard(",
        "aria-label",
    ] {
        assert!(
            !styles.contains(forbidden),
            "keyboard styles.rs should stay token-first static CSS only (`{forbidden}`)."
        );
    }

    for forbidden in [
        "resolve_state(",
        "KeyboardStateInput",
        "normalize_aria_label(",
        "normalize_optional_text(",
        "color-mix(",
        "var(--ui-",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard view.rs should avoid hidden state decisions and style branching (`{forbidden}`)."
        );
    }

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "ui_motion::",
        "spring",
        "keyframe",
        "attach_motion",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !styles.contains(forbidden)
                && !view.contains(forbidden),
            "keyboard should keep motion contract as N/A without embedding engine logic (`{forbidden}`)."
        );
    }

    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/keyboard/src/motion.rs");
    assert!(
        !motion_path.exists(),
        "keyboard should not add motion.rs for static non-animated component."
    );

    assert!(
        check2.contains("- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。"),
        "keyboard checklist should mark component file responsibility item complete."
    );
    assert!(
        check2.contains("`Keyboard` 维持 `mod/logic/styles/view` 分层：`mod.rs` 仅导出 `Keyboard/KeyboardTone`，`logic.rs` 仅做归一与来源派生，`styles.rs` 仅静态 token-first CSS，`view.rs` 仅结构与 headless 语义挂载。该组件无动效状态轴，`motion.rs` 按 N/A 处理且未引入动效引擎实现。"),
        "keyboard checklist should include concrete file responsibility evidence."
    );
}

#[test]
fn keyboard_spec_rs_policy_is_not_applicable_and_not_present() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "use crate::spec::",
        "Spec::new(",
    ] {
        assert!(
            !module.contains(forbidden)
                && !logic.contains(forbidden)
                && !styles.contains(forbidden)
                && !view.contains(forbidden),
            "keyboard should not expose or depend on unnecessary spec.rs builder contracts (`{forbidden}`)."
        );
    }

    let spec_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/keyboard/src/spec.rs");
    assert!(
        !spec_path.exists(),
        "keyboard should not introduce spec.rs for simple component."
    );

    assert!(
        check2.contains("- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。"),
        "keyboard checklist should mark spec.rs policy item complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 为简单单节点语义组件，当前目录无 `spec.rs`；组件说明与约束保留在 `check2.md` 与 `README.md`，未引入额外 Spec Builder 抽象。"),
        "keyboard checklist should include explicit N/A rationale for spec.rs policy item."
    );
    assert!(
        check2.contains("- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。"),
        "keyboard checklist should mark hyper-structure builder item complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 为简单静态语义组件，不属于复杂配置型组件；当前目录无 `spec.rs`，也不暴露 `Spec::new()...render()` 构建器入口。"),
        "keyboard checklist should include explicit N/A rationale for hyper-structure builder item."
    );
}

#[test]
fn keyboard_token_first_static_style_contract_is_aggregated_and_injected() {
    let styles = load_source("styles");
    let view = load_source("view");
    let ui_components_css = load_source("ui_components_css");
    let ui_root = load_source("ui_root");
    let check2 = load_source("check2");

    for required in [
        "pub const CSS: &str = r#\"",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles.contains(required),
            "keyboard styles should keep token-first static CSS contract (`{required}`)."
        );
    }

    for forbidden in [
        "style=move ||",
        "style:",
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"text-",
        "stylist::",
        "stylex",
        "emotion",
    ] {
        assert!(
            !view.contains(forbidden) && !styles.contains(forbidden),
            "keyboard component should not default to utility-first/CSS-in-Rust patterns (`{forbidden}`)."
        );
    }

    for required in [
        "#[cfg(feature = \"component-keyboard\")]",
        "out.push_str(crate::keyboard::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "keyboard css should be aggregated from styles.rs in ui css registry (`{required}`)."
        );
    }

    for required in [
        "#[prop(optional)] inject_components_css: bool,",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should provide centralized component CSS injection path (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。"),
        "keyboard checklist should mark token-first static style contract item complete."
    );
    assert!(
        check2.contains("`Keyboard` 样式集中在 `components/keyboard/src/styles.rs`，并由 `crates/ui/src/css.rs` 通过 `component-keyboard` 聚合，再由 `UiRoot` 的 `inject_components_css` 路径注入；视觉值使用 `var(--ui-*)` token，组件层未引入 Utility-First/CSS-in-Rust 方案。"),
        "keyboard checklist should include concrete token-first style contract evidence."
    );
}

#[test]
fn keyboard_visual_desire_baseline_is_repo_level_and_marked_na_for_component_scope() {
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for forbidden in [
        "HeroUI",
        "screenshot_baseline",
        "visual_regression_matrix",
        "button_input_overlay_baseline",
    ] {
        assert!(
            !logic.contains(forbidden) && !styles.contains(forbidden) && !view.contains(forbidden),
            "keyboard component should not own repository-level visual baseline governance (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。"),
        "keyboard checklist should mark visual-desire item complete."
    );
    assert!(
        check2.contains("N/A：该项是跨组件的仓库级视觉基线治理（默认主题页面、截图基线、Button/Input/Overlay 视觉回归对比）；`Keyboard` 仅消费 token 与语义样式，不拥有全局主题基线与关键组件回归矩阵。"),
        "keyboard checklist should include explicit N/A rationale for visual-desire item."
    );
}

#[test]
fn keyboard_tree_shaking_contract_is_feature_gated_and_not_all_components_for_web_demo() {
    let ui_components_cargo = load_source("ui_components_cargo");
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let web_demo_cargo = load_source("web_demo_cargo");
    let check2 = load_source("check2");

    for required in [
        "component-keyboard = [\"dep:ui-keyboard\"]",
        "ui-keyboard = { path = \"../../components/keyboard\", optional = true }",
    ] {
        assert!(
            ui_components_cargo.contains(required),
            "ui Cargo features should gate keyboard as optional component (`{required}`)."
        );
    }

    for required in [
        "#[cfg(feature = \"component-keyboard\")]",
        "pub use ui_keyboard as keyboard;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib should export keyboard only behind component feature gate (`{required}`)."
        );
    }

    for required in [
        "#[cfg(feature = \"component-keyboard\")]",
        "out.push_str(crate::keyboard::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css aggregation should include keyboard styles only behind feature gate (`{required}`)."
        );
    }

    assert!(
        web_demo_cargo.contains("default-features = false")
            && web_demo_cargo.contains("features = [\"inject-css\", \"web-demo-components\"]")
            && !web_demo_cargo.contains("all-components"),
        "web-demo should not enable all-components implicitly for ui."
    );

    assert!(
        check2.contains("- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。"),
        "keyboard checklist should mark tree-shaking item complete."
    );
    assert!(
        check2.contains("`ui` 已为 `Keyboard` 提供 `component-keyboard = [\"dep:ui-keyboard\"]` 可选特性，且 `ui-keyboard` 依赖为 `optional`；`lib.rs` 与 `css.rs` 分别以 `#[cfg(feature = \"component-keyboard\")]` 条件导出与样式聚合。验证命令 `cargo tree -e features -p ui --no-default-features --features component-keyboard,inject-css` 与 `cargo tree -e features -i ui -p web-demo` 已确认最小特性链与 `web-demo` 未启用 `all-components`。CI 最小特性编译与体积预算属于仓库流水线门禁，组件侧已满足接入前提。"),
        "keyboard checklist should include concrete tree-shaking evidence."
    );
    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "keyboard checklist should mark tree-shaking feature-pruning item complete."
    );
    assert!(
        check2.contains("`crates/ui/Cargo.toml` 已注册 `component-keyboard = [\"dep:ui-keyboard\"]` 且 `ui-keyboard` 为 optional；`crates/ui/src/lib.rs` 与 `crates/ui/src/css.rs` 均以 `#[cfg(feature = \"component-keyboard\")]` 条件导出/聚合，未无条件引入 keyboard。验证：`cargo tree -e features -p ui --no-default-features --features component-keyboard,inject-css` 可见 keyboard 特性链；`cargo tree -e features -i ui -p web-demo` 未出现 `all-components` 拉起。"),
        "keyboard checklist should include concrete feature-pruning evidence."
    );
}

#[test]
fn keyboard_has_no_controllable_state_axis_for_controlled_uncontrolled_triplet() {
    let view = load_source("view");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for forbidden in [
        "value:",
        "on_value_change",
        "default_value",
        "use_controllable_state",
        "use_controllable_open_state_traced",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden),
            "keyboard should not expose controllable/uncontrollable triplet contracts (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("N/A：`Keyboard` 为静态语义展示组件，不维护可控状态轴；不存在 `value/on_value_change/default_value` 的受控协议需求。"),
        "keyboard checklist should mark controlled/uncontrolled triplet as N/A with explicit rationale."
    );
}

#[test]
fn keyboard_checklist_marks_ui_components_boundary_complete_with_local_semantics_evidence() {
    let check2 = load_source("check2");

    for required in [
        "- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "components/keyboard/test/semantics.rs",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include ui evidence `{required}`."
        );
    }
}

#[test]
fn keyboard_engineering_capability_contract_stays_structured_and_runtime_agnostic() {
    let protocol = load_source("protocol");
    let logic = load_source("logic");
    let view = load_source("view");
    let keyboard_cargo = load_source("keyboard_cargo");
    let check2 = load_source("check2");

    for required in [
        "use serde::{Deserialize, Serialize};",
        "pub enum KeyboardComponentSchemaVersion",
        "pub struct KeyboardComponentSpec",
        "pub schema_version: KeyboardComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "keyboard protocol should keep serde-based schema/version contract (`{required}`)."
        );
    }

    assert!(
        keyboard_cargo.contains("serde = { version = \"1.0\", features = [\"derive\"] }"),
        "keyboard crate should keep serde dependency for structured protocol serialization."
    );

    for forbidden in ["tracing", "tokio", "async-std"] {
        assert!(
            !keyboard_cargo.contains(forbidden),
            "keyboard crate should not leak runtime-specific deps (`{forbidden}`)."
        );
    }

    for forbidden in [
        "tracing::",
        "TraceId",
        "tokio::",
        "async_std::",
        "spawn(",
        "Runtime",
    ] {
        assert!(
            !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard component runtime should stay free of tracing/async runtime leakage (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。"),
        "keyboard checklist should mark engineering-capability contract complete."
    );
    assert!(
        check2.contains("`Keyboard` 已在 `components/keyboard/src/protocol.rs` 提供 `KeyboardComponentSchemaVersion + KeyboardComponentSpec` 的 `serde` 协议结构用于 spec 序列化与版本演进；组件 crate 仅引入 `serde`，未引入 `tracing/tokio/async-std` 依赖。"),
        "keyboard checklist should include concrete serde/runtime isolation evidence."
    );
}

#[test]
fn keyboard_styles_defensive_variables_use_two_level_fallback_chain() {
    let styles = load_source("styles");
    let ui_theme_css = load_source("ui_theme_css");
    let check2 = load_source("check2");

    for required in [
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-sm, var(--ui-fallback-radius-sm))",
        "var(--ui-bg-muted, var(--ui-fallback-bg-muted))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-space-2xs, var(--ui-fallback-space-2xs))",
        "var(--ui-space-xs, var(--ui-fallback-space-xs))",
        "var(--ui-font-size-100, var(--ui-fallback-font-size-100))",
        "var(--ui-line-height-100, var(--ui-fallback-line-height-100))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles.contains(required),
            "keyboard styles should use two-level token fallback chain (`{required}`)."
        );
    }

    for forbidden in [
        "var(--ui-font-size-100, 12px)",
        "var(--ui-line-height-100, 16px)",
        "min-height: 1.25rem;",
        "padding: 0.125rem 0.375rem;",
        "min-height: 1.125rem;",
        "padding: 0.0625rem 0.25rem;",
        "outline: 1px solid color-mix",
    ] {
        assert!(
            !styles.contains(forbidden),
            "keyboard styles should not keep naked size fallbacks or legacy one-off values (`{forbidden}`)."
        );
    }

    assert!(
        !has_hex_color_literal(styles),
        "keyboard styles should not hardcode Hex colors in component CSS."
    );

    for required in [
        "--ui-fallback-border-width",
        "--ui-fallback-radius-sm",
        "--ui-fallback-space-xs",
        "--ui-fallback-font-size-100",
        "--ui-fallback-component-height-100",
    ] {
        assert!(
            ui_theme_css.contains(required),
            "ui-theme should provide SSOT fallback terminal tokens (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。"),
        "keyboard checklist should mark defensive-variables item complete."
    );
    assert!(
        check2.contains("`components/keyboard/src/styles.rs` 已将边框/圆角/前景/背景/字号/行高/尺寸与内边距统一为双层回退链"),
        "keyboard checklist should include concrete defensive-variable evidence."
    );
}

#[test]
fn keyboard_css_is_aggregated_in_ui_layer_and_avoids_plain_inline_styles() {
    let ui_components_css = load_source("ui_components_css");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-keyboard\")]",
        "out.push_str(crate::keyboard::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "keyboard CSS should be aggregated into `@layer ui` with feature-gated injection (`{required}`)."
        );
    }

    for forbidden in [
        " style=",
        "style=\"",
        "style=move ||",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
    ] {
        assert!(
            !view.contains(forbidden),
            "keyboard view should not use plain inline style attributes (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。"),
        "keyboard checklist should mark cascade-layer coverage item complete."
    );
    assert!(
        check2.contains("`crates/ui/src/css.rs` 的 `push_components_css`")
            && check2.contains("@layer ui")
            && check2.contains("`Keyboard` 样式通过 `component-keyboard` 条件聚合进入该层"),
        "keyboard checklist should include concrete @layer ui aggregation evidence."
    );
}

#[test]
fn keyboard_motion_contract_is_na_and_guarded_from_component_level_engine_embedding() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let view = load_source("view");
    let keyboard_cargo = load_source("keyboard_cargo");
    let ui_motion_lib = load_source("ui_motion_lib");
    let check2 = load_source("check2");

    for forbidden in [
        "mod motion;",
        "pub mod motion;",
        "attach_motion",
        "stiffness",
        "damping",
        "ui_motion::",
    ] {
        assert!(
            !module.contains(forbidden) && !logic.contains(forbidden) && !view.contains(forbidden),
            "keyboard should keep motion contract as N/A without component-level engine embedding (`{forbidden}`)."
        );
    }

    assert!(
        !keyboard_cargo.contains("ui-motion"),
        "keyboard should not pull ui-motion dependency when no component-local motion contract exists."
    );

    let motion_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../components/keyboard/src/motion.rs");
    assert!(
        !motion_path.exists(),
        "keyboard should not add motion.rs for static semantic component without animation axes."
    );

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should provide shared no-op/reduced-motion backend for non-wasm/SSR (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。"),
        "keyboard checklist should mark motion-contract item complete."
    );
    assert!(
        check2.contains("N/A：`Keyboard` 为静态语义展示组件，无 enter/exit/active 等动效状态轴，不引入 `motion.rs`、`attach_motion` 与 `stiffness/damping` 组件级动效 contract。"),
        "keyboard checklist should include explicit N/A rationale for motion-contract item."
    );
}

#[test]
fn keyboard_ui_components_entry_points_are_correctly_located_and_feature_gated() {
    let ui_components_lib = load_source("ui_components_lib");
    let ui_components_css = load_source("ui_components_css");
    let ui_root = load_source("ui_root");
    let active_highlight = load_source("active_highlight");
    let headless_a11y = load_source("headless_a11y");
    let headless_presence = load_source("headless_presence");
    let headless_controllable_state = load_source("headless_controllable_state");
    let check2 = load_source("check2");

    for required in [
        "#[cfg(feature = \"component-keyboard\")]",
        "pub use ui_keyboard as keyboard;",
    ] {
        assert!(
            ui_components_lib.contains(required),
            "ui lib should expose keyboard behind component feature gate (`{required}`)."
        );
    }

    assert!(
        !ui_components_lib.contains("web_sys::") && !ui_components_lib.contains("wasm_bindgen::"),
        "ui public module entry should not expose platform DOM details."
    );

    for required in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-keyboard\")]",
        "out.push_str(crate::keyboard::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(required),
            "ui css entry should aggregate feature-gated component CSS in @layer ui (`{required}`)."
        );
    }

    for required in [
        "pub fn UiRoot(",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            ui_root.contains(required),
            "UiRoot should centralize theme/css injection and global i18n/id providers (`{required}`)."
        );
    }

    for required in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "pub fn attach_active_highlight_motion(",
        "ui_motion::spring::SpringConfig",
    ] {
        assert!(
            active_highlight.contains(required),
            "active_highlight should stay as shared highlight style + motion driver capability (`{required}`)."
        );
    }

    for forbidden in ["Accordion", "Keyboard", "business", "订单", "payment"] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight should not embed component/business semantics (`{forbidden}`)."
        );
    }

    assert!(
        headless_controllable_state.contains("pub fn use_controllable_state<T>("),
        "open-state primitive should live in ui-headless controllable_state."
    );
    assert!(
        headless_presence.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence"),
        "presence primitive should live in ui-headless presence module."
    );
    assert!(
        headless_a11y.contains("pub fn aria_controls_when_open("),
        "shared a11y utility should live in ui-headless a11y module."
    );

    for missing in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/ui/src/{missing}"));
        assert!(
            !path.exists(),
            "ui should not host deprecated shared primitive file `{missing}`."
        );
    }

    assert!(
        check2.contains("- [x] `ui` 固定入口文件落点正确。"),
        "keyboard checklist should mark ui fixed entry-point location item complete."
    );
    assert!(
        check2.contains("`crates/ui/src/lib.rs` 作为总入口并以 `#[cfg(feature = \\\"component-*\\\")]` 条件导出组件"),
        "keyboard checklist should include concrete ui entry-point evidence."
    );
}

#[test]
fn keyboard_component_directory_standard_file_layout_is_correct() {
    let module = load_source("mod");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "mod logic;",
        "pub mod styles;",
        "mod view;",
        "pub use view::Keyboard;",
    ] {
        assert!(
            module.contains(required),
            "keyboard module boundary should keep standard file-layout exports (`{required}`)."
        );
    }

    for forbidden in ["pub mod logic;", "pub mod view;", "pub mod protocol;"] {
        assert!(
            !module.contains(forbidden),
            "keyboard mod.rs should keep minimal stable exports without overexposing internals (`{forbidden}`)."
        );
    }

    for forbidden in [
        "web_sys::",
        "wasm_bindgen::",
        "NodeRef",
        "Element",
        "HtmlElement",
    ] {
        assert!(
            !logic.contains(forbidden) && !styles.contains(forbidden),
            "keyboard logic/styles should stay platform-agnostic and free of DOM coupling (`{forbidden}`)."
        );
    }

    for required in [
        "pub fn normalize_root_state(input: KeyboardRootInput) -> KeyboardRootState",
        "pub const CSS: &str = r#\"",
        "use ui_headless::{A11yDirection, KeyboardOptions, use_keyboard};",
        "view! {",
    ] {
        assert!(
            logic.contains(required) || styles.contains(required) || view.contains(required),
            "keyboard component files should keep expected per-file responsibilities (`{required}`)."
        );
    }

    for missing in ["render.rs", "motion.rs", "spec.rs"] {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../components/keyboard/src/{missing}"));
        assert!(
            !path.exists(),
            "keyboard should not introduce `{missing}` for this static component layout."
        );
    }

    assert!(
        check2.contains("- [x] 组件目录标准文件落点正确。"),
        "keyboard checklist should mark component-directory standard layout item complete."
    );
    assert!(
        check2.contains(
            "`components/keyboard/src` 维持 `mod.rs + logic.rs + styles.rs + view.rs` 主职责落点"
        ),
        "keyboard checklist should include concrete component-directory layout evidence."
    );
    assert!(
        check2.contains("- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。"),
        "keyboard checklist should mark file-placement discipline item complete."
    );
    assert!(
        check2.contains("`Keyboard` 组件目录落实为 `mod.rs + logic.rs + styles.rs + view.rs`"),
        "keyboard checklist should include concrete file-placement discipline evidence."
    );
}

#[test]
fn keyboard_context_compression_manifest_and_rbi_are_present_and_aligned() {
    let module = load_source("mod");
    let view = load_source("view");
    let manifest = load_source("component_manifest");
    let rbi = load_source("component_rbi");
    let check2 = load_source("check2");

    for required in [
        "id = \"ui-keyboard\"",
        "name = \"Keyboard\"",
        "kind = \"snapshot\"",
        "rbi = \"keyboard.rbi\"",
        "mod_rs = \"mod.rs\"",
        "logic_rs = \"logic.rs\"",
        "styles_rs = \"styles.rs\"",
        "view_rs = \"view.rs\"",
        "spec_builder = false",
        "motion_runtime = false",
        "schema = \"ui.keyboard.agent-contract/v1\"",
    ] {
        assert!(
            manifest.contains(required),
            "keyboard manifest should declare context-compression contract and boundaries (`{required}`)."
        );
    }

    for required in [
        "pub type KeyboardTone = crate::KeyboardTone;",
        "pub const DEFAULT_ARIA_LABEL: &str;",
        "pub mod styles {",
        "pub const CSS: &str;",
        "pub fn Keyboard(",
        "tone: Option<KeyboardTone>",
        "is_compact: Option<bool>",
        "aria_label: Option<String>",
        "class_name: Option<String>",
        "lang: Option<String>",
        "dir: Option<ui_headless::A11yDirection>",
        "children: leptos::children::Children",
        ") -> impl leptos::prelude::IntoView;",
    ] {
        assert!(
            rbi.contains(required),
            "keyboard RBI should project public interface signatures for AI-first indexing (`{required}`)."
        );
    }

    for required in [
        "pub use logic::{DEFAULT_ARIA_LABEL, KeyboardTone};",
        "pub use view::Keyboard;",
        "#[component]",
        "pub fn Keyboard(",
    ] {
        assert!(
            module.contains(required) || view.contains(required),
            "keyboard source should expose the same public surface declared by RBI (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。"),
        "keyboard checklist should mark manifest+rbi context-compression item complete."
    );
    assert!(
        check2.contains("`Keyboard` 已补齐 `components/keyboard/src/Component.toml` 与 `components/keyboard/src/keyboard.rbi`"),
        "keyboard checklist should include concrete manifest/rbi evidence."
    );
}

#[test]
fn keyboard_agent_contract_schema_markers_are_typed_and_whitelisted() {
    let headless = load_source("headless_keyboard");
    let view = load_source("view");
    let logic = load_source("logic");
    let styles = load_source("styles");
    let manifest = load_source("component_manifest");
    let check2 = load_source("check2");

    for required in [
        "pub const KEYBOARD_AGENT_SCHEMA: &str = \"ui.keyboard.agent-contract/v1\";",
        "pub enum KeyboardAgentSchemaVersion",
        "pub enum KeyboardAgentIntent",
        "pub enum KeyboardAgentAction",
        "pub enum KeyboardAgentState",
        "pub enum KeyboardAgentSource",
        "pub enum KeyboardOutputStatus",
        "pub fn resolve_agent_contract_attrs(state: KeyboardState) -> KeyboardAgentContractAttrs",
        "KeyboardAgentState::from_state_attr(state.data_state_attr)",
        "KeyboardAgentSource::from_sources(state.aria_source_attr, state.class_source_attr)",
        "data_ui_output_status: output_status.as_attr()",
    ] {
        assert!(
            headless.contains(required),
            "keyboard headless contract should generate agent markers from typed schema helpers (`{required}`)."
        );
    }

    for required in [
        "data-ui-schema=move || semantics.get().attrs.data_ui_schema",
        "data-ui-schema-version=move || semantics.get().attrs.data_ui_schema_version",
        "data-ui-intent=move || semantics.get().attrs.data_ui_intent",
        "data-ui-action=move || semantics.get().attrs.data_ui_action",
        "data-ui-state=move || semantics.get().attrs.data_ui_state",
        "data-ui-source=move || semantics.get().attrs.data_ui_source",
        "data-ui-output-status=move || semantics.get().attrs.data_ui_output_status",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should mount stable agent-contract schema markers (`{required}`)."
        );
    }

    for required in [
        "schema = \"ui.keyboard.agent-contract/v1\"",
        "\"output_status\"",
        "default = \"verified\"",
        "allowed = [\"draft\", \"verified\", \"committable\"]",
        "owner = \"upstream\"",
        "name = \"render_path\"",
        "\"logic::normalize_root_state\"",
        "\"ui_headless::use_keyboard\"",
        "\"view::Keyboard\"",
        "\"inner_html\"",
        "\"<script\"",
        "\"javascript:\"",
    ] {
        assert!(
            manifest.contains(required),
            "keyboard manifest should declare agent-contract schema and render-path whitelist boundary (`{required}`)."
        );
    }

    for forbidden in [
        "inner_html",
        "dangerously_set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !view.contains(forbidden) && !logic.contains(forbidden) && !styles.contains(forbidden),
            "keyboard render path should not permit script/html injection vectors (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。"),
        "keyboard checklist should mark agent-contract schema item complete."
    );
    assert!(
        check2.contains("这些字段由 `crates/ui-headless/src/keyboard.rs` 的类型化 `KeyboardAgent*` 枚举与 `resolve_agent_contract_attrs(...)` 统一生成"),
        "keyboard checklist should include concrete typed-schema generation evidence for agent markers."
    );
}

#[test]
fn keyboard_llm_render_mode_definition_is_snapshot_only_for_keyboard() {
    let manifest = load_source("component_manifest");
    let check2 = load_source("check2");

    for required in [
        "[capabilities]",
        "snapshot = true",
        "streaming = false",
        "[streaming_policy]",
        "required = false",
        "fallback = \"snapshot\"",
        "[component]",
        "kind = \"snapshot\"",
    ] {
        assert!(
            manifest.contains(required),
            "keyboard manifest should declare snapshot-first LLM render mode policy (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。"),
        "keyboard checklist should mark LLM render mode definition item complete."
    );
    for required in [
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
        "本组件仅消费 `Snapshot`（完整结果一次性渲染），不承担正文 `Streaming`（增量渲染）职责。",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should keep explicit two-mode definition and component scope (`{required}`)."
        );
    }
}

#[test]
fn keyboard_snapshot_is_baseline_capability_and_full_config_snapshot_is_renderable() {
    let manifest = load_source("component_manifest");
    let protocol = load_source("protocol");
    let logic = load_source("logic");
    let view = load_source("view");
    let check2 = load_source("check2");

    for required in [
        "[component]",
        "kind = \"snapshot\"",
        "[capabilities]",
        "snapshot = true",
        "streaming = false",
    ] {
        assert!(
            manifest.contains(required),
            "keyboard manifest should keep snapshot as baseline capability (`{required}`)."
        );
    }

    for required in [
        "pub struct KeyboardComponentSpec",
        "pub schema_version: KeyboardComponentSchemaVersion,",
    ] {
        assert!(
            protocol.contains(required),
            "keyboard protocol should accept full component snapshot payload contract (`{required}`)."
        );
    }

    for required in [
        "pub struct KeyboardRootInput",
        "pub fn normalize_root_state(input: KeyboardRootInput) -> KeyboardRootState",
    ] {
        assert!(
            logic.contains(required),
            "keyboard logic should normalize full snapshot input before rendering (`{required}`)."
        );
    }

    for required in [
        "pub fn Keyboard(",
        "#[prop(optional)] tone: Option<KeyboardTone>,",
        "#[prop(optional, into)] is_compact: Option<bool>,",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] class_name: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "logic::normalize_root_state(KeyboardRootInput {",
    ] {
        assert!(
            view.contains(required),
            "keyboard view should consume full upstream config snapshot and render stably (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。"),
        "keyboard checklist should mark snapshot-baseline item complete."
    );
    assert!(
        check2.contains("`Keyboard` 在 `components/keyboard/src/Component.toml` 声明 `kind=\"snapshot\"` 与 `snapshot=true`，并在 `view.rs` 通过 `Keyboard(...) -> logic::normalize_root_state(...) -> <kbd>` 路径消费完整配置快照后稳定渲染；即使不展示正文，也可在接收上层完整结果后正常渲染。"),
        "keyboard checklist should include concrete snapshot-baseline evidence for keyboard."
    );
}

#[test]
fn keyboard_streaming_policy_is_optional_and_output_status_marker_is_explicit() {
    let manifest = load_source("component_manifest");
    let view = load_source("view");
    let headless = load_source("headless_keyboard");
    let check2 = load_source("check2");

    for required in [
        "[streaming_policy]",
        "required = false",
        "fallback = \"snapshot\"",
        "owner = \"upstream\"",
        "[output_state]",
        "default = \"verified\"",
        "allowed = [\"draft\", \"verified\", \"committable\"]",
        "[capabilities]",
        "streaming = false",
    ] {
        assert!(
            manifest.contains(required),
            "keyboard manifest should encode streaming-optional policy and upstream responsibility (`{required}`)."
        );
    }

    for required in [
        "pub enum KeyboardOutputStatus",
        "KeyboardOutputStatus::Verified",
        "pub data_ui_output_status: &'static str,",
        "data_ui_output_status: output_status.as_attr()",
    ] {
        assert!(
            headless.contains(required),
            "headless keyboard contract should provide explicit output-status semantic marker (`{required}`)."
        );
    }

    assert!(
        view.contains("data-ui-output-status=move || semantics.get().attrs.data_ui_output_status"),
        "keyboard view should mount explicit output-status marker for both snapshot and optional streaming paths."
    );

    assert!(
        check2.contains("- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。"),
        "keyboard checklist should mark streaming-policy-by-component-responsibility item complete."
    );
    for required in [
        "`Streaming Optional` 执行：`components/keyboard/src/Component.toml` 声明 `streaming=false`、`[streaming_policy] required=false`、`fallback=\"snapshot\"`",
        "连续挂载 `data-ui-output-status`（当前 `verified`）与既有 `aria-*`/`data-*` 语义标记",
        "数据校验、断线恢复、重试策略保持 `owner=\"upstream\"` 由上层负责",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should keep concrete streaming-optional and output-status evidence (`{required}`)."
        );
    }
}

#[test]
fn keyboard_rust_hygiene_contract_is_enforced() {
    let logic = load_source("logic");
    let view = load_source("view");
    let headless = load_source("headless_keyboard");
    let primitive = load_source("primitive_keyboard");
    let protocol = load_source("protocol");
    let check_script = load_source("check_script");
    let rust_hygiene_script = load_source("rust_hygiene_script");
    let check2 = load_source("check2");

    for source in [&logic, &view, &headless, &primitive, &protocol] {
        for forbidden in [".unwrap(", ".expect(", "let _ ="] {
            assert!(
                !source.contains(forbidden),
                "keyboard non-test sources should not contain forbidden hygiene pattern (`{forbidden}`)."
            );
        }
    }

    for required in [
        "use std::borrow::Cow;",
        "Vec<Cow<'static, str>>",
        "Cow::Borrowed(\"ui-keyboard\")",
        "Cow::Borrowed(\"ui-keyboard--compact\")",
        "Cow::Borrowed(\"ui-keyboard--custom-class\")",
        "Cow::Owned(base_class_name)",
    ] {
        assert!(
            logic.contains(required),
            "keyboard logic should converge class-name string assembly to Cow-based path (`{required}`)."
        );
    }

    assert!(
        check_script.contains("./scripts/check-rust-hygiene.sh"),
        "repository check gate should invoke rust-hygiene script."
    );

    for required in ["no unwrap/expect", "no let _ =", "Cow<'static, str>"] {
        assert!(
            rust_hygiene_script.contains(required),
            "repository hygiene gate should enforce required rust hygiene contract (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。"),
        "keyboard checklist should mark rust hygiene item complete."
    );
    assert!(
        check2.contains(
            "`components/keyboard/src/logic.rs` 的 class 组装已收敛为 `Cow<'static, str>`"
        ),
        "keyboard checklist should include concrete Cow-based hygiene evidence."
    );
}

#[test]
fn keyboard_semantic_and_performance_regression_item_is_covered_with_applicability_notes() {
    let semantics = load_source("semantics_self");
    let check2 = load_source("check2");

    for required in [
        "fn keyboard_semantics_contract_tests_are_primary_and_snapshot_independent()",
        "fn keyboard_state_markers_are_observable_searchable_and_enumerated()",
        "fn keyboard_performance_governance_uses_component_equivalent_evidence_without_hot_paths()",
        "fn keyboard_focus_stack_gc_is_not_applicable_and_overlay_focus_state_is_absent()",
    ] {
        assert!(
            semantics.contains(required),
            "keyboard semantics suite should include semantic, focus-applicability, and performance evidence (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。"),
        "keyboard checklist should mark semantic/performance regression item complete."
    );
    for required in [
        "断言不依赖视觉快照",
        "不存在焦点流转链路",
        "`render_count` 对“高频/重型组件”要求在本组件按适用范围 N/A",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include explicit applicability evidence (`{required}`)."
        );
    }
}

#[test]
fn keyboard_version_deprecation_migration_item_is_na_without_breaking_schema_upgrade() {
    let protocol = load_source("protocol");
    let check2 = load_source("check2");

    for required in [
        "pub enum KeyboardComponentSchemaVersion",
        "#[default]",
        "V1,",
        "pub struct KeyboardComponentSpec",
    ] {
        assert!(
            protocol.contains(required),
            "keyboard protocol should stay on single v1 schema contract for this change set (`{required}`)."
        );
    }

    for forbidden in [
        "V2",
        "migrate_v1_to_v2",
        "SchemaRegistry",
        "deprecated_window",
    ] {
        assert!(
            !protocol.contains(forbidden),
            "keyboard protocol should not claim version-migration machinery without breaking upgrade (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。"),
        "keyboard checklist should mark version-deprecation migration item complete."
    );
    assert!(
        check2.contains("N/A：本轮 `Keyboard` 变更未引入跨大版本 API 破坏；`components/keyboard/src/protocol.rs` 仍为单一 `KeyboardComponentSchemaVersion::V1` 与 `KeyboardComponentSpec`，不存在 `v2` 协议切换、弃用窗口或 `migrate_v1_to_v2` 迁移函数需求。"),
        "keyboard checklist should include explicit N/A rationale for migration item."
    );
}

#[test]
fn keyboard_docs_copy_paste_ready_playground_matrix_is_present() {
    let docs = load_source("docs_display_extra");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "let keyboard_imports =",
        "title=\"Hello World (Default Path)\"",
        "title=\"State Matrix (Tone / Compact / Source Markers)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for Keyboard)\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "code_imports=keyboard_imports.clone()",
        "Copy action auto-injects missing imports for direct run.",
        "streaming is optional and falls back to snapshot",
        "Keyboard has no controllable state axis",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs page should provide copy-paste-ready playground matrix coverage (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。"),
        "keyboard checklist should mark docs-as-product copy-paste-ready item complete."
    );
    for required in [
        "`Hello World (Default Path)`",
        "`State Matrix (Tone / Compact / Source Markers)`",
        "`Controlled vs Uncontrolled Contrast (N/A for Keyboard)`",
        "`Streaming / Snapshot Contract`",
        "`Source-first Starter (Copy-Paste Ready)`",
        "`apps/docs-app/src/playground.rs::compose_copy_ready_code`",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include concrete docs-playground and copy-ready evidence (`{required}`)."
        );
    }
}

#[test]
fn keyboard_semantics_priority_item_is_complete_and_snapshot_is_not_primary() {
    let module = load_source("mod");
    let view = load_source("view");
    let semantics = load_source("semantics_self");
    let check2 = load_source("check2");

    for required in [
        "#[path = \"../test/semantics.rs\"]",
        "mod semantics_tests;",
        "fn keyboard_semantics_contract_tests_are_primary_and_snapshot_independent()",
        "fn keyboard_state_markers_are_observable_searchable_and_enumerated()",
        "<kbd",
        "aria-label=move || semantics.get().attrs.aria_label",
        "data-state=move || semantics.get().attrs.data_state",
        "data-aria-source=move || semantics.get().attrs.data_aria_source",
        "data-class-source=move || semantics.get().attrs.data_class_source",
        "data-ui-output-status=move || semantics.get().attrs.data_ui_output_status",
    ] {
        assert!(
            module.contains(required) || view.contains(required) || semantics.contains(required),
            "keyboard should keep semantics-first coverage anchored on stable semantic contracts (`{required}`)."
        );
    }

    for forbidden in snapshot_assertion_markers() {
        assert!(
            !semantics.contains(forbidden),
            "keyboard semantic tests should not treat visual snapshots as primary assertions (`{forbidden}`)."
        );
    }

    for forbidden in ["on:keydown", "on:keyup"] {
        assert!(
            !view.contains(forbidden),
            "keyboard has no interactive key-event path in current scope; applicability should stay explicit (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "keyboard checklist should mark semantics-priority item complete."
    );
    for required in [
        "`components/keyboard/test/semantics.rs`",
        "`data-state/data-aria-source/data-class-source/data-ui-* + aria-label`",
        "role 语义由原生 `<kbd>` 标签承载",
        "语义测试中禁止",
        "`on:keydown/on:keyup`",
        "N/A 记录",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include concrete semantics-priority evidence (`{required}`)."
        );
    }
}

#[test]
fn keyboard_e2e_selectors_are_semantic_and_wasm_waits_are_stable() {
    let e2e = load_source("e2e_keyboard_contract");
    let check2 = load_source("check2");

    for required in [
        "await page.goto(\"/#/components/keyboard\")",
        "await page.locator(\"body:not(:has(#boot))\").waitFor();",
        "[data-component=\"keyboard\"]",
        "[data-slot=\"keyboard\"][data-ui-schema=\"ui.keyboard.agent-contract/v1\"][data-ui-output-status=\"verified\"]",
        "[data-slot=\"keyboard\"][data-state=\"default\"][data-aria-source=\"default\"][data-class-source=\"default\"]",
        "[data-slot=\"keyboard\"][data-state=\"compact\"][data-compact=\"true\"]",
        "[data-slot=\"keyboard\"][data-aria-source=\"custom\"][data-class-source=\"custom\"]",
        "data-slot=\"segmented-control-option\"",
        "docs-app keyboard workbench flow is repeatable with semantic ready/settled breakpoints",
    ] {
        assert!(
            e2e.contains(required),
            "keyboard e2e contract should use semantic selectors and wasm-stable ready waits (`{required}`)."
        );
    }

    for forbidden in ["waitForTimeout(", "setTimeout(", "sleep("] {
        assert!(
            !e2e.contains(forbidden),
            "keyboard e2e should avoid fixed-sleep waiting and rely on semantic readiness (`{forbidden}`)."
        );
    }

    assert!(
        check2.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "keyboard checklist should mark e2e selector stability item complete."
    );
    for required in [
        "`e2e/tests/docs_app_keyboard_contract.spec.mjs`",
        "page.locator(\"body:not(:has(#boot))\").waitFor()",
        "`data-slot=\"keyboard\" + data-ui-schema + data-ui-output-status=\"verified\"`",
        "`data-state/data-aria-source/data-class-source/data-compact`",
        "`default|muted|compact`",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include concrete semantic-selector e2e evidence (`{required}`)."
        );
    }
}

#[test]
fn keyboard_repeatable_e2e_flow_is_in_regression_collection_with_semantic_breakpoints() {
    let e2e = load_source("e2e_keyboard_contract");
    let check2 = load_source("check2");

    for required in [
        "docs-app keyboard workbench flow is repeatable with semantic ready/settled breakpoints",
        "await page.reload();",
        "await runKeyboardWorkbenchFlow(page, docsRoot);",
        "await runKeyboardWorkbenchFlow(page, reloadedRoot);",
        "await compactSwitch.focus();",
        "await customAriaSwitch.focus();",
        "await customClassSwitch.focus();",
        "await page.keyboard.press(\"Space\");",
        "await expect(preview).toHaveAttribute(\"data-state\", \"compact\");",
        "await expect(preview).toHaveAttribute(\"data-aria-source\", \"custom\");",
        "await expect(preview).toHaveAttribute(\"data-class-source\", \"custom\");",
    ] {
        assert!(
            e2e.contains(required),
            "keyboard e2e regression collection should keep repeatable flow with semantic breakpoints (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "keyboard checklist should mark repeatable e2e regression collection item complete."
    );
    for required in [
        "`e2e/tests/docs_app_keyboard_contract.spec.mjs`",
        "`docs-app keyboard workbench flow is repeatable with semantic ready/settled breakpoints`",
        "`page.reload()`",
        "`data-state/data-tone/data-compact/data-aria-source/data-class-source/data-custom-class`",
        "`focus + keyboard`",
        "`switch.focus() + page.keyboard.press(\"Space\")`",
        "N/A",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should keep concrete repeatable-regression evidence markers (`{required}`)."
        );
    }
}

#[test]
fn keyboard_docs_examples_parameter_matrix_and_state_matrix_stay_in_sync_with_logic_defaults() {
    let docs = load_source("docs_display_extra");
    let logic = load_source("logic");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "title=\"Hello World (Default Path)\"",
        "title=\"State Matrix (Tone / Compact / Source Markers)\"",
        "title=\"Controlled vs Uncontrolled Contrast (N/A for Keyboard)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "data-slot=\"keyboard-parameter-matrix\"",
        "\"tone\"",
        "KeyboardTone::Default (default)",
        "\"is_compact\"",
        "false (default)",
        "\"aria_label\"",
        "\\\"Keyboard\\\" fallback after trim/normalize",
        "\"class_name\"",
        "optional custom class (default none)",
        "tone=tone",
        "is_compact=is_compact",
        "aria_label=aria_label",
        "class_name=class_name",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs page should keep examples/parameter matrix/state matrix synchronized (`{required}`)."
        );
    }

    for required in [
        "let tone = input.tone.unwrap_or_default();",
        "let is_compact = input.is_compact.unwrap_or(false);",
        "let (aria_label, has_custom_aria_label) = normalize_aria_label(input.aria_label);",
        "let class_name = normalize_optional_text(input.class_name);",
    ] {
        assert!(
            logic.contains(required),
            "keyboard logic defaults should remain explicit and traceable for docs sync (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "keyboard checklist should mark docs/examples/matrix synchronization item complete."
    );
    for required in [
        "`Hello World (Default Path)`",
        "`State Matrix (Tone / Compact / Source Markers)`",
        "`data-slot=\"keyboard-parameter-matrix\"`",
        "`tone/is_compact/aria_label/class_name`",
        "`tone.unwrap_or_default()`",
        "`is_compact.unwrap_or(false)`",
        "`normalize_aria_label(...)`",
        "`normalize_optional_text(...)`",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should keep concrete docs-sync evidence markers (`{required}`)."
        );
    }
}

#[test]
fn keyboard_documentation_as_product_is_beginner_friendly_and_progressive() {
    let readme = load_source("component_readme");
    let docs = load_source("docs_display_extra");
    let check2 = load_source("check2");

    for required in [
        "# Keyboard",
        "## Hello World（最小可用）",
        "## 常见用法",
        "## 新手路径（先用起来，再进阶）",
        "第一步（默认 API）：直接使用 `<Keyboard>\"⌘K\"</Keyboard>`",
        "第二步（常见定制）：按需设置 `tone` 或 `is_compact`。",
        "第三步（高级覆盖）：仅在需要可访问性或样式覆盖时传入 `aria_label` / `class_name`。",
    ] {
        assert!(
            readme.contains(required),
            "keyboard README should provide beginner-friendly progressive docs path (`{required}`)."
        );
    }

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "title=\"Hello World (Default Path)\"",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs-app page should expose beginner default path and progressive playground (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "keyboard checklist should mark beginner-friendly documentation item complete."
    );
    for required in [
        "`components/keyboard/src/README.md`",
        "`Hello World（最小可用）`",
        "`常见用法`",
        "`新手路径（先用起来，再进阶）`",
        "`<Keyboard>\"⌘K\"</Keyboard>`",
        "`tone/is_compact`",
        "`aria_label/class_name`",
        "`Hello World (Default Path)`",
        "Interactive Playground",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include concrete beginner-doc evidence markers (`{required}`)."
        );
    }
}

#[test]
fn keyboard_docs_app_interactive_playground_is_available_and_repeatable() {
    let docs = load_source("docs_display_extra");
    let e2e = load_source("e2e_keyboard_contract");
    let check2 = load_source("check2");

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "title=\"Interactive Playground (展示 / Config / Code / CSS Test)\"",
        "let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0));",
        "let (workbench_key_index, set_workbench_key_index) = signal(Some(0));",
        "let (workbench_is_compact, set_workbench_is_compact) = signal(false);",
        "let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);",
        "let (workbench_custom_class, set_workbench_custom_class) = signal(false);",
        "test_config_signal=workbench_config",
        "code_signal=workbench_code",
        "test_css_source=keyboard_test_css_source",
        "set_selected_index=set_workbench_tone_index",
        "set_selected_index=set_workbench_key_index",
        "Switch checked=workbench_is_compact set_checked=set_workbench_is_compact",
        "Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria",
        "Switch checked=workbench_custom_class set_checked=set_workbench_custom_class",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs page should expose interactive playground controls and realtime preview wiring (`{required}`)."
        );
    }

    for required in [
        "async function runKeyboardWorkbenchFlow(page, docsRoot) {",
        "docs-app keyboard workbench flow is repeatable with semantic ready/settled breakpoints",
        "await runKeyboardWorkbenchFlow(page, docsRoot);",
        "await page.reload();",
        "await runKeyboardWorkbenchFlow(page, reloadedRoot);",
    ] {
        assert!(
            e2e.contains(required),
            "keyboard e2e should keep interactive playground key path repeatable (`{required}`)."
        );
    }

    assert!(
        check2.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "keyboard checklist should mark interactive playground item complete."
    );
    for required in [
        "`Interactive Playground (展示 / Config / Code / CSS Test)`",
        "`Tone/Key Text/is_compact/Custom aria_label/Custom class_name`",
        "`test_config_signal=workbench_config`",
        "`runKeyboardWorkbenchFlow(...)`",
        "reload 重跑",
        "N/A",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include concrete interactive-playground evidence markers (`{required}`)."
        );
    }
}

#[test]
fn keyboard_source_first_docs_are_copy_paste_ready_with_imports_and_source_paths() {
    let docs = load_source("docs_display_extra");
    let readme = load_source("component_readme");
    let playground = load_source("docs_playground");
    let e2e = load_source("e2e_keyboard_contract");
    let check2 = load_source("check2");

    for required in [
        "title=\"Source-first Starter (Copy-Paste Ready)\"",
        "description=\"Copy action auto-injects missing imports for direct run.\"",
        "code_signal=source_first_code",
        "code_imports=keyboard_imports.clone()",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs page should expose source-first copy-ready starter with imports wiring (`{required}`)."
        );
    }

    for required in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "let missing_imports = missing_import_lines(&raw, &imports);",
        "if missing_imports.is_empty()",
        "format!(\"{}\\n\\n{raw}\", missing_imports.join(\"\\n\"))",
    ] {
        assert!(
            playground.contains(required),
            "docs playground should auto-inject missing imports for copy-ready snippets (`{required}`)."
        );
    }

    for required in [
        "docs-app keyboard source-first snippet is copy-paste ready with imports",
        "hasText: \"Source-first Starter (Copy-Paste Ready)\"",
        "data-copyable\", \"true\"",
        "use leptos::prelude::*;",
        "use ui::{Keyboard, KeyboardTone};",
    ] {
        assert!(
            e2e.contains(required),
            "keyboard e2e should lock source-first copy-ready behavior and imports (`{required}`)."
        );
    }

    for required in [
        "## Source-first Copy-Paste Ready",
        "复制片段默认包含可直接运行的 imports",
        "`use leptos::prelude::*;` 与 `use ui::{Keyboard, KeyboardTone};`",
        "依赖前提：已在应用侧引入 `ui`",
        "`component-keyboard`",
        "`components/keyboard/src/mod.rs`",
        "`components/keyboard/src/logic.rs`",
        "`components/keyboard/src/view.rs`",
        "`components/keyboard/src/styles.rs`",
    ] {
        assert!(
            readme.contains(required),
            "keyboard README should provide source-first dependencies and real source locations (`{required}`)."
        );
    }

    assert!(
        check2.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "keyboard checklist should mark source-first copy-paste-ready item complete."
    );
    for required in [
        "`Source-first Starter (Copy-Paste Ready)`",
        "`code_imports=keyboard_imports`",
        "`compose_copy_ready_code(...)`",
        "`data-copyable=\\\"true\\\"`",
        "`use leptos::prelude::*;`",
        "`use ui::{Keyboard, KeyboardTone};`",
        "`components/keyboard/src/README.md`",
        "`mod/logic/view/styles`",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should include concrete source-first evidence markers (`{required}`)."
        );
    }
}

#[test]
fn keyboard_heroui_strategy_and_component_docs_stay_synchronized() {
    let heroui = load_source("heroui_strategy");
    let docs_index = load_source("docs_pages_index");
    let docs = load_source("docs_display_extra");
    let readme = load_source("component_readme");
    let check2 = load_source("check2");

    for required in [
        "### Keyboard 同步记录（2026-02-20）",
        "`Keyboard` 维持 display primitive 定位，参数主轴保持 `tone/is_compact/aria_label/class_name`",
        "`component_doc!(\"Keyboard\", \"keyboard\", \"Display\", display_extra::keyboard)`",
        "`#/components/keyboard`",
        "`components/keyboard/src/README.md`",
        "`docs/research/spectrum-heroui-style-interface-study.md`",
        "不需要追加",
    ] {
        assert!(
            heroui.contains(required),
            "HeroUI strategy doc should keep keyboard synchronization evidence (`{required}`)."
        );
    }

    assert!(
        docs_index.contains(
            "component_doc!(\"Keyboard\", \"keyboard\", \"Display\", display_extra::keyboard),"
        ),
        "docs components index should keep keyboard entry discoverable."
    );

    for required in [
        "pub(super) fn keyboard() -> AnyView {",
        "title=\"Hello World (Default Path)\"",
        "title=\"State Matrix (Tone / Compact / Source Markers)\"",
    ] {
        assert!(
            docs.contains(required),
            "keyboard docs page should remain accessible with synced example matrix (`{required}`)."
        );
    }

    assert!(
        readme.contains("# Keyboard"),
        "keyboard README should remain accessible as equivalent component documentation entry."
    );

    assert!(
        check2.contains("- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。"),
        "keyboard checklist should mark HeroUI strategy/documentation synchronization item complete."
    );
    for required in [
        "`Keyboard 同步记录（2026-02-20）`",
        "`tone/is_compact/aria_label/class_name`",
        "`component_doc!(\"Keyboard\", \"keyboard\", \"Display\", display_extra::keyboard)`",
        "`#/components/keyboard`",
        "`components/keyboard/src/README.md`",
        "`docs/research/spectrum-heroui-style-interface-study.md`",
        "N/A",
    ] {
        assert!(
            check2.contains(required),
            "keyboard checklist should keep concrete HeroUI-doc-sync evidence markers (`{required}`)."
        );
    }
}
