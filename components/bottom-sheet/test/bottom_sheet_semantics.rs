use std::fs;
use std::path::Path;

fn resolve_path(rel_path: &str) -> std::path::PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    if let Some(suffix) = rel_path.strip_prefix("src/bottom_sheet/") {
        let workspace_dir = manifest_dir
            .parent()
            .and_then(Path::parent)
            .unwrap_or_else(|| {
                panic!("workspace root should be two levels above {manifest_dir:?}")
            });
        return workspace_dir
            .join("components/bottom-sheet/src")
            .join(suffix);
    }

    manifest_dir.join(rel_path)
}

fn load_source(rel_path: &str) -> String {
    let path = resolve_path(rel_path);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn bottom_sheet_does_not_expose_logic_or_view_modules() {
    let source = load_source("src/bottom_sheet/mod.rs");

    for needle in ["pub mod logic", "pub mod view"] {
        assert!(
            !source.contains(needle),
            "BottomSheet internals should stay private; found `{needle}`."
        );
    }
}

#[test]
fn bottom_sheet_files_follow_single_responsibility_boundaries() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use logic::{BottomSheetState, BottomSheetStateInput, DEFAULT_CLOSE_LABEL};",
        "pub use motion::BottomSheetMotion;",
        "pub use view::BottomSheet;",
    ] {
        assert!(
            mod_source.contains(needle),
            "BottomSheet mod.rs should keep a minimal export boundary (`{needle}`).",
        );
    }

    for forbidden in ["fn ", "struct ", "enum ", "impl "] {
        assert!(
            !mod_source.contains(forbidden),
            "BottomSheet mod.rs should not carry implementation details (`{forbidden}`).",
        );
    }

    for needle in [
        "pub fn resolve_title(",
        "pub fn resolve_close_label(",
        "pub struct BottomSheetDeriveInput",
        "pub fn derive_view_state(",
    ] {
        assert!(
            logic_source.contains(needle),
            "BottomSheet logic.rs should own normalization/derivation responsibilities (`{needle}`).",
        );
    }

    for forbidden in [
        "view!",
        "<div",
        "data-slot",
        "on:click",
        "style=",
        "ui-bottom-sheet__",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic.rs should not contain DOM/style implementation details (`{forbidden}`).",
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "var(--ui-space-",
        "var(--ui-fg",
    ] {
        assert!(
            styles_source.contains(needle),
            "BottomSheet styles.rs should stay token-first static CSS (`{needle}`).",
        );
    }

    for forbidden in ["view!", "Signal<", "Callback<", "on:click", "on:keydown"] {
        assert!(
            !styles_source.contains(forbidden),
            "BottomSheet styles.rs should not include runtime or interaction logic (`{forbidden}`).",
        );
    }

    for needle in [
        "pub fn BottomSheet(",
        "<Sheet",
        "logic::derive_view_state(logic::BottomSheetDeriveInput {",
        "data-state=state.state_attr",
        "motion=motion.sheet",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view.rs should compose structure + semantic mounts (`{needle}`).",
        );
    }

    for forbidden in [
        "resolve_state(BottomSheetStateInput {",
        "pub fn derive_view_state(",
        "request_animation_frame",
        "spring(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view.rs should not re-implement primitive state rules or motion engine (`{forbidden}`).",
        );
    }

    for needle in [
        "pub struct BottomSheetMotion",
        "pub fn sanitize_motion(motion: BottomSheetMotion) -> BottomSheetMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
    ] {
        assert!(
            motion_source.contains(needle),
            "BottomSheet motion.rs should map component semantics onto shared motion contract (`{needle}`).",
        );
    }

    for forbidden in [
        "request_animation_frame",
        "keyframes",
        "spring(",
        "web_sys::",
        "js_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion.rs should not embed shared motion engine implementation (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_does_not_introduce_spec_rs_for_non_complex_scope() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let protocol_source = load_source("src/bottom_sheet/protocol.rs");
    let check_source = load_source("../../components/bottom-sheet/check2.md");
    let spec_path = resolve_path("src/bottom_sheet/spec.rs");

    assert!(
        !spec_path.exists(),
        "BottomSheet should not introduce `spec.rs` without complex schema/builder requirements; found {:?}.",
        spec_path
    );

    for forbidden in ["mod spec", "pub mod spec", "pub use spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "BottomSheet mod.rs should not export a spec module in non-complex scope (`{forbidden}`).",
        );
    }

    for forbidden in ["Spec::new(", "BottomSheetSpec", "struct Spec"] {
        assert!(
            !view_source.contains(forbidden)
                && !logic_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "BottomSheet should not expose spec-builder entrypoints in component modules (`{forbidden}`).",
        );
    }

    assert!(
        check_source.contains("`spec.rs` 只用于少数复杂组件（如 button），避免泛滥"),
        "BottomSheet checklist should document why `spec.rs` is intentionally not introduced for this component.",
    );
}

#[test]
fn bottom_sheet_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let protocol_source = load_source("src/bottom_sheet/protocol.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");
    let spec_path = resolve_path("src/bottom_sheet/spec.rs");

    assert!(
        !spec_path.exists(),
        "BottomSheet should keep Hyper-Structure Builder path as N/A in non-complex scope."
    );

    for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::", "spec::"] {
        assert!(
            !mod_source.contains(forbidden),
            "BottomSheet module boundary should not expose spec module token `{forbidden}`."
        );
    }

    for forbidden in ["Spec::new(", "BottomSheetSpec", "struct Spec"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !motion_source.contains(forbidden)
                && !protocol_source.contains(forbidden),
            "BottomSheet should not expose Hyper-Structure builder token `{forbidden}` in non-complex scope."
        );
    }

    assert!(
        check2_source.contains("N/A-by-design：`BottomSheet` 不是复杂配置固化型组件"),
        "BottomSheet check2 should keep explicit N/A rationale for Hyper-Structure Builder item."
    );
}

#[test]
fn bottom_sheet_uses_logic_state_model() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/bottom_sheet.rs");

    assert!(
        mod_source.contains(
            "pub use logic::{BottomSheetState, BottomSheetStateInput, DEFAULT_CLOSE_LABEL};"
        ),
        "BottomSheet module should re-export state contracts from logic."
    );

    for needle in [
        "pub use ui_state_primitives::bottom_sheet::{",
        "BottomSheetState",
        "BottomSheetStateInput",
        "pub enum BottomSheetVisibility",
        "pub enum BottomSheetAttachment",
        "normalize_optional_text",
        "resolve_title",
        "resolve_close_label",
        "resolve_description_text",
        "resolve_handle_visibility",
        "resolve_close_button_visibility",
        "resolve_attachment",
        "resolve_detached",
        "resolve_bottom_inset_px",
        "resolve_dismissable",
        "resolve_keyboard_dismiss_disabled",
        "resolve_on_exit_complete",
        "has_slot",
        "pub struct BottomSheetDeriveInput",
        "pub struct BottomSheetDerivedState",
        "pub fn derive_view_state(",
        "resolve_state",
        "compose_class_name",
    ] {
        assert!(
            logic_source.contains(needle),
            "BottomSheet logic should source primitive state contracts from ui-state-primitives; missing `{needle}`."
        );
    }

    for needle in [
        "pub struct BottomSheetStateInput",
        "pub struct BottomSheetState",
        "pub fn normalize_optional_text(",
        "pub fn normalize_required_text(",
        "pub fn normalize_id_base(",
        "pub fn normalize_bottom_inset_px(",
        "pub fn resolve_state(",
        "pub fn compose_class_name(",
    ] {
        assert!(
            primitive_source.contains(needle),
            "BottomSheet state primitive should include `{needle}` in ui-state-primitives."
        );
    }

    for needle in [
        "logic::normalize_id_base(id_base)",
        "logic::resolve_title(title)",
        "logic::normalize_optional_text(description)",
        "logic::resolve_description_text(description.get_value())",
        "logic::resolve_bottom_inset_px(bottom_inset_px)",
        "let handle_visibility = logic::resolve_handle_visibility(is_handle_visible, show_handle);",
        "let close_button_visibility =",
        "logic::resolve_close_button_visibility(is_close_button_visible, show_close_button);",
        "let attachment = logic::resolve_attachment(is_detached, detached);",
        "logic::resolve_close_label(close_label)",
        "logic::resolve_dismissable(is_dismissable)",
        "logic::resolve_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled)",
        "logic::resolve_on_exit_complete(on_exit_complete)",
        "logic::derive_view_state(logic::BottomSheetDeriveInput {",
        "has_description: logic::has_slot(&description.get_value())",
        "has_footer: logic::has_slot(&footer.get_value())",
        "handle_visibility,",
        "close_button_visibility,",
        "attachment,",
        "has_custom_motion",
        "let state = derived_state.state;",
        "logic::compose_class_name(class_name, state)",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view should derive state via logic helpers; missing `{needle}`."
        );
    }
}

#[test]
fn bottom_sheet_composes_sheet_with_bottom_placement_and_motion_contract() {
    let source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "<Sheet",
        "placement=SheetPlacement::Bottom",
        "is_dismissable=is_dismissable",
        "is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled",
        "motion=motion.sheet",
        "on_exit_complete=on_exit_complete",
    ] {
        assert!(
            source.contains(needle),
            "BottomSheet should compose Sheet with stable overlay + motion contracts (`{needle}`)."
        );
    }
}

#[test]
fn bottom_sheet_api_naming_contract_prefers_is_on_prefix_and_keeps_compat_aliases() {
    let source = load_source("src/bottom_sheet/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "#[prop(optional)] is_handle_visible: Option<bool>",
        "#[prop(optional)] is_close_button_visible: Option<bool>",
        "#[prop(optional)] is_detached: Option<bool>",
        "#[prop(optional)] close_label: Option<&'static str>",
        "#[prop(optional)] bottom_inset_px: Option<f64>",
        "#[prop(optional)] is_dismissable: Option<bool>",
        "#[prop(optional)] is_keyboard_dismiss_disabled: Option<bool>",
        "let handle_visibility = logic::resolve_handle_visibility(is_handle_visible, show_handle);",
        "let close_button_visibility =",
        "logic::resolve_close_button_visibility(is_close_button_visible, show_close_button);",
        "let attachment = logic::resolve_attachment(is_detached, detached);",
        "let close_label = logic::resolve_close_label(close_label);",
        "let bottom_inset_px = logic::resolve_bottom_inset_px(bottom_inset_px);",
        "let is_dismissable = logic::resolve_dismissable(is_dismissable);",
        "let is_keyboard_dismiss_disabled =",
        "logic::resolve_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled);",
        "show_handle: handle_visibility.is_visible()",
        "show_close_button: close_button_visibility.is_visible()",
        "detached: attachment.is_detached()",
    ] {
        assert!(
            source.contains(needle),
            "BottomSheet API naming/compat contract should include `{needle}`.",
        );
    }

    for forbidden in [
        "on_open_change",
        "default_open",
        "onOpenChange",
        "defaultOpen",
    ] {
        assert!(
            !source.contains(forbidden),
            "BottomSheet should avoid cross-component alias drift (`{forbidden}`).",
        );
    }

    for needle in ["is_detached=true", "is_close_button_visible=false"] {
        assert!(
            docs_source.contains(needle),
            "BottomSheet docs should use canonical naming (`{needle}`).",
        );
    }
}

#[test]
fn bottom_sheet_control_contract_is_controlled_surface_with_primitive_uncontrolled_pair() {
    let view_source = load_source("src/bottom_sheet/view.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/overlay_trigger.rs");

    for needle in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "<Sheet",
        "open=open",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet should keep controlled open contract (`{needle}`).",
        );
    }

    for forbidden in ["default_open", "on_open_change", "set_open(", "signal("] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet should not own uncontrolled open state machine (`{forbidden}`).",
        );
    }

    for needle in [
        "pub struct OverlayTriggerStateOptions {",
        "pub default_open: Option<bool>,",
        "pub on_open_change: Option<OverlayOnOpenChange>,",
        "default_value: options.default_open,",
        "on_change: options.on_open_change,",
    ] {
        assert!(
            primitive_source.contains(needle),
            "Uncontrolled pair should be provided by ui-state-primitives overlay_trigger (`{needle}`).",
        );
    }
}

#[test]
fn bottom_sheet_state_primitive_source_stays_decoupled_from_component_store_state_machines() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "pub use ui_state_primitives::bottom_sheet::{",
        "BottomSheetStateInput",
        "resolve_state(BottomSheetStateInput {",
    ] {
        assert!(
            logic_source.contains(needle),
            "BottomSheet logic should source reusable state contracts from ui-state-primitives (`{needle}`).",
        );
    }

    for forbidden in [
        "OverlayTriggerStateOptions",
        "default_open",
        "on_open_change",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "Signal<",
        "web_sys::",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should stay as mapping-only layer, not local state/store machine (`{forbidden}`).",
        );
    }

    for needle in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "<Sheet",
        "open=open",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view should consume controlled open state from caller (`{needle}`).",
        );
    }

    for forbidden in [
        "OverlayTriggerStateOptions",
        "default_open",
        "on_open_change",
        "create_signal(",
        "RwSignal",
        "ReadSignal",
        "WriteSignal",
        "web_sys::",
        "apps::",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not bind business stores or rebuild uncontrolled primitives (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_async_loading_protocol_surface() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "use_async_action",
        "is_loading",
        "aria-busy",
        "retry",
        "on_retry",
        "error_message",
        "data-loading",
        "data-error",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic is sync-only and should not define async loading/error protocol (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view is sync-only and should not expose async loading/error protocol (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not claim async loading/error protocol for a sync-only component (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_emits_baseline_style_state_data_attributes() {
    let source = load_source("src/bottom_sheet/view.rs");

    for attr in [
        "data-slot=\"bottom-sheet\"",
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-handle=state.handle_attr",
        "data-close-button=state.close_button_attr",
        "data-detached=state.detached_attr",
        "data-custom-class=state.has_custom_class_name.then_some(\"true\")",
        "data-motion-source=derived_state.motion_source_attr",
        "data-custom-motion=derived_state.has_custom_motion.then_some(\"true\")",
        "data-slot=\"bottom-sheet-handle\"",
        "data-slot=\"bottom-sheet-title\"",
        "data-slot=\"bottom-sheet-description\"",
        "data-slot=\"bottom-sheet-body\"",
        "data-slot=\"bottom-sheet-footer\"",
        "data-bottom-inset=state.inset_attr",
    ] {
        assert!(
            source.contains(attr),
            "BottomSheet should expose `{attr}` for baseline-style state inspection and styling."
        );
    }
}

#[test]
fn bottom_sheet_state_markers_cover_axes_sources_and_closed_value_sets() {
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let sheet_logic_source = load_source("src/sheet/logic.rs");

    for marker in [
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-handle=state.handle_attr",
        "data-close-button=state.close_button_attr",
        "data-detached=state.detached_attr",
        "data-bottom-inset=state.inset_attr",
        "data-motion-source=derived_state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            bottom_sheet_view_source.contains(marker),
            "BottomSheet should expose stable state/source marker `{marker}` for selector-based verification.",
        );
    }

    for marker in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-placement=root_state.placement_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "Sheet should provide stable observable markers for open/close and source dimensions (`{marker}`).",
        );
    }

    for marker in [
        "if is_open { \"open\" } else { \"closed\" }",
        "if is_dismissable {",
        "\"dismissable\"",
        "\"locked\"",
        "if is_keyboard_dismiss_disabled {",
        "\"disabled\"",
        "\"enabled\"",
        "SheetPlacement::Bottom => \"bottom\"",
        "SheetPlacement::Left => \"left\"",
        "SheetPlacement::Right => \"right\"",
        "source_axis_attr: \"default|custom\"",
        "dismiss_source_attr: if input.is_dismissable == DEFAULT_DISMISSABLE {",
        "keyboard_dismiss_source_attr: if input.is_keyboard_dismiss_disabled",
        "aria_labelledby_source_attr: if input.has_custom_aria_labelledby {",
        "aria_describedby_source_attr: if input.has_custom_aria_describedby {",
        "motion_source_attr: if input.has_custom_motion {",
    ] {
        assert!(
            sheet_logic_source.contains(marker),
            "Sheet logic should keep marker values enumerable and closed-set (`{marker}`).",
        );
    }
}

#[test]
fn bottom_sheet_type_system_and_semantic_markers_form_machine_readable_contract() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/bottom_sheet.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "pub enum BottomSheetVisibility",
        "pub enum BottomSheetAttachment",
        "pub struct BottomSheetDeriveInput",
        "pub handle_visibility: BottomSheetVisibility",
        "pub close_button_visibility: BottomSheetVisibility",
        "pub attachment: BottomSheetAttachment",
        "pub fn derive_view_state(input: BottomSheetDeriveInput) -> BottomSheetDerivedState",
        "pub fn resolve_handle_visibility(",
        "pub fn resolve_close_button_visibility(",
        "pub fn resolve_attachment(",
    ] {
        assert!(
            logic_source.contains(needle),
            "BottomSheet logic should keep discrete axes in typed contracts (`{needle}`).",
        );
    }

    for forbidden in [
        "pub handle_visibility: String",
        "pub close_button_visibility: String",
        "pub attachment: String",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet typed state axes should not regress into free-form string protocol (`{forbidden}`).",
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-handle=state.handle_attr",
        "data-close-button=state.close_button_attr",
        "data-detached=state.detached_attr",
        "data-bottom-inset=state.inset_attr",
        "data-motion-source=derived_state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(marker),
            "BottomSheet view should expose machine-readable semantic marker `{marker}`.",
        );
    }

    for closed_value in [
        "\"with-description\"",
        "\"title-only\"",
        "\"present\"",
        "\"absent\"",
        "\"shown\"",
        "\"hidden\"",
        "\"true\"",
        "\"false\"",
        "\"none\"",
        "\"sm\"",
        "\"md\"",
        "\"lg\"",
        "\"xl\"",
        "\"default\"",
        "\"custom\"",
    ] {
        assert!(
            primitive_source.contains(closed_value),
            "BottomSheet primitive should keep closed marker value set (`{closed_value}`).",
        );
    }
}

