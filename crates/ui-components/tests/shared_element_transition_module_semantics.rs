use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {:?}: {}", path, e))
}

#[test]
fn shared_element_transition_compat_module_is_removed() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("src/shared_element_transition/mod.rs");
    assert!(!path.exists(), "compat module  should not exist.",);
}

#[test]
fn crate_root_does_not_register_shared_element_transition_compat_module() {
    let source = load_source("src/lib.rs");

    assert!(
        !source.contains("pub mod shared_element_transition;"),
        "crate root should not include legacy.",
    );
}

#[test]
fn shared_element_transition_status_primitives_boundary_is_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let state_primitives_root = load_source("../../crates/ui-state-primitives/src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; component implementation must stay removed."
    );

    assert!(
        !crate_root.contains("pub mod shared_element_transition;"),
        "ui-components crate root should not expose removed shared_element_transition component."
    );
    assert!(
        !state_primitives_root.contains("shared_element_transition"),
        "ui-state-primitives should not host orphan shared_element_transition state primitive module after component removal."
    );

    for required in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "不存在组件实现层状态机可越层写入",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep status-primitives boundary marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_ui_headless_boundary_is_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no component view layer should remain."
    );

    assert!(
        !crate_root.contains("pub mod shared_element_transition;"),
        "ui-components crate root should not expose removed shared_element_transition component."
    );

    for required in [
        "pub mod a11y;",
        "pub mod presence;",
        "pub use a11y::",
        "pub use presence::{Presence, use_presence};",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless should keep shared semantic contract entry `{}`.",
            required
        );
    }

    assert!(
        headless_a11y.contains("pub fn locale_attrs("),
        "ui-headless a11y contract should keep lang/dir adapter entry."
    );

    assert!(
        !headless_lib.contains("shared_element_transition")
            && !headless_a11y.contains("shared_element_transition"),
        "ui-headless should not carry removed shared_element_transition component-specific semantic implementation."
    );

    for required in [
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "组件模块不存在 + crate root 未导出",
        "`ui-headless` 仅保留通用契约入口",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep ui-headless boundary marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_ui_motion_boundary_is_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_web = load_source("../../crates/ui-motion/src/web.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no component motion mapping file should remain."
    );

    assert!(
        !crate_root.contains("pub mod shared_element_transition;"),
        "ui-components crate root should not expose removed shared_element_transition component."
    );

    for required in [
        "pub mod keyframes;",
        "pub mod options;",
        "pub mod presets;",
        "pub mod spring;",
        "#[cfg(target_arch = \"wasm32\")]\npub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]\npub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(required),
            "ui-motion should keep generic runtime/driver contract entry `{}`.",
            required
        );
    }

    for forbidden in [
        "shared_element_transition",
        "data-slot=",
        "role=",
        "aria-",
        "on:keydown",
    ] {
        assert!(
            !ui_motion_lib.contains(forbidden) && !ui_motion_web.contains(forbidden),
            "ui-motion should not carry component/view semantics token `{}`.",
            forbidden
        );
    }

    for required in [
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "组件模块不存在 + crate root 未导出",
        "`ui-motion` 仅保留通用执行后端（`spring/keyframes/web`）与 non-wasm no-op/stub",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep ui-motion boundary marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_ui_theme_boundary_is_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let ui_theme_tokens = load_source("../../crates/ui-theme/src/tokens.rs");
    let ui_theme_theme = load_source("../../crates/ui-theme/src/theme.rs");
    let ui_theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let ui_theme_baseline_test = load_source("../../crates/ui-theme/tests/token_scale_baseline.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no component styles/theme mapping file should remain."
    );

    assert!(
        !crate_root.contains("pub mod shared_element_transition;"),
        "ui-components crate root should not expose removed shared_element_transition component."
    );

    for required in [
        "single source of truth for token taxonomy and baselines",
        "pub enum TokenScale",
        "pub struct ThemeTokens",
    ] {
        assert!(
            ui_theme_tokens.contains(required),
            "ui-theme tokens should keep canonical token taxonomy marker `{}`.",
            required
        );
    }

    for required in [
        "pub enum ThemeSystem",
        "pub enum ThemeColor",
        "pub enum ThemeScale",
        "pub struct ThemeContext",
        "pub fn baseline_two(color: ThemeColor, scale: ThemeScale) -> Self",
    ] {
        assert!(
            ui_theme_theme.contains(required),
            "ui-theme should keep system/color/scale context mapping marker `{}`.",
            required
        );
    }

    for required in [
        "pub fn theme_to_css_variables(theme: &Theme) -> String",
        "--ui-system",
        "--ui-color",
        "--ui-scale",
    ] {
        assert!(
            ui_theme_css.contains(required),
            "ui-theme css output should keep variable emission marker `{}`.",
            required
        );
    }

    for required in [
        "fn token_scale_baselines_are_regression_testable()",
        "Theme::baseline_two(ThemeColor::Light, ThemeScale::Medium)",
        "Theme::baseline_two(ThemeColor::Dark, ThemeScale::Medium)",
        "Theme::baseline_two(ThemeColor::Oled, ThemeScale::Medium)",
    ] {
        assert!(
            ui_theme_baseline_test.contains(required),
            "ui-theme baseline regression tests should keep marker `{}`.",
            required
        );
    }

    for required in [
        "Token 统一基线落点固定",
        "crates/ui-theme/src/tokens.rs",
        "crates/ui-theme/src/theme.rs",
        "crates/ui-theme/src/css.rs",
        "WCAG 2.1 AA",
        "Light/Dark/OLED",
    ] {
        assert!(
            styling_spec.contains(required),
            "styling spec should keep ui-theme contract marker `{}`.",
            required
        );
    }

    for required in [
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "组件模块不存在 + crate root 未导出 + `ui-theme` 仍以 `tokens.rs -> theme.rs -> css.rs` 作为唯一 token 链路",
        "shared_element_transition_ui_theme_boundary_is_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep ui-theme boundary marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_ui_components_boundary_is_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let css_aggregator = load_source("src/css.rs");
    let ui_root = load_source("src/root.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no component assembly files should remain."
    );

    assert!(
        !crate_root.contains("pub mod shared_element_transition;"),
        "ui-components crate root should not expose removed shared_element_transition component."
    );

    for required in [
        "ui-state-primitives + ui-headless + ui-theme",
        "mod css;",
        "mod root;",
        "pub use root::UiRoot;",
        "pub use ui_headless::{MenuItemKind, OnPress};",
        "pub use ui_theme::Theme;",
    ] {
        assert!(
            crate_root.contains(required),
            "ui-components crate root should keep assembly marker `{}`.",
            required
        );
    }

    assert!(
        !crate_root.contains("pub use web_sys"),
        "ui-components public API should not expose web_sys details from crate root."
    );

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
    ] {
        assert!(
            css_aggregator.contains(required),
            "ui-components css aggregation should keep feature-gated marker `{}`.",
            required
        );
    }

    for required in [
        "use ui_headless::{UiI18n, provide_ui_i18n};",
        "use ui_theme::{SemanticOverrides, Theme, css};",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(required),
            "ui-components root should keep composition marker `{}`.",
            required
        );
    }

    for required in [
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "组件目录仅 `check2.md` + crate root 未导出该组件 + `ui-components` 仍在 `lib.rs/root.rs` 组合 `ui-headless` 与 `ui-theme` 并通过 feature gate 聚合公共 API",
        "shared_element_transition_ui_components_boundary_is_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep ui-components boundary marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_api_naming_contract_is_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no API surface file should remain."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );

    for required in [
        "- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。",
        "N/A：`shared_element_transition` 兼容组件已移除，当前无该组件公共 props/回调 API 可发生命名漂移",
        "shared_element_transition_api_naming_contract_is_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep API naming contract marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_section2_state_api_contracts_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no state/api implementation should remain."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );

    for required in [
        "- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。",
        "- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。",
        "- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。",
        "- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。",
        "- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。",
        "- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。",
        "- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。",
        "- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "shared_element_transition_section2_state_api_contracts_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep section2 marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_section3_semantics_and_style_contracts_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let css_aggregator = load_source("src/css.rs");
    let ui_root = load_source("src/root.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no semantics/style implementation should remain."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );

    for required in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
    ] {
        assert!(
            css_aggregator.contains(required),
            "ui-components css aggregation should keep token-first/tree-shaking marker `{}`.",
            required
        );
    }

    assert!(
        ui_root.contains("crate::css::push_components_css(&mut out);"),
        "UiRoot should keep centralized component css injection boundary."
    );

    for required in [
        "- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。",
        "- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。",
        "- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。",
        "- [x] 测试验证“语义契约”而不只验证视觉快照。",
        "- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。",
        "- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。",
        "- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。",
        "- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。",
        "- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。",
        "- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "shared_element_transition_section3_semantics_and_style_contracts_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep section3 marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_section4_platform_performance_contracts_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let motion_lib = load_source("../../crates/ui-motion/src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no platform/perf implementation should remain."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );

    for required in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\");",
    ] {
        assert!(
            headless_lib.contains(required),
            "ui-headless should keep web/ssr mutual exclusion marker `{}`.",
            required
        );
    }

    for required in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            motion_lib.contains(required),
            "ui-motion should keep non-wasm no-op marker `{}`.",
            required
        );
    }

    for required in [
        "- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。",
        "- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。",
        "- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。",
        "- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。",
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。",
        "- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。",
        "- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。",
        "- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "shared_element_transition_section4_platform_performance_contracts_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep section4 marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_section5_file_placement_contracts_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = manifest_dir.join("src");
    let component_dir = src_dir.join("shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata."
    );

    for required in ["lib.rs", "css.rs", "root.rs", "active_highlight.rs"] {
        assert!(
            src_dir.join(required).exists(),
            "ui-components source root should keep required entry file `{}`.",
            required
        );
    }

    for forbidden in ["overlay_open.rs", "presence.rs", "a11y.rs"] {
        assert!(
            !src_dir.join(forbidden).exists(),
            "ui-components source root should not carry forbidden file `{}`.",
            forbidden
        );
    }

    for required in [
        "- [x] `ui-components` 固定入口文件落点正确。",
        "- [x] 组件目录标准文件落点正确。",
        "shared_element_transition_section5_file_placement_contracts_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep section5 marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_section6_agent_contracts_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no agent contract view should remain."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "shared_element_transition_section6_agent_contracts_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep section6 marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_section7_docs_and_e2e_contracts_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata; no docs/e2e implementation should remain."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );

    for required in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "N/A：`shared_element_transition` 兼容组件已移除",
        "shared_element_transition_section7_docs_and_e2e_contracts_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep section7 marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_anti_patterns_are_documented_and_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir.join("src/shared_element_transition");
    let check2_source = load_source("src/shared_element_transition/check2.md");
    let crate_root = load_source("src/lib.rs");
    let state_primitives_root = load_source("../../crates/ui-state-primitives/src/lib.rs");
    let headless_lib = load_source("../../crates/ui-headless/src/lib.rs");

    let mut entries: Vec<String> = fs::read_dir(&component_dir)
        .unwrap_or_else(|e| panic!("read_dir failed for {:?}: {}", component_dir, e))
        .map(|entry| {
            let entry = entry.unwrap_or_else(|e| panic!("DirEntry read failed: {}", e));
            entry.file_name().to_string_lossy().to_string()
        })
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec!["check2.md".to_string()],
        "shared_element_transition directory should only keep checklist metadata."
    );

    assert!(
        !crate_root.contains("shared_element_transition"),
        "ui-components crate root should not expose removed shared_element_transition symbols."
    );
    assert!(
        !state_primitives_root.contains("shared_element_transition"),
        "ui-state-primitives should not carry removed shared_element_transition implementation."
    );
    assert!(
        !headless_lib.contains("shared_element_transition"),
        "ui-headless should not carry removed shared_element_transition implementation."
    );

    for required in [
        "- [x] 在 `status-primitives`（当前 `ui-state-primitives`）写 DOM/样式逻辑。",
        "- [x] 在 `ui-headless` 写视觉和动画编排。",
        "- [x] 在 `view` 层隐藏关键状态决策。",
        "- [x] 新增参数但不纳入统一命名与契约。",
        "- [x] 用并行数组/隐式约定替代显式语义结构（如 `labels + children`）。",
        "- [x] 公共 API 泄露底层实现细节类型。",
        "- [x] 用临时补丁破坏跨组件一致性。",
        "- [x] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。",
        "shared_element_transition_anti_patterns_are_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep anti-pattern marker `{}`.",
            required
        );
    }
}

