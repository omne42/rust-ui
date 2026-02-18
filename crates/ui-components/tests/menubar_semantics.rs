use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn menubar_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/menubar/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "Menubar internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn menubar_module_exposes_slot_and_state_contracts() {
    let source = load_source("src/menubar/mod.rs");

    for needle in [
        "pub struct MenubarMenu",
        "pub enum MenubarSlot",
        "pub enum MenuOpenFocusStrategy",
        "pub struct MenubarMenuIds",
        "pub struct MenubarMenuResolved",
        "pub struct MenubarPartStateInput",
        "pub struct MenubarPartState",
        "DEFAULT_ID_BASE",
        "DEFAULT_CLOSE_ON_ACTION",
        "DEFAULT_PLACEMENT",
        "pub use crate::dropdown_menu::DropdownMenuMotion as MenubarMotion;",
    ] {
        assert!(
            source.contains(needle),
            "menubar::mod should include `{needle}` contracts."
        );
    }
}

#[test]
fn menubar_is_exported_from_module_and_crate_root() {
    let module_source = load_source("src/menubar/mod.rs");
    let crate_source = load_source("src/lib.rs");

    assert!(
        module_source.contains("pub use view::Menubar;"),
        "menubar module should export `Menubar`."
    );
    assert!(
        crate_source.contains("pub use menubar::{Menubar, MenubarMenu, MenubarMotion};"),
        "crate root should re-export menubar contracts."
    );
}

#[test]
fn menubar_logic_exposes_state_helpers() {
    let source = load_source("src/menubar/logic.rs");

    for needle in [
        "pub fn state_attr(menu_count: usize, has_open_menu: bool)",
        "pub fn menu_attr(menu_count: usize)",
        "pub fn action_attr(close_on_action: bool)",
        "pub fn open_mode_attr(is_controlled: bool)",
        "pub fn normalize_optional_text(value: Option<String>)",
        "pub fn normalize_id_base(id_base: String)",
        "pub fn resolve_menus(id_base: &str, menus: Vec<MenubarMenu>)",
        "pub fn sanitize_open_index_for_menus(",
        "pub fn next_enabled_menu_index(",
        "pub fn resolve_state(input: MenubarPartStateInput) -> MenubarPartState",
        "pub fn compose_class_name(base_class_name: Option<String>, state: MenubarPartState)",
    ] {
        assert!(
            source.contains(needle),
            "Menubar logic should include `{needle}` for centralized state/source contracts."
        );
    }
}

#[test]
fn menubar_view_uses_logic_contracts_and_source_markers() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_menus(&id_base.get_value(), menus)",
        "logic::resolve_state(MenubarPartStateInput {",
        "slot: MenubarSlot::Root",
        "logic::compose_class_name(class_name.get_value(), root_state_for_class.get())",
        "data-slot=move || root_state.get().slot_attr",
        "data-state=move || root_state.get().state_attr",
        "data-menus=move || root_state.get().menu_attr",
        "data-action-mode=move || root_state.get().action_attr",
        "data-open-mode=move || root_state.get().open_mode_attr",
        "data-id-source=move || root_state.get().id_source_attr",
        "data-class-source=move || root_state.get().class_source_attr",
        "data-close-on-action-source=move || root_state.get().close_on_action_source_attr",
        "data-placement-source=move || root_state.get().placement_source_attr",
        "data-open-index-source=move || root_state.get().open_index_source_attr",
        "data-default-open-index-source=move || root_state.get().default_open_index_source_attr",
        "data-open-index-change-source=move || root_state.get().open_index_change_source_attr",
        "data-motion-source=move || root_state.get().motion_source_attr",
        "data-custom-id=move || root_state.get().has_custom_id_base.then_some(\"true\")",
        "data-custom-open-index=move || root_state.get().has_custom_open_index.then_some(\"true\")",
        "data-custom-motion=move || root_state.get().has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            source.contains(needle),
            "Menubar view should include `{needle}` for stable state/source marker contracts."
        );
    }
}