#[test]
fn bottom_sheet_focus_restoration_uses_global_stack_and_policy_chain() {
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");

    for needle in [
        "use_focus_trap(FocusTrapOptions::enabled(panel_ref))",
        "use_overlay_stack_registration()",
        "is_topmost.get()",
        "on:keydown=on_key_down",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "Sheet should wire focus/overlay stack contract via `{needle}`.",
        );
    }

    for needle in [
        "static FOCUS_MANAGER_STACK",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap(",
        "RestorePolicy::Selector(",
        "RestorePolicy::FallbackTo(",
        "derive_restore_policy(",
        "restore_focus_chain(",
        "if let Some(body) = document.body() {",
    ] {
        assert!(
            focus_trap_source.contains(needle),
            "ui-headless focus trap should keep global stack/policy-based restore chain (`{needle}`).",
        );
    }

    for forbidden in [
        "previous_focus: NodeRef",
        "restore_target_ref: NodeRef",
        "focus_restore_ref: NodeRef",
    ] {
        assert!(
            !focus_trap_source.contains(forbidden),
            "Focus restore target should not be persisted as private NodeRef (`{forbidden}`).",
        );
    }

    for forbidden in [
        "let previous_focus_ref: NodeRef",
        "let restore_target_ref: NodeRef",
        "document.body().unwrap().focus()",
    ] {
        assert!(
            !bottom_sheet_view_source.contains(forbidden) && !sheet_view_source.contains(forbidden),
            "BottomSheet/Sheet should not implement ad-hoc NodeRef/body-focus restore path (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_styles_use_defensive_variable_fallback_chain() {
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let theme_css_source = load_source("../ui-theme/src/css.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-border-width, var(--ui-fallback-border-width))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-lg, var(--ui-fallback-radius-lg))",
        "var(--ui-shadow-md, var(--ui-fallback-shadow-md))",
        "var(--ui-fg-muted, var(--ui-fallback-fg-muted))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md)))",
        "var(--ui-space-3xs, var(--ui-fallback-space-3xs, var(--ui-fallback-space-2xs)))",
        "var(--ui-font-size-150, var(--ui-fallback-font-size-150))",
        "var(--ui-line-height-150, var(--ui-fallback-line-height-150))",
        "var(--ui-accent, var(--ui-fallback-accent))",
    ] {
        assert!(
            styles_source.contains(required),
            "BottomSheet styles should keep defensive variable fallback chain `{required}`.",
        );
    }

    for forbidden in [
        "var(--ui-border-width, 1px)",
        "var(--ui-heading-h5-font-size, 16px)",
        "var(--ui-heading-h5-line-height, 24px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "outline: 1px solid",
        "#",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "BottomSheet styles should avoid hardcoded terminal fallback `{forbidden}`.",
        );
    }

    for required in [
        "--ui-fallback-fg:",
        "--ui-fallback-fg-muted:",
        "--ui-fallback-bg:",
        "--ui-fallback-border:",
        "--ui-fallback-border-width:",
        "--ui-fallback-accent:",
        "--ui-fallback-radius-lg:",
        "--ui-fallback-space-2xs:",
        "--ui-fallback-space-xs:",
        "--ui-fallback-space-sm:",
        "--ui-fallback-space-md:",
        "--ui-fallback-shadow-md:",
        "--ui-fallback-font-size-150:",
        "--ui-fallback-line-height-150:",
    ] {
        assert!(
            theme_css_source.contains(required),
            "ui-theme css output should provide bottom-sheet fallback variable `{required}`.",
        );
    }

    for required in [
        "- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。",
        "bottom_sheet_styles_use_defensive_variable_fallback_chain",
    ] {
        assert!(
            check2_source.contains(required),
            "bottom-sheet checklist should keep defensive-variables evidence `{required}`.",
        );
    }
}

#[test]
fn bottom_sheet_defensive_variables_check_script_covers_style_fallback_contract() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_styles_use_defensive_variable_fallback_chain";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn bottom_sheet_styles_include_state_marker_contracts() {
    let source = load_source("src/bottom_sheet/styles.rs");

    for selector in [
        ".ui-bottom-sheet[data-motion-source=\"custom\"]",
        ".ui-bottom-sheet[data-custom-motion=\"true\"]",
        ".ui-bottom-sheet--detached",
        ".ui-bottom-sheet[data-detached=\"false\"]",
        ".ui-bottom-sheet--inset-md",
        ".ui-bottom-sheet__handle-bar",
        ".ui-bottom-sheet--close-shown .ui-bottom-sheet__header",
        ".ui-bottom-sheet[data-close-button=\"shown\"] .ui-bottom-sheet__header",
        ".ui-bottom-sheet[data-footer=\"present\"] .ui-bottom-sheet__footer",
        ".ui-bottom-sheet--title-only .ui-bottom-sheet__description",
        ".ui-bottom-sheet--custom-class",
    ] {
        assert!(
            source.contains(selector),
            "BottomSheet styles should include `{selector}` as stable state-marker contracts."
        );
    }
}

#[test]
fn bottom_sheet_styles_avoid_structural_guessing_and_runtime_inline_style_logic() {
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for selector in [
        ".ui-bottom-sheet[data-handle=\"hidden\"] .ui-bottom-sheet__handle",
        ".ui-bottom-sheet[data-close-button=\"hidden\"] .ui-bottom-sheet__close",
        ".ui-bottom-sheet[data-state=\"title-only\"] .ui-bottom-sheet__description",
        ".ui-bottom-sheet[data-footer=\"present\"] .ui-bottom-sheet__footer",
        ".ui-bottom-sheet[data-detached=\"true\"]",
        ".ui-bottom-sheet[data-bottom-inset=\"md\"]",
    ] {
        assert!(
            styles_source.contains(selector),
            "BottomSheet styles should use explicit semantic selectors (`{selector}`) for visual state transitions.",
        );
    }

    for forbidden in [
        ":nth-child(",
        ":nth-of-type(",
        ":first-child",
        ":last-child",
        ":has(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "BottomSheet styles should not depend on fragile structural selectors (`{forbidden}`).",
        );
    }

    for forbidden in ["style=", "style:"] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not encode runtime business style logic via inline style (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions()
{
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let sheet_logic_source = load_source("src/sheet/logic.rs");
    let sheet_logic_test_source = load_source("../../components/sheet/test/logic.rs");
    let overlay_trigger_source =
        load_source("../../crates/ui-state-primitives/src/overlay_trigger.rs");
    let semantics_test_source = load_source("tests/bottom_sheet_semantics.rs");

    for marker in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "data-state=state.state_attr",
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-motion-source=derived_state.motion_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
    ] {
        assert!(
            bottom_sheet_view_source.contains(marker) || sheet_view_source.contains(marker),
            "BottomSheet semantic contract coverage should include marker `{marker}`."
        );
    }

    for marker in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "<Sheet",
        "open=open",
    ] {
        assert!(
            bottom_sheet_view_source.contains(marker),
            "BottomSheet should cover controlled-usage branch semantics (`{marker}`).",
        );
    }

    for marker in [
        "pub struct OverlayTriggerStateOptions {",
        "pub default_open: Option<bool>,",
        "pub on_open_change: Option<OverlayOnOpenChange>,",
    ] {
        assert!(
            overlay_trigger_source.contains(marker),
            "Primitive layer should cover uncontrolled pair for overlay open state (`{marker}`).",
        );
    }

    for marker in [
        "dismiss_attr(true), \"dismissable\"",
        "dismiss_attr(false), \"locked\"",
        "keyboard_dismiss_attr(false), \"enabled\"",
        "keyboard_dismiss_attr(true), \"disabled\"",
        "data-dismiss=root_state.dismiss_attr",
        "data-keyboard-dismiss=root_state.keyboard_dismiss_attr",
    ] {
        assert!(
            sheet_logic_test_source.contains(marker)
                || sheet_logic_source.contains(marker)
                || sheet_view_source.contains(marker),
            "Disabled/locked branch semantics should be covered (`{marker}`).",
        );
    }

    for marker in [
        "on:keydown=on_key_down",
        "if logic::should_close_on_escape(",
        "fn should_close_on_escape_requires_topmost_non_composing_non_prevented_escape()",
    ] {
        assert!(
            sheet_view_source.contains(marker)
                || sheet_logic_source.contains(marker)
                || sheet_logic_test_source.contains(marker),
            "Keyboard-path semantics should be covered (`{marker}`).",
        );
    }

    for marker in [
        "on:click=move |_| {",
        "if is_dismissable {",
        "on:pointerdown=move |ev| ev.stop_propagation()",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "Pointer-path semantics should be covered (`{marker}`).",
        );
    }

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "SSR/wasm semantic path split should be explicit (`{marker}`).",
        );
    }

    for forbidden in ["assert_snapshot!", "insta::assert_", "to_match_snapshot"] {
        assert!(
            !semantics_test_source.contains(forbidden),
            "BottomSheet semantics tests should not rely on snapshot-only assertions (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_close_button_contracts_are_preserved() {
    let source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "data-slot=\"bottom-sheet-close\"",
        "<Button",
        "aria_label=close_label",
        "on_press=on_close",
    ] {
        assert!(
            source.contains(needle),
            "BottomSheet should preserve close button contracts (`{needle}`)."
        );
    }
}

#[test]
fn bottom_sheet_a11y_and_i18n_contracts_are_wired_and_overridable() {
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let sheet_source = load_source("src/sheet/view.rs");
    let headless_a11y_source = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "title: String",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] close_label: Option<&'static str>",
        "let title = logic::resolve_title(title);",
        "let close_label = logic::resolve_close_label(close_label);",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
        "lang=lang.clone()",
        "lang=lang",
        "dir=dir",
        "aria_label=close_label",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view should expose i18n/a11y override surface (`{needle}`).",
        );
    }

    for forbidden in ["Close bottom sheet", "Bottom sheet"] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not hardcode user-visible fallback copy (`{forbidden}`); fallback belongs in logic.",
        );
    }

    for needle in [
        "pub const DEFAULT_TITLE: &str = \"Bottom sheet\";",
        "pub const DEFAULT_CLOSE_LABEL: &str = \"Close bottom sheet\";",
        "pub fn resolve_title(value: String) -> String",
        "pub fn resolve_close_label(value: Option<&'static str>) -> &'static str",
    ] {
        assert!(
            logic_source.contains(needle),
            "BottomSheet logic should own fallback text normalization (`{needle}`).",
        );
    }

    for needle in [
        "overlay_dialog_attrs(",
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "lang=move || panel_lang.with_value(|value| value.clone())",
        "dir=panel_dir",
    ] {
        assert!(
            sheet_source.contains(needle),
            "Sheet should host reusable overlay a11y semantics for BottomSheet (`{needle}`).",
        );
    }

    for needle in [
        "pub struct OverlayDialogA11yAttrs {",
        "pub aria_labelledby: Option<String>,",
        "pub aria_describedby: Option<String>,",
        "pub lang: Option<String>,",
        "pub dir: Option<&'static str>,",
        "pub fn overlay_dialog_attrs(",
    ] {
        assert!(
            headless_a11y_source.contains(needle),
            "Shared a11y helpers should come from ui-headless (`{needle}`).",
        );
    }
}

#[test]
fn bottom_sheet_motion_contract_exposes_default_and_custom_sheet_checks() {
    let source = load_source("src/bottom_sheet/motion.rs");
    let test_source = load_source("../../components/bottom-sheet/test/motion.rs");

    for needle in [
        "pub struct BottomSheetMotion",
        "pub sheet: crate::sheet::SheetMotion",
        "fn default_motion_uses_default_sheet_motion_contract()",
        "fn supports_custom_sheet_motion_contract()",
    ] {
        assert!(
            source.contains(needle) || test_source.contains(needle),
            "BottomSheet motion module should include `{needle}` for baseline-level contract coverage."
        );
    }
}

#[test]
fn bottom_sheet_motion_sanitizes_custom_contract_values() {
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let motion_test_source = load_source("../../components/bottom-sheet/test/motion.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "pub fn sanitize_motion(motion: BottomSheetMotion) -> BottomSheetMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
        "fn sanitize_motion_delegates_to_sheet_contract()",
    ] {
        assert!(
            motion_source.contains(needle) || motion_test_source.contains(needle),
            "BottomSheet motion should include `{needle}` so invalid custom motion contracts cannot leak into runtime behavior.",
        );
    }

    assert!(
        view_source.contains("let motion = crate::bottom_sheet::motion::sanitize_motion(motion);"),
        "BottomSheet view should sanitize motion before forwarding to Sheet.",
    );
}

#[test]
fn bottom_sheet_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let bottom_sheet_motion_source = load_source("src/bottom_sheet/motion.rs");
    let sheet_motion_source = load_source("src/sheet/motion.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for needle in [
        "pub struct BottomSheetMotion",
        "pub sheet: crate::sheet::SheetMotion",
        "pub fn sanitize_motion(motion: BottomSheetMotion) -> BottomSheetMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
    ] {
        assert!(
            bottom_sheet_motion_source.contains(needle),
            "BottomSheet motion module should keep component-scoped contract mapping `{needle}`.",
        );
    }

    for needle in [
        "stiffness: if value.stiffness.is_finite() && value.stiffness > 0.0",
        "damping: if value.damping.is_finite() && value.damping > 0.0",
        "pub fn sanitize_motion(motion: SheetMotion) -> SheetMotion",
        "motion::attach_motion(root_ref, open, placement, on_exit_complete, motion);",
    ] {
        assert!(
            sheet_motion_source.contains(needle) || sheet_view_source.contains(needle),
            "Sheet motion pipeline should keep contractualized spring + attach path `{needle}`.",
        );
    }

    for needle in [
        "let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());",
        "if prefers_reduced_motion {",
        "let reduced_motion = prefers_reduced_motion.get_value();",
        "if reduced_motion {",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "Sheet motion should keep explicit reduced-motion branch `{needle}`.",
        );
    }

    let non_wasm_attach_marker = "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(";
    let non_wasm_start = sheet_motion_source
        .find(non_wasm_attach_marker)
        .unwrap_or_else(|| panic!("Sheet motion should expose non-wasm attach_motion branch."));
    let non_wasm_end = sheet_motion_source[non_wasm_start..]
        .find("#[cfg(test)]")
        .map(|offset| non_wasm_start + offset)
        .unwrap_or(sheet_motion_source.len());
    let non_wasm_block = &sheet_motion_source[non_wasm_start..non_wasm_end];

    for needle in ["if !is_open.get() {", "finish_exit.run(());"] {
        assert!(
            non_wasm_block.contains(needle),
            "Sheet non-wasm motion fallback should stay predictable/no-op-safe (`{needle}`).",
        );
    }

    for required in [
        "- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。",
        "bottom_sheet_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe",
    ] {
        assert!(
            check2_source.contains(required),
            "bottom-sheet checklist should keep motion-contract evidence `{required}`.",
        );
    }
}