#[test]
fn shared_element_transition_final_gate_verdict_is_documented_and_enforced() {
    let check2_source = load_source("src/shared_element_transition/check2.md");

    for required in [
        "- [x] 架构正确（边界不破）。",
        "- [x] 行为正确（状态与交互语义成立）。",
        "- [x] 可访问性达标（默认可用）。",
        "- [x] 默认主题美学质量达标（与可访问性同级门禁）。",
        "- [x] 可测试（契约可断言）。",
        "- [x] 可维护（命名和模式一致）。",
        "- [x] 可解释（人和自动化都能读懂）。",
        "- [x] 改动在正确层。",
        "- [x] 命名与全库一致。",
        "- [x] 无效状态被限制或归一化。",
        "- [x] 暴露必要语义标记。",
        "- [x] 覆盖 reduced-motion / SSR / wasm 分支。",
        "- [x] 文档与示例同步更新。",
        "- [x] 门禁完整通过（fmt/clippy/test/smoke 等）。",
        "shared_element_transition_final_gate_verdict_is_documented_and_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "shared_element_transition checklist should keep final gate marker `{}`.",
            required
        );
    }

    assert!(
        !check2_source.contains("- [ ]"),
        "shared_element_transition checklist should not keep unchecked entries after this verification pass."
    );
}