#[test]
fn menubar_supports_controlled_and_uncontrolled_open_state() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "open_index: Option<Signal<Option<usize>>>",
        "default_open_index: Option<usize>",
        "on_open_index_change: Option<Callback<Option<usize>>>",
        "overlay_open::use_controllable_state(",
        "let has_custom_open_index = open_index.is_some()",
        "let has_custom_default_open_index = default_open_index.is_some()",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should support `{needle}` for controllable open index state."
        );
    }
}

#[test]
fn menubar_exposes_keyboard_and_trigger_contracts() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "role=\"menubar\"",
        "role=\"menuitem\"",
        "on:keydown=on_key_down",
        "on:pointerenter=on_pointer_enter",
        "crate::menubar::focus_strategy_for_open_key(&key)",
        "logic::next_enabled_menu_index(menus.get_value().as_ref(), index, 1)",
        "logic::next_enabled_menu_index(menus.get_value().as_ref(), index, -1)",
        "focus_trigger(&trigger_refs, next_index);",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should wire `{needle}` to match menubar keyboard + pointer semantics."
        );
    }
}

#[test]
fn menubar_renders_menu_in_popover_with_presence_and_motion() {
    let source = load_source("src/menubar/view.rs");

    for needle in [
        "use_presence(open)",
        "<Popover",
        "motion=motion.popover",
        "is_modal=false",
        "<Menu",
        "on_exit_complete=presence.finish_exit",
    ] {
        assert!(
            source.contains(needle),
            "Menubar should compose popover/presence/menu via `{needle}` for motion-safe unmounting."
        );
    }
}

#[test]
fn menubar_styles_include_state_and_source_markers() {
    let source = load_source("src/menubar/styles.rs");

    for needle in [
        ".ui-menubar {",
        ".ui-menubar__trigger {",
        ".ui-menubar--open",
        ".ui-menubar[data-state=\"open\"]",
        ".ui-menubar--persistent",
        ".ui-menubar[data-action-mode=\"keep-open\"]",
        ".ui-menubar[data-open-mode=\"controlled\"]",
        ".ui-menubar--custom-id",
        ".ui-menubar[data-id-source=\"custom\"]",
        ".ui-menubar[data-class-source=\"custom\"]",
        ".ui-menubar--custom-close-on-action",
        ".ui-menubar[data-close-on-action-source=\"custom\"]",
        ".ui-menubar--custom-open-index",
        ".ui-menubar[data-open-index-source=\"custom\"]",
        ".ui-menubar--custom-open-index-change",
        ".ui-menubar[data-open-index-change-source=\"custom\"]",
        ".ui-menubar[data-motion-source=\"custom\"]",
        ".ui-menubar[data-custom-motion=\"true\"]",
    ] {
        assert!(
            source.contains(needle),
            "Menubar styles should include `{needle}` as stable state/source contracts."
        );
    }
}

#[test]
fn menubar_uses_dropdown_menu_motion_alias_contract() {
    let mod_source = load_source("src/menubar/mod.rs");
    let dropdown_motion_source = load_source("src/dropdown_menu/motion.rs");

    for needle in [
        "pub use crate::dropdown_menu::DropdownMenuMotion as MenubarMotion;",
        "pub struct DropdownMenuMotion",
        "pub popover: PopoverMotion",
    ] {
        assert!(
            mod_source.contains(needle) || dropdown_motion_source.contains(needle),
            "Menubar motion alias contract should include `{needle}` for baseline-style spring customization."
        );
    }
}

#[test]
fn menubar_docs_page_contains_state_source_playground() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn menubar() -> AnyView",
        "title=\"Menubar\"",
        "slug=\"menubar\"",
        "State + Source Markers",
        "data-id-source",
        "data-class-source",
        "data-close-on-action-source",
        "data-open-index-source",
        "data-motion-source",
        "<Menubar",
    ] {
        assert!(
            source.contains(needle),
            "Menubar docs page should contain `{needle}`."
        );
    }
}

#[test]
fn menubar_docs_controlled_state_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Controlled Open + Persistent + Disabled Menu\"",
        "id_base=\"docs-menubar-controlled\".to_string()",
        "close_on_action=false",
        "open_index=controlled_open",
        "on_open_index_change=on_open_index_change",
        "open menu index:",
    ] {
        assert!(
            source.contains(needle),
            "Menubar docs controlled-state playground should contain `{needle}`.",
        );
    }
}