#[test]
fn bottom_sheet_motion_contract_check_script_covers_platform_guard() {
    let script_source = load_source("../../scripts/check-ui-components-platforms.sh");

    let needle = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        script_source.contains(needle),
        "platform check script should enforce `{needle}`.",
    );
}

#[test]
fn bottom_sheet_css_is_aggregated() {
    let source = load_source("src/css.rs");

    assert!(
        source.contains("out.push_str(crate::bottom_sheet::styles::CSS);"),
        "ui-components css aggregator should include bottom_sheet styles."
    );
}

#[test]
fn bottom_sheet_cascade_layer_and_runtime_style_contract_is_enforced() {
    let css_entry_source = load_source("src/css.rs");
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-bottom_sheet\")]",
        "out.push_str(crate::bottom_sheet::styles::CSS);",
        "out.push_str(\"\\n}\\n\");",
    ] {
        assert!(
            css_entry_source.contains(needle),
            "ui-components css entry should keep cascade-layer contract marker `{needle}`.",
        );
    }

    let combined_view_source = format!("{bottom_sheet_view_source}\n{sheet_view_source}");
    for style_line in combined_view_source
        .lines()
        .filter(|line| line.contains("style="))
    {
        assert!(
            style_line.contains("--"),
            "BottomSheet runtime inline style should be CSS custom properties only: `{style_line}`.",
        );
        for forbidden in [
            "top:",
            "left:",
            "right:",
            "bottom:",
            "width:",
            "height:",
            "position:",
        ] {
            assert!(
                !style_line.contains(forbidden),
                "BottomSheet runtime inline style should avoid plain layout property `{forbidden}`.",
            );
        }
    }

    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"right:",
        "style=\"bottom:",
        "style=\"width:",
        "style=\"height:",
        "style=\"position:",
        "style:top",
        "style:left",
        "style:right",
        "style:bottom",
        "style:width",
        "style:height",
        "style:position",
    ] {
        assert!(
            !combined_view_source.contains(forbidden),
            "BottomSheet/Sheet should avoid plain inline style token `{forbidden}`.",
        );
    }

    for required in [
        "- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\\\"top: 10px\\\"`）。",
        "bottom_sheet_cascade_layer_and_runtime_style_contract_is_enforced",
    ] {
        assert!(
            check2_source.contains(required),
            "bottom-sheet checklist should keep cascade-layer/runtime-style evidence `{required}`.",
        );
    }
}

#[test]
fn bottom_sheet_cascade_layer_check_script_covers_layer_and_inline_style_guard() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    let needle = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        script_source.contains(needle),
        "contract-hygiene check script should enforce `{needle}`.",
    );
}

#[test]
fn bottom_sheet_token_first_static_css_contract_is_respected() {
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");

    for needle in [
        "out.push_str(crate::bottom_sheet::styles::CSS);",
        "crate::css::push_components_css(&mut out);",
        "<style>{move || css_text.get()}</style>",
    ] {
        assert!(
            css_source.contains(needle) || root_source.contains(needle),
            "BottomSheet styles should be aggregated and injected via UiRoot (`{needle}`).",
        );
    }

    for needle in [
        "var(--ui-space-",
        "var(--ui-bg, var(--ui-fallback-bg))",
        "var(--ui-fg, var(--ui-fallback-fg))",
        "var(--ui-border, var(--ui-fallback-border))",
        "var(--ui-radius-",
        "var(--ui-shadow-",
    ] {
        assert!(
            styles_source.contains(needle),
            "BottomSheet styles should be token-first and consume theme variables (`{needle}`).",
        );
    }

    for forbidden in [
        "--ui-bottom-sheet-",
        "var(--ui-border-width, 1px)",
        "var(--ui-heading-h5-font-size, 16px)",
        "var(--ui-heading-h5-line-height, 24px)",
        "var(--ui-font-size-150, 14px)",
        "var(--ui-line-height-150, 20px)",
        "margin-bottom: 8px;",
        "margin-bottom: 16px;",
        "margin-bottom: 24px;",
        "margin-bottom: 32px;",
        "width: 2.75rem;",
        "height: 0.3125rem;",
        "border-radius: 9999px;",
        "@apply ",
        "tailwind",
        "tw-",
        "stylex",
        "css!(",
        "styled(",
    ] {
        assert!(
            !styles_source.contains(forbidden),
            "BottomSheet component styles should not rely on private tokens, utility-first, or CSS-in-Rust shortcuts (`{forbidden}`).",
        );
    }

    for forbidden in [
        "style=",
        "class=\"flex",
        "class=\"grid",
        "class=\"px-",
        "class=\"py-",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not inject runtime business styles or utility-first class contracts (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_tree_shaking_contract_is_feature_gated_end_to_end() {
    let ui_components_manifest = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let web_demo_manifest = load_source("../../apps/web-demo/Cargo.toml");

    for needle in [
        "default = [\"inject-css\", \"all-components\"]",
        "all-components = [",
        "component-bottom_sheet = [",
        "\"component-bottom_sheet\",",
    ] {
        assert!(
            ui_components_manifest.contains(needle),
            "ui-components feature manifest should keep tree-shaking entry `{needle}`.",
        );
    }

    assert!(
        lib_source.contains(
            "#[cfg(feature = \"component-bottom_sheet\")]\n#[path = \"../../../components/bottom-sheet/src/mod.rs\"]\npub mod bottom_sheet;"
        ),
        "BottomSheet module export should be gated by `component-bottom_sheet` in lib.rs.",
    );
    assert!(
        lib_source.contains(
            "#[cfg(all(feature = \"web-demo-components\", not(feature = \"all-components\")))]\npub use web_demo_components::*;"
        ),
        "web-demo component surface should stay behind `web-demo-components` without forcing `all-components`.",
    );
    assert!(
        lib_source.contains("#[cfg(feature = \"all-components\")]\npub use all_components::*;"),
        "all-components export should remain explicitly gated.",
    );

    assert!(
        css_source.contains(
            "#[cfg(feature = \"component-bottom_sheet\")]\n    out.push_str(crate::bottom_sheet::styles::CSS);"
        ),
        "BottomSheet CSS aggregation should be gated by `component-bottom_sheet` in css.rs.",
    );

    assert!(
        web_demo_manifest.contains("ui-components = { path = \"../../crates/ui-components\", default-features = false, features = [\"inject-css\", \"web-demo-components\"] }"),
        "web-demo should consume ui-components through explicit minimal features.",
    );
    assert!(
        !web_demo_manifest.contains("all-components"),
        "web-demo must not implicitly pull `all-components`.",
    );
}

#[test]
fn bottom_sheet_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("../../scripts/check-ui-components-tree-shaking.sh");

    for needle in [
        "BOTTOM_SHEET_MIN_FEATURES=\"component-bottom_sheet,inject-css\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_tree_shaking_contract_is_feature_gated_end_to_end",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "BOTTOM_SHEET_TREE_OUTPUT=\"$(cargo tree -e features -i ui-components -p ui-components --no-default-features --features \"$BOTTOM_SHEET_MIN_FEATURES\")\"",
        "if ! grep -q 'feature \"component-bottom_sheet\" (command-line)' <<<\"$BOTTOM_SHEET_TREE_OUTPUT\"",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$BOTTOM_SHEET_TREE_OUTPUT\"",
        "if grep -q 'all-components' <<<\"$BOTTOM_SHEET_TREE_OUTPUT\"",
        "cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features \"$BOTTOM_SHEET_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should enforce bottom-sheet minimal-feature gate `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for needle in [
        "- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。",
        "bottom_sheet_tree_shaking_contract_is_feature_gated_end_to_end",
        "bottom_sheet_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "bottom_sheet_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "`component-bottom_sheet = [\"component-sheet\", \"component-button\"]`",
        "`#[cfg(feature = \"component-bottom_sheet\")]`",
        "`out.push_str(crate::bottom_sheet::styles::CSS);`",
        "`scripts/check-ui-components-tree-shaking.sh`",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "bottom-sheet check2 tree-shaking section should reference `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_docs_page_contains_custom_motion_playground() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn bottom_sheet() -> AnyView",
        "title=\"BottomSheet\"",
        "slug=\"bottom-sheet\"",
        "Custom Motion Contract",
        "let custom_motion_code = Signal::derive(move || {",
        "<BottomSheet",
    ] {
        assert!(
            source.contains(needle),
            "bottom_sheet docs page should contain `{needle}`."
        );
    }
}

#[test]
fn bottom_sheet_visual_quality_gate_is_backed_by_theme_visual_baseline_docs() {
    let docs_registry_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let baseline_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");

    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
        "theme_visual_baseline::theme_visual_baseline",
    ] {
        assert!(
            docs_registry_source.contains(needle),
            "docs registry should expose ThemeVisualBaseline entry for visual-quality gating (`{needle}`).",
        );
    }

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "title=\"Default Theme Visual Baseline\"",
        "Includes Button/Input/Overlay for visual regression snapshots.",
        "hierarchy, spacing rhythm, contrast layers, and interactive feedback",
        "Default theme should feel trustworthy at first glance",
        "use ui_components::{Button, ButtonVariant, Input, OnPress, Overlay};",
        "<Button variant=ButtonVariant::Accent>",
        "<Input",
        "<Overlay",
        "Open Overlay Baseline",
    ] {
        assert!(
            baseline_page_source.contains(needle),
            "ThemeVisualBaseline docs should keep explicit visual-quality baseline signals (`{needle}`).",
        );
    }
}

#[test]
fn bottom_sheet_dx_hello_world_is_minimal_and_copy_paste_ready() {
    let view_source = load_source("src/bottom_sheet/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "open: Signal<bool>",
        "on_close: OnPress",
        "id_base: String",
        "title: String",
        "children: ChildrenFn",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet baseline API should expose simple required props (`{needle}`).",
        );
    }

    for forbidden in [
        "state: BottomSheetState",
        "#[prop(optional)] state:",
        "state: Signal<BottomSheetState>",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet baseline API should not require internal state objects (`{forbidden}`).",
        );
    }

    for needle in [
        "title=\"Hello World (Minimal Path)\"",
        "let hello_world_code = Signal::derive(move || {",
        "<BottomSheet open=open id_base=\"bottom-sheet\".to_string() title=\"Bottom sheet\".to_string() on_close=on_close>",
    ] {
        assert!(
            docs_source.contains(needle),
            "BottomSheet docs should include minimal hello-world path (`{needle}`).",
        );
    }

    let hello_marker = "let hello_world_code = Signal::derive(move || {";
    let hello_pos = docs_source.find(hello_marker).unwrap_or_else(|| {
        panic!("docs should contain `{hello_marker}` for hello-world extraction")
    });
    let hello_tail = &docs_source[hello_pos..];
    let raw_start_rel = hello_tail
        .find("r#\"")
        .unwrap_or_else(|| panic!("hello-world snippet should be a raw string literal"));
    let raw_start = raw_start_rel + 3;
    let raw_end = hello_tail[raw_start..]
        .find("\"#")
        .unwrap_or_else(|| panic!("hello-world snippet should have closing raw string marker"))
        + raw_start;
    let snippet = &hello_tail[raw_start..raw_end];
    let non_empty_lines = snippet
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();

    assert!(
        non_empty_lines <= 5,
        "BottomSheet hello-world snippet should stay within 5 non-empty lines; got {non_empty_lines} lines."
    );
}

#[test]
fn bottom_sheet_is_not_collection_api_and_rejects_parallel_item_conventions() {
    let view_source = load_source("src/bottom_sheet/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in ["title: String", "children: ChildrenFn"] {
        assert!(
            view_source.contains(needle),
            "BottomSheet should stay single-content API (`{needle}`).",
        );
    }

    for forbidden in [
        "#[prop(optional, into)] labels:",
        "#[prop(optional, into)] titles:",
        "#[prop(optional, into)] panels:",
        "labels: Vec<",
        "titles: Vec<",
        "panels: Vec<",
        "item_specs",
        "ItemSpec",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet should not expose collection-style parallel array/spec API (`{forbidden}`).",
        );
    }

    for forbidden in [
        "labels + children",
        "titles + panels",
        "<BottomSheetItem",
        "ItemSpec",
    ] {
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not recommend collection-style API conventions (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_dragging_micro_loop_or_drag_end_protocol() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");

    for forbidden in [
        "Dragging",
        "DragEnd",
        "Action::DragEnd",
        "drag_state",
        "dragging_state",
        "pointermove",
        "on_pointermove",
        "touchmove",
        "request_animation_frame",
        "set_interval(",
        "delta_y",
        "velocity_y",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not host drag macro/micro state machine (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not run local drag frame loop without explicit drag protocol (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should stay contract-only and not embed drag loop semantics (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_two_pass_measure_rectification_geometry_pipeline() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");

    for forbidden in [
        "Intent",
        "Measure",
        "Rectification",
        "getBoundingClientRect",
        "offset_width",
        "offset_height",
        "client_width",
        "client_height",
        "ResizeObserver",
        "IntersectionObserver",
        "DOMRect",
        "layout_rect",
        "measured_rect",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not implement geometry rectification pipeline without explicit measurement feature (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not run DOM measurement pass for a non-measured overlay (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should stay timing-contract only, not geometry-measurement pipeline (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_collection_registration_protocol_surface() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "RegistrationContext",
        "Register",
        "Unregister",
        "items_order",
        "HashSet",
        "ItemId",
        "roving_index",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not host collection registration protocol (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not depend on dynamic item registration protocol (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not imply collection registration API (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_slot_projection_keepalive_lifecycle_protocol() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "Lazy",
        "KeepAlive",
        "Eager",
        "NotifyHidden",
        "notify_hidden",
        "slot_projection",
        "keep_alive",
        "projection_mode",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not define slot projection lifecycle protocol (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not implement KeepAlive projection hooks without explicit projection feature (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should not encode KeepAlive lifecycle semantics (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not claim slot projection policy API (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_env_stream_subscription_or_event_flood_pipeline() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "ResizeObserver",
        "IntersectionObserver",
        "BreakpointChanged",
        "ThemeChanged",
        "on_resize",
        "on_intersection",
        "match_media",
        "add_event_listener",
        "debounce",
        "throttle",
        "env_stream",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not host env-stream action pipeline (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not subscribe raw environment events for this component (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should not encode env-stream subscriptions (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not claim env-stream API surface (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_event_light_cone_batch_collection_protocol() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "Context Bus",
        "SelectionState::All",
        "selection_state",
        "selector_bus",
        "batch_select",
        "bulk_selection",
        "prop drilling",
        "grid_selection",
        "table_selection",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not define large-collection event-light-cone protocol (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not rely on table/grid batch selection event topology (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not describe event-light-cone batch API (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_causality_bus_traceid_pipeline() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "TraceId",
        "trace_id",
        "CausalityBus",
        "causality_bus",
        "broadcast(",
        "subscriber",
        "publish(",
        "derived_command",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not require causality-bus pipeline in current scope (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not expose causality-bus transport concerns (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should not carry causality-bus metadata (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not imply TraceId/causality-bus API (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_has_no_foreign_zone_escape_hatch_surface() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let sheet_motion_source = load_source("src/sheet/motion.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for forbidden in [
        "ForeignZone",
        "foreign_zone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "echarts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "external_instance",
        "foreign_instance",
        "chart_instance",
        "map_instance",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not introduce imperative foreign-zone protocol (`{forbidden}`).",
        );
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not expose imperative third-party instance API (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should stay component-contract only (`{forbidden}`).",
        );
        assert!(
            !sheet_view_source.contains(forbidden),
            "Sheet view should not expose foreign-zone escape-hatch contract in this scope (`{forbidden}`).",
        );
        assert!(
            !sheet_motion_source.contains(forbidden),
            "Sheet motion should not embed third-party imperative integration protocol (`{forbidden}`).",
        );
        assert!(
            !docs_source.contains(forbidden),
            "BottomSheet docs should not claim third-party imperative integration API (`{forbidden}`).",
        );
    }

    for forbidden in [
        "pub use ECharts",
        "pub use Mapbox",
        "pub use Leaflet",
        "pub use GoogleMap",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "BottomSheet public module should not re-export third-party imperative instances (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_avoids_hydration_discontinuity_time_random_and_uses_stable_id_inputs() {
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let bottom_sheet_logic_source = load_source("src/bottom_sheet/logic.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let sheet_logic_source = load_source("src/sheet/logic.rs");
    let id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");

    for forbidden in [
        "now(",
        "Date::now",
        "SystemTime::now",
        "UNIX_EPOCH",
        "Uuid::",
        "uuid::",
        "new_v4(",
        "thread_rng(",
        "rand::random",
        "Math::random",
        "randomUUID",
    ] {
        assert!(
            !bottom_sheet_view_source.contains(forbidden),
            "BottomSheet view should not generate time/random ids during render (`{forbidden}`).",
        );
        assert!(
            !bottom_sheet_logic_source.contains(forbidden),
            "BottomSheet logic should stay deterministic and avoid time/random init (`{forbidden}`).",
        );
        assert!(
            !sheet_view_source.contains(forbidden),
            "Sheet view should not introduce hydration-unstable time/random ids (`{forbidden}`).",
        );
        assert!(
            !sheet_logic_source.contains(forbidden),
            "Sheet logic should stay deterministic and avoid random/time-based init (`{forbidden}`).",
        );
    }

    for needle in [
        "id_base: String",
        "let id_base = logic::normalize_id_base(id_base);",
        "let title_id = format!(\"{id_base}-title\");",
        "let description_id = format!(\"{id_base}-description\");",
    ] {
        assert!(
            bottom_sheet_view_source.contains(needle),
            "BottomSheet should derive SSR/hydration-stable ids from explicit deterministic input (`{needle}`).",
        );
    }

    for needle in [
        "pub struct UiIdProvider",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider>",
    ] {
        assert!(
            id_provider_source.contains(needle),
            "ui-headless should expose deterministic IdProvider injection hooks (`{needle}`).",
        );
    }
}

#[test]
fn bottom_sheet_platform_paths_are_explicit_and_non_wasm_is_browser_safe() {
    let sheet_view_source = load_source("src/sheet/view.rs");
    let sheet_motion_source = load_source("src/sheet/motion.rs");
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let ui_motion_source = load_source("../../crates/ui-motion/src/lib.rs");

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "Sheet view should keep explicit wasm/non-wasm platform split (`{marker}`).",
        );
        assert!(
            sheet_motion_source.contains(marker),
            "Sheet motion should keep explicit wasm/non-wasm platform split (`{marker}`).",
        );
    }

    for marker in [
        "#[cfg(target_arch = \"wasm32\")]",
        "pub mod web;",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions)",
    ] {
        assert!(
            ui_motion_source.contains(marker),
            "ui-motion should provide explicit cross-platform backend split with non-wasm no-op (`{marker}`).",
        );
    }

    let non_wasm_attach_marker = "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(";
    let non_wasm_start = sheet_motion_source
        .find(non_wasm_attach_marker)
        .unwrap_or_else(|| {
            panic!("Sheet motion should expose non-wasm attach_motion fallback branch")
        });
    let non_wasm_end = sheet_motion_source[non_wasm_start..]
        .find("#[cfg(test)]")
        .map(|offset| non_wasm_start + offset)
        .unwrap_or(sheet_motion_source.len());
    let non_wasm_block = &sheet_motion_source[non_wasm_start..non_wasm_end];

    for forbidden in [
        "web_sys::",
        "js_sys::",
        "window(",
        "document(",
        "unwrap(",
        "expect(",
        "panic!(",
    ] {
        assert!(
            !non_wasm_block.contains(forbidden),
            "Sheet non-wasm motion fallback should not touch browser-only APIs (`{forbidden}`).",
        );
    }

    for marker in [
        "Effect::new(move |_| {",
        "if !is_open.get() {",
        "finish_exit.run(());",
    ] {
        assert!(
            non_wasm_block.contains(marker),
            "Sheet non-wasm motion fallback should stay predictable and SSR-safe (`{marker}`).",
        );
    }

    for forbidden in ["web_sys::", "js_sys::"] {
        assert!(
            !bottom_sheet_view_source.contains(forbidden),
            "BottomSheet view should not directly depend on browser-only bindings (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_reduced_motion_and_ssr_wasm_semantics_are_consistent() {
    let sheet_motion_source = load_source("src/sheet/motion.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());",
        "if prefers_reduced_motion {",
        "if reduced_motion {",
        "\"--ui-sheet-backdrop-opacity\"",
        "\"--ui-sheet-panel-opacity\"",
        "\"--ui-sheet-panel-x\"",
        "\"--ui-sheet-panel-y\"",
        "if !open {",
        "finish_exit.run(());",
    ] {
        assert!(
            sheet_motion_source.contains(needle),
            "Sheet motion should explicitly cover reduced-motion fallback path (`{needle}`).",
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "let is_composing = ev.is_composing();",
        "let default_prevented = ev.default_prevented();",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "let is_composing = false;",
        "let default_prevented = false;",
        "if logic::should_close_on_escape(",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "Sheet should keep explicit wasm enhancement + SSR fallback while sharing one semantic close contract (`{needle}`).",
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "let id_base = logic::normalize_id_base(id_base);",
        "let title_id = format!(\"{id_base}-title\");",
        "let description_id = format!(\"{id_base}-description\");",
        "<Sheet",
        "open=open",
        "motion=motion.sheet",
    ] {
        assert!(
            sheet_view_source.contains(needle) || bottom_sheet_view_source.contains(needle),
            "BottomSheet/Sheet SSR-hydration semantic markers should remain stable across wasm and SSR branches (`{needle}`).",
        );
    }
}

#[test]
fn bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_bottom_sheet_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let todo_source = load_source("../../docs/plan/TODO.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。",
        "N/A：`BottomSheet` 暂未接入精确 `render_count` 自动化计数",
        "render_count",
        "等价证据",
        "渲染次数预算为 `1`",
    ] {
        assert!(
            check2_source.contains(needle),
            "BottomSheet checklist should keep performance-governance evidence token `{needle}`.",
        );
    }

    for needle in [
        "component_doc!(",
        "\"BottomSheet\"",
        "\"bottom-sheet\"",
        "\"Overlays\"",
        "overlays_extra::bottom_sheet",
    ] {
        assert!(
            pages_source.contains(needle),
            "BottomSheet docs catalog should keep marker `{needle}` for perf coverage traversal.",
        );
    }

    for needle in [
        "title=\"BottomSheet\"",
        "slug=\"bottom-sheet\"",
        "<ComponentPage",
    ] {
        assert!(
            docs_bottom_sheet_page_source.contains(needle),
            "BottomSheet docs page should mount through ComponentPage contract `{needle}`.",
        );
    }

    for needle in [
        "_ => UiPerfBudget::mount_only(120.0),",
        "let perf_budget = component_page_perf_budget(slug);",
        "let perf_name = format!(\"ComponentPage::{slug}\");",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs component shell should keep mount-only perf budget/probe contract `{needle}`.",
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
        "\"mount-only\"",
    ] {
        assert!(
            perf_probe_source.contains(needle),
            "UiPerfProbe should expose stable perf marker `{needle}`.",
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
            "docs coverage e2e should keep blocking perf regression assertion `{needle}`.",
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "perf governance should keep explicit render_count follow-up marker `{needle}`.",
        );
    }

    for needle in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-components --test accordion_semantics docs_perf_probe_budgets_are_wired_for_component_pages",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(needle),
            "performance gate script should include `{needle}`.",
        );
    }

    for needle in [
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-handle=state.handle_attr",
        "data-close-button=state.close_button_attr",
        "data-detached=state.detached_attr",
        "data-bottom-inset=state.inset_attr",
        "data-motion-source=derived_state.motion_source_attr",
        "data-class-source=state.class_source_attr",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view should expose stable attribution markers for perf triage `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_view_macro_complexity_is_split_into_semantic_subrenders() {
    let view_source = load_source("src/bottom_sheet/view.rs");

    assert!(
        view_source.lines().count() <= 320,
        "BottomSheet view.rs should stay bounded; split further if this grows too large."
    );
    assert_eq!(
        view_source.matches("view! {").count(),
        8,
        "BottomSheet should keep one shell split and semantic subrender blocks instead of one giant view!."
    );
    assert_eq!(
        view_source.matches("data-slot=\"bottom-sheet\"").count(),
        1,
        "BottomSheet root markup should be declared once and reused across shell branches."
    );
    assert_eq!(
        view_source
            .matches("render_bottom_sheet_content(content_input.clone())")
            .count(),
        2,
        "BottomSheet description/no-description shell branches should share one extracted content renderer."
    );

    for needle in [
        "struct BottomSheetContentInput",
        "fn render_bottom_sheet_handle(",
        "fn render_bottom_sheet_close_button(",
        "fn render_bottom_sheet_header(",
        "fn render_bottom_sheet_body(",
        "fn render_bottom_sheet_footer(",
        "fn render_bottom_sheet_content(input: BottomSheetContentInput) -> impl IntoView",
        "if state.show_description {",
        "aria_describedby=description_id.clone()",
        "aria_labelledby=title_id.clone()",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet view macro-splitting contract should keep `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_prefers_functional_split_without_extra_component_noise() {
    let view_source = load_source("src/bottom_sheet/view.rs");

    assert_eq!(
        view_source.matches("#[component]").count(),
        1,
        "BottomSheet view should keep a single top-level component and avoid promoting lightweight fragments to local #[component].",
    );

    for needle in [
        "pub fn BottomSheet(",
        "fn render_bottom_sheet_handle(show_handle: bool) -> impl IntoView",
        "fn render_bottom_sheet_close_button(",
        "fn render_bottom_sheet_header(",
        "fn render_bottom_sheet_body(children: StoredValue<ChildrenFn>) -> impl IntoView",
        "fn render_bottom_sheet_footer(",
        "fn render_bottom_sheet_content(input: BottomSheetContentInput) -> impl IntoView",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet functional-split contract should keep `{needle}`.",
        );
    }

    assert!(
        !view_source.contains("#[component]\nfn render_bottom_sheet"),
        "BottomSheet should not turn helper fragments into extra local #[component] units.",
    );
}

#[test]
fn bottom_sheet_static_fragments_are_constantized_and_templated() {
    let view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "const CLOSE_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const CLOSE_ICON_FILL: &str = \"none\";",
        "const CLOSE_ICON_PATH_D: &str = \"M5 5l10 10M15 5L5 15\";",
        "const CLOSE_ICON_STROKE_WIDTH: &str = \"1.5\";",
        "fn render_bottom_sheet_close_icon() -> impl IntoView",
        "{render_bottom_sheet_close_icon()}",
        "fn render_bottom_sheet_footer(",
    ] {
        assert!(
            view_source.contains(needle),
            "BottomSheet static-fragment contract should keep `{needle}`.",
        );
    }

    assert_eq!(
        view_source.matches("M5 5l10 10M15 5L5 15").count(),
        1,
        "BottomSheet close icon path should have a single static source-of-truth constant."
    );
    assert_eq!(
        view_source.matches("<svg ").count(),
        1,
        "BottomSheet should template the close SVG once instead of duplicating inline fragments."
    );
    assert_eq!(
        view_source
            .matches("data-slot=\"bottom-sheet-footer\"")
            .count(),
        1,
        "BottomSheet footer static slot markup should be declared once in a shared template."
    );
}

#[test]
fn bottom_sheet_disallows_inner_html_in_component_surface() {
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let protocol_source = load_source("src/bottom_sheet/protocol.rs");
    let docs_page_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for forbidden in [
        "inner_html=",
        "set_inner_html(",
        ".set_inner_html(",
        "dangerously_set_inner_html",
        "outer_html",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view should not inject HTML dynamically (`{forbidden}`).",
        );
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic should not hold HTML injection APIs (`{forbidden}`).",
        );
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion should not hold HTML injection APIs (`{forbidden}`).",
        );
        assert!(
            !styles_source.contains(forbidden),
            "BottomSheet styles should not hold HTML injection APIs (`{forbidden}`).",
        );
        assert!(
            !protocol_source.contains(forbidden),
            "BottomSheet protocol should not hold HTML injection APIs (`{forbidden}`).",
        );
        assert!(
            !docs_page_source.contains(forbidden),
            "BottomSheet docs page should not expose inner_html injection path (`{forbidden}`).",
        );
    }

    for needle in [
        "- [x] `inner_html` 使用约束",
        "N/A：`BottomSheet` 当前无 `inner_html` 注入路径",
        "bottom_sheet_disallows_inner_html_in_component_surface",
    ] {
        assert!(
            check2_source.contains(needle),
            "BottomSheet checklist should keep inner_html safety evidence `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_respects_ui_headless_web_ssr_mutual_exclusion_contract() {
    let headless_lib_source = load_source("../../crates/ui-headless/src/lib.rs");
    let headless_manifest_source = load_source("../../crates/ui-headless/Cargo.toml");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            headless_lib_source.contains(needle),
            "ui-headless should protect web/ssr mutual exclusion via compile_error guard (`{needle}`).",
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            headless_manifest_source.contains(needle),
            "ui-headless feature matrix should keep explicit web/ssr split (`{needle}`).",
        );
    }

    for needle in [
        "use ui_headless::{",
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
    ] {
        assert!(
            sheet_view_source.contains(needle),
            "Sheet should keep consuming headless interaction contracts (`{needle}`).",
        );
    }

    for forbidden in [
        "feature = \"ssr\"",
        "feature = \"web\" && feature = \"ssr\"",
    ] {
        assert!(
            !bottom_sheet_view_source.contains(forbidden) && !sheet_view_source.contains(forbidden),
            "BottomSheet/Sheet should not bypass headless feature mutual-exclusion contract in component code (`{forbidden}`).",
        );
    }
}

#[test]
fn bottom_sheet_docs_custom_motion_playground_locks_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Custom Motion Contract\"",
        "let custom_motion_code = Signal::derive(move || {",
        "motion=BottomSheetMotion {",
        "sheet: ui_components::SheetMotion {",
        "initial_offset_px: 64.0",
        "id_base=\"docs-bottom-sheet-motion\".to_string()",
        "motion=BottomSheetMotion {",
        "description=\"Custom sheet motion flips data-motion-source to custom.\".to_string()",
        "Use devtools to inspect data-motion-source/custom-motion.",
    ] {
        assert!(
            source.contains(needle),
            "bottom_sheet docs custom-motion playground should contain `{needle}`."
        );
    }
}

#[test]
fn bottom_sheet_docs_page_covers_primary_playgrounds() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "pub(super) fn bottom_sheet() -> AnyView",
        "title=\"BottomSheet\"",
        "slug=\"bottom-sheet\"",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "Playground title=\"Semantic Bottom Sheet\"",
        "Playground title=\"Detached + Title Only + Custom Class\"",
        "Playground title=\"Custom Motion Contract\"",
    ] {
        assert!(
            source.contains(needle),
            "overlays-extra docs page should contain `{needle}` for BottomSheet.",
        );
    }
}

#[test]
fn bottom_sheet_docs_playgrounds_lock_state_matrix_contract_values() {
    let source = load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Hello World (Minimal Path)\"",
        "id_base=\"docs-bottom-sheet-hello\".to_string()",
        "title=\"Bottom sheet\".to_string()",
        "title=\"State Matrix\"",
        "data-slot=\"bottom-sheet-state-matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"bottom-sheet-controlled-uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"bottom-sheet-streaming-contract\"",
        "data-slot=\"bottom-sheet-source-first\"",
        "data-slot=\"bottom-sheet-source-paths\"",
        "title=\"Semantic Bottom Sheet\"",
        "id_base=\"docs-bottom-sheet-semantic\".to_string()",
        "title=\"Update available\".to_string()",
        "description=\"A newer version with security improvements is ready to install.\".to_string()",
        "footer=move || view! {",
        "title=\"Detached + Title Only + Custom Class\"",
        "id_base=\"docs-bottom-sheet-detached\".to_string()",
        "is_detached=true",
        "bottom_inset_px=16.0",
        "is_close_button_visible=false",
        "class_name=\"docs-bottom-sheet-custom\".to_string()",
        "title=\"Custom Motion Contract\"",
        "id_base=\"docs-bottom-sheet-motion\".to_string()",
        "motion=BottomSheetMotion {",
        "initial_offset_px: 64.0",
        "description=\"Custom sheet motion flips data-motion-source to custom.\".to_string()",
    ] {
        assert!(
            source.contains(needle),
            "bottom-sheet docs playgrounds should contain `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated() {
    let cargo_source = load_source("Cargo.toml");
    let crate_root_source = load_source("src/lib.rs");
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let bottom_sheet_logic_source = load_source("src/bottom_sheet/logic.rs");
    let bottom_sheet_motion_source = load_source("src/bottom_sheet/motion.rs");
    let bottom_sheet_styles_source = load_source("src/bottom_sheet/styles.rs");
    let docs_lib_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let debug_overlay_e2e = load_source("../../e2e/tests/docs_app_debug_overlay.spec.mjs");
    let wasm_debug_script = load_source("../../scripts/check-ui-components-wasm-debug.sh");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for needle in ["macro_rules! wasm_debug_proxy"] {
        assert!(
            crate_root_source.contains(needle),
            "ui-components should keep wasm debug capability isolated via `{needle}`.",
        );
    }

    for needle in [
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "sheet-wasm-debug = [\"component-sheet\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui-components Cargo features should keep explicit wasm-debug opt-in marker `{needle}`.",
        );
    }

    assert!(
        !cargo_source.contains("bottom_sheet-wasm-debug")
            && !cargo_source.contains("bottom-sheet-wasm-debug"),
        "BottomSheet should not expose a dedicated wasm-debug feature because debug timeline/replay comes from global ui-trace overlay.",
    );

    let bottom_sheet_combined = format!(
        "{bottom_sheet_view_source}\n{bottom_sheet_logic_source}\n{bottom_sheet_motion_source}\n{bottom_sheet_styles_source}"
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
            !bottom_sheet_combined.contains(forbidden),
            "BottomSheet production contract should not leak wasm-debug internals `{forbidden}`.",
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-motion-source=motion_source_attr",
        "data-class-source=state.class_source_attr",
        "data-custom-motion=has_custom_motion.then_some(\"true\")",
    ] {
        assert!(
            bottom_sheet_view_source.contains(marker),
            "BottomSheet should expose stable source/state markers for debug attribution via `{marker}`.",
        );
    }

    for needle in [
        "let trace = use_ui_trace();",
        "let last_open = RwSignal::new(open.get_untracked());",
        "trace.emit(\"bottom-sheet\", UiTraceEventKind::OpenChange { open: next_open });",
        "<Sheet",
        "open=open",
        "on_close=on_close",
        "on_press=on_close",
    ] {
        assert!(
            bottom_sheet_view_source.contains(needle),
            "BottomSheet interaction chain should remain traceable/replayable via `{needle}`.",
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
            "docs-app should keep wasm dev visual-entry gate `{needle}`.",
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "ui_headless::UiTraceEventKind::Inspect { tag, data_slot }",
        "trace.emit(",
        "fn render_events(trace: ui_headless::UiTrace) -> AnyView",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "docs debug overlay should keep wasm dev visual-entry/timeline marker `{needle}`.",
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
        "let event = UiTraceEvent {",
        "ts_ms: now_ms(),",
        "if events.len() > MAX_EVENTS {",
    ] {
        assert!(
            trace_source.contains(needle),
            "ui-headless trace contract should keep timestamp/source event markers `{needle}`.",
        );
    }

    for needle in [
        "debug overlay captures traced open/close events",
        "data-slot=\"ui-debug-overlay-event\"",
        "data-kind=\"open-change\"",
    ] {
        assert!(
            debug_overlay_e2e.contains(needle),
            "debug overlay e2e should keep trace regression contract `{needle}`.",
        );
    }

    let wasm_debug_needle = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated";
    assert!(
        wasm_debug_script.contains(wasm_debug_needle),
        "wasm-debug check script should enforce `{wasm_debug_needle}`.",
    );

    for needle in [
        "WASM 调试要求：关键状态可追踪",
        "开发模式下至少能追踪关键状态变更来源与前后值",
        "关键交互链路应支持最小可复现记录",
        "调试开关默认不进入生产包体与公共 API",
    ] {
        assert!(
            check2_source.contains(needle),
            "BottomSheet checklist should keep wasm-debug governance contract marker `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_dx_playground_supports_hot_reload_context_and_isolated_workbench() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "let (show_settings_panel, set_show_settings_panel) = signal(false);",
        "let (show_code_panel, set_show_code_panel) = signal(false);",
        "let (show_test_panel, set_show_test_panel) = signal(false);",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "placeholder=\"/* Original CSS is loaded. Edit directly, or use :scope for local targeting. */\"",
        "\"Show test\"",
        "\"Restore original CSS\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "Playground should keep DX hot-reload marker `{needle}`.",
        );
    }

    let docs_start = docs_source
        .find("pub(super) fn bottom_sheet() -> AnyView")
        .expect("bottom-sheet docs section should exist");
    let docs_end = docs_source
        .find("pub(super) fn tray() -> AnyView")
        .expect("tray docs section should exist after bottom-sheet");
    let bottom_sheet_docs = &docs_source[docs_start..docs_end];

    for needle in [
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "title=\"Semantic Bottom Sheet\"",
        "title=\"Detached + Title Only + Custom Class\"",
        "title=\"Custom Motion Contract\"",
        "let (present_hello, set_present_hello) = signal(open_hello.get_untracked());",
        "let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());",
        "let (present_detached, set_present_detached) = signal(open_detached.get_untracked());",
        "let (present_custom_motion, set_present_custom_motion) =",
        "let on_hello_exit_complete = Callback::new(move |_| set_present_hello.set(false));",
        "let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));",
        "let on_detached_exit_complete = Callback::new(move |_| set_present_detached.set(false));",
        "let on_custom_motion_exit_complete =",
        "<Show when=move || present_hello.get()>",
        "<Show when=move || present_semantic.get()>",
        "<Show when=move || present_detached.get()>",
        "<Show when=move || present_custom_motion.get()>",
        "<span class=\"ui-muted\">\"open: \" {move || open_hello_raw.get()}</span>",
        "<span class=\"ui-muted\">\"open: \" {move || open_semantic_raw.get()}</span>",
        "<span class=\"ui-muted\">\"open: \" {move || open_detached_raw.get()}</span>",
        "<span class=\"ui-muted\">\"open: \" {move || open_custom_motion_raw.get()}</span>",
    ] {
        assert!(
            bottom_sheet_docs.contains(needle),
            "BottomSheet docs should provide isolated interactive workbench/context marker `{needle}`.",
        );
    }

    for forbidden in [
        "BOTTOM_SHEET_WORKBENCH_STORAGE_KEY",
        "load_bottom_sheet_workbench_state(",
        "save_bottom_sheet_workbench_state(",
        "clear_bottom_sheet_workbench_state(",
        "localStorage",
        "sessionStorage",
    ] {
        assert!(
            !bottom_sheet_docs.contains(forbidden),
            "BottomSheet workbench should keep persistence optional and non-default; found forbidden token `{forbidden}`.",
        );
    }
}

#[test]
fn bottom_sheet_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let cargo_source = load_source("Cargo.toml");
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let checklist_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        !manifest_dir.join("src/bottom_sheet/spec.rs").exists(),
        "BottomSheet should keep spec/schema boundary as N/A for simple component scope."
    );
    assert!(
        cargo_source.contains("component-bottom_sheet = [")
            && cargo_source.contains("\"component-sheet\"")
            && cargo_source.contains("\"component-button\""),
        "BottomSheet feature should keep explicit minimal deps without serde/spec fan-out."
    );
    assert!(
        !cargo_source.contains("component-bottom_sheet = [\"dep:serde\"")
            && !cargo_source.contains("component-bottom_sheet = [\"dep:serde_json\""),
        "BottomSheet should not opt into serde/spec migration dependencies without explicit public schema contract.",
    );

    for forbidden in [
        "mod spec;",
        "pub mod spec;",
        "mod protocol;",
        "pub mod protocol;",
    ] {
        assert!(
            !mod_source.contains(forbidden),
            "BottomSheet module boundary should keep spec/protocol serialization contracts private or absent (`{forbidden}`)."
        );
    }

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
            "BottomSheet engineering contract should keep serde/spec path as N/A and avoid `{forbidden}`."
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
            "BottomSheet checklist should keep engineering governance rule `{required}`.",
        );
    }
}

#[test]
fn bottom_sheet_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events()
 {
    let cargo_source = load_source("Cargo.toml");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let combined = [
        load_source("src/bottom_sheet/mod.rs"),
        load_source("src/bottom_sheet/logic.rs"),
        load_source("src/bottom_sheet/view.rs"),
        load_source("src/bottom_sheet/styles.rs"),
        load_source("src/bottom_sheet/motion.rs"),
    ]
    .join("\n");

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "pub enum UiTraceEventKind {",
    ] {
        assert!(
            cargo_source.contains(required) || trace_source.contains(required),
            "engineering baseline should keep canonical tracing contract marker `{required}`.",
        );
    }

    assert!(
        !cargo_source.contains("bottom_sheet-wasm-debug")
            && !cargo_source.contains("bottom-sheet-wasm-debug"),
        "BottomSheet should not define component-local tracing feature when no dedicated debug runtime is required."
    );

    for required in [
        "let trace = use_ui_trace();",
        "trace.emit(\"bottom-sheet\", UiTraceEventKind::OpenChange { open: next_open });",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet tracing should stay aligned with shared ui-headless trace semantics via `{required}`.",
        );
    }

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "info_span!(",
        "debug_span!(",
        "const BOTTOM_SHEET_TRACE_TARGET",
        "target: \"ui_components::bottom_sheet::",
    ] {
        assert!(
            !combined.contains(forbidden),
            "BottomSheet should avoid ad-hoc tracing semantic drift token `{forbidden}`.",
        );
    }
}

#[test]
fn bottom_sheet_engineering_contract_avoids_runtime_leaks_in_public_api_surface() {
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");

    let sources = [
        &mod_source,
        &logic_source,
        &view_source,
        &styles_source,
        &motion_source,
    ];

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
                "BottomSheet engineering contract should not leak runtime marker `{forbidden}`."
            );
        }
    }

    for forbidden in ["pub use web_sys", "pub use tokio", "pub use async_std"] {
        assert!(
            !mod_source.contains(forbidden),
            "BottomSheet public module boundary should avoid leaking platform/runtime token `{forbidden}`."
        );
    }
}

#[test]
fn bottom_sheet_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("src/css.rs");
    let root_source = load_source("src/root.rs");
    let active_highlight_source = load_source("../ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let checklist_source = load_source("../../components/bottom-sheet/check2.md");

    for needle in [
        "#[cfg(feature = \"component-bottom_sheet\")]\n#[path = \"../../../components/bottom-sheet/src/mod.rs\"]\npub mod bottom_sheet;",
        "pub use root::UiRoot;",
    ] {
        assert!(
            lib_source.contains(needle),
            "ui-components lib entry should keep stable export/gate marker `{needle}`."
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
            "ui-components lib entry should not expose internal platform/details marker `{forbidden}`."
        );
    }

    for needle in [
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-bottom_sheet\")]\n    out.push_str(crate::bottom_sheet::styles::CSS);",
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

    for forbidden in ["#[component]", "pub fn BottomSheet(", "ui-bottom-sheet"] {
        assert!(
            !active_highlight_source.contains(forbidden),
            "active_highlight should remain generic shared utility, not component-business implementation: `{forbidden}`."
        );
    }

    assert!(
        manifest_dir
            .join("../ui-visual-primitive/src/active_highlight.rs")
            .exists(),
        "ui-components should keep shared `../ui-visual-primitive/src/active_highlight.rs` entry."
    );
    assert!(
        !manifest_dir.join("src/overlay_open.rs").exists(),
        "ui-components should not define `src/overlay_open.rs`; open-state primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/presence.rs").exists(),
        "ui-components should not define `src/presence.rs`; presence primitive belongs to ui-headless."
    );
    assert!(
        !manifest_dir.join("src/a11y.rs").exists(),
        "ui-components should not define `src/a11y.rs`; shared a11y helpers belong to ui-headless."
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
        "- [x] `ui-components` 固定入口文件落点正确。",
        "`crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。",
        "`crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。",
        "`crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。",
        "`crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。",
        "`crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。",
        "`crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。",
        "`crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。",
    ] {
        assert!(
            checklist_source.contains(required),
            "BottomSheet checklist should keep fixed-entry governance rule `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_component_directory_standard_file_layout_is_enforced() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    let component_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(|workspace| workspace.join("components/bottom-sheet/src"))
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    for required_file in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
        let path = component_dir.join(required_file);
        assert!(
            path.exists(),
            "BottomSheet component directory should include `{required_file}` at {:?}.",
            path
        );
    }

    for forbidden_file in ["render.rs", "spec.rs"] {
        let path = component_dir.join(forbidden_file);
        assert!(
            !path.exists(),
            "BottomSheet should not include optional/legacy file `{forbidden_file}` in current scope: {:?}.",
            path
        );
    }

    for required in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use view::BottomSheet;",
    ] {
        assert!(
            mod_source.contains(required),
            "BottomSheet mod.rs should keep minimal stable boundary marker `{required}`."
        );
    }

    for required in [
        "pub fn resolve_title(",
        "pub fn derive_view_state(",
        "pub struct BottomSheetDeriveInput",
    ] {
        assert!(
            logic_source.contains(required),
            "BottomSheet logic.rs should keep normalization/derivation marker `{required}`."
        );
    }
    for forbidden in [
        "view!",
        "<div",
        "data-slot",
        "on:click",
        "ui-bottom-sheet__",
    ] {
        assert!(
            !logic_source.contains(forbidden),
            "BottomSheet logic.rs should not carry render/style details (`{forbidden}`)."
        );
    }

    for required in ["pub const CSS: &str = r#\"", "var(--ui-"] {
        assert!(
            styles_source.contains(required),
            "BottomSheet styles.rs should keep token-first static CSS marker `{required}`."
        );
    }
    for forbidden in ["view!", "Signal<", "Callback<", "on:click"] {
        assert!(
            !styles_source.contains(forbidden),
            "BottomSheet styles.rs should not include runtime interaction marker `{forbidden}`."
        );
    }

    for required in ["pub fn BottomSheet(", "<Sheet", "logic::derive_view_state("] {
        assert!(
            view_source.contains(required),
            "BottomSheet view.rs should keep render + semantic mounting marker `{required}`."
        );
    }
    for forbidden in [
        "resolve_state(BottomSheetStateInput {",
        "pub fn derive_view_state(",
    ] {
        assert!(
            !view_source.contains(forbidden),
            "BottomSheet view.rs should not re-implement state machine marker `{forbidden}`."
        );
    }

    for required in [
        "pub struct BottomSheetMotion",
        "pub fn sanitize_motion(motion: BottomSheetMotion) -> BottomSheetMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
    ] {
        assert!(
            motion_source.contains(required),
            "BottomSheet motion.rs should keep semantic-to-motion mapping marker `{required}`."
        );
    }
    for forbidden in [
        "request_animation_frame",
        "spring(",
        "web_sys::",
        "js_sys::",
    ] {
        assert!(
            !motion_source.contains(forbidden),
            "BottomSheet motion.rs should not embed motion engine/browser details (`{forbidden}`)."
        );
    }

    for required in [
        "- [x] 组件目录标准文件落点正确。",
        "bottom_sheet_component_directory_standard_file_layout_is_enforced",
        "bottom_sheet_files_follow_single_responsibility_boundaries",
        "bottom_sheet_does_not_introduce_spec_rs_for_non_complex_scope",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet checklist should keep directory-layout evidence marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_engineering_check_script_covers_serde_tracing_and_runtime_boundaries() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");

    for needle in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_engineering_contract_avoids_runtime_leaks_in_public_api_surface",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_ui_components_fixed_entry_files_follow_layered_boundaries",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_component_directory_standard_file_layout_is_enforced",
    ] {
        assert!(
            script_source.contains(needle),
            "engineering check script should enforce `{needle}`.",
        );
    }
}

#[test]
fn bottom_sheet_file_placement_discipline_contract_is_explicit_for_interactive_component_scope() {
    bottom_sheet_component_directory_standard_file_layout_is_enforced();

    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_dir = manifest_dir
        .parent()
        .and_then(Path::parent)
        .map(|workspace| workspace.join("components/bottom-sheet/src"))
        .unwrap_or_else(|| panic!("workspace root should be two levels above {manifest_dir:?}"));

    let lib_sidecar_path = component_dir.join("lib.rs");
    let protocol_sidecar_path = component_dir.join("protocol.rs");
    assert!(
        lib_sidecar_path.exists() && protocol_sidecar_path.exists(),
        "BottomSheet should keep protocol sidecars (`lib.rs` + `protocol.rs`) explicit in component source directory."
    );

    let lib_sidecar_source = fs::read_to_string(&lib_sidecar_path)
        .unwrap_or_else(|e| panic!("read_to_string failed for {lib_sidecar_path:?}: {e}"));
    assert!(
        lib_sidecar_source.contains("pub const COMPONENT_ID"),
        "BottomSheet lib.rs sidecar should only expose stable component identity."
    );
    for forbidden in [
        "pub fn BottomSheet(",
        "derive_view_state(",
        "BottomSheetStateInput",
        "attach_motion(",
    ] {
        assert!(
            !lib_sidecar_source.contains(forbidden),
            "BottomSheet lib.rs sidecar must not absorb implementation detail `{forbidden}`."
        );
    }

    let protocol_sidecar_source = load_source("src/bottom_sheet/protocol.rs");
    for forbidden in [
        "pub fn BottomSheet(",
        "derive_view_state(",
        "attach_motion(",
        "view!",
        "on:click",
        "data-slot",
    ] {
        assert!(
            !protocol_sidecar_source.contains(forbidden),
            "BottomSheet protocol.rs sidecar must stay out of render/logic/motion implementation (`{forbidden}`)."
        );
    }

    let checklist_source = load_source("../../components/bottom-sheet/check2.md");
    for required in [
        "- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。",
        "`protocol.rs` 与 `lib.rs` 仅作为协议/入口辅助文件",
        "bottom_sheet_file_placement_discipline_contract_is_explicit_for_interactive_component_scope",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            checklist_source.contains(required),
            "BottomSheet checklist should keep file-placement discipline evidence marker `{required}`."
        );
    }

    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");
    let script_command = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_file_placement_discipline_contract_is_explicit_for_interactive_component_scope";
    assert!(
        script_source.contains(script_command),
        "component-files check script should enforce `{script_command}`."
    );
}

#[test]
fn bottom_sheet_component_files_check_script_covers_hyper_structure_builder_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let script_command = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component";
    assert!(
        script_source.contains(script_command),
        "component-files check script should enforce `{script_command}`."
    );
}

#[test]
fn bottom_sheet_check2_marks_hyper_structure_builder_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。",
        "N/A-by-design：`BottomSheet` 不是复杂配置固化型组件",
        "bottom_sheet_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component",
        "bottom_sheet_component_files_check_script_covers_hyper_structure_builder_contract",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet checklist should keep Hyper-Structure Builder marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let component_src_dir = manifest_dir.join("../../components/bottom-sheet/src");
    let manifest_path = component_src_dir.join("Component.toml");
    let rbi_path = component_src_dir.join("bottom_sheet.rbi");

    assert!(
        manifest_path.exists(),
        "BottomSheet should provide Component.toml for context compression."
    );
    assert!(
        rbi_path.exists(),
        "BottomSheet should provide bottom_sheet.rbi for API signature projection."
    );

    let manifest_source = load_source("../../components/bottom-sheet/src/Component.toml");
    let rbi_source = load_source("../../components/bottom-sheet/src/bottom_sheet.rbi");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for required in [
        "schema_version = \"1\"",
        "name = \"BottomSheet\"",
        "crate = \"ui-bottom-sheet\"",
        "name = \"open\"",
        "name = \"on_close\"",
        "name = \"id_base\"",
        "name = \"title\"",
        "name = \"description\"",
        "name = \"footer\"",
        "name = \"lang\"",
        "name = \"dir\"",
        "name = \"motion\"",
        "name = \"bottom_inset_px\"",
        "name = \"class_name\"",
        "name = \"context_compression_manifest\"",
        "name = \"rbi_signature_projection\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "BottomSheet Component.toml should include `{required}`."
        );
    }

    for required in [
        "pub struct BottomSheetMotion {",
        "pub fn BottomSheet(",
        "open: leptos::prelude::Signal<bool>",
        "on_close: crate::OnPress",
        "id_base: String",
        "title: String",
        "children: leptos::children::ChildrenFn",
        "description: Option<String>",
        "footer: Option<leptos::children::ViewFn>",
        "dir: Option<ui_headless::A11yDirection>",
        "motion: BottomSheetMotion",
        "bottom_inset_px: Option<f64>",
        "on_exit_complete: Option<leptos::prelude::Callback<()>>",
        "class_name: Option<String>",
    ] {
        assert!(
            rbi_source.contains(required),
            "bottom_sheet.rbi should include signature projection marker `{required}`."
        );
    }

    for required in [
        "pub fn BottomSheet(",
        "open: Signal<bool>",
        "on_close: OnPress",
        "id_base: String",
        "title: String",
        "#[prop(optional, into)] description: Option<String>",
        "#[prop(optional, into)] footer: Option<ViewFn>",
        "#[prop(optional, into)] lang: Option<String>",
        "#[prop(optional)] dir: Option<A11yDirection>",
        "#[prop(optional)] motion: BottomSheetMotion",
        "#[prop(optional)] bottom_inset_px: Option<f64>",
        "#[prop(optional)] on_exit_complete: Option<Callback<()>>",
        "#[prop(optional, into)] class_name: Option<String>",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet view API should include `{required}` for manifest/RBI alignment."
        );
    }
}

#[test]
fn bottom_sheet_component_files_check_script_covers_context_compression_manifest_rbi_contract() {
    let script_source = load_source("../../scripts/check-ui-components-component-files.sh");

    let script_command = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        script_source.contains(script_command),
        "component-files check script should enforce `{script_command}`."
    );
}

#[test]
fn bottom_sheet_check2_marks_context_compression_manifest_rbi_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。",
        "components/bottom-sheet/src/Component.toml",
        "components/bottom-sheet/src/bottom_sheet.rbi",
        "bottom_sheet_context_compression_manifest_and_rbi_projection_are_present_and_current",
        "bottom_sheet_component_files_check_script_covers_context_compression_manifest_rbi_contract",
        "scripts/check-ui-components-component-files.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet checklist should keep context-compression marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_agent_contract_schema_governance_rules() {
    let checklist_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。",
        "关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。",
        "Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。",
        "契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。",
        "配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。",
    ] {
        assert!(
            checklist_source.contains(required),
            "BottomSheet checklist should keep Agent Contract governance rule `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_agent_contract_is_schema_typed_and_machine_readable() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let manifest_source = load_source("../../components/bottom-sheet/src/Component.toml");

    for required in [
        "pub enum BottomSheetAgentSchemaVersion",
        "pub enum BottomSheetAgentIntent",
        "pub enum BottomSheetAgentAction",
        "pub enum BottomSheetAgentStateAxis",
        "pub enum BottomSheetAgentSourceAxis",
        "pub enum BottomSheetAgentOutputStatus",
        "pub enum BottomSheetAgentStreamSupport",
        "pub enum BottomSheetAgentStreamMode",
        "pub enum BottomSheetAgentStreamFallback",
        "pub enum BottomSheetAgentRenderPolicy",
        "pub struct BottomSheetAgentContract",
        "pub struct BottomSheetAgentContractInput",
        "pub fn resolve_agent_contract(input: BottomSheetAgentContractInput) -> BottomSheetAgentContract",
    ] {
        assert!(
            logic_source.contains(required),
            "BottomSheet agent contract typing should include `{required}`."
        );
    }

    for required in [
        "let agent_contract = Signal::derive(move || {",
        "logic::resolve_agent_contract(logic::BottomSheetAgentContractInput {",
        "data-ui-schema=move || agent_contract.get().schema_name",
        "data-ui-schema-version=move || agent_contract.get().schema_version.as_str()",
        "data-ui-intent=move || agent_contract.get().intent.as_str()",
        "data-ui-action=move || agent_contract.get().action.as_str()",
        "data-ui-state=move || agent_contract.get().state.as_str()",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-render-policy=move || agent_contract.get().render_policy.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet view should mount schemaized agent contract field `{required}`."
        );
    }

    for required in [
        "name = \"agent_contract_schema_markers\"",
        "schema = \"ui.bottom-sheet.agent-contract.v1\"",
        "attr = \"data-ui-schema\"",
        "attr = \"data-ui-intent\"",
        "attr = \"data-ui-action\"",
        "attr = \"data-ui-state\"",
        "attr = \"data-ui-source\"",
        "attr = \"data-ui-stream-mode\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "BottomSheet Component.toml should keep agent-contract marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let combined = format!("{logic_source}\n{view_source}");

    for required in [
        "agent_contract.get().schema_name",
        "agent_contract.get().schema_version.as_str()",
        "agent_contract.get().intent.as_str()",
        "agent_contract.get().action.as_str()",
        "agent_contract.get().state.as_str()",
        "agent_contract.get().source.as_str()",
        "agent_contract.get().stream_mode.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet should derive Agent Contract field from typed contract via `{required}`."
        );
    }

    for forbidden in [
        "data-ui-schema=format!(",
        "data-ui-intent=format!(",
        "data-ui-action=format!(",
        "data-ui-state=format!(",
        "data-ui-source=format!(",
        "data-ui-stream-mode=format!(",
    ] {
        assert!(
            !combined.contains(forbidden),
            "BottomSheet should avoid free-form schema string splicing token `{forbidden}`."
        );
    }
}

#[test]
fn bottom_sheet_agent_contract_render_path_is_whitelist_safe_and_script_injection_free() {
    let view_source = load_source("src/bottom_sheet/view.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let manifest_source = load_source("../../components/bottom-sheet/src/Component.toml");
    let combined = format!("{view_source}\n{logic_source}\n{styles_source}\n{motion_source}");

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
            "BottomSheet Agent Contract render path should stay whitelist-safe without `{forbidden}`."
        );
    }

    for required in [
        "[[agent_contract_whitelist]]",
        "name = \"render_path\"",
        "blocked = [\"inner_html\", \"<script\", \"javascript:\"]",
    ] {
        assert!(
            manifest_source.contains(required),
            "BottomSheet Component.toml should keep render whitelist policy marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_contract_hygiene_script_covers_agent_contract_schema_guards() {
    let script_source = load_source("../../scripts/check-ui-components-contract-hygiene.sh");

    for required in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_agent_contract_schema_governance_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_agent_contract_is_schema_typed_and_machine_readable",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_agent_contract_render_path_is_whitelist_safe_and_script_injection_free",
    ] {
        assert!(
            script_source.contains(required),
            "contract-hygiene script should enforce `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_streaming_definition_is_llm_output_only_with_two_modes() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "`Streaming`：LLM 还在生成，界面边生成边显示。",
        "`Snapshot`：LLM 全部生成完成后，一次性显示。",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet check2 should pin streaming two-mode definition marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_streaming_display_modes_are_limited_to_streaming_and_snapshot() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let manifest_source = load_source("../../components/bottom-sheet/src/Component.toml");

    for required in [
        "pub enum BottomSheetAgentStreamMode {",
        "Streaming,",
        "Snapshot,",
        "Self::Streaming => \"streaming\"",
        "Self::Snapshot => \"snapshot\"",
        "stream_support: BottomSheetAgentStreamSupport::Optional,",
        "stream_mode: BottomSheetAgentStreamMode::Snapshot,",
        "stream_fallback: BottomSheetAgentStreamFallback::Snapshot,",
    ] {
        assert!(
            logic_source.contains(required),
            "BottomSheet logic should keep stream-mode contract marker `{required}`."
        );
    }

    for required in [
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet view should expose stream contract marker `{required}`."
        );
    }

    for required in [
        "name = \"stream_support\"",
        "attr = \"data-ui-stream-support\"",
        "name = \"stream_mode\"",
        "attr = \"data-ui-stream-mode\"",
        "values = [\"streaming\", \"snapshot\"]",
        "name = \"stream_fallback\"",
        "attr = \"data-ui-stream-fallback\"",
    ] {
        assert!(
            manifest_source.contains(required),
            "BottomSheet Component.toml should keep stream marker `{required}`."
        );
    }

    for forbidden in ["token-by-token", "delta-patch-mode", "chunk-stream-mode"] {
        assert!(
            !logic_source.contains(forbidden)
                && !view_source.contains(forbidden)
                && !manifest_source.contains(forbidden),
            "BottomSheet stream contract should avoid undefined mode token `{forbidden}`."
        );
    }
}

#[test]
fn bottom_sheet_streaming_script_covers_two_mode_definition_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_streaming_display_modes_are_limited_to_streaming_and_snapshot",
    ] {
        assert!(
            script_source.contains(required),
            "streaming check script should enforce `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_streaming_two_mode_definition_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。",
        "bottom_sheet_check2_documents_streaming_definition_is_llm_output_only_with_two_modes",
        "bottom_sheet_streaming_display_modes_are_limited_to_streaming_and_snapshot",
        "bottom_sheet_streaming_script_covers_two_mode_definition_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet check2 should keep streaming two-mode evidence marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_snapshot_as_default_baseline_capability() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "所有组件都应能消费“完整生成结果”并稳定渲染。",
        "即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet check2 should pin snapshot baseline marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for required in [
        "let title = logic::resolve_title(title);",
        "let description = logic::normalize_optional_text(description);",
        "let footer = StoredValue::new(footer);",
        "let children = StoredValue::new(children);",
        "let derived_state = logic::derive_view_state(logic::BottomSheetDeriveInput {",
        "let content_input = BottomSheetContentInput {",
        "{move || render_bottom_sheet_content(content_input.clone())}",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet view should keep stable snapshot rendering marker `{required}`."
        );
    }

    for required in [
        "pub fn resolve_agent_contract(input: BottomSheetAgentContractInput) -> BottomSheetAgentContract",
        "stream_mode: BottomSheetAgentStreamMode::Snapshot,",
        "stream_fallback: BottomSheetAgentStreamFallback::Snapshot,",
        "output_status: BottomSheetAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(required),
            "BottomSheet logic should keep snapshot baseline marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_streaming_script_covers_snapshot_baseline_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_snapshot_as_default_baseline_capability",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_snapshot_baseline_consumes_complete_result_and_renders_stably",
    ] {
        assert!(
            script_source.contains(required),
            "streaming check script should enforce `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_snapshot_baseline_capability_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。",
        "bottom_sheet_check2_documents_snapshot_as_default_baseline_capability",
        "bottom_sheet_snapshot_baseline_consumes_complete_result_and_renders_stably",
        "bottom_sheet_streaming_script_covers_snapshot_baseline_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet check2 should keep snapshot baseline evidence marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_streaming_required_optional_classification_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "`Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。",
        "`Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。",
        "无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。",
        "数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。",
        "`BottomSheet` 归类为 `Streaming Optional`",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet checklist should keep streaming responsibility marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");

    for required in [
        "data-slot=\"bottom-sheet\"",
        "data-state=state.state_attr",
        "data-class-source=state.class_source_attr",
        "data-ui-stream-support=move || agent_contract.get().stream_support.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "aria_labelledby=title_id.clone()",
        "aria_describedby=description_id.clone()",
    ] {
        assert!(
            view_source.contains(required),
            "BottomSheet should keep continuous role/aria/data semantics via `{required}` in optional-streaming scope."
        );
    }

    for required in [
        "stream_support: BottomSheetAgentStreamSupport::Optional,",
        "stream_mode: BottomSheetAgentStreamMode::Snapshot,",
        "stream_fallback: BottomSheetAgentStreamFallback::Snapshot,",
        "output_status: BottomSheetAgentOutputStatus::Verified,",
    ] {
        assert!(
            logic_source.contains(required),
            "BottomSheet logic should keep optional-streaming contract marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let combined = format!("{logic_source}\n{view_source}");

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
            "BottomSheet should keep validation/retry/resilience policy in upper layer; component must not include `{forbidden}`."
        );
    }
}

#[test]
fn bottom_sheet_streaming_script_covers_streaming_responsibility_contract() {
    let script_source = load_source("../../scripts/check-ui-components-streaming.sh");

    for required in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_streaming_required_optional_classification_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
    ] {
        assert!(
            script_source.contains(required),
            "streaming check script should enforce `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_streaming_scope_as_optional_with_snapshot_fallback() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。",
        "bottom_sheet_check2_documents_streaming_required_optional_classification_rules",
        "bottom_sheet_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous",
        "bottom_sheet_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer",
        "bottom_sheet_streaming_script_covers_streaming_responsibility_contract",
        "scripts/check-ui-components-streaming.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet check2 should keep streaming scope evidence marker `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources()
{
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let protocol_source = load_source("src/bottom_sheet/protocol.rs");
    let lib_source = load_source("src/bottom_sheet/lib.rs");
    let combined = format!(
        "{mod_source}\n{logic_source}\n{styles_source}\n{view_source}\n{motion_source}\n{protocol_source}\n{lib_source}"
    );

    for forbidden in ["unwrap(", "expect(", "unwrap_err(", "let _ ="] {
        assert!(
            !combined.contains(forbidden),
            "BottomSheet non-test sources should forbid rust-hygiene violation `{forbidden}`."
        );
    }
}

#[test]
fn bottom_sheet_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let combined = format!("{logic_source}\n{view_source}");

    for forbidden in [
        "\"ui-bottom-sheet\".to_string()",
        "\"ui-bottom-sheet--detached\".to_string()",
        "\"ui-bottom-sheet--custom-class\".to_string()",
        "String::from(\"ui-bottom-sheet\")",
        ".to_owned()",
    ] {
        assert!(
            !combined.contains(forbidden),
            "BottomSheet fallback normalization should avoid string clone hotspot `{forbidden}`."
        );
    }

    assert!(
        combined.contains("Cow<'static, str>")
            || (!combined.contains(".to_string()") && !combined.contains("String::from(")),
        "BottomSheet string hotspots should converge to `Cow<'static, str>` or be absent."
    );
}

#[test]
fn bottom_sheet_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let script_source = load_source("../../scripts/check-rust-hygiene.sh");
    let engineering_script = load_source("../../scripts/check-ui-components-engineering.sh");

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

    for required in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(required),
            "engineering check script should enforce `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_rust_hygiene_contract_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。",
        "RUST_HYGIENE_SCOPE='components/bottom-sheet' ./scripts/check-rust-hygiene.sh",
        "bottom_sheet_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "bottom_sheet_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "bottom_sheet_rust_hygiene_script_enforces_repo_level_hygiene_guards",
        "scripts/check-ui-components-engineering.sh",
    ] {
        assert!(
            check2_source.contains(required),
            "BottomSheet check2 rust-hygiene section should reference `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let semantics_source = load_source("tests/bottom_sheet_semantics.rs");
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions()",
        "fn bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking()",
        "fn bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            semantics_source.contains(required_test),
            "BottomSheet semantic/performance regression suite should include `{required_test}`.",
        );
    }

    for marker in [
        "data-state=state.state_attr",
        "data-motion-source=derived_state.motion_source_attr",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
        "data-ui-stream-mode=move || agent_contract.get().stream_mode.as_str()",
        "data-ui-stream-fallback=move || agent_contract.get().stream_fallback.as_str()",
    ] {
        assert!(
            bottom_sheet_view_source.contains(marker),
            "BottomSheet view should expose aria/data semantic marker `{marker}`.",
        );
    }

    for marker in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
        "on:pointerdown=move |ev| ev.stop_propagation()",
        "use_focus_trap(",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "BottomSheet focus/interaction flow should keep semantic marker `{marker}` in Sheet.",
        );
    }

    for marker in ["restore_focus_chain(", "FocusTrapFrame", "FallbackTo("] {
        assert!(
            focus_trap_source.contains(marker),
            "ui-headless focus stack path should expose `{marker}` for BottomSheet focus restoration.",
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count follow-up governance should include `{marker}`.",
        );
    }

    for marker in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui-components --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`.",
        );
    }

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions",
        "bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking",
        "bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "render_count",
        "等价证据",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "BottomSheet check2 semantic/performance section should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let bottom_sheet_view_source = load_source("src/bottom_sheet/view.rs");
    let sheet_view_source = load_source("src/sheet/view.rs");
    let semantics_source = load_source("tests/bottom_sheet_semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "data-state=state.state_attr",
        "data-description=state.description_attr",
        "data-footer=state.footer_attr",
        "data-handle=state.handle_attr",
        "data-close-button=state.close_button_attr",
        "data-detached=state.detached_attr",
        "data-bottom-inset=state.inset_attr",
        "data-motion-source=derived_state.motion_source_attr",
        "data-class-source=state.class_source_attr",
        "data-ui-source=move || agent_contract.get().source.as_str()",
        "data-ui-output-status=move || agent_contract.get().output_status.as_str()",
    ] {
        assert!(
            bottom_sheet_view_source.contains(marker),
            "BottomSheet semantic-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "role=\"dialog\"",
        "aria-modal=\"true\"",
        "aria-labelledby=move || aria_labelledby.get()",
        "aria-describedby=move || aria_describedby.get()",
        "on:keydown=on_key_down",
        "on:pointerdown=move |ev| ev.stop_propagation()",
    ] {
        assert!(
            sheet_view_source.contains(marker),
            "Sheet semantic-priority contract should keep marker `{marker}`."
        );
    }

    for marker in [
        "fn bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions(",
        "fn bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks(",
    ] {
        assert!(
            semantics_source.contains(marker),
            "BottomSheet semantics suite should keep semantic-priority marker `{marker}`."
        );
    }

    let snapshot_macro = ["assert", "_snapshot!"].concat();
    let insta_snapshot = ["insta::assert", "_snapshot"].concat();
    assert!(
        !semantics_source.contains(&snapshot_macro) && !semantics_source.contains(&insta_snapshot),
        "BottomSheet semantic-priority path should avoid snapshot-only assertions.",
    );

    let script_needle = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn bottom_sheet_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-components-performance.sh");

    for marker in [
        "echo \"[perf] contract: bottom-sheet semantic test priority\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should include bottom-sheet semantic-priority marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("../../components/bottom-sheet/src/Component.toml");
    let rbi_source = load_source("../../components/bottom-sheet/src/bottom_sheet.rbi");
    let mod_source = load_source("src/bottom_sheet/mod.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let styles_source = load_source("src/bottom_sheet/styles.rs");
    let motion_source = load_source("src/bottom_sheet/motion.rs");
    let protocol_source = load_source("src/bottom_sheet/protocol.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "schema_version = \"1\"",
        "name = \"BottomSheet\"",
        "crate = \"ui-bottom-sheet\"",
    ] {
        assert!(
            manifest_source.contains(marker),
            "BottomSheet manifest should keep stable v1 schema marker `{marker}`."
        );
    }

    for marker in [
        "pub fn BottomSheet(",
        "open: leptos::prelude::Signal<bool>",
        "on_close: crate::OnPress",
        "id_base: String",
        "title: String",
    ] {
        assert!(
            rbi_source.contains(marker),
            "BottomSheet RBI should keep stable public API marker `{marker}`."
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
        "codemod",
    ] {
        assert!(
            !manifest_source.contains(forbidden)
                && !rbi_source.contains(forbidden)
                && !combined.contains(forbidden),
            "BottomSheet should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for marker in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `BottomSheet` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-components-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "BottomSheet check2 should keep version-migration governance marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-components-engineering.sh");
    let marker = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade";

    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for marker in [
        "const BOTTOM_SHEET_DOC_IMPORTS: &str =",
        "code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"bottom-sheet-state-matrix\"",
        "data-slot=\"bottom-sheet-controlled-uncontrolled\"",
        "data-slot=\"bottom-sheet-streaming-contract\"",
        "requested mode:",
        "requested output status:",
        "effective component status: data-ui-output-status=verified",
        "data-slot=\"bottom-sheet-source-first\"",
        "data-slot=\"bottom-sheet-source-paths\"",
        "component-bottom_sheet",
        "inject-css",
        "compose_copy_ready_code",
    ] {
        assert!(
            docs_source.contains(marker),
            "BottomSheet docs should keep copy-ready + state-matrix + stream/snapshot contract marker `{marker}`.",
        );
    }

    for marker in [
        "const DEFAULT_PLAYGROUND_IMPORTS: &str =",
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String",
        "missing_import_lines(&raw, &imports)",
        "code_imports: Option<String>",
    ] {
        assert!(
            playground_source.contains(marker),
            "Playground copy pipeline should keep import completion marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");
    let marker = "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot";

    assert!(
        script_source.contains(marker),
        "DX check script should enforce `{marker}`."
    );
}

#[test]
fn bottom_sheet_check2_marks_docs_product_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。",
        "apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "BOTTOM_SHEET_DOC_IMPORTS",
        "compose_copy_ready_code",
        "bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "bottom_sheet_dx_check_script_covers_docs_product_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "BottomSheet check2 docs-product section should reference `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_docs_sync_and_state_matrix_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for required in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。",
        "文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。",
        "文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。",
    ] {
        assert!(
            check2_source.contains(required),
            "bottom-sheet check2 should keep docs-sync/state-matrix rule `{required}`."
        );
    }
}