#[test]
fn menubar_docs_state_source_playground_locks_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"State + Source Markers\"",
        "id_base=\"docs-menubar-markers\".to_string()",
        "close_on_action=false",
        "placement=ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()",
        "open_index=marker_open",
        "default_open_index=1",
        "on_open_index_change=on_marker_open_change",
        "class_name=\"docs-menubar-custom\".to_string()",
        "let marker_motion = ui_components::MenubarMotion {",
        "initial_scale: 0.94",
        "offset_y_px: 10.0",
        "motion=marker_motion",
        "Inspect data-id-source / data-class-source / data-close-on-action-source / data-open-index-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "Menubar docs state/source playground should contain `{needle}`.",
        );
    }
}

#[test]
fn menubar_docs_page_covers_primary_playgrounds() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "pub(super) fn menubar() -> AnyView",
        "title=\"Menubar\"",
        "slug=\"menubar\"",
        "description=\"baseline-compatible persistent menubar with horizontal trigger navigation, baseline-style state/source attrs, and baseline-level spring popover motion reuse.\"",
        "<Playground title=\"Desktop Menubar + Action Dispatch\" code_signal=code>",
        "<Playground title=\"Controlled Open + Persistent + Disabled Menu\" code_signal=states_code>",
        "<Playground title=\"State + Source Markers\" code_signal=marker_code>",
        "<Menubar",
    ] {
        assert!(
            source.contains(needle),
            "collections_command docs should include `{needle}` for menubar primary playground coverage.",
        );
    }
}

#[test]
fn menubar_docs_playgrounds_lock_state_matrix_contract_values() {
    let source =
        load_source("../../apps/docs-app/src/pages/components/pages/collections_command.rs");

    for needle in [
        "title=\"Desktop Menubar + Action Dispatch\"",
        "id_base=\"docs-menubar-default\".to_string()",
        "last action (menu:item): ",
        "title=\"Controlled Open + Persistent + Disabled Menu\"",
        "id_base=\"docs-menubar-controlled\".to_string()",
        "close_on_action=false",
        "open_index=controlled_open",
        "on_open_index_change=on_open_index_change",
        "class_name=\"docs-menubar-custom\".to_string()",
        "open menu index: ",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-menubar-markers\".to_string()",
        "placement=ui_components::menubar::DEFAULT_PLACEMENT.flip_vertical()",
        "open_index=marker_open",
        "default_open_index=1",
        "on_open_index_change=on_marker_open_change",
        "let marker_motion = ui_components::MenubarMotion {",
        "initial_scale: 0.94",
        "offset_y_px: 10.0",
        "motion=marker_motion",
        "Inspect data-id-source / data-class-source / data-close-on-action-source / data-open-index-source / data-motion-source in DevTools.",
    ] {
        assert!(
            source.contains(needle),
            "menubar docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn menubar_check2_marks_architecture_layer_definitions_complete() {
    let check2_source = load_source("src/menubar/check2.md");

    for needle in [
        "- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。",
        "- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。",
        "- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。",
        "- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。",
        "- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。",
    ] {
        assert!(
            check2_source.contains(needle),
            "menubar/check2.md should keep architecture-layer marker `{needle}`.",
        );
    }
}

#[test]
fn menubar_check2_marks_semantics_first_testing_complete() {
    let check2_source = load_source("src/menubar/check2.md");

    for needle in [
        "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。",
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
    ] {
        assert!(
            check2_source.contains(needle),
            "menubar/check2.md should keep semantics-first marker `{needle}`.",
        );
    }
}

#[test]
fn menubar_check2_marks_final_merge_gates_complete() {
    let check2_source = load_source("src/menubar/check2.md");

    for needle in [
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
    ] {
        assert!(
            check2_source.contains(needle),
            "menubar/check2.md should keep final-gate marker `{needle}`.",
        );
    }
}

#[test]
fn menubar_check2_has_no_remaining_unchecked_items() {
    let check2_source = load_source("src/menubar/check2.md");
    assert!(
        !check2_source.contains("- [ ]"),
        "menubar/check2.md should not keep unchecked checklist items once governance is complete."
    );
}