#[test]
fn bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let logic_source = load_source("src/bottom_sheet/logic.rs");
    let view_source = load_source("src/bottom_sheet/view.rs");
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "pub const DEFAULT_TITLE: &str = \"Bottom sheet\";",
        "pub const DEFAULT_CLOSE_LABEL: &str = \"Close bottom sheet\";",
        "pub const DEFAULT_DISMISSABLE: bool = true;",
        "pub const DEFAULT_KEYBOARD_DISMISS_DISABLED: bool = false;",
        "pub const DEFAULT_BOTTOM_INSET_PX: f64 = 0.0;",
        "pub fn resolve_handle_visibility(",
        "pub fn resolve_close_button_visibility(",
        "pub fn resolve_attachment(",
        "pub fn resolve_dismissable(",
        "pub fn resolve_keyboard_dismiss_disabled(",
    ] {
        assert!(
            logic_source.contains(marker),
            "bottom-sheet logic API/default contract should keep marker `{marker}` for docs sync."
        );
    }

    for marker in [
        "#[prop(optional)] is_handle_visible: Option<bool>",
        "#[prop(optional)] is_close_button_visible: Option<bool>",
        "#[prop(optional)] is_detached: Option<bool>",
        "#[prop(optional)] bottom_inset_px: Option<f64>",
        "#[prop(optional)] is_dismissable: Option<bool>",
        "#[prop(optional)] is_keyboard_dismiss_disabled: Option<bool>",
        "#[prop(optional)] show_handle: Option<bool>",
        "#[prop(optional)] show_close_button: Option<bool>",
        "#[prop(optional)] detached: Option<bool>",
    ] {
        assert!(
            view_source.contains(marker),
            "bottom-sheet view API surface should keep marker `{marker}` for docs sync."
        );
    }

    for marker in [
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"bottom-sheet-state-matrix\"",
        "data-slot=\"bottom-sheet-controlled-uncontrolled\"",
        "data-slot=\"bottom-sheet-defaults-contract\"",
        "components/bottom-sheet/src/logic.rs",
        "DEFAULT_TITLE = \\\"Bottom sheet\\\"",
        "DEFAULT_CLOSE_LABEL = \\\"Close bottom sheet\\\"",
        "DEFAULT_DISMISSABLE = true",
        "DEFAULT_KEYBOARD_DISMISS_DISABLED = false",
        "DEFAULT_BOTTOM_INSET_PX = 0.0",
        "resolve_handle_visibility(is_handle_visible, show_handle)",
        "resolve_close_button_visibility(is_close_button_visible, show_close_button)",
        "resolve_attachment(is_detached, detached)",
        "resolve_dismissable(is_dismissable)",
        "resolve_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled)",
    ] {
        assert!(
            docs_source.contains(marker),
            "bottom-sheet docs should keep synced example/matrix/default marker `{marker}`."
        );
    }

    for marker in [
        "- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。",
        "apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet",
        "bottom_sheet_check2_documents_docs_sync_and_state_matrix_rules",
        "bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "scripts/check-ui-components-dx.sh",
    ] {
        assert!(
            check2_source.contains(marker),
            "components/bottom-sheet/check2.md should keep docs-sync evidence marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_dx_check_script_covers_docs_sync_and_state_matrix_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: bottom-sheet docs examples + api/state matrix sync with logic API/defaults\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_docs_sync_and_state_matrix_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include docs-sync/state-matrix marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_docs_sync_and_state_matrix_item_complete() {
    let source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        source.contains("- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。"),
        "bottom-sheet check2 should mark docs-sync/state-matrix checklist item complete."
    );

    for marker in [
        "apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"bottom-sheet-defaults-contract\"",
        "DEFAULT_TITLE",
        "DEFAULT_CLOSE_LABEL",
        "DEFAULT_DISMISSABLE",
        "DEFAULT_KEYBOARD_DISMISS_DISABLED",
        "DEFAULT_BOTTOM_INSET_PX",
        "bottom_sheet_check2_documents_docs_sync_and_state_matrix_rules",
        "bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults",
        "bottom_sheet_dx_check_script_covers_docs_sync_and_state_matrix_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            source.contains(marker),
            "bottom-sheet check2 docs-sync/state-matrix section should reference `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_documentation_as_product_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。",
        "每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。",
        "文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。",
        "“只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 documentation-as-product section should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_documentation_entry_exists_with_beginner_first_progression() {
    let readme_source = load_source("../../components/bottom-sheet/src/README.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for marker in [
        "# BottomSheet",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "默认路径：先用 `open + id_base + title + on_close`",
        "进阶控制：按需启用 `description + footer + motion + is_detached`",
        "### Advanced Example（高级入口）",
    ] {
        assert!(
            readme_source.contains(marker),
            "bottom-sheet README should include beginner-first marker `{marker}`.",
        );
    }

    let readme_hello = readme_source
        .find("## Hello World（最小可用）")
        .expect("bottom-sheet README should include hello-world section");
    let readme_beginner = readme_source
        .find("## 先用起来，再进阶")
        .expect("bottom-sheet README should include beginner-first progression section");
    let readme_common = readme_source
        .find("## 常见用法")
        .expect("bottom-sheet README should include common-usage section");
    let readme_advanced = readme_source
        .find("### Advanced Example（高级入口）")
        .expect("bottom-sheet README should include advanced section");
    assert!(
        readme_hello < readme_beginner
            && readme_beginner < readme_common
            && readme_common < readme_advanced,
        "bottom-sheet README should keep beginner-first progression order (hello -> beginner -> common -> advanced).",
    );

    for marker in [
        "overlays_extra::bottom_sheet",
        "\"BottomSheet\"",
        "\"bottom-sheet\"",
        "pub(super) fn bottom_sheet() -> AnyView",
        "title=\"BottomSheet\"",
        "slug=\"bottom-sheet\"",
        "title=\"Hello World (Minimal Path)\"",
        "title=\"State Matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
    ] {
        assert!(
            pages_source.contains(marker) || docs_source.contains(marker),
            "bottom-sheet docs entry should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_dx_check_script_covers_documentation_as_product_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: bottom-sheet documentation-as-product keeps beginner-first docs entry\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_documentation_as_product_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_documentation_entry_exists_with_beginner_first_progression",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include documentation-as-product marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_documentation_as_product_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        check2_source.contains("- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。"),
        "bottom-sheet check2 should mark documentation-as-product item complete.",
    );

    for marker in [
        "components/bottom-sheet/src/README.md",
        "apps/docs-app/src/pages/components/pages.rs",
        "## Hello World（最小可用）",
        "## 先用起来，再进阶",
        "## 常见用法",
        "bottom_sheet_check2_documents_documentation_as_product_rules",
        "bottom_sheet_documentation_entry_exists_with_beginner_first_progression",
        "bottom_sheet_dx_check_script_covers_documentation_as_product_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 documentation-as-product section should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_interactive_playground_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。",
        "Playground 至少支持基础 props 调整、状态切换、交互反馈观察。",
        "对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。",
        "Playground 作为验收面，需可重复复现关键交互路径。",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 interactive-playground section should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_docs_app_provides_interactive_playground_for_props_state_and_preview() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for marker in [
        "pub(super) fn bottom_sheet() -> AnyView",
        "title=\"State Matrix\"",
        "data-slot=\"bottom-sheet-state-matrix\"",
        "SegmentedControl",
        "id_base=\"docs-bottom-sheet-state-matrix\".to_string()",
        "selected_index=state_matrix_index",
        "set_selected_index=set_state_matrix_index",
        "open=open_state_matrix",
        "is_detached=is_detached",
        "is_close_button_visible=is_close_button_visible",
        "\"description: \"",
        "\"detached: \"",
        "\"close-button-visible: \"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"bottom-sheet-controlled-uncontrolled\"",
        "open=compare_controlled_open",
        "\"open: \"",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"bottom-sheet-streaming-contract\"",
        "data-requested-stream-mode=move || stream_requested_mode.get()",
        "data-requested-output-status=move || stream_requested_output_status.get()",
    ] {
        assert!(
            docs_source.contains(marker),
            "bottom-sheet docs interactive playground should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_interactive_playground_reuses_repeatable_semantic_e2e_flow() {
    let e2e_source = load_source("../../e2e/tests/docs_app_bottom_sheet_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for marker in [
        "docs-app bottom-sheet key flow is repeatable with semantic breakpoints",
        "docs-app bottom-sheet high-risk paths keep overlay focus keyboard and settled semantic breakpoints",
        "[data-slot=\"bottom-sheet-e2e-semantic-controls\"]",
        "[data-slot=\"bottom-sheet-e2e-open-semantic\"]",
        "[data-slot=\"bottom-sheet-e2e-motion-controls\"]",
        "[data-slot=\"bottom-sheet-e2e-open-motion\"]",
        "for (const cycle of [1, 2]) {",
        "bottom-sheet key flow cycle ${cycle}",
        "toHaveAttribute(\"data-state\", \"with-description\")",
        "toHaveAttribute(\"data-footer\", \"present\")",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-custom-motion\", \"true\")",
        "await expectFocusInsidePanel(semanticPanel);",
        "await expectBottomSheetSettledClosed(semanticPanel, semanticRoot, semanticSheet);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "bottom-sheet interactive e2e flow should include `{marker}`.",
        );
    }

    for marker in [
        "data-slot=\"bottom-sheet-e2e-semantic-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-semantic\"",
        "data-slot=\"bottom-sheet-e2e-motion-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-motion\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "bottom-sheet docs should expose stable interactive anchor `{marker}` for repeatable e2e replay.",
        );
    }
}

#[test]
fn bottom_sheet_dx_check_script_covers_interactive_playground_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: bottom-sheet interactive playground docs acceptance surface\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_interactive_playground_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_interactive_playground_reuses_repeatable_semantic_e2e_flow",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include interactive-playground marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_interactive_playground_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        check2_source.contains("- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。"),
        "bottom-sheet check2 should mark interactive-playground item complete.",
    );

    for marker in [
        "title=\"State Matrix\"",
        "data-slot=\"bottom-sheet-state-matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"bottom-sheet-controlled-uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "N/A：`BottomSheet` 非 AI Spec 组件",
        "bottom_sheet_check2_documents_interactive_playground_rules",
        "bottom_sheet_docs_app_provides_interactive_playground_for_props_state_and_preview",
        "bottom_sheet_interactive_playground_reuses_repeatable_semantic_e2e_flow",
        "bottom_sheet_dx_check_script_covers_interactive_playground_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 interactive-playground section should retain marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_source_first_copy_paste_ready_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。",
        "docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。",
        "若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。",
        "文档代码与当前实现必须同步，防止示例漂移。",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 source-first section should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for marker in [
        "data-slot=\"bottom-sheet-source-first\"",
        "data-slot=\"bottom-sheet-source-paths\"",
        "<code>\"Show code\"</code>",
        "BOTTOM_SHEET_DOC_IMPORTS",
        "compose_copy_ready_code",
        "component-bottom_sheet",
        "inject-css",
        "components/bottom-sheet/src/mod.rs",
        "components/bottom-sheet/src/logic.rs",
        "components/bottom-sheet/src/view.rs",
        "components/bottom-sheet/src/styles.rs",
        "components/bottom-sheet/src/motion.rs",
    ] {
        assert!(
            docs_source.contains(marker),
            "bottom-sheet source-first docs should include `{marker}`.",
        );
    }

    for marker in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "DEFAULT_PLAYGROUND_IMPORTS",
        "code_imports",
        "data-slot=\"playground-toggle-code\"",
        "<CodeBlock code=resolved_code.get() />",
    ] {
        assert!(
            playground_source.contains(marker),
            "playground copy pipeline should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_dx_check_script_covers_source_first_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: bottom-sheet source-first docs are copy-paste-ready with real paths and deps\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_source_first_copy_paste_ready_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should include source-first marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_source_first_copy_paste_ready_contract_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        check2_source.contains(
            "- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。"
        ),
        "bottom-sheet check2 should mark source-first copy-paste-ready item complete."
    );

    for marker in [
        "apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet",
        "apps/docs-app/src/playground.rs::compose_copy_ready_code",
        "bottom_sheet_check2_documents_source_first_copy_paste_ready_rules",
        "bottom_sheet_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
        "bottom_sheet_dx_check_script_covers_source_first_copy_paste_ready_contract",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 source-first section should retain marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_heroui_benchmark_docs_sync_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。",
        "组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。",
        "“仅代码更新无文档更新”在接口变更场景下直接判不通过。",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 heroui-benchmark docs-sync section should include `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_heroui_strategy_and_component_docs_are_synchronized_and_indexable() {
    let strategy_source = load_source("../../docs/spec/heroui-parameter-design-strategy.md");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let readme_source = load_source("../../components/bottom-sheet/src/README.md");

    for marker in [
        "### BottomSheet 同步记录（2026-02-20）",
        "参数模型同步：`BottomSheet` 参数主轴保持 `open/on_close`",
        "component_doc!(\"BottomSheet\", \"bottom-sheet\", \"Overlays\", overlays_extra::bottom_sheet)",
        "`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet()`",
        "`components/bottom-sheet/src/README.md` 提供等价组件文档入口",
        "不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`",
        "仅代码更新无文档更新在接口变更场景下不允许合入",
    ] {
        assert!(
            strategy_source.contains(marker),
            "heroui strategy doc should include bottom-sheet synchronization marker `{marker}`.",
        );
    }

    for marker in [
        "component_doc!(",
        "\"BottomSheet\"",
        "\"bottom-sheet\"",
        "overlays_extra::bottom_sheet",
    ] {
        assert!(
            pages_source.contains(marker),
            "component docs index should expose bottom-sheet entry marker `{marker}`.",
        );
    }

    for marker in [
        "pub(super) fn bottom_sheet() -> AnyView",
        "title=\"BottomSheet\"",
        "slug=\"bottom-sheet\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "docs-app bottom-sheet page should stay indexable via marker `{marker}`.",
        );
    }

    for marker in ["# BottomSheet", "## Hello World（最小可用）"] {
        assert!(
            readme_source.contains(marker),
            "bottom-sheet README should remain an equivalent component doc entry via `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_dx_check_script_covers_heroui_benchmark_docs_sync_contract() {
    let script_source = load_source("../../scripts/check-ui-components-dx.sh");

    for marker in [
        "echo \"[dx] contract: bottom-sheet heroui benchmark strategy + docs entry synchronization\"",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_heroui_benchmark_docs_sync_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
    ] {
        assert!(
            script_source.contains(marker),
            "DX check script should enforce heroui-benchmark docs-sync contract `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_heroui_benchmark_docs_sync_contract_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。",
        "bottom_sheet_check2_documents_heroui_benchmark_docs_sync_rules",
        "bottom_sheet_heroui_strategy_and_component_docs_are_synchronized_and_indexable",
        "bottom_sheet_dx_check_script_covers_heroui_benchmark_docs_sync_contract",
        "docs/spec/heroui-parameter-design-strategy.md",
        "scripts/check-ui-components-dx.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 should keep heroui-benchmark docs-sync evidence marker `{marker}`.",
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_semantic_test_priority_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        check2_source.contains(
            "- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。"
        ),
        "bottom-sheet check2 should mark semantic-test-priority item complete."
    );

    for marker in [
        "components/bottom-sheet/test/bottom_sheet_semantics.rs::bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions",
        "components/bottom-sheet/test/bottom_sheet_semantics.rs::bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "components/bottom-sheet/test/bottom_sheet_semantics.rs::bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "scripts/check-ui-components-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 semantic-test-priority section should include `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_e2e_selector_and_stable_wait_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。",
        "E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。",
        "WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。",
        "若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 should keep e2e selector/wait rule `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_e2e_selector_contract_uses_semantic_markers_and_stable_waits() {
    let e2e_source = load_source("../../e2e/tests/docs_app_bottom_sheet_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for marker in [
        "docs-app bottom-sheet contract uses semantic selectors with settled waits",
        "body:not(:has(#boot))",
        "data-slot=\"bottom-sheet-e2e-semantic-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-semantic\"",
        "[data-slot=\"sheet-panel\"][role=\"dialog\"][aria-labelledby=\"docs-bottom-sheet-semantic-title\"]",
        "[data-slot=\"bottom-sheet\"]",
        "toHaveAttribute(\"data-state\", \"open\")",
        "toHaveAttribute(\"data-open\", \"true\")",
        "toHaveAttribute(\"data-placement\", \"bottom\")",
        "toHaveAttribute(\"aria-modal\", \"true\")",
        "toHaveAttribute(\"data-ui-output-status\", \"verified\")",
    ] {
        assert!(
            e2e_source.contains(marker),
            "bottom-sheet e2e selector contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"bottom-sheet-e2e-semantic-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-semantic\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "bottom-sheet docs controls should expose stable e2e selector marker `{marker}`."
        );
    }

    for forbidden in [
        "waitForTimeout(",
        "nth-child(",
        "getByText(",
        "locator(\"text=",
    ] {
        assert!(
            !e2e_source.contains(forbidden),
            "bottom-sheet e2e selector contract should avoid brittle selector/wait token `{forbidden}`."
        );
    }
}

#[test]
fn bottom_sheet_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths() {
    let e2e_source = load_source("../../e2e/tests/docs_app_bottom_sheet_contract.spec.mjs");
    let docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for marker in [
        "docs-app bottom-sheet motion path uses semantic ready and settled breakpoints",
        "data-slot=\"bottom-sheet-e2e-motion-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-motion\"",
        "[data-slot=\"sheet-panel\"][role=\"dialog\"][aria-labelledby=\"docs-bottom-sheet-motion-title\"]",
        "toHaveAttribute(\"data-motion-source\", \"custom\")",
        "toHaveAttribute(\"data-custom-motion\", \"true\")",
        "locator('[data-slot=\"sheet-backdrop\"]').first()",
        "await backdrop.click();",
        "await expect(panel).toHaveCount(0);",
        "await expect(bottomSheetRoot).toHaveCount(0);",
        "await expect(sheetRoot).toHaveCount(0);",
        "await page.keyboard.press(\"Escape\");",
    ] {
        assert!(
            e2e_source.contains(marker),
            "bottom-sheet e2e ready/settled contract should include `{marker}`."
        );
    }

    for marker in [
        "data-slot=\"bottom-sheet-e2e-motion-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-motion\"",
    ] {
        assert!(
            docs_source.contains(marker),
            "bottom-sheet docs controls should expose motion e2e marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_e2e_script_covers_selector_and_ready_settled_contract() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-bottom-sheet.sh");

    for marker in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_e2e_selector_and_stable_wait_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
    ] {
        assert!(
            script_source.contains(marker),
            "bottom-sheet e2e script should include `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_e2e_selector_stability_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        check2_source.contains("- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。"),
        "bottom-sheet check2 should mark e2e selector stability item complete."
    );

    for marker in [
        "e2e/tests/docs_app_bottom_sheet_contract.spec.mjs",
        "body:not(:has(#boot))",
        "data-slot=\"bottom-sheet-e2e-semantic-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-semantic\"",
        "data-slot=\"bottom-sheet-e2e-motion-controls\"",
        "data-slot=\"bottom-sheet-e2e-open-motion\"",
        "ready/settled",
        "bottom_sheet_e2e_selector_contract_uses_semantic_markers_and_stable_waits",
        "bottom_sheet_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths",
        "scripts/check-ui-components-e2e-bottom-sheet.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 e2e selector section should include `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_documents_e2e_repeatable_key_flow_rules() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    for marker in [
        "- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。",
        "至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。",
        "回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。",
        "高风险路径（overlay、focus、keyboard、async）优先进入回归集合。",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 should keep replayable e2e critical-flow rule `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_e2e_key_flow_is_repeatable_and_failure_points_are_semantic() {
    let e2e_source = load_source("../../e2e/tests/docs_app_bottom_sheet_contract.spec.mjs");

    for marker in [
        "docs-app bottom-sheet key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2]) {",
        "bottom-sheet key flow cycle ${cycle}",
        "await openSemantic.focus();",
        "await expect(openSemantic).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(semanticRoot).toHaveAttribute(\"data-state\", \"with-description\");",
        "await expect(semanticRoot).toHaveAttribute(\"data-description\", \"present\");",
        "await expect(semanticRoot).toHaveAttribute(\"data-footer\", \"present\");",
        "await expectFocusInsidePanel(semanticPanel);",
        "await page.keyboard.press(\"Tab\");",
        "await semanticPanel.press(\"Escape\");",
        "await expectBottomSheetSettledClosed(semanticPanel, semanticRoot, semanticSheet);",
        "await expect(openSemantic).toBeFocused();",
    ] {
        assert!(
            e2e_source.contains(marker),
            "bottom-sheet replayable key-flow e2e should keep semantic breakpoint marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints() {
    let e2e_source = load_source("../../e2e/tests/docs_app_bottom_sheet_contract.spec.mjs");

    for marker in [
        "docs-app bottom-sheet high-risk paths keep overlay focus keyboard and settled semantic breakpoints",
        "await openMotion.focus();",
        "await expect(openMotion).toBeFocused();",
        "await page.keyboard.press(\"Enter\");",
        "await expect(motionRoot).toHaveAttribute(\"data-motion-source\", \"custom\");",
        "await expect(motionRoot).toHaveAttribute(\"data-custom-motion\", \"true\");",
        "await expectFocusInsidePanel(motionPanel);",
        "await page.keyboard.press(\"Tab\");",
        "await page.keyboard.press(\"Shift+Tab\");",
        "await backdrop.click();",
        "await expectBottomSheetSettledClosed(motionPanel, motionRoot, motionSheet);",
    ] {
        assert!(
            e2e_source.contains(marker),
            "bottom-sheet high-risk e2e path should keep semantic breakpoint marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_e2e_check_script_covers_repeatable_key_flow_contracts() {
    let script_source = load_source("../../scripts/check-ui-components-e2e-bottom-sheet.sh");

    for marker in [
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_check2_documents_e2e_repeatable_key_flow_rules",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "cargo test -p ui-components --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
    ] {
        assert!(
            script_source.contains(marker),
            "bottom-sheet e2e check script should include replayable critical-flow marker `{marker}`."
        );
    }
}

#[test]
fn bottom_sheet_check2_marks_replayable_e2e_critical_flow_item_complete() {
    let check2_source = load_source("../../components/bottom-sheet/check2.md");

    assert!(
        check2_source.contains("- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。"),
        "bottom-sheet check2 should mark replayable e2e critical-flow item complete."
    );

    for marker in [
        "docs-app bottom-sheet key flow is repeatable with semantic breakpoints",
        "for (const cycle of [1, 2])",
        "await expectFocusInsidePanel(semanticPanel)",
        "docs-app bottom-sheet high-risk paths keep overlay focus keyboard and settled semantic breakpoints",
        "overlay/focus/keyboard",
        "async N/A",
        "bottom_sheet_e2e_key_flow_is_repeatable_and_failure_points_are_semantic",
        "bottom_sheet_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints",
        "scripts/check-ui-components-e2e-bottom-sheet.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "bottom-sheet check2 replayable e2e critical-flow section should include `{marker}`."
        );
    }
}
