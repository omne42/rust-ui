use std::fs;
use std::path::Path;

fn load_source(rel_path: &str) -> String {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mapped = match rel_path {
        "Cargo.toml" => "../../crates/ui/Cargo.toml".to_string(),
        "../Cargo.toml" => "../../crates/ui/Cargo.toml".to_string(),
        "src/lib.rs" => "../../crates/ui/src/lib.rs".to_string(),
        "src/overlays/mod.rs" => "src/mod.rs".to_string(),
        "src/overlays/check2.md" => "src/check2.md".to_string(),
        _ => rel_path.to_string(),
    };
    let path = manifest_dir.join(mapped);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read_to_string failed for {path:?}: {e}"))
}

#[test]
fn overlays_module_reexports_overlay_family_contracts() {
    let source = load_source("src/overlays/mod.rs");

    for needle in [
        "pub use crate::overlay::{Overlay, OverlayMotion};",
        "pub use crate::popover::{Popover, PopoverMotion};",
        "pub use crate::modal::Modal;",
        "pub use crate::tray::{Tray, TrayMotion};",
    ] {
        assert!(
            source.contains(needle),
            "overlays module should expose `{needle}` for ui-baseline overlays compatibility."
        );
    }
}

#[test]
fn overlays_logic_consumes_state_primitives_without_local_state_machine() {
    let source = load_source("src/logic.rs");
    let primitive_source = load_source("../../crates/ui-state-primitives/src/overlays.rs");

    for needle in [
        "pub use ui_state_primitives::overlays::{",
        "DEFAULT_ID_BASE",
        "OverlaysRootState",
        "OverlaysRootStateInput",
        "normalize_id_base",
        "normalize_optional_text",
        "resolve_root_state",
        "pub fn compose_root_class_name(",
    ] {
        assert!(
            source.contains(needle),
            "overlays logic should include `{needle}` while consuming ui-state-primitives."
        );
    }

    for forbidden in [
        "pub enum OverlaysLayerKind",
        "pub struct OverlaysRootStateInput",
        "pub struct OverlaysRootState",
        "pub fn resolve_root_state(input: OverlaysRootStateInput)",
    ] {
        assert!(
            !source.contains(forbidden),
            "overlays logic should not reimplement primitive `{forbidden}`."
        );
    }

    assert!(
        primitive_source.contains(
            "pub fn resolve_root_state(input: OverlaysRootStateInput) -> OverlaysRootState"
        ),
        "overlays state primitive should live in ui-state-primitives."
    );
}

#[test]
fn overlays_headless_semantics_are_delegated_to_ui_headless_layer() {
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");

    for forbidden in ["on:keydown", "on:pointerdown"] {
        assert!(
            !overlays_view.contains(forbidden),
            "OverlaysRoot should stay assembly-only and avoid local interaction semantics `{forbidden}`."
        );
    }
    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "labeled_group_attrs(",
    ] {
        assert!(
            overlays_view.contains(needle),
            "OverlaysRoot should use ui-headless a11y utility `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
        "FocusTrapOptions",
        "ModalOptions",
    ] {
        assert!(
            overlay_view.contains(needle),
            "Overlay should consume `{needle}` from ui-headless."
        );
    }

    for needle in [
        "use ui_headless::{",
        "use_focus_trap",
        "use_popover_position",
        "use_overlay_stack_registration",
        "PopoverPositionOptions",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover should consume `{needle}` from ui-headless."
        );
    }

    assert!(
        !modal_view.contains("use ui_headless::") && modal_view.contains("<Overlay"),
        "Modal should compose Overlay for interaction semantics instead of reimplementing headless contracts."
    );

    for needle in [
        "use ui_headless::{A11yDirection, overlay_dialog_attrs};",
        "lang: Option<String>",
        "dir: Option<A11yDirection>",
        "overlay_dialog_attrs(",
    ] {
        assert!(
            tray_view.contains(needle),
            "Tray should consume `{needle}` from ui-headless a11y contracts."
        );
    }
}

#[test]
fn overlays_open_state_pairing_contract_is_explicit_and_stable() {
    let modal_view = load_source("../modal/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
        "let open = input.is_open;",
        "default_open: input.default_open.unwrap_or(DEFAULT_OPEN)",
        "use_controllable_open_state_traced(",
        "request_open_change.run(false);",
    ] {
        assert!(
            modal_view.contains(needle) || modal_logic.contains(needle),
            "Modal open-axis controlled/uncontrolled contract should include `{needle}`."
        );
    }

    for forbidden in [
        "#[prop(optional)] open: Option<Signal<bool>>",
        "input.is_open.or(input.open)",
        "OpenAlias",
    ] {
        assert!(
            !modal_view.contains(forbidden) && !modal_logic.contains(forbidden),
            "Modal should not keep half-controlled open alias drift `{forbidden}`."
        );
    }

    for (component, source) in [
        ("Overlay", overlay_view),
        ("Popover", popover_view),
        ("Tray", tray_view),
    ] {
        assert!(
            source.contains("open: Signal<bool>") && source.contains("on_close: OnPress"),
            "{component} should remain intent-only and consume parent-owned open signal."
        );
        for forbidden in ["default_open", "on_open_change"] {
            assert!(
                !source.contains(forbidden),
                "{component} should not introduce local open-state ownership via `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_default_value_normalization_is_centralized_in_logic_layer() {
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    for (component, source) in [
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Sheet", sheet_view.as_str()),
        ("Tray", tray_view.as_str()),
    ] {
        for forbidden in [
            "unwrap_or_else(|| Callback::new(|_| {}))",
            "get_value().unwrap_or_default()",
        ] {
            assert!(
                !source.contains(forbidden),
                "{component} view should not normalize defaults inline via `{forbidden}`."
            );
        }
    }

    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let tray_logic = load_source("../tray/src/logic.rs");

    for (component, source) in [
        ("Overlay", overlay_logic.as_str()),
        ("Popover", popover_logic.as_str()),
        ("Sheet", sheet_logic.as_str()),
    ] {
        assert!(
            source.contains("pub fn normalize_on_exit_complete(callback: Option<Callback<()>>)"),
            "{component} logic should own on_exit_complete default normalization."
        );
    }

    assert!(
        tray_logic.contains("pub fn normalize_on_exit_complete(callback: Option<Callback<()>>)"),
        "Tray logic should own on_exit_complete default normalization."
    );
    assert!(
        tray_logic.contains("pub fn normalize_optional_attr(value: Option<String>) -> String"),
        "Tray logic should own optional aria/description fallback normalization."
    );
}

#[test]
fn overlays_state_normalization_is_centralized_in_logic_layer() {
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_view_non_comment = sheet_view
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");

    for (component, source, forbidden) in [
        ("Overlay", overlay_view.as_str(), "OverlayPartStateInput {"),
        ("Popover", popover_view.as_str(), "PopoverPartStateInput {"),
        ("Tray", tray_view.as_str(), "TrayPartStateInput {"),
    ] {
        assert!(
            !source.contains(forbidden),
            "{component} view should not assemble raw part-state inputs via `{forbidden}`."
        );
    }

    for forbidden in ["struct SheetStateInputs", "fn resolve_part_state("] {
        assert!(
            !sheet_view_non_comment.contains(forbidden),
            "Sheet view should not carry local state-normalization helper `{forbidden}`."
        );
    }

    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let tray_logic = load_source("../tray/src/logic.rs");

    for (component, source, needle) in [
        (
            "Overlay",
            overlay_logic.as_str(),
            "fn resolve_states(input: OverlayStateInputs) -> OverlayResolvedStates",
        ),
        (
            "Popover",
            popover_logic.as_str(),
            "fn resolve_states(input: PopoverStateInputs) -> PopoverResolvedStates",
        ),
        (
            "Sheet",
            sheet_logic.as_str(),
            "fn resolve_states(input: SheetStateInputs) -> SheetResolvedStates",
        ),
        (
            "Tray",
            tray_logic.as_str(),
            "fn resolve_states(input: TrayStateInputs) -> TrayResolvedStates",
        ),
    ] {
        assert!(
            source.contains(needle),
            "{component} logic should own centralized state normalization via `{needle}`."
        );
    }

    for needle in [
        "pub enum OverlayDismissMode",
        "pub enum OverlayKeyboardDismissMode",
        "pub dismiss_mode: OverlayDismissMode",
        "pub keyboard_dismiss_mode: OverlayKeyboardDismissMode",
    ] {
        assert!(
            overlay_logic.contains(needle),
            "Overlay logic should type constrain discrete mode axis via `{needle}`."
        );
    }
    for forbidden in [
        "pub is_dismissable: bool",
        "pub is_keyboard_dismiss_disabled: bool",
    ] {
        assert!(
            !overlay_logic.contains(forbidden),
            "Overlay state inputs should avoid bool explosion via `{forbidden}`."
        );
    }

    for needle in [
        "pub enum PopoverModalMode",
        "pub modal_mode: PopoverModalMode",
    ] {
        assert!(
            popover_logic.contains(needle),
            "Popover logic should type constrain modal mode via `{needle}`."
        );
    }
    assert!(
        !popover_logic.contains("pub is_modal: bool"),
        "Popover state inputs should avoid bool modal axis."
    );

    for needle in [
        "pub enum SheetDismissMode",
        "pub enum SheetKeyboardDismissMode",
        "pub dismiss_mode: SheetDismissMode",
        "pub keyboard_dismiss_mode: SheetKeyboardDismissMode",
    ] {
        assert!(
            sheet_logic.contains(needle),
            "Sheet logic should type constrain discrete mode axis via `{needle}`."
        );
    }

    for needle in [
        "pub enum TrayDescriptionMode",
        "pub enum TrayFooterMode",
        "pub enum TrayCloseButtonMode",
        "pub enum TraySizeMode",
        "pub enum TrayDismissMode",
        "pub enum TrayKeyboardDismissMode",
        "pub description_mode: TrayDescriptionMode",
        "pub footer_mode: TrayFooterMode",
        "pub close_button_mode: TrayCloseButtonMode",
        "pub size_mode: TraySizeMode",
        "pub dismiss_mode: TrayDismissMode",
        "pub keyboard_dismiss_mode: TrayKeyboardDismissMode",
    ] {
        assert!(
            tray_logic.contains(needle),
            "Tray logic should type constrain discrete status via `{needle}`."
        );
    }
    for forbidden in [
        "pub has_description: bool",
        "pub has_footer: bool",
        "pub show_close_button: bool",
        "pub is_fixed_height: bool",
        "pub is_dismissable: bool",
        "pub is_keyboard_dismiss_disabled: bool",
    ] {
        assert!(
            !tray_logic.contains(forbidden),
            "Tray state inputs should avoid bool explosion via `{forbidden}`."
        );
    }
}

#[test]
fn overlays_state_primitive_sources_are_consumed_with_store_boundary() {
    let overlays_logic = load_source("src/logic.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");

    assert!(
        overlays_logic.contains("pub use ui_state_primitives::overlays::{"),
        "OverlaysRoot logic should consume root-state primitives from ui-state-primitives."
    );
    assert!(
        tray_logic.contains("pub use ui_state_primitives::tray::{"),
        "Tray logic should consume tray-state primitives from ui-state-primitives."
    );

    for (component, source) in [
        ("Overlay logic", overlay_logic.as_str()),
        ("Popover logic", popover_logic.as_str()),
        ("Sheet logic", sheet_logic.as_str()),
        ("Overlay view", overlay_view.as_str()),
        ("Popover view", popover_view.as_str()),
        ("Sheet view", sheet_view.as_str()),
        ("Tray view", tray_view.as_str()),
    ] {
        for forbidden in [
            "redux",
            "zustand",
            "pinia",
            "mobx",
            "app_store",
            "global_store",
            "use_store",
            "store::",
            "SignalStore",
            "AppStateStore",
        ] {
            assert!(
                !source.contains(forbidden),
                "{component} should not bind business/global store details via `{forbidden}`."
            );
        }
    }

    for (component, source) in [
        ("Overlay logic", overlay_logic.as_str()),
        ("Popover logic", popover_logic.as_str()),
        ("Sheet logic", sheet_logic.as_str()),
    ] {
        for forbidden in [
            "RwSignal",
            "ReadSignal",
            "WriteSignal",
            "StoredValue",
            "web_sys::",
            "wasm_bindgen",
        ] {
            assert!(
                !source.contains(forbidden),
                "{component} should stay as mapping logic and avoid framework/store containers `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_have_no_async_loading_error_retry_protocol_surface() {
    let overlays_view = load_source("src/view.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");

    for (component, view_source, logic_source) in [
        (
            "OverlaysRoot",
            overlays_view.as_str(),
            overlays_logic.as_str(),
        ),
        ("Overlay", overlay_view.as_str(), overlay_logic.as_str()),
        ("Popover", popover_view.as_str(), popover_logic.as_str()),
        ("Modal", modal_view.as_str(), modal_logic.as_str()),
        ("Tray", tray_view.as_str(), tray_logic.as_str()),
        ("Sheet", sheet_view.as_str(), sheet_logic.as_str()),
    ] {
        for forbidden in [
            "is_loading",
            "aria-busy",
            "aria_busy",
            "use_async_action",
            "on_retry",
            "error_state",
        ] {
            assert!(
                !view_source.contains(forbidden) && !logic_source.contains(forbidden),
                "{component} is currently non-async and should not expose async protocol token `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_dx_paradox_keeps_minimal_api_path_without_internal_state_wiring() {
    let overlays_readme = load_source("src/README.md");
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    assert!(
        overlays_readme
            .contains("基础路径无需手动接线 `ui-state-primitives` / `ui-headless` 状态机"),
        "Overlays README should document that basic usage does not require manual primitives/headless wiring."
    );
    assert!(
        overlays_readme.contains(
            "  <Modal default_open=true id_base=\"m\".to_string() title=\"Hello\".to_string() on_close=Callback::new(|_| {})>"
        ) && overlays_readme.contains("    <div>\"Hello overlays\"</div>"),
        "Overlays README should keep a <=5-line Hello World path with default props."
    );
    assert!(
        overlays_docs.contains("title=\"Hello World (Minimal Path)\"")
            && overlays_docs.contains(
                "description=\"Default path: no manual state-machine wiring, simple props only.\""
            )
            && overlays_docs.contains("MODAL_MINIMAL_PLAYGROUND_CODE"),
        "docs-app overlays page should expose a minimal default-path playground."
    );

    for source in [
        overlays_view.as_str(),
        overlay_view.as_str(),
        popover_view.as_str(),
        modal_view.as_str(),
        tray_view.as_str(),
        sheet_view.as_str(),
    ] {
        for forbidden in [
            "#[prop(optional)] state:",
            "#[prop(optional, into)] state:",
            "#[prop(into)] state:",
            "#[prop(optional)] store:",
            "#[prop(optional)] machine:",
        ] {
            assert!(
                !source.contains(forbidden),
                "Overlays family public API should not require internal state/machine object prop `{forbidden}`."
            );
        }
    }

    for needle in [
        "#[prop(optional)] is_open: Option<Signal<bool>>",
        "#[prop(optional)] default_open: Option<bool>",
        "#[prop(optional)] on_open_change: Option<Callback<bool>>",
    ] {
        assert!(
            modal_view.contains(needle),
            "Modal should keep simple default path and optional advanced control via `{needle}`."
        );
    }
}

#[test]
fn overlays_parent_item_composition_rule_is_not_applicable_and_no_parallel_array_api_leaks() {
    let overlays_readme = load_source("src/README.md");
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    // Overlays family is non-collection by design: no Parent<Item> tree contract should be required.
    for (scope, source) in [
        ("overlays README", overlays_readme.as_str()),
        ("overlays mod", overlays_mod.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlay view", overlay_view.as_str()),
        ("popover view", popover_view.as_str()),
        ("modal view", modal_view.as_str()),
        ("tray view", tray_view.as_str()),
        ("sheet view", sheet_view.as_str()),
        ("overlays docs", overlays_docs.as_str()),
        ("overlays extra docs", overlays_extra_docs.as_str()),
    ] {
        for forbidden in [
            "labels + children",
            "titles + panels",
            "ItemSpec",
            "items: Vec<",
            "titles: Vec<",
            "labels: Vec<",
            "<Item",
            "item_specs",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not introduce collection-style implicit pairing API `{forbidden}`."
            );
        }
    }

    for required in [
        "pub use crate::overlay::{Overlay, OverlayMotion};",
        "pub use crate::popover::{Popover, PopoverMotion};",
        "pub use crate::modal::Modal;",
        "pub use crate::tray::{Tray, TrayMotion};",
    ] {
        assert!(
            overlays_mod.contains(required),
            "Overlays module should stay as explicit family composition via `{required}`."
        );
    }
}

#[test]
fn overlays_have_no_dragging_macro_micro_state_machine_path() {
    let overlays_view = load_source("src/view.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let tray_motion = load_source("../tray/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");

    for (scope, source) in [
        ("OverlaysRoot view", overlays_view.as_str()),
        ("OverlaysRoot logic", overlays_logic.as_str()),
        ("Overlay view", overlay_view.as_str()),
        ("Overlay logic", overlay_logic.as_str()),
        ("Popover view", popover_view.as_str()),
        ("Popover logic", popover_logic.as_str()),
        ("Modal view", modal_view.as_str()),
        ("Modal logic", modal_logic.as_str()),
        ("Tray view", tray_view.as_str()),
        ("Tray logic", tray_logic.as_str()),
        ("Sheet view", sheet_view.as_str()),
        ("Sheet logic", sheet_logic.as_str()),
        ("Overlay motion", overlay_motion.as_str()),
        ("Popover motion", popover_motion.as_str()),
        ("Tray motion", tray_motion.as_str()),
        ("Sheet motion", sheet_motion.as_str()),
    ] {
        for forbidden in [
            "Dragging",
            "DragStart",
            "DragEnd",
            "Action::DragEnd",
            "on:drag",
            "on:dragstart",
            "on:dragend",
            "on:pointermove",
            "on:mousemove",
            "on:touchmove",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not include drag macro/micro state-machine token `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_two_pass_geometry_pipeline_is_delegated_and_idempotent() {
    let popover_view = load_source("../popover/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let headless_popover_position = load_source("../../crates/ui-headless/src/popover_position.rs");
    let headless_popover_position_tests =
        load_source("../../crates/ui-headless/src/test/popover_position.rs");

    for needle in [
        "use_popover_position(PopoverPositionOptions {",
        "position.top_px.get()",
        "position.left_px.get()",
        "position.anchor_width_px.get()",
        "data-placement=move || position.placement.get().as_str()",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover view should keep Intent->Measure->Rectification pipeline hook via `{needle}`."
        );
    }

    for forbidden in ["get_bounding_client_rect", "compute_popover_position("] {
        assert!(
            !popover_logic.contains(forbidden),
            "Popover logic should not absorb DOM measurement/rectification detail `{forbidden}`."
        );
    }

    for needle in [
        "fn compute_popover_position(",
        "let anchor_rect = anchor_el.get_bounding_client_rect();",
        "let panel_rect = panel_el.get_bounding_client_rect();",
        "let computed = compute_popover_position(",
        "let should_flip =",
        "left = left.clamp(",
        "top = top.clamp(",
        "const POSITION_EPSILON_PX: f64 = 0.01;",
        "fn should_update_scalar(current: f64, next: f64) -> bool",
        "if should_update_scalar(top_px.get_untracked(), computed.top) {",
        "if should_update_scalar(left_px.get_untracked(), computed.left) {",
        "if raf_pending.get_value() {",
        "window.request_animation_frame(",
        "ResizeObserver::new(",
        "observer.observe(&anchor_el);",
        "observer.observe(&panel_el);",
    ] {
        assert!(
            headless_popover_position.contains(needle),
            "ui-headless popover_position should contain two-pass + idempotent guard detail `{needle}`."
        );
    }

    for needle in [
        "fn scalar_update_guard_ignores_sub_epsilon_noise()",
        "fn scalar_update_guard_accepts_meaningful_delta()",
        "fn flips_to_top_when_bottom_does_not_fit()",
    ] {
        assert!(
            headless_popover_position_tests.contains(needle),
            "ui-headless popover_position tests should lock rectification/idempotent behavior via `{needle}`."
        );
    }
}

#[test]
fn overlays_registration_protocol_is_not_applicable_without_dynamic_item_registry() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_readme = load_source("src/README.md");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for (scope, source) in [
        ("overlays mod", overlays_mod.as_str()),
        ("overlays logic", overlays_logic.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlays README", overlays_readme.as_str()),
        ("overlay logic", overlay_logic.as_str()),
        ("overlay view", overlay_view.as_str()),
        ("popover logic", popover_logic.as_str()),
        ("popover view", popover_view.as_str()),
        ("modal logic", modal_logic.as_str()),
        ("modal view", modal_view.as_str()),
        ("tray logic", tray_logic.as_str()),
        ("tray view", tray_view.as_str()),
        ("sheet logic", sheet_logic.as_str()),
        ("sheet view", sheet_view.as_str()),
        ("overlays docs", overlays_docs.as_str()),
        ("overlays extra docs", overlays_extra_docs.as_str()),
    ] {
        for forbidden in [
            "RegistrationContext",
            "Register",
            "Unregister",
            "items_order",
            "HashSet",
            "BTreeSet",
            "set_items_order",
            "register_item",
            "unregister_item",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not include dynamic item-registration protocol token `{forbidden}`."
            );
        }
    }

    for required in [
        "pub use crate::overlay::{Overlay, OverlayMotion};",
        "pub use crate::popover::{Popover, PopoverMotion};",
        "pub use crate::modal::Modal;",
        "pub use crate::tray::{Tray, TrayMotion};",
    ] {
        assert!(
            overlays_mod.contains(required),
            "overlays module should remain family-composition only via `{required}`."
        );
    }
}

#[test]
fn overlays_slot_projection_strategy_is_not_applicable_without_projection_modes() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_motion = load_source("src/motion.rs");
    let overlays_readme = load_source("src/README.md");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let modal_motion = load_source("../modal/src/motion.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let tray_motion = load_source("../tray/src/motion.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for (scope, source) in [
        ("overlays mod", overlays_mod.as_str()),
        ("overlays logic", overlays_logic.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlays motion", overlays_motion.as_str()),
        ("overlays README", overlays_readme.as_str()),
        ("overlay logic", overlay_logic.as_str()),
        ("overlay view", overlay_view.as_str()),
        ("overlay motion", overlay_motion.as_str()),
        ("popover logic", popover_logic.as_str()),
        ("popover view", popover_view.as_str()),
        ("popover motion", popover_motion.as_str()),
        ("modal logic", modal_logic.as_str()),
        ("modal view", modal_view.as_str()),
        ("modal motion", modal_motion.as_str()),
        ("tray logic", tray_logic.as_str()),
        ("tray view", tray_view.as_str()),
        ("tray motion", tray_motion.as_str()),
        ("sheet logic", sheet_logic.as_str()),
        ("sheet view", sheet_view.as_str()),
        ("sheet motion", sheet_motion.as_str()),
        ("overlays docs", overlays_docs.as_str()),
        ("overlays extra docs", overlays_extra_docs.as_str()),
    ] {
        for forbidden in [
            "Lazy",
            "KeepAlive",
            "Eager",
            "NotifyHidden",
            "slot_projection",
            "projection_mode",
            "on_hidden",
            "pause_effects",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not expose slot-projection strategy token `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_env_streams_are_delegated_to_headless_with_backpressure_guards() {
    let overlays_view = load_source("src/view.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let headless_popover_position = load_source("../../crates/ui-headless/src/popover_position.rs");

    assert!(
        popover_view.contains("use_popover_position(PopoverPositionOptions {"),
        "Popover view should delegate env-stream sampling via use_popover_position contract."
    );
    for forbidden in [
        "ResizeObserver::new(",
        "request_animation_frame(",
        "on:resize",
        "on:scroll",
    ] {
        assert!(
            !popover_view.contains(forbidden),
            "Popover view should avoid raw env-stream implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "let raf_pending = StoredValue::new_local(false);",
        "if raf_pending.get_value() {",
        "window.request_animation_frame(",
        "web_sys::ResizeObserver::new(",
    ] {
        assert!(
            headless_popover_position.contains(needle),
            "ui-headless popover_position should include env-stream backpressure guard `{needle}`."
        );
    }

    for (scope, source) in [
        ("OverlaysRoot view", overlays_view.as_str()),
        ("OverlaysRoot logic", overlays_logic.as_str()),
        ("Overlay view", overlay_view.as_str()),
        ("Overlay logic", overlay_logic.as_str()),
        ("Popover logic", popover_logic.as_str()),
        ("Modal view", modal_view.as_str()),
        ("Modal logic", modal_logic.as_str()),
        ("Tray view", tray_view.as_str()),
        ("Tray logic", tray_logic.as_str()),
        ("Sheet view", sheet_view.as_str()),
        ("Sheet logic", sheet_logic.as_str()),
    ] {
        for forbidden in [
            "on:resize",
            "on:scroll",
            "IntersectionObserver",
            "matchMedia",
            "BreakpointChanged",
            "ThemeChanged",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not introduce raw env-stream token `{forbidden}` in component layer."
            );
        }
    }
}

#[test]
fn overlays_event_light_cone_is_not_applicable_without_bulk_collection_bus() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_readme = load_source("src/README.md");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for (scope, source) in [
        ("overlays mod", overlays_mod.as_str()),
        ("overlays logic", overlays_logic.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlays README", overlays_readme.as_str()),
        ("overlay logic", overlay_logic.as_str()),
        ("overlay view", overlay_view.as_str()),
        ("popover logic", popover_logic.as_str()),
        ("popover view", popover_view.as_str()),
        ("modal logic", modal_logic.as_str()),
        ("modal view", modal_view.as_str()),
        ("tray logic", tray_logic.as_str()),
        ("tray view", tray_view.as_str()),
        ("sheet logic", sheet_logic.as_str()),
        ("sheet view", sheet_view.as_str()),
        ("overlays docs", overlays_docs.as_str()),
        ("overlays extra docs", overlays_extra_docs.as_str()),
    ] {
        for forbidden in [
            "Context Bus",
            "ContextBus",
            "SelectionState::All",
            "SelectionState",
            "bulk_select",
            "batch_select",
            "prop drilling",
            "prop_drilling",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not expose event-light-cone bulk-collection token `{forbidden}`."
            );
        }
    }

    for required in [
        "pub use crate::overlay::{Overlay, OverlayMotion};",
        "pub use crate::popover::{Popover, PopoverMotion};",
        "pub use crate::modal::Modal;",
        "pub use crate::tray::{Tray, TrayMotion};",
    ] {
        assert!(
            overlays_mod.contains(required),
            "overlays module should remain family composition only via `{required}`."
        );
    }
}

#[test]
fn overlays_causality_bus_is_not_applicable_without_trace_propagation_bus() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_readme = load_source("src/README.md");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_docs =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for (scope, source) in [
        ("overlays mod", overlays_mod.as_str()),
        ("overlays logic", overlays_logic.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlays README", overlays_readme.as_str()),
        ("overlay logic", overlay_logic.as_str()),
        ("overlay view", overlay_view.as_str()),
        ("popover logic", popover_logic.as_str()),
        ("popover view", popover_view.as_str()),
        ("modal logic", modal_logic.as_str()),
        ("modal view", modal_view.as_str()),
        ("tray logic", tray_logic.as_str()),
        ("tray view", tray_view.as_str()),
        ("sheet logic", sheet_logic.as_str()),
        ("sheet view", sheet_view.as_str()),
        ("overlays docs", overlays_docs.as_str()),
        ("overlays extra docs", overlays_extra_docs.as_str()),
    ] {
        for forbidden in [
            "TraceId",
            "trace_id",
            "Causality Bus",
            "CausalityBus",
            "causality_bus",
            "broadcast_to_subscribers",
            "command_bus",
            "event_bus",
            "subscriber",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not expose causality-bus token `{forbidden}`."
            );
        }
    }

    for required in [
        "pub use crate::overlay::{Overlay, OverlayMotion};",
        "pub use crate::popover::{Popover, PopoverMotion};",
        "pub use crate::modal::Modal;",
        "pub use crate::tray::{Tray, TrayMotion};",
    ] {
        assert!(
            overlays_mod.contains(required),
            "overlays module should remain family composition only via `{required}`."
        );
    }
}

#[test]
fn overlays_a11y_i18n_l10n_contracts_are_headless_first_and_text_source_driven() {
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");

    for needle in [
        "pub const DEFAULT_ARIA_LABEL: &str = \"Overlays\";",
        "pub fn normalize_aria_label(value: Option<String>) -> (String, bool)",
    ] {
        assert!(
            overlays_logic.contains(needle),
            "overlays logic should keep a11y label normalization contract `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{A11yDirection, labeled_group_attrs};",
        "#[prop(optional, into)] aria_label: Option<String>,",
        "#[prop(optional, into)] lang: Option<String>,",
        "#[prop(optional)] dir: Option<A11yDirection>,",
        "let (aria_label, has_custom_aria_label) = logic::normalize_aria_label(aria_label);",
        "labeled_group_attrs(aria_label, logic::normalize_optional_text(lang), dir);",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label.clone()",
        "lang=group_a11y.lang.clone()",
        "dir=group_a11y.dir",
        "data-aria-label-source=if has_custom_aria_label { \"custom\" } else { \"default\" }",
        "data-custom-aria-label=has_custom_aria_label.then_some(\"true\")",
    ] {
        assert!(
            overlays_view.contains(needle),
            "OverlaysRoot view should expose a11y/i18n contract token `{needle}`."
        );
    }
    assert!(
        !overlays_view.contains("aria-label=\"Overlays\""),
        "OverlaysRoot view should not hardcode screen-reader visible label literals."
    );

    for needle in [
        "use ui_headless::{",
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
        "on:keydown=on_key_down",
        "role=role",
        "aria-modal=\"true\"",
    ] {
        assert!(
            overlay_view.contains(needle),
            "Overlay should keep interactive a11y semantics through `{needle}`."
        );
    }

    for needle in [
        "use ui_headless::{",
        "use_focus_trap",
        "use_modal",
        "use_overlay_stack_registration",
        "on:keydown=on_key_down",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover should keep keyboard/focus semantics through `{needle}`."
        );
    }

    for (scope, source) in [
        ("Modal", modal_view.as_str()),
        ("Tray", tray_view.as_str()),
        ("Sheet", sheet_view.as_str()),
    ] {
        for needle in [
            "A11yDirection",
            "lang: Option<String>",
            "dir: Option<",
            "overlay_dialog_attrs(",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should consume headless dialog a11y contract `{needle}`."
            );
        }
    }

    assert!(
        tray_view.contains(
            "#[prop(optional, default = logic::DEFAULT_CLOSE_LABEL)] close_label: &'static str,"
        ),
        "Tray should keep visible close-label default in logic constant instead of view literal."
    );
    assert!(
        !tray_view.contains("\"Close tray\""),
        "Tray view should not hardcode user-facing close label text literal."
    );

    for needle in [
        "pub fn labeled_group_attrs(",
        "pub fn overlay_dialog_attrs(",
        "pub fn locale_attrs(",
    ] {
        assert!(
            headless_a11y.contains(needle),
            "ui-headless a11y helpers should expose `{needle}` for shared semantic contracts."
        );
    }
}

#[test]
fn overlays_state_markers_are_observable_queryable_and_closed_set() {
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlays_primitive = load_source("../../crates/ui-state-primitives/src/overlays.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let overlays_docs = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let nav_sheet_e2e = load_source("../../e2e/tests/docs_app_nav_sheet.spec.mjs");

    for needle in [
        "data-slot=\"overlays\"",
        "data-state=move || state.get().data_state_attr",
        "data-layer=move || state.get().layer_kind_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-aria-label-source=if has_custom_aria_label { \"custom\" } else { \"default\" }",
        "data-id-source=move || state.get().id_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            overlays_view.contains(needle),
            "OverlaysRoot should expose observable/queryable marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-role-source=root_state.role_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "role=role",
        "aria-modal=\"true\"",
    ] {
        assert!(
            overlay_view.contains(needle),
            "Overlay should expose semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-placement=move || position.placement.get().as_str()",
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        "data-motion-source=root_state.motion_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover should expose semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open=move || is_open.get().then_some(\"true\")",
        "data-closed=move || (!is_open.get()).then_some(\"true\")",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            modal_view.contains(needle),
            "Modal should expose semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-state=root_state.state_attr",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-size-source=root_state.size_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-title-source=root_state.title_source_attr",
        "data-description-source=root_state.description_source_attr",
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
    ] {
        assert!(
            tray_view.contains(needle),
            "Tray should expose semantic marker `{needle}`."
        );
    }

    for needle in [
        "data-state=move || logic::state_attr_for_open(open.get())",
        "data-open=move || open.get().then_some(\"true\")",
        "data-closed=move || (!open.get()).then_some(\"true\")",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-aria-labelledby-source=root_state.aria_labelledby_source_attr",
        "data-aria-describedby-source=root_state.aria_describedby_source_attr",
        "role=\"dialog\"",
        "aria-modal=\"true\"",
    ] {
        assert!(
            sheet_view.contains(needle),
            "Sheet should expose semantic marker `{needle}`."
        );
    }

    for needle in [
        "pub enum OverlaysLayerKind",
        "pub fn as_attr(self) -> &'static str",
        "\"stack\"",
        "\"modal\"",
        "\"non-modal\"",
        "data_state_attr: if input.open { \"open\" } else { \"closed\" }",
        "id_source_attr: if input.has_custom_id_base {",
        "class_source_attr: if input.has_custom_class_name {",
    ] {
        assert!(
            overlays_primitive.contains(needle),
            "ui-state-primitives overlays should keep closed-set marker mapping `{needle}`."
        );
    }

    for needle in [
        "pub enum ModalOpenMode",
        "pub enum ModalOpenSource",
        "pub enum ModalOpenChangeSource",
        "pub enum ModalOpenPropSource",
        "pub fn as_attr(self) -> &'static str",
        "\"controlled\"",
        "\"uncontrolled\"",
        "\"default\"",
        "\"implicit-default\"",
        "\"custom\"",
        "\"none\"",
    ] {
        assert!(
            modal_logic.contains(needle),
            "Modal logic should keep closed-set attr mapping `{needle}`."
        );
    }

    for needle in [
        "pub fn state_attr_for_open(is_open: bool) -> &'static str",
        "\"open\"",
        "\"closed\"",
        "\"custom\"",
        "\"default\"",
    ] {
        let found = overlay_logic.contains(needle)
            || popover_logic.contains(needle)
            || sheet_logic.contains(needle);
        assert!(
            found,
            "overlay/popover/sheet logic should keep closed-set marker token `{needle}`."
        );
    }

    for needle in [
        "title=\"State + Source Markers\"",
        "Inspect data-state / data-layer / data-id-source / data-class-source in DevTools.",
    ] {
        assert!(
            overlays_docs.contains(needle),
            "docs overlays page should publish marker-based verification guidance `{needle}`."
        );
    }

    for needle in [
        "'[data-slot=\"sheet\"][data-state=\"open\"][data-placement=\"left\"]'",
        "'[data-slot=\"sheet-panel\"][role=\"dialog\"]'",
    ] {
        assert!(
            nav_sheet_e2e.contains(needle),
            "e2e sheet regression should query semantic selector `{needle}`."
        );
    }
    for forbidden in [":nth-child", ":nth-of-type"] {
        assert!(
            !nav_sheet_e2e.contains(forbidden),
            "e2e selector strategy should avoid DOM-order dependent token `{forbidden}`."
        );
    }
}

#[test]
fn overlays_type_system_and_semantic_markers_define_machine_readable_contracts() {
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let tray_logic = load_source("../tray/src/logic.rs");

    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");

    let overlay_logic_tests = load_source("../overlay/test/logic.rs");
    let popover_logic_tests = load_source("../popover/src/test/logic.rs");
    let modal_logic_tests = load_source("../modal/test/logic.rs");
    let sheet_logic_tests = load_source("../sheet/test/logic.rs");
    let tray_logic_tests = load_source("../tray/test/logic.rs");
    let overlays_module_tests = load_source("tests/overlays_module_semantics.rs");

    for needle in [
        "pub enum OverlayDismissMode",
        "pub enum OverlayKeyboardDismissMode",
        "pub dismiss_mode: OverlayDismissMode",
        "pub keyboard_dismiss_mode: OverlayKeyboardDismissMode",
    ] {
        assert!(
            overlay_logic.contains(needle),
            "Overlay logic should model discrete axes with enums: `{needle}`."
        );
    }
    for needle in [
        "pub enum PopoverModalMode",
        "pub modal_mode: PopoverModalMode",
    ] {
        assert!(
            popover_logic.contains(needle),
            "Popover logic should model modal axis with enum: `{needle}`."
        );
    }
    for needle in [
        "pub enum ModalOpenMode",
        "pub enum ModalOpenSource",
        "pub enum ModalOpenChangeSource",
        "pub enum ModalOpenPropSource",
    ] {
        assert!(
            modal_logic.contains(needle),
            "Modal logic should keep typed open contracts: `{needle}`."
        );
    }
    for needle in [
        "pub enum SheetPlacement",
        "pub enum SheetDismissMode",
        "pub enum SheetKeyboardDismissMode",
    ] {
        assert!(
            sheet_logic.contains(needle),
            "Sheet logic should keep typed placement/dismiss contracts: `{needle}`."
        );
    }
    for needle in [
        "pub enum TrayDescriptionMode",
        "pub enum TrayFooterMode",
        "pub enum TrayCloseButtonMode",
        "pub enum TraySizeMode",
        "pub enum TrayDismissMode",
        "pub enum TrayKeyboardDismissMode",
    ] {
        assert!(
            tray_logic.contains(needle),
            "Tray logic should keep typed discrete contracts: `{needle}`."
        );
    }

    for forbidden in [
        "dismiss_mode: String",
        "modal_mode: String",
        "placement: String",
        "description_mode: String",
        "footer_mode: String",
    ] {
        let leaked = overlay_logic.contains(forbidden)
            || popover_logic.contains(forbidden)
            || modal_logic.contains(forbidden)
            || sheet_logic.contains(forbidden)
            || tray_logic.contains(forbidden);
        assert!(
            !leaked,
            "Discrete state inputs should avoid string protocols: `{forbidden}`."
        );
    }

    for needle in [
        "fn resolve_states(input: OverlayStateInputs) -> OverlayResolvedStates",
        "fn resolve_states(input: PopoverStateInputs) -> PopoverResolvedStates",
        "fn resolve_states(input: SheetStateInputs) -> SheetResolvedStates",
        "fn resolve_states(input: TrayStateInputs) -> TrayResolvedStates",
        "pub fn normalize_open_state(input: ModalOpenStateInput) -> ModalOpenState",
        "pub fn resolve_open_contract(state: &ModalOpenState) -> ModalOpenContract",
    ] {
        let found = overlay_logic.contains(needle)
            || popover_logic.contains(needle)
            || modal_logic.contains(needle)
            || sheet_logic.contains(needle)
            || tray_logic.contains(needle);
        assert!(
            found,
            "Invalid combinations should be normalized via logic-layer contracts: `{needle}`."
        );
    }

    for needle in [
        "data-state",
        "data-open",
        "data-closed",
        "data-layer",
        "data-open-source",
        "data-open-mode",
        "data-open-change-source",
        "data-open-prop-source",
        "data-dismiss-source",
        "data-keyboard-dismiss-source",
        "data-placement-source",
        "data-modal-source",
        "data-id-source",
        "data-class-source",
    ] {
        let found = overlays_view.contains(needle)
            || overlay_view.contains(needle)
            || popover_view.contains(needle)
            || modal_view.contains(needle)
            || sheet_view.contains(needle)
            || tray_view.contains(needle);
        assert!(
            found,
            "Views should expose machine-readable semantic marker `{needle}`."
        );
    }

    for needle in [
        "overlay_mode_enums_map_bool_inputs_to_closed_set",
        "popover_modal_mode_enum_maps_bool_inputs_to_closed_set",
        "sheet_mode_enums_map_bool_inputs_to_closed_set",
        "tray_mode_enums_map_bool_inputs_to_closed_set",
        "normalize_open_state_supports_controlled_and_uncontrolled_modes",
        "resolve_states_centralizes_slot_state_derivation",
        "resolve_state_tracks_source_markers",
        "overlays_state_markers_are_observable_queryable_and_closed_set",
        "overlays_state_normalization_is_centralized_in_logic_layer",
    ] {
        let found = overlay_logic_tests.contains(needle)
            || popover_logic_tests.contains(needle)
            || modal_logic_tests.contains(needle)
            || sheet_logic_tests.contains(needle)
            || tray_logic_tests.contains(needle)
            || overlays_module_tests.contains(needle);
        assert!(
            found,
            "Contract breakpoints should stay test-locatable via `{needle}`."
        );
    }
}

#[test]
fn overlays_focus_stack_and_gc_use_global_focus_manager_contract() {
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let headless_focus_trap = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let headless_overlay_stack = load_source("../../crates/ui-headless/src/overlay_stack.rs");

    for (scope, source) in [
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Sheet", sheet_view.as_str()),
    ] {
        for needle in [
            "use_overlay_stack_registration()",
            "use_focus_trap(",
            "with_scope_id(",
            "with_restore_policy(RestorePolicy::FallbackTo(",
            ".with_fallback_selector(",
            "[data-slot=\"ui-root\"]",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should use global focus stack + fallback selector contract `{needle}`."
            );
        }
    }

    for (scope, source) in [
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Sheet", sheet_view.as_str()),
    ] {
        for forbidden in [
            "restore_target",
            "stored_restore",
            "StoredValue<NodeRef",
            "RwSignal<NodeRef",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not persist NodeRef as restore target `{forbidden}`."
            );
        }
    }

    assert!(
        !modal_view.contains("use_focus_trap(") && modal_view.contains("<Overlay"),
        "Modal should compose Overlay focus policy instead of owning a private restore target."
    );
    assert!(
        !tray_view.contains("use_focus_trap(") && tray_view.contains("<Sheet"),
        "Tray should compose Sheet focus policy instead of owning a private restore target."
    );

    for needle in [
        "thread_local! {",
        "FOCUS_MANAGER_STACK",
        "focus_manager_push_trap(",
        "focus_manager_pop_trap(",
        "focus_manager_peek_trap(",
        "derive_restore_policy(",
        "restore_focus_chain(",
        "RestorePolicy::Selector",
        "RestorePolicy::FallbackTo",
    ] {
        assert!(
            headless_focus_trap.contains(needle),
            "ui-headless focus trap should provide global focus manager token `{needle}`."
        );
    }

    assert!(
        !headless_focus_trap.contains("RestorePolicy::NodeRef"),
        "focus restore policy should not carry NodeRef-based restore target variants."
    );

    for needle in [
        "pub struct OverlayStack",
        "pub fn provide_overlay_stack() -> OverlayStack",
        "pub fn use_overlay_stack_registration() -> OverlayRegistration",
        "pub fn register(&self) -> OverlayRegistration",
    ] {
        assert!(
            headless_overlay_stack.contains(needle),
            "overlay stack should keep layered topmost registration contract `{needle}`."
        );
    }
}

#[test]
fn overlays_escape_hatches_foreign_zone_are_not_applicable_without_imperative_instances() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_motion = load_source("src/motion.rs");
    let overlay_mod = load_source("../overlay/src/mod.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_mod = load_source("../popover/src/mod.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_mod = load_source("../modal/src/mod.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_mod = load_source("../sheet/src/mod.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_mod = load_source("../tray/src/mod.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");

    let overlays_cargo = load_source("../Cargo.toml");
    let overlay_cargo = load_source("../overlay/Cargo.toml");
    let popover_cargo = load_source("../popover/Cargo.toml");
    let modal_cargo = load_source("../modal/Cargo.toml");
    let sheet_cargo = load_source("../sheet/Cargo.toml");
    let tray_cargo = load_source("../tray/Cargo.toml");

    for forbidden in [
        "Foreign Zone",
        "ForeignZone",
        "YieldControl",
        "CleanupForeign",
        "ECharts",
        "Mapbox",
        "Leaflet",
        "GoogleMap",
        "chart_instance",
        "map_instance",
        "imperative_handle",
        "imperative_instance",
    ] {
        let leaked = overlays_mod.contains(forbidden)
            || overlays_logic.contains(forbidden)
            || overlays_view.contains(forbidden)
            || overlays_motion.contains(forbidden)
            || overlay_mod.contains(forbidden)
            || overlay_logic.contains(forbidden)
            || overlay_view.contains(forbidden)
            || popover_mod.contains(forbidden)
            || popover_logic.contains(forbidden)
            || popover_view.contains(forbidden)
            || modal_mod.contains(forbidden)
            || modal_logic.contains(forbidden)
            || modal_view.contains(forbidden)
            || sheet_mod.contains(forbidden)
            || sheet_logic.contains(forbidden)
            || sheet_view.contains(forbidden)
            || tray_mod.contains(forbidden)
            || tray_logic.contains(forbidden)
            || tray_view.contains(forbidden);
        assert!(
            !leaked,
            "overlays family should not carry foreign-zone imperative token `{forbidden}`."
        );
    }

    for forbidden in [
        "EChart",
        "Mapbox",
        "#[prop(optional)] chart",
        "#[prop(optional)] map",
        "#[prop(optional)] foreign",
        "pub struct Foreign",
        "pub type Foreign",
    ] {
        let leaked = overlays_mod.contains(forbidden)
            || overlays_view.contains(forbidden)
            || overlay_mod.contains(forbidden)
            || overlay_view.contains(forbidden)
            || popover_mod.contains(forbidden)
            || popover_view.contains(forbidden)
            || modal_mod.contains(forbidden)
            || modal_view.contains(forbidden)
            || sheet_mod.contains(forbidden)
            || sheet_view.contains(forbidden)
            || tray_mod.contains(forbidden)
            || tray_view.contains(forbidden);
        assert!(
            !leaked,
            "overlays public/component API should not expose imperative third-party handle `{forbidden}`."
        );
    }

    for forbidden in ["echarts", "mapbox", "leaflet", "google-maps"] {
        let leaked = overlays_cargo.contains(forbidden)
            || overlay_cargo.contains(forbidden)
            || popover_cargo.contains(forbidden)
            || modal_cargo.contains(forbidden)
            || sheet_cargo.contains(forbidden)
            || tray_cargo.contains(forbidden);
        assert!(
            !leaked,
            "overlays family dependency surface should avoid foreign SDK token `{forbidden}`."
        );
    }
}

#[test]
fn overlays_hydration_discontinuity_contract_avoids_entropy_and_keeps_seeded_id_provider_path() {
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let overlays_readme = load_source("src/README.md");
    let modal_readme = load_source("../modal/src/README.md");
    let tray_readme = load_source("../tray/src/README.md");

    let overlays_primitive = load_source("../../crates/ui-state-primitives/src/overlays.rs");
    let ui_root_source = load_source("../../crates/ui/src/root.rs");
    let headless_id_provider_source = load_source("../../crates/ui-headless/src/id_provider.rs");

    let sources = [
        overlays_logic.as_str(),
        overlays_view.as_str(),
        overlay_logic.as_str(),
        overlay_view.as_str(),
        popover_logic.as_str(),
        popover_view.as_str(),
        modal_logic.as_str(),
        modal_view.as_str(),
        sheet_logic.as_str(),
        sheet_view.as_str(),
        tray_logic.as_str(),
        tray_view.as_str(),
    ];

    for forbidden in [
        "SystemTime::now",
        "UNIX_EPOCH",
        "Date::now",
        "Uuid::new_v4",
        "uuid::",
        "rand::",
        "thread_rng",
        "getrandom",
        "nanoid",
    ] {
        assert!(
            !sources.iter().any(|source| source.contains(forbidden)),
            "overlays family hydration init should avoid entropy source `{forbidden}`."
        );
    }

    for needle in [
        "pub const DEFAULT_ID_BASE: &str = \"overlays-root\";",
        "pub fn normalize_id_base(value: Option<String>) -> (String, bool) {",
    ] {
        assert!(
            overlays_primitive.contains(needle),
            "OverlaysRoot deterministic id normalization should include `{needle}`."
        );
    }

    for needle in [
        "let (id_base, has_custom_id_base) = logic::normalize_id_base(id_base);",
        "let title_id = format!(\"{id_base}-title\");",
        "let description_id = format!(\"{id_base}-description\");",
        "let id_base = logic::normalize_id_base(id_base);",
    ] {
        assert!(
            overlays_view.contains(needle)
                || modal_view.contains(needle)
                || tray_view.contains(needle),
            "overlays family should keep deterministic id derivation path via `{needle}`."
        );
    }

    for needle in [
        "#[prop(optional, default = 1)] id_seed: u64",
        "provide_ui_id_provider(id_seed);",
    ] {
        assert!(
            ui_root_source.contains(needle),
            "UiRoot should keep deterministic IdProvider seed injection `{needle}`."
        );
    }

    for needle in [
        "pub struct UiIdProvider {",
        "pub fn provide_ui_id_provider(seed: u64) -> UiIdProvider {",
        "pub fn use_ui_id_provider() -> Option<UiIdProvider> {",
        "pub fn next_prefixed_id(self, prefix: &str) -> String {",
    ] {
        assert!(
            headless_id_provider_source.contains(needle),
            "ui-headless id provider contract should include `{needle}`."
        );
    }

    assert!(
        overlays_readme.contains("`id_base`") && overlays_readme.contains("\"overlays-root\""),
        "Overlays docs should keep deterministic id-base fallback contract."
    );
    assert!(
        modal_readme.contains("`id_base`") && modal_readme.contains("`ui-modal`"),
        "Modal docs should keep deterministic id-base fallback contract."
    );
    assert!(
        tray_readme.contains("`id_base`") && tray_readme.contains("\"ui-tray\""),
        "Tray docs should keep deterministic id-base fallback contract."
    );
}

#[test]
fn overlays_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_styles = load_source("src/styles.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_motion = load_source("src/motion.rs");

    let overlay_mod = load_source("../overlay/src/mod.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let overlay_styles = load_source("../overlay/src/styles.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let overlay_motion = load_source("../overlay/src/motion.rs");

    let popover_mod = load_source("../popover/src/mod.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_styles = load_source("../popover/src/styles.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_motion = load_source("../popover/src/motion.rs");

    let sheet_mod = load_source("../sheet/src/mod.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_styles = load_source("../sheet/src/styles.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");

    let modal_mod = load_source("../modal/src/mod.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let modal_styles = load_source("../modal/src/styles.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let modal_motion = load_source("../modal/src/motion.rs");

    let tray_mod = load_source("../tray/src/mod.rs");
    let tray_logic = load_source("../tray/src/logic.rs");
    let tray_styles = load_source("../tray/src/styles.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let tray_motion = load_source("../tray/src/motion.rs");

    let ui_headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let ui_headless_cargo = load_source("../../crates/ui-headless/Cargo.toml");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");

    for (scope, source) in [
        ("Overlay view", overlay_view.as_str()),
        ("Popover view", popover_view.as_str()),
        ("Sheet view", sheet_view.as_str()),
        ("Overlay motion", overlay_motion.as_str()),
        ("Popover motion", popover_motion.as_str()),
        ("Sheet motion", sheet_motion.as_str()),
    ] {
        for needle in [
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep explicit wasm/non-wasm cfg guard `{needle}`."
            );
        }
    }

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(needle),
            "ui-headless should preserve web/ssr mutex guard `{needle}`."
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo.contains(needle),
            "ui-headless feature surface should keep explicit platform split `{needle}`."
        );
    }

    for needle in [
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion should provide deterministic non-wasm stub contract via `{needle}`."
        );
    }

    for (scope, source) in [
        ("overlays mod", overlays_mod.as_str()),
        ("overlays logic", overlays_logic.as_str()),
        ("overlays styles", overlays_styles.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlays motion", overlays_motion.as_str()),
        ("overlay mod", overlay_mod.as_str()),
        ("overlay logic", overlay_logic.as_str()),
        ("overlay styles", overlay_styles.as_str()),
        ("overlay view", overlay_view.as_str()),
        ("popover mod", popover_mod.as_str()),
        ("popover logic", popover_logic.as_str()),
        ("popover styles", popover_styles.as_str()),
        ("popover view", popover_view.as_str()),
        ("sheet mod", sheet_mod.as_str()),
        ("sheet logic", sheet_logic.as_str()),
        ("sheet styles", sheet_styles.as_str()),
        ("sheet view", sheet_view.as_str()),
        ("modal mod", modal_mod.as_str()),
        ("modal logic", modal_logic.as_str()),
        ("modal styles", modal_styles.as_str()),
        ("modal view", modal_view.as_str()),
        ("modal motion", modal_motion.as_str()),
        ("tray mod", tray_mod.as_str()),
        ("tray logic", tray_logic.as_str()),
        ("tray styles", tray_styles.as_str()),
        ("tray view", tray_view.as_str()),
        ("tray motion", tray_motion.as_str()),
    ] {
        for forbidden in ["web_sys", "wasm_bindgen", "js_sys"] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not hard-reference browser-only symbol `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_headless_web_ssr_mutex_guard_is_preserved() {
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let ui_headless_lib = load_source("../../crates/ui-headless/src/lib.rs");
    let ui_headless_cargo = load_source("../../crates/ui-headless/Cargo.toml");

    for needle in [
        "#[cfg(all(feature = \"web\", feature = \"ssr\"))]",
        "compile_error!(\"features `web` and `ssr` are mutually exclusive; enable exactly one\")",
    ] {
        assert!(
            ui_headless_lib.contains(needle),
            "ui-headless should preserve web/ssr mutex compile_error guard `{needle}`."
        );
    }

    for needle in [
        "default = [\"web\"]",
        "web = [\"leptos/csr\"]",
        "ssr = [\"leptos/ssr\"]",
    ] {
        assert!(
            ui_headless_cargo.contains(needle),
            "ui-headless feature surface should keep explicit web/ssr split `{needle}`."
        );
    }

    for (scope, source, needles) in [
        (
            "Overlay",
            overlay_view.as_str(),
            vec![
                "use ui_headless::{",
                "use_focus_trap",
                "use_modal",
                "use_overlay_stack_registration",
            ],
        ),
        (
            "Popover",
            popover_view.as_str(),
            vec![
                "use ui_headless::{",
                "use_focus_trap",
                "use_modal",
                "use_overlay_stack_registration",
                "use_popover_position",
            ],
        ),
        (
            "Sheet",
            sheet_view.as_str(),
            vec![
                "use ui_headless::{",
                "use_focus_trap",
                "use_modal",
                "use_overlay_stack_registration",
            ],
        ),
        (
            "Tray",
            tray_view.as_str(),
            vec!["use ui_headless::{", "overlay_dialog_attrs"],
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "{scope} should consume ui-headless contract `{needle}`."
            );
        }
    }
}

#[test]
fn overlays_styles_depend_on_explicit_state_markers_not_dom_shape_guessing() {
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");

    let overlay_styles = load_source("../overlay/src/styles.rs");
    let popover_styles = load_source("../popover/src/styles.rs");
    let modal_styles = load_source("../modal/src/styles.rs");
    let tray_styles = load_source("../tray/src/styles.rs");
    let sheet_styles = load_source("../sheet/src/styles.rs");
    let popover_logic = load_source("../popover/src/logic.rs");

    for (scope, source) in [
        ("Overlay", overlay_styles.as_str()),
        ("Popover", popover_styles.as_str()),
        ("Modal", modal_styles.as_str()),
        ("Tray", tray_styles.as_str()),
        ("Sheet", sheet_styles.as_str()),
    ] {
        for forbidden in [":nth-child", ":nth-of-type"] {
            assert!(
                !source.contains(forbidden),
                "{scope} styles should not guess state via fragile structural selector `{forbidden}`."
            );
        }
    }

    for (scope, source, needles) in [
        (
            "Overlay",
            overlay_styles.as_str(),
            vec![
                ".ui-overlay[data-state=\"open\"]",
                ".ui-overlay[data-state=\"closed\"]",
                ".ui-overlay[data-dismissable=\"true\"] .ui-overlay__backdrop",
            ],
        ),
        (
            "Popover",
            popover_styles.as_str(),
            vec![
                ".ui-popover[data-state=\"open\"]",
                ".ui-popover[data-state=\"closed\"]",
                ".ui-popover__panel[data-placement=\"bottom-start\"]",
            ],
        ),
        (
            "Modal",
            modal_styles.as_str(),
            vec![
                ".ui-modal[data-state=\"with-description\"]",
                ".ui-modal[data-description=\"present\"]",
                ".ui-modal[data-description=\"absent\"]",
            ],
        ),
        (
            "Tray",
            tray_styles.as_str(),
            vec![
                ".ui-tray[data-state=\"with-description\"]",
                ".ui-tray[data-close-button=\"shown\"] .ui-tray__header",
                ".ui-tray[data-footer=\"present\"] .ui-tray__footer",
            ],
        ),
        (
            "Sheet",
            sheet_styles.as_str(),
            vec![
                ".ui-sheet[data-state=\"open\"]",
                ".ui-sheet[data-state=\"closed\"]",
                ".ui-sheet[data-dismissable=\"true\"] .ui-sheet__backdrop",
            ],
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "{scope} styles should branch from explicit semantic markers via `{needle}`."
            );
        }
    }

    for (scope, source) in [
        ("OverlaysRoot", overlays_view.as_str()),
        ("Overlay", overlay_view.as_str()),
        ("Modal", modal_view.as_str()),
        ("Tray", tray_view.as_str()),
        ("Sheet", sheet_view.as_str()),
    ] {
        assert!(
            !source.contains("style="),
            "{scope} view should avoid inline style business logic."
        );
    }
    assert!(
        popover_view.contains("style=panel_vars"),
        "Popover view may only pass runtime geometry through css variables via `style=panel_vars`."
    );
    for forbidden in ["style=\"top:", "style=\"left:", "style=\"transform:"] {
        assert!(
            !popover_view.contains(forbidden),
            "Popover view should not inline concrete business style `{forbidden}`."
        );
    }

    for needle in [
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String",
        "--ui-popover-top: {top_px}px;",
        "--ui-popover-left: {left_px}px;",
        "--ui-popover-anchor-width: {anchor_width_px}px;",
    ] {
        assert!(
            popover_logic.contains(needle),
            "Popover logic should keep runtime style payload css-variable-only via `{needle}`."
        );
    }

    for needle in [
        "top: var(--ui-popover-top, var(--ui-fallback-min-inline-size-none));",
        "left: var(--ui-popover-left, var(--ui-fallback-min-inline-size-none));",
        "var(--ui-popover-anchor-width, var(--ui-fallback-min-inline-size-none))",
    ] {
        assert!(
            popover_styles.contains(needle),
            "Popover styles should consume runtime css variables through `{needle}`."
        );
    }
}

#[test]
fn overlays_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals() {
    let popover_styles = load_source("../popover/src/styles.rs");
    let sheet_styles = load_source("../sheet/src/styles.rs");
    let tray_styles = load_source("../tray/src/styles.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let contract_hygiene_script = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "--ui-fallback-min-inline-size-none",
        "--ui-fallback-border-width",
        "--ui-fallback-border",
        "--ui-fallback-space-md",
        "--ui-fallback-space-lg",
        "--ui-fallback-space-sm",
        "--ui-fallback-space-xs",
        "--ui-fallback-space-2xs",
        "--ui-fallback-space-3xs",
        "--ui-fallback-radius-lg",
        "--ui-fallback-bg",
        "--ui-fallback-fg",
        "--ui-fallback-fg-muted",
        "--ui-fallback-shadow-md",
        "--ui-fallback-shadow-sm",
        "--ui-fallback-heading-h5-font-size",
        "--ui-fallback-heading-h5-line-height",
        "--ui-fallback-font-size-150",
        "--ui-fallback-line-height-150",
        "--ui-fallback-component-height-100",
        "--ui-fallback-overlay-z-index",
        "--ui-fallback-overlay-panel-min-width",
        "--ui-fallback-overlay-viewport-inset",
        "--ui-fallback-overlay-enter-offset-y",
        "--ui-fallback-overlay-enter-scale",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme css should provide fallback SSOT token `{needle}` for overlays defensive styles."
        );
    }

    for needle in [
        "top: var(--ui-popover-top, var(--ui-fallback-min-inline-size-none));",
        "left: var(--ui-popover-left, var(--ui-fallback-min-inline-size-none));",
        "var(--ui-popover-anchor-width, var(--ui-fallback-min-inline-size-none))",
        "padding: var(--ui-space-md, var(--ui-fallback-space-md));",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "border: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "var(--ui-border, var(--ui-fallback-border));",
        "border-radius: var(--ui-radius-lg, var(--ui-fallback-radius-lg));",
        "box-shadow: var(--ui-shadow-md, var(--ui-fallback-shadow-md));",
        "--ui-popover-scale: var(",
        "--ui-fallback-overlay-enter-scale",
        "--ui-popover-y: var(",
        "--ui-fallback-overlay-enter-offset-y",
    ] {
        assert!(
            popover_styles.contains(needle),
            "Popover styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "z-index: var(--ui-overlay-z-index, var(--ui-fallback-overlay-z-index));",
        "outline: var(--ui-border-width, var(--ui-fallback-border-width)) dashed",
        "var(--ui-border, var(--ui-fallback-border)) 72%",
        "var(--ui-fg, var(--ui-fallback-fg)) 24%",
        "background: var(--ui-bg, var(--ui-fallback-bg));",
        "color: var(--ui-fg, var(--ui-fallback-fg));",
        "box-shadow: var(--ui-shadow-lg, var(--ui-fallback-shadow-sm));",
        "var(--ui-sheet-panel-x, var(--ui-fallback-min-inline-size-none))",
        "var(--ui-sheet-panel-y, var(--ui-fallback-min-inline-size-none))",
        "var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "padding: var(--ui-space-lg, var(--ui-fallback-space-lg));",
    ] {
        assert!(
            sheet_styles.contains(needle),
            "Sheet styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    for needle in [
        "gap: var(--ui-space-sm, var(--ui-fallback-space-sm));",
        "var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))",
        "var(--ui-space-lg, var(--ui-fallback-space-lg))",
        "gap: var(--ui-space-xs, var(--ui-fallback-space-xs));",
        "gap: var(--ui-space-2xs, var(--ui-fallback-space-2xs));",
        "var(--ui-component-height-100, var(--ui-fallback-component-height-100))",
        "var(--ui-space-md, var(--ui-fallback-space-md))",
        "font-size: var(",
        "--ui-fallback-heading-h5-font-size",
        "--ui-fallback-heading-h5-line-height",
        "font-size: var(--ui-font-size-150, var(--ui-fallback-font-size-150));",
        "line-height: var(--ui-line-height-150, var(--ui-fallback-line-height-150));",
        "color: var(--ui-fg-muted, var(--ui-fallback-fg-muted));",
        "border-top: var(--ui-border-width, var(--ui-fallback-border-width)) solid",
        "top: var(--ui-space-3xs, var(--ui-fallback-space-3xs));",
        "right: var(--ui-space-3xs, var(--ui-fallback-space-3xs));",
    ] {
        assert!(
            tray_styles.contains(needle),
            "Tray styles should keep defensive fallback chain marker `{needle}`."
        );
    }

    let combined = [popover_styles, sheet_styles, tray_styles].join("\n");
    for forbidden in [
        "top: var(--ui-popover-top, 0px);",
        "left: var(--ui-popover-left, 0px);",
        "var(--ui-popover-anchor-width, 0px)",
        "border: 1px solid var(--ui-border);",
        "z-index: var(--ui-overlay-z-index);",
        "outline: 1px dashed color-mix(in oklab, var(--ui-border) 72%, transparent);",
        "transform: translate3d(var(--ui-sheet-panel-x, 0px), var(--ui-sheet-panel-y, 0px), 0);",
        "font-size: var(--ui-heading-h5-font-size, 16px);",
        "line-height: var(--ui-heading-h5-line-height, 24px);",
        "font-size: var(--ui-font-size-150, 14px);",
        "line-height: var(--ui-line-height-150, 20px);",
        "color: var(--ui-fg-muted);",
        "border-top: 1px solid var(--ui-border);",
        "gap: var(--ui-space-sm);",
    ] {
        assert!(
            !combined.contains(forbidden),
            "overlays defensive variables contract should forbid `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract hygiene script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_cascade_layer_and_runtime_style_contract_is_enforced() {
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let ui_root = load_source("../../crates/ui/src/root.rs");
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let contract_hygiene_script = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for needle in [
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-overlay\")]",
        "out.push_str(crate::overlay::styles::CSS);",
        "#[cfg(feature = \"component-overlays\")]",
        "out.push_str(crate::overlays::styles::CSS);",
        "#[cfg(feature = \"component-popover\")]",
        "out.push_str(crate::popover::styles::CSS);",
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "#[cfg(feature = \"component-sheet\")]",
        "out.push_str(crate::sheet::styles::CSS);",
        "#[cfg(feature = \"component-tray\")]",
        "out.push_str(crate::tray::styles::CSS);",
        "out.push_str(\"}\\n\");",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css entry should keep overlays aggregation inside `@layer ui` via `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(needle),
            "UiRoot should inject aggregated css through `{needle}`."
        );
    }

    for (scope, source) in [
        ("OverlaysRoot", overlays_view.as_str()),
        ("Overlay", overlay_view.as_str()),
        ("Modal", modal_view.as_str()),
        ("Sheet", sheet_view.as_str()),
        ("Tray", tray_view.as_str()),
    ] {
        for forbidden in ["style=", "style:"] {
            assert!(
                !source.contains(forbidden),
                "{scope} should forbid inline business style marker `{forbidden}`."
            );
        }
    }

    assert!(
        popover_view.contains("style=panel_vars"),
        "Popover may only use css-variable payload bridge via `style=panel_vars`."
    );
    for forbidden in [
        "style=\"top:",
        "style=\"left:",
        "style=\"transform:",
        "style=\"width:",
        "style=\"height:",
    ] {
        assert!(
            !popover_view.contains(forbidden),
            "Popover should not inline plain style declaration `{forbidden}`."
        );
    }

    for needle in [
        "pub fn compose_panel_vars(top_px: f64, left_px: f64, anchor_width_px: f64) -> String",
        "--ui-popover-top: {top_px}px;",
        "--ui-popover-left: {left_px}px;",
        "--ui-popover-anchor-width: {anchor_width_px}px;",
    ] {
        assert!(
            popover_logic.contains(needle),
            "Popover runtime style path should stay css-variable-only via `{needle}`."
        );
    }

    let script_needle =
        "cargo test -p ui-overlays overlays_cascade_layer_and_runtime_style_contract_is_enforced";
    assert!(
        contract_hygiene_script.contains(script_needle),
        "contract hygiene script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only() {
    let overlays_tests = load_source("tests/overlays_module_semantics.rs");
    let overlay_semantics = load_source("../../components/overlay/test/overlay_semantics.rs");
    let popover_semantics = load_source("../../components/popover/src/test/popover_semantics.rs");
    let modal_semantics = load_source("../../components/modal/test/modal_semantics.rs");
    let modal_logic_tests = load_source("../modal/test/logic.rs");
    let nav_sheet_e2e = load_source("../../e2e/tests/docs_app_nav_sheet.spec.mjs");

    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");

    let ui_components_cargo = load_source("../../crates/ui/Cargo.toml");
    let overlays_cargo = load_source("Cargo.toml");

    for needle in [
        "data-role-source=root_state.role_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        ".ui-overlay[data-dismissable=\"true\"] .ui-overlay__backdrop",
        "aria-modal=\"true\"",
    ] {
        assert!(
            overlay_semantics.contains(needle),
            "Overlay semantic tests should assert semantic-contract token `{needle}`."
        );
    }

    for needle in [
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        ".ui-popover__panel[data-placement=\"bottom-start\"]",
    ] {
        assert!(
            popover_semantics.contains(needle),
            "Popover semantic tests should assert semantic-contract token `{needle}`."
        );
    }

    for needle in [
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
    ] {
        assert!(
            modal_semantics.contains(needle),
            "Modal semantic tests should assert semantic-contract token `{needle}`."
        );
    }

    for needle in [
        "normalize_open_state_supports_controlled_and_uncontrolled_modes",
        "resolve_open_contract_derives_mode_and_source_markers",
        "ModalOpenMode::Controlled",
        "ModalOpenMode::Uncontrolled",
    ] {
        assert!(
            modal_logic_tests.contains(needle),
            "Controlled/uncontrolled branch should be covered via `{needle}`."
        );
    }

    for needle in [
        "is_keyboard_dismiss_disabled=true",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        ".ui-overlay[data-keyboard-dismiss-disabled=\"true\"] .ui-overlay__panel",
    ] {
        let covered = overlay_semantics.contains(needle) || overlays_tests.contains(needle);
        assert!(
            covered,
            "Disabled-like overlay semantic axis should be covered via `{needle}`."
        );
    }

    for (scope, source) in [
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Sheet", sheet_view.as_str()),
    ] {
        for needle in ["on:keydown", "on:pointerdown"] {
            assert!(
                source.contains(needle),
                "{scope} view should keep interaction semantic path `{needle}`."
            );
        }
    }
    for needle in ["openNav.click()", "keyboard.press(\"Escape\")"] {
        assert!(
            nav_sheet_e2e.contains(needle),
            "e2e should include pointer+keyboard interaction evidence `{needle}`."
        );
    }

    for (scope, source) in [
        ("Overlay view", overlay_view.as_str()),
        ("Popover view", popover_view.as_str()),
        ("Sheet view", sheet_view.as_str()),
        ("Overlay motion", overlay_motion.as_str()),
        ("Popover motion", popover_motion.as_str()),
        ("Sheet motion", sheet_motion.as_str()),
    ] {
        for needle in [
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep wasm/non-wasm branch evidence `{needle}`."
            );
        }
    }

    for source in [
        overlay_semantics.as_str(),
        popover_semantics.as_str(),
        modal_semantics.as_str(),
        nav_sheet_e2e.as_str(),
    ] {
        for forbidden in [
            "assert_snapshot",
            "insta::",
            "to_match_snapshot",
            "snapshot!",
        ] {
            assert!(
                !source.contains(forbidden),
                "Semantic contract tests should not rely on visual snapshot token `{forbidden}`."
            );
        }
    }

    for source in [ui_components_cargo.as_str(), overlays_cargo.as_str()] {
        for forbidden in ["insta", "snapshot"] {
            assert!(
                !source.contains(forbidden),
                "test dependencies should avoid snapshot-only tooling token `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks()
 {
    let overlays_tests = load_source("tests/overlays_module_semantics.rs");
    let overlay_semantics = load_source("../../components/overlay/test/overlay_semantics.rs");
    let popover_semantics = load_source("../../components/popover/src/test/popover_semantics.rs");
    let modal_semantics = load_source("../../components/modal/test/modal_semantics.rs");
    let drawer_semantics = load_source("../../components/drawer/test/drawer_semantics.rs");
    let bottom_sheet_semantics =
        load_source("../../components/bottom-sheet/test/bottom_sheet_semantics.rs");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "fn overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only()",
        "data-role-source=root_state.role_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "\"assert_snapshot\"",
        "\"to_match_snapshot\"",
        "\"snapshot!\"",
    ] {
        assert!(
            overlays_tests.contains(marker),
            "overlays semantic-priority suite should keep marker `{marker}`."
        );
    }

    for marker in [
        "data-role-source=root_state.role_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "aria-modal=\"true\"",
    ] {
        assert!(
            overlay_semantics.contains(marker),
            "overlay semantic coverage should include `{marker}`."
        );
    }

    for marker in [
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            popover_semantics.contains(marker),
            "popover semantic coverage should include `{marker}`."
        );
    }

    for marker in [
        "modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
    ] {
        assert!(
            modal_semantics.contains(marker),
            "modal semantic-priority coverage should include `{marker}`."
        );
    }

    for marker in [
        "drawer_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
    ] {
        assert!(
            drawer_semantics.contains(marker),
            "drawer semantic-priority coverage should include `{marker}`."
        );
    }

    for marker in [
        "fn bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions()",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
    ] {
        assert!(
            bottom_sheet_semantics.contains(marker),
            "bottom-sheet semantic coverage should include `{marker}`."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks";
    assert!(
        perf_script_source.contains(script_needle),
        "performance script should include overlays semantic-priority gate `{script_needle}`."
    );
}

#[test]
fn overlays_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks() {
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let drawer_view = load_source("../drawer/src/view.rs");

    let overlay_semantics = load_source("../../components/overlay/test/overlay_semantics.rs");
    let popover_semantics = load_source("../../components/popover/src/test/popover_semantics.rs");
    let modal_semantics = load_source("../../components/modal/test/modal_semantics.rs");
    let drawer_semantics = load_source("../../components/drawer/test/drawer_semantics.rs");

    for marker in [
        "data-role-source=root_state.role_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
    ] {
        assert!(
            overlay_view.contains(marker) && overlay_semantics.contains(marker),
            "overlay semantic marker `{marker}` must be synchronized with semantic tests."
        );
    }

    for marker in [
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            popover_view.contains(marker) && popover_semantics.contains(marker),
            "popover semantic marker `{marker}` must be synchronized with semantic tests."
        );
    }

    for marker in [
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
    ] {
        assert!(
            modal_view.contains(marker) && modal_semantics.contains(marker),
            "modal semantic marker `{marker}` must be synchronized with semantic tests."
        );
    }

    for marker in [
        "data-open-source=open_value_source.as_attr()",
        "data-open-action-source=move || open_action_source.get().as_attr()",
        "data-placement-source=root_state.placement_source_attr",
    ] {
        assert!(
            drawer_view.contains(marker) && drawer_semantics.contains(marker),
            "drawer semantic marker `{marker}` must be synchronized with semantic tests."
        );
    }
}

#[test]
fn overlays_performance_script_covers_semantic_test_priority_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "echo \"[perf] contract: overlays semantic test priority\"",
        "cargo test -p ui-overlays overlays_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks",
    ] {
        assert!(
            script_source.contains(marker),
            "performance script should include overlays semantic-priority marker `{marker}`."
        );
    }
}

#[test]
fn overlays_component_files_follow_layered_responsibilities() {
    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_styles = load_source("src/styles.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_motion = load_source("src/motion.rs");

    for needle in [
        "mod logic;",
        "pub mod motion;",
        "pub mod styles;",
        "mod view;",
        "pub use motion::OverlaysMotion;",
        "pub use view::OverlaysRoot;",
    ] {
        assert!(
            overlays_mod.contains(needle),
            "mod.rs should keep minimal export boundary token `{needle}`."
        );
    }
    for forbidden in ["view! {", "on:keydown", "on:pointerdown", "node_ref="] {
        assert!(
            !overlays_mod.contains(forbidden),
            "mod.rs should not carry implementation detail `{forbidden}`."
        );
    }

    for needle in [
        "pub use ui_state_primitives::overlays::{",
        "pub fn normalize_aria_label(",
        "pub fn compose_root_class_name(",
    ] {
        assert!(
            overlays_logic.contains(needle),
            "logic.rs should keep normalization/derivation token `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "on:click=",
        "on:keydown=",
        "node_ref=",
        "web_sys",
        "position: fixed",
    ] {
        assert!(
            !overlays_logic.contains(forbidden),
            "logic.rs should not carry DOM/render/style detail `{forbidden}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        ".ui-overlays",
        "var(--ui-overlays-",
        "data-state=\"open\"",
        "data-layer=\"modal\"",
    ] {
        assert!(
            overlays_styles.contains(needle),
            "styles.rs should keep token-first static CSS token `{needle}`."
        );
    }
    for forbidden in [
        "view! {",
        "use ui_headless::",
        "on:click=",
        "request_animation_frame",
    ] {
        assert!(
            !overlays_styles.contains(forbidden),
            "styles.rs should not carry headless/render/runtime detail `{forbidden}`."
        );
    }

    for needle in [
        "view! {",
        "labeled_group_attrs(",
        "role=group_a11y.role",
        "aria-label=group_a11y.aria_label.clone()",
        "data-slot=\"overlays\"",
    ] {
        assert!(
            overlays_view.contains(needle),
            "view.rs should keep render + headless挂载 token `{needle}`."
        );
    }
    for forbidden in [
        "SpringAnimator::new",
        "request_animation_frame",
        "pub struct OverlaysMotion",
    ] {
        assert!(
            !overlays_view.contains(forbidden),
            "view.rs should not carry motion engine/mapping detail `{forbidden}`."
        );
    }

    for needle in [
        "pub struct OverlaysMotion",
        "pub fn sanitize_motion(motion: OverlaysMotion) -> OverlaysMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "tray: crate::tray::motion::sanitize_motion(motion.tray)",
    ] {
        assert!(
            overlays_motion.contains(needle),
            "motion.rs should keep semantic-motion mapping token `{needle}`."
        );
    }
    for forbidden in [
        "SpringAnimator::new",
        "request_animation_frame",
        "view! {",
        "data-slot",
        "aria-",
    ] {
        assert!(
            !overlays_motion.contains(forbidden),
            "motion.rs should not carry engine/render/aria detail `{forbidden}`."
        );
    }
}

#[test]
fn overlays_component_directory_standard_files_follow_contract_and_na_spec() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for rel_dir in [
        "src",
        "../overlay/src",
        "../popover/src",
        "../modal/src",
        "../sheet/src",
        "../tray/src",
    ] {
        let dir = manifest_dir.join(rel_dir);
        for required in ["mod.rs", "logic.rs", "styles.rs", "view.rs", "motion.rs"] {
            let path = dir.join(required);
            assert!(
                path.exists(),
                "component directory should include required standard file `{}`.",
                path.display()
            );
        }
        for forbidden in ["render.rs", "spec.rs"] {
            let path = dir.join(forbidden);
            assert!(
                !path.exists(),
                "component directory should not include forbidden file `{}`.",
                path.display()
            );
        }
    }

    for (scope, source) in [
        ("overlays mod", load_source("src/overlays/mod.rs")),
        ("overlay mod", load_source("../overlay/src/mod.rs")),
        ("popover mod", load_source("../popover/src/mod.rs")),
        ("modal mod", load_source("../modal/src/mod.rs")),
        ("sheet mod", load_source("../sheet/src/mod.rs")),
        ("tray mod", load_source("../tray/src/mod.rs")),
    ] {
        for needle in [
            "mod logic;",
            "pub mod motion;",
            "pub mod styles;",
            "mod view;",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep standard module boundary marker `{needle}`."
            );
        }
        for forbidden in ["pub mod logic;", "pub mod view;"] {
            assert!(
                !source.contains(forbidden),
                "{scope} should keep logic/view internals private `{forbidden}`."
            );
        }
    }

    for (scope, source) in [
        ("overlays logic", load_source("src/logic.rs")),
        ("overlay logic", load_source("../overlay/src/logic.rs")),
        ("popover logic", load_source("../popover/src/logic.rs")),
        ("modal logic", load_source("../modal/src/logic.rs")),
        ("sheet logic", load_source("../sheet/src/logic.rs")),
        ("tray logic", load_source("../tray/src/logic.rs")),
    ] {
        for forbidden in [
            "view! {",
            "on:click",
            "on:keydown",
            "on:pointerdown",
            "node_ref=",
            "web_sys::",
            "request_animation_frame",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not include render/runtime detail `{forbidden}`."
            );
        }
    }

    for (scope, source) in [
        ("overlays styles", load_source("src/styles.rs")),
        ("overlay styles", load_source("../overlay/src/styles.rs")),
        ("popover styles", load_source("../popover/src/styles.rs")),
        ("modal styles", load_source("../modal/src/styles.rs")),
        ("sheet styles", load_source("../sheet/src/styles.rs")),
        ("tray styles", load_source("../tray/src/styles.rs")),
    ] {
        for needle in ["pub const CSS: &str = r#\"", "var(--ui-"] {
            assert!(
                source.contains(needle),
                "{scope} should keep static token-based CSS marker `{needle}`."
            );
        }
        for forbidden in [
            "view! {",
            "on:click",
            "on:keydown",
            "on:pointerdown",
            "use ui_headless::",
            "request_animation_frame",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not include view/headless/runtime detail `{forbidden}`."
            );
        }
    }

    for (scope, source) in [
        ("overlays view", load_source("src/view.rs")),
        ("overlay view", load_source("../overlay/src/view.rs")),
        ("popover view", load_source("../popover/src/view.rs")),
        ("modal view", load_source("../modal/src/view.rs")),
        ("sheet view", load_source("../sheet/src/view.rs")),
        ("tray view", load_source("../tray/src/view.rs")),
    ] {
        for needle in ["view! {", "data-state="] {
            assert!(
                source.contains(needle),
                "{scope} should keep render + state-marker token `{needle}`."
            );
        }
        for forbidden in ["mod render;", "include!(\"render.rs\")"] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not drift to render.rs pattern `{forbidden}`."
            );
        }
    }

    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");
    let tray_motion = load_source("../tray/src/motion.rs");
    let overlays_motion = load_source("src/motion.rs");
    let modal_motion = load_source("../modal/src/motion.rs");

    for (scope, source, motion_name) in [
        ("overlay motion", overlay_motion.as_str(), "OverlayMotion"),
        ("popover motion", popover_motion.as_str(), "PopoverMotion"),
        ("sheet motion", sheet_motion.as_str(), "SheetMotion"),
    ] {
        assert!(
            source.contains(&format!("pub struct {motion_name}")),
            "{scope} should expose `{motion_name}` contract."
        );
        assert!(
            source.contains("pub fn sanitize_motion(") && source.contains("pub fn attach_motion("),
            "{scope} should keep sanitize + attach mapping contract."
        );
    }

    for (scope, source, motion_name) in [
        ("tray motion", tray_motion.as_str(), "TrayMotion"),
        (
            "overlays motion",
            overlays_motion.as_str(),
            "OverlaysMotion",
        ),
    ] {
        assert!(
            source.contains(&format!("pub struct {motion_name}")),
            "{scope} should expose `{motion_name}` contract."
        );
        assert!(
            source.contains("pub fn sanitize_motion("),
            "{scope} should keep sanitize mapping contract."
        );
    }

    for needle in [
        "pub fn default_motion_contract() -> OverlayMotion",
        "pub fn normalize_motion(motion: OverlayMotion) -> OverlayMotion",
        "pub fn is_custom_motion(motion: OverlayMotion) -> bool",
    ] {
        assert!(
            modal_motion.contains(needle),
            "modal motion should keep wrapper motion contract marker `{needle}`."
        );
    }

    for (scope, source) in [
        ("overlay motion", overlay_motion.as_str()),
        ("popover motion", popover_motion.as_str()),
        ("sheet motion", sheet_motion.as_str()),
        ("tray motion", tray_motion.as_str()),
        ("overlays motion", overlays_motion.as_str()),
        ("modal motion", modal_motion.as_str()),
    ] {
        for forbidden in ["view! {", "data-state=", "role=", "aria-"] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not include view/a11y rendering concern `{forbidden}`."
            );
        }
    }

    let component_files_script = load_source("../../scripts/check-ui-component-files.sh");
    let script_needle = "cargo test -p ui-overlays overlays_component_directory_standard_files_follow_contract_and_na_spec";
    assert!(
        component_files_script.contains(script_needle),
        "component-files script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_file_placement_discipline_is_strict_for_component_scope() {
    overlays_component_directory_standard_files_follow_contract_and_na_spec();

    let component_files_script = load_source("../../scripts/check-ui-component-files.sh");
    let script_needle = "cargo test -p ui-overlays overlays_file_placement_discipline_is_strict_for_component_scope";
    assert!(
        component_files_script.contains(script_needle),
        "component-files script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_hyper_structure_builder_spec_is_not_applicable_for_simple_component() {
    overlays_spec_rs_stays_absent_without_complex_schema_requirement();

    let button_spec = load_source("../button/src/spec.rs");
    for needle in [
        "pub struct ButtonSpec",
        "impl ButtonSpec {",
        "pub fn new() -> Self {",
        "pub fn render(self) -> impl IntoView {",
    ] {
        assert!(
            button_spec.contains(needle),
            "complex-component baseline should keep hyper-structure builder marker `{needle}`."
        );
    }

    let component_files_script = load_source("../../scripts/check-ui-component-files.sh");
    let script_needle = "cargo test -p ui-overlays overlays_hyper_structure_builder_spec_is_not_applicable_for_simple_component";
    assert!(
        component_files_script.contains(script_needle),
        "component-files script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_context_compression_manifest_and_rbi_projection_are_present_and_current() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    for required_file in [
        "src/Component.toml",
        "src/overlays.rbi",
        "../overlay/src/Component.toml",
        "../overlay/src/overlay.rbi",
        "../popover/src/Component.toml",
        "../popover/src/popover.rbi",
        "../modal/src/Component.toml",
        "../modal/src/modal.rbi",
        "../sheet/src/Component.toml",
        "../sheet/src/sheet.rbi",
        "../tray/src/Component.toml",
        "../tray/src/tray.rbi",
    ] {
        let path = manifest_dir.join(required_file);
        assert!(
            path.exists(),
            "overlays context-compression artifact should exist: `{required_file}`."
        );
    }

    for (source, needles) in [
        (
            load_source("src/Component.toml"),
            vec![
                "schema_version = \"1\"",
                "name = \"OverlaysRoot\"",
                "crate = \"ui-overlays\"",
                "name = \"context_compression_manifest\"",
                "name = \"rbi_signature_projection\"",
            ],
        ),
        (
            load_source("../overlay/src/Component.toml"),
            vec![
                "schema_version = \"1\"",
                "name = \"Overlay\"",
                "crate = \"ui-overlay\"",
                "name = \"context_compression_manifest\"",
                "name = \"rbi_signature_projection\"",
            ],
        ),
        (
            load_source("../popover/src/Component.toml"),
            vec![
                "schema_version = \"1\"",
                "name = \"Popover\"",
                "crate = \"ui-popover\"",
                "name = \"context_compression_manifest\"",
                "name = \"rbi_signature_projection\"",
            ],
        ),
        (
            load_source("../modal/src/Component.toml"),
            vec![
                "schema_version = \"1\"",
                "name = \"Modal\"",
                "crate = \"ui-modal\"",
                "name = \"context_compression_manifest\"",
                "name = \"rbi_signature_projection\"",
            ],
        ),
        (
            load_source("../sheet/src/Component.toml"),
            vec![
                "schema_version = \"1\"",
                "name = \"Sheet\"",
                "crate = \"ui-sheet\"",
                "name = \"context_compression_manifest\"",
                "name = \"rbi_signature_projection\"",
            ],
        ),
        (
            load_source("../tray/src/Component.toml"),
            vec![
                "schema_version = \"1\"",
                "name = \"Tray\"",
                "crate = \"ui-tray\"",
                "name = \"context_compression_manifest\"",
                "name = \"rbi_signature_projection\"",
            ],
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "Component.toml context-compression marker should include `{needle}`."
            );
        }
    }

    for (source, needles) in [
        (
            load_source("src/overlays.rbi"),
            vec![
                "pub struct OverlaysMotion {",
                "pub fn resolve_root_state(input: OverlaysRootStateInput) -> OverlaysRootState;",
                "pub fn OverlaysRoot(",
            ],
        ),
        (
            load_source("../overlay/src/overlay.rbi"),
            vec!["pub struct OverlayMotion {", "pub fn Overlay("],
        ),
        (
            load_source("../popover/src/popover.rbi"),
            vec!["pub struct PopoverMotion {", "pub fn Popover("],
        ),
        (
            load_source("../modal/src/modal.rbi"),
            vec!["pub struct ModalOpenContract {", "pub fn Modal("],
        ),
        (
            load_source("../sheet/src/sheet.rbi"),
            vec!["pub enum SheetPlacement {", "pub fn Sheet("],
        ),
        (
            load_source("../tray/src/tray.rbi"),
            vec!["pub struct TrayMotion {", "pub fn Tray("],
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "RBI projection should include signature marker `{needle}`."
            );
        }
    }

    for (source, needles) in [
        (
            load_source("src/view.rs"),
            vec![
                "pub fn OverlaysRoot(",
                "#[prop(optional, into)] id_base: Option<String>",
                "#[prop(optional)] is_open: bool",
                "#[prop(optional)] is_modal: bool",
                "children: Children,",
            ],
        ),
        (
            load_source("../popover/src/view.rs"),
            vec![
                "pub fn Popover(",
                "anchor_ref: NodeRef<html::Button>",
                "#[prop(optional)] placement: PopoverPlacement",
                "#[prop(optional)] motion: PopoverMotion",
                "children: ChildrenFn,",
            ],
        ),
        (
            load_source("../sheet/src/view.rs"),
            vec![
                "pub fn Sheet(",
                "#[prop(optional)] placement: SheetPlacement",
                "#[prop(optional)] motion: SheetMotion",
                "#[prop(optional, default = logic::DEFAULT_DISMISSABLE)] is_dismissable: bool",
            ],
        ),
        (
            load_source("../tray/src/view.rs"),
            vec![
                "pub fn Tray(",
                "#[prop(optional, into)] footer: Option<ViewFn>",
                "#[prop(optional)] motion: TrayMotion",
                "#[prop(optional, into)] class_name: Option<String>",
            ],
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "view signature should include `{needle}` for manifest/rbi drift detection."
            );
        }
    }

    let component_files_script = load_source("../../scripts/check-ui-component-files.sh");
    let script_needle = "cargo test -p ui-overlays overlays_context_compression_manifest_and_rbi_projection_are_present_and_current";
    assert!(
        component_files_script.contains(script_needle),
        "component-files script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_agent_contract_is_schema_typed_and_machine_readable() {
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let docs_overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let docs_overlays_extra =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let component_files_script = load_source("../../scripts/check-ui-contract-hygiene.sh");

    for (scope, source) in [
        ("OverlaysRoot", overlays_view.as_str()),
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Modal", modal_view.as_str()),
        ("Sheet", sheet_view.as_str()),
        ("Tray", tray_view.as_str()),
    ] {
        assert!(
            source.contains("data-state="),
            "{scope} should expose machine-readable state marker `data-state`."
        );
    }

    for (scope, source) in [
        ("OverlaysRoot", overlays_view.as_str()),
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Modal", modal_view.as_str()),
        ("Sheet", sheet_view.as_str()),
        ("Tray", tray_view.as_str()),
    ] {
        assert!(
            source.contains("-source"),
            "{scope} should expose source markers (`data-*-source`) for stable agent parsing."
        );
    }

    for needle in [
        "data-ui-schema=",
        "data-ui-schema-version=",
        "data-ui-intent=",
        "data-ui-action=",
        "data-ui-state=",
        "data-ui-source=",
        "data-ui-config-policy=",
        "data-ui-output-status=",
    ] {
        assert!(
            modal_view.contains(needle),
            "Modal should keep schema-like agent marker `{needle}`."
        );
    }

    for needle in [
        "data-ui-schema=agent_contract.schema_attr",
        "data-ui-intent=agent_contract.intent_attr",
        "data-ui-action=agent_contract.action_attr",
        "data-ui-state-axis=agent_contract.state_axis_attr",
        "data-ui-source-axis=agent_contract.source_axis_attr",
        "data-ui-render-mode=agent_contract.render_mode_attr",
        "data-ui-streaming=agent_contract.streaming_attr",
        "data-ui-fallback=agent_contract.fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            sheet_view.contains(needle),
            "Sheet should keep schema-like agent marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ModalAgentSchemaVersion",
        "pub enum ModalAgentIntent",
        "pub enum ModalAgentAction",
        "pub enum ModalAgentState",
        "pub enum ModalAgentSource",
        "pub enum ModalAgentConfigPolicy",
        "pub enum ModalAgentOutputStatus",
        "pub struct ModalAgentContract",
        "pub fn resolve_agent_contract(input: ModalAgentContractInput) -> ModalAgentContract",
        "ModalAgentConfigPolicy::Whitelist",
    ] {
        assert!(
            modal_logic.contains(needle),
            "Modal logic should type-drive agent contract field `{needle}`."
        );
    }

    for needle in [
        "pub struct SheetAgentContract",
        "pub fn agent_contract() -> SheetAgentContract",
        "schema_attr: \"sheet.v1\"",
        "intent_attr: \"overlay\"",
        "action_attr: \"dismiss\"",
        "state_axis_attr: \"open\"",
        "source_axis_attr: \"default|custom\"",
    ] {
        assert!(
            sheet_logic.contains(needle),
            "Sheet logic should keep explicit typed agent-contract field `{needle}`."
        );
    }

    for forbidden in [
        "format!(\"data-ui-",
        "push_str(\"data-ui-",
        "inner_html",
        "set_inner_html",
        "<script",
        "javascript:",
    ] {
        assert!(
            !modal_view.contains(forbidden)
                && !sheet_view.contains(forbidden)
                && !overlay_view.contains(forbidden)
                && !popover_view.contains(forbidden)
                && !tray_view.contains(forbidden)
                && !docs_overlays.contains(forbidden)
                && !docs_overlays_extra.contains(forbidden),
            "overlays agent contract / render chain should avoid free-form unsafe token `{forbidden}`."
        );
    }

    let script_needle =
        "cargo test -p ui-overlays overlays_agent_contract_is_schema_typed_and_machine_readable";
    assert!(
        component_files_script.contains(script_needle),
        "contract-hygiene script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_streaming_definition_is_llm_output_only_with_two_modes() {
    let docs_overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "// Modal is not an LLM body reader surface.",
        "// Drawer is not an LLM body reader surface.",
        "heading=\"LLM output contract\".to_string()",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"modal-streaming-contract\"",
        "data-slot=\"drawer-streaming-contract\"",
        "data-ui-output-mode=snapshot|streaming",
    ] {
        assert!(
            docs_overlays.contains(needle),
            "overlays docs should keep streaming scope/mode marker `{needle}`."
        );
    }

    let option_blocks: Vec<&str> = docs_overlays
        .split("let stream_mode_options = vec![")
        .skip(1)
        .collect();
    assert!(
        !option_blocks.is_empty(),
        "overlays docs should define stream mode options for streaming/snapshot examples."
    );

    for (idx, block) in option_blocks.iter().enumerate() {
        let Some((options_block, _)) = block.split_once("];") else {
            panic!("stream mode options block #{idx} should terminate with `];`");
        };
        let option_lines: Vec<&str> = options_block
            .lines()
            .map(str::trim)
            .filter(|line| line.ends_with(".to_string(),"))
            .collect();
        assert_eq!(
            option_lines,
            vec![
                "\"Snapshot\".to_string(),",
                "\"Streaming (fallback=snapshot)\".to_string(),"
            ],
            "stream mode options block #{idx} should expose exactly snapshot + streaming fallback modes."
        );
    }

    for needle in [
        "render_mode_attr: \"snapshot\"",
        "streaming_attr: \"optional\"",
        "fallback_attr: \"snapshot\"",
    ] {
        assert!(
            sheet_logic.contains(needle),
            "Sheet agent contract should keep `{needle}` for two-mode rendering semantics."
        );
    }

    let script_needle =
        "cargo test -p ui-overlays overlays_streaming_definition_is_llm_output_only_with_two_modes";
    assert!(
        streaming_script.contains(script_needle),
        "streaming checklist gate should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_snapshot_baseline_consumes_complete_result_and_renders_stably() {
    let docs_overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let modal_logic = load_source("../modal/src/logic.rs");
    let drawer_logic = load_source("../drawer/src/logic.rs");
    let contextual_help_logic = load_source("../contextual-help/src/logic.rs");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "description=\"Modal is streaming-optional and snapshot-first (`fallback=snapshot`).\"",
        "description=\"Drawer is streaming-optional and snapshot-first (`fallback=snapshot`).\"",
        "\"effective component status: data-ui-output-status=verified\"",
        "\"Snapshot is the baseline rendering mode for ContextualHelp.\"",
        "\"This component defaults to snapshot rendering while exposing streaming/snapshot markers.\"",
    ] {
        assert!(
            docs_overlays.contains(needle),
            "overlays docs should keep snapshot-baseline evidence `{needle}`."
        );
    }

    assert!(
        docs_overlays
            .matches("\"This component defaults to snapshot rendering.\"")
            .count()
            >= 2,
        "overlays docs should show snapshot-default rendering for both modal and drawer."
    );

    for needle in [
        "render_mode_attr: \"snapshot\"",
        "fallback_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
    ] {
        assert!(
            sheet_logic.contains(needle),
            "Sheet contract should keep snapshot baseline field `{needle}`."
        );
    }

    for needle in [
        "output_status: ModalAgentOutputStatus::Verified",
        "output_status: DrawerAgentOutputStatus::Verified",
    ] {
        assert!(
            modal_logic.contains(needle) || drawer_logic.contains(needle),
            "overlay docs baseline should stay aligned with verified snapshot status `{needle}`."
        );
    }

    for needle in [
        "CONTEXTUAL_HELP_LLM_OUTPUT_FALLBACK_MODE",
        "ContextualHelpLlmOutputMode::Snapshot => ContextualHelpLlmOutputStatus::Verified",
    ] {
        assert!(
            contextual_help_logic.contains(needle),
            "ContextualHelp logic should keep snapshot baseline mapping `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_snapshot_baseline_consumes_complete_result_and_renders_stably";
    assert!(
        streaming_script.contains(script_needle),
        "streaming checklist gate should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_streaming_required_optional_classification_rules_are_scope_driven_and_boundary_safe() {
    let docs_overlays = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let drawer_view = load_source("../drawer/src/view.rs");
    let contextual_help_logic = load_source("../contextual-help/src/logic.rs");
    let contextual_help_view = load_source("../contextual-help/src/view.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let streaming_script = load_source("../../scripts/check-ui-streaming.sh");

    for needle in [
        "// Modal is not an LLM body reader surface.",
        "// Drawer is not an LLM body reader surface.",
        "description=\"Modal is streaming-optional and snapshot-first (`fallback=snapshot`).\"",
        "description=\"Drawer is streaming-optional and snapshot-first (`fallback=snapshot`).\"",
        "\"effective component status: data-ui-output-status=verified\"",
        "footer=move || view! { \"Streaming Optional; fallback=snapshot.\" }",
    ] {
        assert!(
            docs_overlays.contains(needle),
            "overlays docs should keep streaming optional scope marker `{needle}`."
        );
    }

    for needle in [
        "pub enum ContextualHelpStreamingRequirement",
        "Required,",
        "Optional,",
        "pub fn resolve_streaming_policy(is_reader_surface: bool) -> ContextualHelpStreamingPolicy",
        "ContextualHelpStreamingRequirement::Required",
        "ContextualHelpStreamingRequirement::Optional",
    ] {
        assert!(
            contextual_help_logic.contains(needle),
            "ContextualHelp logic should keep scope-driven required/optional classifier `{needle}`."
        );
    }

    for needle in [
        "let streaming_policy = logic::resolve_streaming_policy(false);",
        "data-ui-streaming-requirement=streaming_policy.requirement.as_attr()",
        "data-ui-streaming-fallback=streaming_policy.fallback_mode.as_attr()",
        "data-ui-output-status=llm_output_status.as_attr()",
        "role=\"dialog\"",
    ] {
        assert!(
            contextual_help_view.contains(needle),
            "ContextualHelp view should expose machine-readable streaming/output semantics via `{needle}`."
        );
    }

    for needle in [
        "streaming_attr: \"optional\"",
        "fallback_attr: \"snapshot\"",
        "output_status_attr: \"verified\"",
        "data-ui-streaming=agent_contract.streaming_attr",
        "data-ui-fallback=agent_contract.fallback_attr",
        "data-ui-output-status=agent_contract.output_status_attr",
    ] {
        assert!(
            sheet_logic.contains(needle) || sheet_view.contains(needle),
            "Sheet contract should keep optional streaming + explicit output markers `{needle}`."
        );
    }

    let needle = "data-ui-output-status=move || agent_contract.get().output_status.as_str()";
    assert!(
        modal_view.contains(needle) && drawer_view.contains(needle),
        "Modal/Drawer should keep explicit output-status marker `{needle}`."
    );

    for forbidden in [
        "retry",
        "backoff",
        "reconnect",
        "websocket",
        "EventSource",
        "reqwest",
        "fetch(",
    ] {
        assert!(
            !sheet_logic.contains(forbidden)
                && !sheet_view.contains(forbidden)
                && !overlay_logic.contains(forbidden)
                && !modal_view.contains(forbidden)
                && !drawer_view.contains(forbidden)
                && !contextual_help_logic.contains(forbidden)
                && !contextual_help_view.contains(forbidden),
            "streaming resilience/validation concern `{forbidden}` should stay outside overlays component layer."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_streaming_required_optional_classification_rules_are_scope_driven_and_boundary_safe";
    assert!(
        streaming_script.contains(script_needle),
        "streaming checklist gate should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources() {
    let source_files = [
        ("overlays/logic.rs", load_source("src/logic.rs")),
        ("overlays/view.rs", load_source("src/view.rs")),
        ("overlays/motion.rs", load_source("src/motion.rs")),
        ("overlay/logic.rs", load_source("../overlay/src/logic.rs")),
        ("overlay/view.rs", load_source("../overlay/src/view.rs")),
        ("overlay/motion.rs", load_source("../overlay/src/motion.rs")),
        ("popover/logic.rs", load_source("../popover/src/logic.rs")),
        ("popover/view.rs", load_source("../popover/src/view.rs")),
        ("popover/motion.rs", load_source("../popover/src/motion.rs")),
        ("modal/logic.rs", load_source("../modal/src/logic.rs")),
        ("modal/view.rs", load_source("../modal/src/view.rs")),
        ("modal/motion.rs", load_source("../modal/src/motion.rs")),
        ("sheet/logic.rs", load_source("../sheet/src/logic.rs")),
        ("sheet/view.rs", load_source("../sheet/src/view.rs")),
        ("sheet/motion.rs", load_source("../sheet/src/motion.rs")),
        ("drawer/logic.rs", load_source("../drawer/src/logic.rs")),
        ("drawer/view.rs", load_source("../drawer/src/view.rs")),
        ("drawer/motion.rs", load_source("../drawer/src/motion.rs")),
        (
            "contextual-help/logic.rs",
            load_source("../contextual-help/src/logic.rs"),
        ),
        (
            "contextual-help/view.rs",
            load_source("../contextual-help/src/view.rs"),
        ),
        (
            "contextual-help/motion.rs",
            load_source("../contextual-help/src/motion.rs"),
        ),
    ];

    for (name, source) in source_files {
        for forbidden in [".unwrap(", ".unwrap_err(", ".expect("] {
            assert!(
                !source.contains(forbidden),
                "{name} should not contain forbidden non-test call `{forbidden}`."
            );
        }
        assert!(
            !source.contains("let _ ="),
            "{name} should not swallow side effects via `let _ =`."
        );
    }
}

#[test]
fn overlays_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent() {
    let overlays_logic = load_source("src/logic.rs");
    let overlay_logic = load_source("../overlay/src/logic.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let sheet_logic = load_source("../sheet/src/logic.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");

    for (name, source) in [
        ("overlays/logic.rs", overlays_logic.as_str()),
        ("popover/logic.rs", popover_logic.as_str()),
        ("sheet/logic.rs", sheet_logic.as_str()),
        ("overlay/view.rs", overlay_view.as_str()),
        ("popover/view.rs", popover_view.as_str()),
        ("sheet/view.rs", sheet_view.as_str()),
    ] {
        assert!(
            source.contains("Cow<'static, str>")
                || source.contains("Cow::Borrowed")
                || source.contains("use std::borrow::Cow;"),
            "{name} should model string hot paths with `Cow<'static, str>`."
        );
    }

    for (name, source) in [
        ("overlays/logic.rs", overlays_logic.as_str()),
        ("overlay/logic.rs", overlay_logic.as_str()),
        ("popover/logic.rs", popover_logic.as_str()),
        ("popover/view.rs", popover_view.as_str()),
        ("sheet/logic.rs", sheet_logic.as_str()),
        ("sheet/view.rs", sheet_view.as_str()),
    ] {
        for forbidden in [".to_string()", ".to_owned()", "String::from("] {
            assert!(
                !source.contains(forbidden),
                "{name} should avoid string clone hotspot token `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_rust_hygiene_script_enforces_repo_level_hygiene_guards() {
    let rust_hygiene_script = load_source("../../scripts/check-rust-hygiene.sh");
    let check_script = load_source("../../scripts/check.sh");
    let engineering_script = load_source("../../scripts/check-ui-engineering.sh");

    for needle in [
        "forbidden unwrap/expect in non-test code",
        "forbidden let _ = in non-test code",
        "string clone hotspots (prefer Cow<'static, str>)",
        "find crates apps -type f -name '*.rs' -path '*/src/*' | sort",
    ] {
        assert!(
            rust_hygiene_script.contains(needle),
            "repo rust-hygiene script should keep guard `{needle}`."
        );
    }

    assert!(
        check_script.contains("./scripts/check-rust-hygiene.sh"),
        "top-level check pipeline should run rust-hygiene script."
    );

    for needle in [
        "cargo test -p ui-overlays overlays_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources",
        "cargo test -p ui-overlays overlays_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent",
        "cargo test -p ui-overlays overlays_rust_hygiene_script_enforces_repo_level_hygiene_guards",
    ] {
        assert!(
            engineering_script.contains(needle),
            "engineering script should enforce overlays rust-hygiene gate `{needle}`."
        );
    }
}

#[test]
fn overlays_spec_rs_stays_absent_without_complex_schema_requirement() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for rel in [
        "src/spec.rs",
        "../overlay/src/spec.rs",
        "../popover/src/spec.rs",
        "../modal/src/spec.rs",
        "../tray/src/spec.rs",
        "../sheet/src/spec.rs",
    ] {
        let path = manifest_dir.join(rel);
        assert!(
            !path.exists(),
            "simple overlays family should not add `spec.rs` by default: {path:?}"
        );
    }

    for rel in [
        "src/protocol.rs",
        "test/protocol.rs",
        "../overlay/src/protocol.rs",
        "../overlay/test/protocol.rs",
        "../popover/src/protocol.rs",
        "../popover/src/test/protocol.rs",
        "../modal/src/protocol.rs",
        "../modal/test/protocol.rs",
        "../tray/src/protocol.rs",
        "../tray/test/protocol.rs",
        "../sheet/src/protocol.rs",
        "../sheet/test/protocol.rs",
    ] {
        let path = manifest_dir.join(rel);
        assert!(
            path.exists(),
            "schema/protocol contracts should stay in protocol paths: {path:?}"
        );
    }

    for (scope, source) in [
        ("overlays", load_source("src/overlays/mod.rs")),
        ("overlay", load_source("../overlay/src/mod.rs")),
        ("popover", load_source("../popover/src/mod.rs")),
        ("modal", load_source("../modal/src/mod.rs")),
        ("tray", load_source("../tray/src/mod.rs")),
        ("sheet", load_source("../sheet/src/mod.rs")),
    ] {
        for forbidden in ["mod spec;", "pub mod spec;", "pub use spec::"] {
            assert!(
                !source.contains(forbidden),
                "{scope} module should not export/enable speculative spec layer `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_motion_contract_is_mapped_to_ui_motion_runtime() {
    let overlays_motion = load_source("src/motion.rs");
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let tray_motion = load_source("../tray/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_spring = load_source("../../crates/ui-motion/src/spring.rs");

    for needle in [
        "pub struct OverlaysMotion",
        "overlay: crate::overlay::motion::sanitize_motion(motion.overlay)",
        "popover: crate::popover::motion::sanitize_motion(motion.popover)",
        "tray: crate::tray::motion::sanitize_motion(motion.tray)",
    ] {
        assert!(
            overlays_motion.contains(needle),
            "Overlays motion should include `{needle}`."
        );
    }
    for forbidden in [
        "SpringAnimator::new",
        "request_animation_frame",
        "keyframes_to_js",
    ] {
        assert!(
            !overlays_motion.contains(forbidden),
            "Overlays aggregator motion should not implement runtime engine detail `{forbidden}`."
        );
    }

    for needle in [
        "ui_motion::spring::SpringAnimator::new",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
    ] {
        assert!(
            overlay_motion.contains(needle),
            "Overlay motion should include `{needle}` for wasm/non-wasm split runtime contract."
        );
    }

    for needle in [
        "ui_motion::spring::sanitize_config(value, default)",
        "ui_motion::spring::SpringAnimator::new",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "if !is_open.get() {",
    ] {
        assert!(
            popover_motion.contains(needle),
            "Popover motion should include `{needle}` for mapped contract + no-op path."
        );
    }

    for needle in [
        "pub struct TrayMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
    ] {
        assert!(
            tray_motion.contains(needle),
            "Tray motion should include `{needle}` and delegate to Sheet contract."
        );
    }
    assert!(
        !tray_motion.contains("SpringAnimator::new"),
        "Tray motion should remain an assembly mapping and avoid local motion engine code."
    );

    for needle in [
        "ui_motion::presets::spring_slide()",
        "ui_motion::spring::SpringAnimator::new",
        "ui_motion::web::prefers_reduced_motion()",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            sheet_motion.contains(needle),
            "Sheet motion should include `{needle}` for reduced-motion + runtime mapping."
        );
    }

    for needle in [
        "pub mod keyframes;",
        "pub mod options;",
        "pub mod presets;",
        "pub mod spring;",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion crate should include `{needle}` as runtime primitive boundary."
        );
    }

    for forbidden in [
        "aria-",
        "data-slot",
        "role=\"",
        "Overlay",
        "Popover",
        "Tray",
    ] {
        assert!(
            !ui_motion_lib.contains(forbidden),
            "ui-motion crate should not contain component semantics `{forbidden}`."
        );
    }

    for needle in [
        "pub struct SpringConfig",
        "pub fn sanitize_config(value: SpringConfig, fallback: SpringConfig) -> SpringConfig",
        "pub struct SpringAnimator",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring runtime should include `{needle}`."
        );
    }
}

#[test]
fn overlays_ui_motion_non_wasm_stub_contract_is_predictable_and_tooling_safe() {
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");
    let modal_motion = load_source("../modal/src/motion.rs");
    let tray_motion = load_source("../tray/src/motion.rs");

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub mod web {",
        "pub fn prefers_reduced_motion() -> bool {",
        "true",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
        "fn non_wasm_web_backend_is_predictable_noop() {",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion non-wasm stub contract should include `{needle}`."
        );
    }

    for (scope, source, needles) in [
        (
            "Overlay",
            overlay_motion.as_str(),
            vec![
                "#[cfg(not(target_arch = \"wasm32\"))]",
                "pub fn attach_motion(",
                "if !is_open.get() {",
                "finish_exit.run(());",
            ],
        ),
        (
            "Popover",
            popover_motion.as_str(),
            vec![
                "#[cfg(not(target_arch = \"wasm32\"))]",
                "pub fn attach_motion(",
                "if !is_open.get() {",
                "on_exit_complete.run(());",
            ],
        ),
        (
            "Sheet",
            sheet_motion.as_str(),
            vec![
                "#[cfg(not(target_arch = \"wasm32\"))]",
                "pub fn attach_motion(",
                "if !is_open.get() {",
                "finish_exit.run(());",
            ],
        ),
    ] {
        for needle in needles {
            assert!(
                source.contains(needle),
                "{scope} should keep non-wasm safe-degrade contract `{needle}`."
            );
        }
    }

    for (scope, source) in [
        ("Modal", modal_motion.as_str()),
        ("Tray", tray_motion.as_str()),
    ] {
        for forbidden in [
            "SpringAnimator::new",
            "request_animation_frame",
            "web_sys",
            "wasm_bindgen",
            "js_sys",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} motion assembly should not assume runtime animation handle `{forbidden}`."
            );
        }
    }

    let non_wasm_segments = [
        ("Overlay", overlay_motion.as_str()),
        ("Popover", popover_motion.as_str()),
        ("Sheet", sheet_motion.as_str()),
    ]
    .into_iter()
    .filter_map(|(scope, source)| {
        let marker = "#[cfg(not(target_arch = \"wasm32\"))]\npub fn attach_motion(";
        source
            .find(marker)
            .map(|start| (scope, &source[start..source.len()]))
    })
    .collect::<Vec<_>>();

    for (scope, segment) in non_wasm_segments {
        for forbidden in [
            "panic!(",
            "unwrap(",
            "expect(",
            "todo!(",
            "unreachable!(",
            "web_sys",
        ] {
            assert!(
                !segment.contains(forbidden),
                "{scope} non-wasm motion path should avoid panic/DOM hard dependency `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_reduced_motion_ssr_wasm_branches_are_covered_without_semantic_split() {
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");
    let ui_motion_lib = load_source("../../crates/ui-motion/src/lib.rs");
    let ui_motion_spring = load_source("../../crates/ui-motion/src/spring.rs");

    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");

    for (scope, source) in [
        ("Overlay motion", overlay_motion.as_str()),
        ("Popover motion", popover_motion.as_str()),
        ("Sheet motion", sheet_motion.as_str()),
    ] {
        for needle in [
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep wasm/non-wasm branch marker `{needle}`."
            );
        }
    }

    for needle in [
        "if ui_motion::web::prefers_reduced_motion() {",
        "on_exit_complete.run(());",
    ] {
        assert!(
            popover_motion.contains(needle),
            "Popover motion should keep reduced-motion downgrade branch `{needle}`."
        );
    }

    for needle in [
        "let prefers_reduced_motion = StoredValue::new(ui_motion::web::prefers_reduced_motion());",
        "if prefers_reduced_motion {",
        "if reduced_motion {",
        "finish_exit.run(());",
    ] {
        assert!(
            sheet_motion.contains(needle),
            "Sheet motion should keep reduced-motion downgrade branch `{needle}`."
        );
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "self.inner.state.set(SpringState::new(target));",
        "(self.inner.apply.borrow_mut())(target);",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring runtime should keep reduced-motion immediate-settle path `{needle}`."
        );
    }

    for needle in [
        "#[cfg(not(target_arch = \"wasm32\"))]",
        "pub fn prefers_reduced_motion() -> bool {",
        "pub fn animate(_element: &(), _keyframes: &[MotionKeyframe], _options: MotionOptions) {}",
    ] {
        assert!(
            ui_motion_lib.contains(needle),
            "ui-motion lib should keep non-wasm predictable fallback `{needle}`."
        );
    }

    for (scope, source) in [
        ("Overlay view", overlay_view.as_str()),
        ("Popover view", popover_view.as_str()),
        ("Sheet view", sheet_view.as_str()),
    ] {
        for needle in [
            "data-state=move || logic::state_attr_for_open(open.get())",
            "data-open=move || open.get().then_some(\"true\")",
            "data-closed=move || (!open.get()).then_some(\"true\")",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep stable semantic marker `{needle}` across platform branches."
            );
        }
    }

    for (scope, source) in [
        ("Overlay view", overlay_view.as_str()),
        ("Popover view", popover_view.as_str()),
        ("Sheet view", sheet_view.as_str()),
    ] {
        for needle in [
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
            "let is_composing =",
            "let default_prevented =",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep explicit event-branch cfg guard `{needle}`."
            );
        }
    }
}

#[test]
fn overlays_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe() {
    let overlay_motion = load_source("../overlay/src/motion.rs");
    let popover_motion = load_source("../popover/src/motion.rs");
    let modal_motion = load_source("../modal/src/motion.rs");
    let sheet_motion = load_source("../sheet/src/motion.rs");
    let tray_motion = load_source("../tray/src/motion.rs");
    let ui_motion_spring = load_source("../../crates/ui-motion/src/spring.rs");
    let platforms_script = load_source("../../scripts/check-ui-platforms.sh");

    for (scope, source) in [
        ("Overlay motion", overlay_motion.as_str()),
        ("Popover motion", popover_motion.as_str()),
        ("Sheet motion", sheet_motion.as_str()),
    ] {
        for needle in [
            "pub struct",
            "pub spring: ui_motion::spring::SpringConfig",
            "impl Default for",
            "pub fn sanitize_motion(",
            "pub fn attach_motion(",
            "#[cfg(target_arch = \"wasm32\")]",
            "#[cfg(not(target_arch = \"wasm32\"))]",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep component-scoped motion contract marker `{needle}`."
            );
        }
    }

    for needle in [
        "if ui_motion::web::prefers_reduced_motion() {",
        "if prefers_reduced_motion {",
        "if reduced_motion {",
    ] {
        assert!(
            popover_motion.contains(needle) || sheet_motion.contains(needle),
            "overlays motion path should keep reduced-motion downgrade marker `{needle}`."
        );
    }

    for (scope, source) in [
        ("Overlay non-wasm", overlay_motion.as_str()),
        ("Popover non-wasm", popover_motion.as_str()),
        ("Sheet non-wasm", sheet_motion.as_str()),
    ] {
        for needle in [
            "#[cfg(not(target_arch = \"wasm32\"))]",
            "if !is_open.get() {",
        ] {
            assert!(
                source.contains(needle),
                "{scope} should keep predictable non-wasm safe-degrade marker `{needle}`."
            );
        }
        for forbidden in ["panic!(", "unwrap()", "expect("] {
            assert!(
                !source.contains(forbidden),
                "{scope} non-wasm path should avoid fragile runtime assumption `{forbidden}`."
            );
        }
    }

    for needle in [
        "if crate::web::prefers_reduced_motion() {",
        "pub fn set_target(&self, target: f64) {",
    ] {
        assert!(
            ui_motion_spring.contains(needle),
            "ui-motion spring runtime should keep reduced-motion immediate settle marker `{needle}`."
        );
    }

    for needle in [
        "pub const MODAL_MOTION_CONTRACT_STIFFNESS: f64",
        "pub const MODAL_MOTION_CONTRACT_DAMPING: f64",
        "pub fn default_motion_contract() -> OverlayMotion",
        "overlay_motion::sanitize_motion(default_motion_contract())",
        "pub fn is_custom_motion(motion: OverlayMotion) -> bool",
    ] {
        assert!(
            modal_motion.contains(needle),
            "Modal motion should keep explicit component contract marker `{needle}`."
        );
    }

    for needle in [
        "pub struct TrayMotion",
        "sheet: crate::sheet::motion::sanitize_motion(motion.sheet)",
    ] {
        assert!(
            tray_motion.contains(needle),
            "Tray motion should map to sheet motion contract via `{needle}`."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe";
    assert!(
        platforms_script.contains(script_needle),
        "platforms check script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_ui_components_fixed_entry_files_follow_layered_boundaries() {
    let ui_components_lib = load_source("../../crates/ui/src/lib.rs");
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let ui_components_root = load_source("../../crates/ui/src/root.rs");
    let active_highlight = load_source("../../crates/ui-visual-primitive/src/active_highlight.rs");
    let headless_controllable = load_source("../../crates/ui-headless/src/controllable_state.rs");
    let headless_presence = load_source("../../crates/ui-headless/src/presence.rs");
    let headless_a11y = load_source("../../crates/ui-headless/src/a11y.rs");
    let entrypoints_script = load_source("../../scripts/check-ui-entrypoints.sh");
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    for needle in [
        "mod css;",
        "#[cfg(feature = \"component-overlay\")]",
        "pub mod overlay;",
        "#[cfg(feature = \"component-overlays\")]",
        "pub mod overlays;",
        "#[cfg(feature = \"component-modal\")]",
        "pub mod modal;",
        "#[cfg(feature = \"component-popover\")]",
        "pub use ui_popover as popover;",
        "#[cfg(feature = \"component-sheet\")]",
        "pub use ui_sheet as sheet;",
        "#[cfg(feature = \"component-tray\")]",
        "pub use ui_tray as tray;",
    ] {
        assert!(
            ui_components_lib.contains(needle),
            "ui lib entry should keep feature-gated public surface marker `{needle}`."
        );
    }

    for forbidden in [
        "pub mod overlay_open;",
        "pub mod presence;",
        "pub mod a11y;",
        "pub use web_sys::",
        "pub use wasm_bindgen::",
    ] {
        assert!(
            !ui_components_lib.contains(forbidden),
            "ui lib entry should not expose forbidden entrypoint/platform marker `{forbidden}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "pub fn push_components_css(out: &mut String) {",
        "out.push_str(\"\\n@layer ui {\\n\");",
        "#[cfg(feature = \"component-overlay\")]",
        "out.push_str(crate::overlay::styles::CSS);",
        "#[cfg(feature = \"component-overlays\")]",
        "out.push_str(crate::overlays::styles::CSS);",
        "#[cfg(feature = \"component-popover\")]",
        "out.push_str(crate::popover::styles::CSS);",
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "#[cfg(feature = \"component-sheet\")]",
        "out.push_str(crate::sheet::styles::CSS);",
        "#[cfg(feature = \"component-tray\")]",
        "out.push_str(crate::tray::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css entry should keep feature-gated overlays aggregation marker `{needle}`."
        );
    }

    for needle in [
        "pub fn UiRoot(",
        "#[prop(optional)] inject_components_css: bool,",
        "#[prop(optional)] i18n: UiI18n,",
        "provide_ui_i18n(i18n);",
        "provide_ui_id_provider(id_seed);",
        "out.push_str(css::BASE_CSS);",
        "out.push_str(&theme.get().to_css_variables());",
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_components_root.contains(needle),
            "UiRoot should keep centralized injection/i18n marker `{needle}`."
        );
    }

    for needle in [
        "pub const CSS: &str = r#\"",
        "pub struct ActiveHighlightMotion",
        "struct ActiveHighlightMotionDriver",
        "pub fn attach_active_highlight_motion(",
        "#[cfg(target_arch = \"wasm32\")]",
        "#[cfg(not(target_arch = \"wasm32\"))]",
    ] {
        assert!(
            active_highlight.contains(needle),
            "active_highlight primitive should keep shared motion capability marker `{needle}`."
        );
    }
    for forbidden in [
        "on_open_change",
        "is_open",
        "Overlay",
        "Popover",
        "Modal",
        "Tray",
    ] {
        assert!(
            !active_highlight.contains(forbidden),
            "active_highlight primitive should not embed component business semantics `{forbidden}`."
        );
    }

    for forbidden in [
        "../../crates/ui/src/overlay_open.rs",
        "../../crates/ui/src/presence.rs",
        "../../crates/ui/src/a11y.rs",
    ] {
        let path = manifest_dir.join(forbidden);
        assert!(
            !path.exists(),
            "forbidden ui entrypoint file should not exist: {path:?}"
        );
    }

    for needle in [
        "pub fn use_controllable_state<T>(",
        "pub fn use_controllable_open_state_traced(",
    ] {
        assert!(
            headless_controllable.contains(needle),
            "headless controllable-state canonical path should keep `{needle}`."
        );
    }
    assert!(
        headless_presence.contains("pub fn use_presence(is_open: Signal<bool>) -> Presence {"),
        "headless presence canonical path should expose `use_presence`."
    );
    assert!(
        headless_a11y.contains(
            "pub fn aria_controls_when_open(open: Signal<bool>, controls_id: String) -> Signal<Option<String>> {"
        ),
        "headless a11y canonical path should expose `aria_controls_when_open`."
    );

    let script_needle = "cargo test -p ui-overlays overlays_ui_components_fixed_entry_files_follow_layered_boundaries";
    assert!(
        entrypoints_script.contains(script_needle),
        "entrypoints check script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_theme_tokens_are_sourced_from_ui_theme_and_consumed_by_styles() {
    let overlay_styles = load_source("../overlay/src/styles.rs");
    let popover_styles = load_source("../popover/src/styles.rs");
    let modal_styles = load_source("../modal/src/styles.rs");
    let tray_styles = load_source("../tray/src/styles.rs");
    let sheet_styles = load_source("../sheet/src/styles.rs");
    let theme_tokens = load_source("../../crates/ui-theme/src/tokens.rs");
    let theme_mapping = load_source("../../crates/ui-theme/src/theme.rs");
    let theme_css = load_source("../../crates/ui-theme/src/css.rs");
    let styling_spec = load_source("../../docs/spec/styling.md");

    for needle in [
        "pub struct OverlayLayoutTokens",
        "pub struct ThemeTokens",
        "pub overlay_layout: OverlayLayoutTokens",
    ] {
        assert!(
            theme_tokens.contains(needle),
            "ui-theme tokens should define `{needle}` as the overlay baseline source."
        );
    }

    for needle in [
        "pub fn overlay_layout_tokens(ctx: ThemeContext) -> OverlayLayoutTokens",
        "pub fn default_overlay_layout_tokens() -> OverlayLayoutTokens",
    ] {
        assert!(
            theme_mapping.contains(needle),
            "ui-theme mapping should expose `{needle}`."
        );
    }

    for needle in [
        "--ui-overlay-z-index",
        "--ui-overlay-panel-min-width",
        "--ui-overlay-viewport-inset",
        "--ui-overlay-enter-offset-y",
        "--ui-overlay-enter-scale",
    ] {
        assert!(
            theme_css.contains(needle),
            "ui-theme css emission should contain `{needle}`."
        );
    }

    for needle in [
        "--ui-overlay-panel-min-width",
        "--ui-overlay-viewport-inset",
        "--ui-overlay-enter-offset-y",
        "--ui-overlay-enter-scale",
        "--ui-space-lg",
        "--ui-space-md",
        "--ui-component-height-100",
        "--ui-space-3xs",
    ] {
        let consumed = overlay_styles.contains(needle)
            || popover_styles.contains(needle)
            || modal_styles.contains(needle)
            || tray_styles.contains(needle)
            || sheet_styles.contains(needle);
        assert!(
            consumed,
            "overlays family styles should consume ui-theme variable `{needle}`."
        );
    }

    for forbidden in [
        "z-index: 1000;",
        "padding: 24px;",
        "width: min(36rem, calc(100vw - 2rem));",
        "gap: 12px;",
        "padding-right: 44px;",
        "block-size: min(28rem, 78vh);",
        "max-block-size: min(80vh, 36rem);",
    ] {
        let leaked = overlay_styles.contains(forbidden)
            || popover_styles.contains(forbidden)
            || modal_styles.contains(forbidden)
            || tray_styles.contains(forbidden)
            || sheet_styles.contains(forbidden);
        assert!(
            !leaked,
            "overlays styles should avoid hardcoded visual baseline `{forbidden}`."
        );
    }

    for needle in [
        "--ui-overlay-panel-min-width",
        "--ui-overlay-viewport-inset",
        "--ui-overlay-enter-offset-y",
    ] {
        assert!(
            styling_spec.contains(needle),
            "styling spec should include `{needle}` so token taxonomy stays traceable."
        );
    }
}

#[test]
fn overlays_token_first_static_style_contract_is_aggregated_and_framework_agnostic() {
    let ui_components_css = load_source("../../crates/ui/src/css.rs");
    let ui_root = load_source("../../crates/ui/src/root.rs");

    let overlays_mod = load_source("src/overlays/mod.rs");
    let overlays_logic = load_source("src/logic.rs");
    let overlays_view = load_source("src/view.rs");
    let overlays_styles = load_source("src/styles.rs");
    let overlays_motion = load_source("src/motion.rs");

    let overlay_view = load_source("../overlay/src/view.rs");
    let overlay_styles = load_source("../overlay/src/styles.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let popover_logic = load_source("../popover/src/logic.rs");
    let popover_styles = load_source("../popover/src/styles.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let modal_styles = load_source("../modal/src/styles.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let tray_styles = load_source("../tray/src/styles.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let sheet_styles = load_source("../sheet/src/styles.rs");

    for needle in [
        "#[cfg(feature = \"component-overlay\")]",
        "out.push_str(crate::overlay::styles::CSS);",
        "#[cfg(feature = \"component-overlays\")]",
        "out.push_str(crate::overlays::styles::CSS);",
        "#[cfg(feature = \"component-popover\")]",
        "out.push_str(crate::popover::styles::CSS);",
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "#[cfg(feature = \"component-sheet\")]",
        "out.push_str(crate::sheet::styles::CSS);",
        "#[cfg(feature = \"component-tray\")]",
        "out.push_str(crate::tray::styles::CSS);",
    ] {
        assert!(
            ui_components_css.contains(needle),
            "ui css aggregator should include token-first style wiring `{needle}`."
        );
    }

    for needle in [
        "if inject_components_css.get_value() {",
        "crate::css::push_components_css(&mut out);",
        "ui_layout::push_components_css(&mut out);",
    ] {
        assert!(
            ui_root.contains(needle),
            "UiRoot should inject component css through `{needle}`."
        );
    }

    for (scope, source) in [
        ("OverlaysRoot", overlays_styles.as_str()),
        ("Overlay", overlay_styles.as_str()),
        ("Popover", popover_styles.as_str()),
        ("Modal", modal_styles.as_str()),
        ("Tray", tray_styles.as_str()),
        ("Sheet", sheet_styles.as_str()),
    ] {
        assert!(
            source.contains("pub const CSS: &str = r#\""),
            "{scope} should keep static css constants in styles.rs."
        );
        assert!(
            source.contains("var(--ui-"),
            "{scope} styles should consume ui-theme token variables."
        );
    }

    for (scope, source) in [
        ("OverlaysRoot", overlays_view.as_str()),
        ("Overlay", overlay_view.as_str()),
        ("Modal", modal_view.as_str()),
        ("Tray", tray_view.as_str()),
        ("Sheet", sheet_view.as_str()),
    ] {
        assert!(
            !source.contains("style="),
            "{scope} view should avoid inline business styling."
        );
    }
    assert!(
        popover_view.contains("style=panel_vars"),
        "Popover view should only pass necessary runtime css variables via `style=panel_vars`."
    );
    for needle in [
        "pub fn compose_panel_vars(",
        "--ui-popover-top: {top_px}px;",
        "--ui-popover-left: {left_px}px;",
        "--ui-popover-anchor-width: {anchor_width_px}px;",
    ] {
        assert!(
            popover_logic.contains(needle),
            "Popover runtime style bridge should remain css-variable-only via `{needle}`."
        );
    }

    for (scope, source) in [
        ("overlays mod", overlays_mod.as_str()),
        ("overlays logic", overlays_logic.as_str()),
        ("overlays view", overlays_view.as_str()),
        ("overlays styles", overlays_styles.as_str()),
        ("overlays motion", overlays_motion.as_str()),
        ("overlay styles", overlay_styles.as_str()),
        ("popover styles", popover_styles.as_str()),
        ("modal styles", modal_styles.as_str()),
        ("tray styles", tray_styles.as_str()),
        ("sheet styles", sheet_styles.as_str()),
    ] {
        for forbidden in [
            "tailwind",
            "class-variance-authority",
            "cva(",
            "stylist",
            "stylex",
            "emotion",
            "linaria",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} should not default to utility-first/css-in-rust contract token `{forbidden}`."
            );
        }
    }
}

#[test]
fn overlays_visual_desire_has_default_theme_baseline_page_and_screenshot_regression() {
    let theme_baseline_page =
        load_source("../../apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs");
    let docs_pages_registry = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let visual_e2e = load_source("../../e2e/tests/docs_app_theme_visual_baseline.spec.mjs");

    for needle in [
        "title=\"ThemeVisualBaseline\"",
        "slug=\"theme-visual-baseline\"",
        "Default theme visual baseline for hierarchy, contrast, and interaction cues.",
        "title=\"Default Theme Visual Baseline\"",
        "first-impression quality: hierarchy, spacing rhythm, contrast layers, and interactive feedback",
    ] {
        assert!(
            theme_baseline_page.contains(needle),
            "theme visual baseline page should include visual-desire guidance `{needle}`."
        );
    }

    for needle in [
        "data-slot=\"theme-visual-baseline\"",
        "data-slot=\"theme-visual-baseline-button\"",
        "data-slot=\"theme-visual-baseline-input\"",
        "data-slot=\"theme-visual-baseline-overlay\"",
        "<Button variant=ButtonVariant::Accent>\"Primary Action\"</Button>",
        "<Input",
        "<Overlay",
    ] {
        assert!(
            theme_baseline_page.contains(needle),
            "theme baseline page should include Button/Input/Overlay visual baseline token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(",
        "\"ThemeVisualBaseline\"",
        "\"theme-visual-baseline\"",
    ] {
        assert!(
            docs_pages_registry.contains(needle),
            "docs pages registry should include theme baseline entry token `{needle}`."
        );
    }

    for needle in [
        "theme visual baseline renders button/input/overlay",
        "theme visual baseline screenshots",
        "E2E_VISUAL_BASELINE",
        "toHaveScreenshot(",
        "\"docs-app-theme-visual-baseline-page.png\"",
        "\"docs-app-theme-visual-baseline-button.png\"",
        "\"docs-app-theme-visual-baseline-input.png\"",
        "\"docs-app-theme-visual-baseline-overlay.png\"",
    ] {
        assert!(
            visual_e2e.contains(needle),
            "visual baseline e2e should include `{needle}`."
        );
    }

    for forbidden in ["btn btn-primary", "form-control", "panel panel-default"] {
        assert!(
            !theme_baseline_page.contains(forbidden),
            "theme baseline should avoid legacy bootstrap-like token `{forbidden}`."
        );
    }
}

#[test]
fn overlays_performance_governance_contract_is_budgeted_traceable_and_blocking() {
    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    let pages_source = load_source("../../apps/docs-app/src/pages/components/pages.rs");
    let perf_probe_source = load_source("../../apps/docs-app/src/perf_probe.rs");
    let coverage_source = load_source("../../e2e/tests/docs_app_components_coverage.spec.mjs");
    let perf_script_source = load_source("../../scripts/check-ui-performance.sh");
    let todo_source = load_source("../../docs/plan/TODO.md");

    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");

    for needle in [
        "fn component_page_perf_budget(slug: &'static str) -> UiPerfBudget {",
        "\"button\" => UiPerfBudget {",
        "\"input\" => UiPerfBudget {",
        "_ => UiPerfBudget::mount_only(120.0),",
        "<UiPerfProbe name=perf_name budget=perf_budget>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell should keep performance budget baseline token `{needle}`."
        );
    }

    for needle in [
        "component_doc!(\"Overlay\", \"overlay\", \"Overlays\", overlays::overlay),",
        "component_doc!(\"Popover\", \"popover\", \"Overlays\", overlays::popover),",
        "component_doc!(\"Modal\", \"modal\", \"Overlays\", overlays::modal),",
        "component_doc!(\"Tray\", \"tray\", \"Overlays\", overlays_extra::tray),",
    ] {
        assert!(
            pages_source.contains(needle),
            "docs pages registry should keep overlays route token `{needle}`."
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
            "UiPerfProbe should expose perf regression marker `{needle}`."
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
            "docs coverage e2e should keep perf assertion `{needle}`."
        );
    }

    for needle in [
        "data-state=move || state.get().data_state_attr",
        "data-open=move || state.get().is_open.then_some(\"true\")",
        "data-closed=move || state.get().is_closed.then_some(\"true\")",
        "data-id-source=move || state.get().id_source_attr",
        "data-class-source=move || state.get().class_source_attr",
    ] {
        assert!(
            overlays_view.contains(needle),
            "OverlaysRoot should keep performance attribution marker `{needle}`."
        );
    }

    for needle in [
        "data-motion-source=root_state.motion_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-exit-source=root_state.exit_source_attr",
    ] {
        assert!(
            overlay_view.contains(needle),
            "Overlay should keep performance attribution marker `{needle}`."
        );
    }

    for needle in [
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        "data-class-source=root_state.class_source_attr",
    ] {
        assert!(
            popover_view.contains(needle),
            "Popover should keep performance attribution marker `{needle}`."
        );
    }

    for needle in [
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            modal_view.contains(needle),
            "Modal should keep performance attribution marker `{needle}`."
        );
    }

    for needle in [
        "data-motion-source=root_state.motion_source_attr",
        "data-placement-source=root_state.placement_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
    ] {
        assert!(
            sheet_view.contains(needle),
            "Sheet should keep performance attribution marker `{needle}`."
        );
    }

    for needle in [
        "data-footer-source=root_state.footer_source_attr",
        "data-close-source=root_state.close_source_attr",
        "data-size-source=root_state.size_source_attr",
        "data-dismiss-source=root_state.dismiss_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "data-motion-source=root_state.motion_source_attr",
    ] {
        assert!(
            tray_view.contains(needle),
            "Tray should keep performance attribution marker `{needle}`."
        );
    }

    for needle in [
        "echo \"[perf] contract: overlays performance governance\"",
        "cargo test -p ui-overlays overlays_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test button_semantics button_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test input_semantics --no-default-features --features component-input,inject-css input_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            perf_script_source.contains(needle),
            "performance gate script should include `{needle}`."
        );
    }

    for needle in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(needle),
            "performance follow-up plan should keep `{needle}`."
        );
    }
}

#[test]
fn overlays_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()
 {
    let local_semantics = load_source("tests/overlays_module_semantics.rs");
    let overlay_semantics = load_source("../../components/overlay/test/overlay_semantics.rs");
    let popover_semantics = load_source("../../components/popover/src/test/popover_semantics.rs");
    let modal_semantics = load_source("../../components/modal/test/modal_semantics.rs");
    let focus_trap_source = load_source("../../crates/ui-headless/src/focus_trap.rs");
    let todo_source = load_source("../../docs/plan/TODO.md");

    for required_test in [
        "fn overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only()",
        "fn overlays_focus_stack_and_gc_use_global_focus_manager_contract()",
        "fn overlays_performance_governance_contract_is_budgeted_traceable_and_blocking()",
        "fn overlays_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement()",
    ] {
        assert!(
            local_semantics.contains(required_test),
            "semantic/performance regression suite should include `{required_test}`."
        );
    }

    for marker in [
        "data-role-source=root_state.role_source_attr",
        "data-keyboard-dismiss-source=root_state.keyboard_dismiss_source_attr",
        "aria-modal=\"true\"",
        ".ui-overlay[data-dismissable=\"true\"] .ui-overlay__backdrop",
    ] {
        assert!(
            overlay_semantics.contains(marker),
            "Overlay semantics should assert aria/data/focus marker `{marker}`."
        );
    }

    for marker in [
        "data-placement-source=root_state.placement_source_attr",
        "data-modal-source=root_state.modal_source_attr",
        ".ui-popover__panel[data-placement=\"bottom-start\"]",
    ] {
        assert!(
            popover_semantics.contains(marker),
            "Popover semantics should assert aria/data marker `{marker}`."
        );
    }

    for marker in [
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
        "aria-modal=\"true\"",
    ] {
        assert!(
            modal_semantics.contains(marker),
            "Modal semantics should assert aria/data marker `{marker}`."
        );
    }

    for marker in [
        "focus_manager_push_trap(FocusTrapFrame {",
        "focus_manager_pop_trap(",
        "restore_focus_chain(",
    ] {
        assert!(
            focus_trap_source.contains(marker),
            "ui-headless focus manager should keep focus-flow marker `{marker}`."
        );
    }

    for marker in [
        "render_count",
        "建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据",
    ] {
        assert!(
            todo_source.contains(marker),
            "render_count governance follow-up should include `{marker}`."
        );
    }
}

#[test]
fn overlays_semantics_and_performance_script_covers_contract() {
    let script_source = load_source("../../scripts/check-ui-performance.sh");

    for marker in [
        "cargo test -p ui-overlays overlays_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "cargo test -p ui-overlays overlays_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "cargo test -p ui --test accordion_semantics perf_render_count_follow_up_is_tracked_in_plan",
    ] {
        assert!(
            script_source.contains(marker),
            "performance check script should include `{marker}`."
        );
    }
}

#[test]
fn overlays_check2_marks_semantics_and_performance_regression_contract_complete() {
    let check2_source = load_source("src/overlays/check2.md");

    for marker in [
        "- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。",
        "overlays_semantic_contract_tests_cover_matrix_and_do_not_rely_on_snapshots_only",
        "overlays_focus_stack_and_gc_use_global_focus_manager_contract",
        "overlays_performance_governance_contract_is_budgeted_traceable_and_blocking",
        "overlays_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement",
        "`render_count` 自动化回归仍在仓库统一 follow-up",
        "scripts/check-ui-performance.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(marker),
            "overlays check2 semantic/performance section should include `{marker}`."
        );
    }
}

#[test]
fn overlays_view_macro_complexity_is_bounded_by_semantic_subblocks() {
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let check2_source = load_source("src/overlays/check2.md");

    for needle in [
        "fn render_modal_title(",
        "fn render_modal_description(",
        "fn render_modal_body(",
        "fn render_modal_sections(",
        "{render_modal_title(",
        "{render_modal_body(",
    ] {
        assert!(
            modal_view.contains(needle),
            "modal view should keep semantic sub-block renderer `{needle}`."
        );
    }

    for needle in [
        "fn render_backdrop(",
        "fn render_panel(inputs: SheetPanelInputs) -> impl IntoView {",
        "{render_backdrop(",
        "{render_panel(",
    ] {
        assert!(
            sheet_view.contains(needle),
            "sheet view should keep semantic sub-block renderer `{needle}`."
        );
    }

    for needle in [
        "struct TrayPanelRenderInputs {",
        "fn render_tray_close_slot(",
        "fn render_tray_header_slot(",
        "fn render_tray_body_slot(",
        "fn render_tray_footer_slot(",
        "fn render_tray_panel(inputs: TrayPanelRenderInputs) -> AnyView {",
        "{panel}",
    ] {
        assert!(
            tray_view.contains(needle),
            "tray view should keep semantic sub-block renderer `{needle}`."
        );
    }

    for forbidden in ["if root_state.show_description {", "} else {"] {
        assert!(
            !tray_view.contains(forbidden),
            "tray view should avoid duplicated giant branch `{forbidden}`."
        );
    }

    assert_eq!(
        tray_view.matches("<Sheet").count(),
        1,
        "tray view should keep a single sheet wrapper instead of duplicated branches."
    );
    assert_eq!(
        tray_view
            .matches("data-custom-keyboard-dismiss=(root_state.keyboard_dismiss_source_attr == \"custom\").then_some(\"true\")")
            .count(),
        1,
        "tray root marker wiring should be emitted once after macro split."
    );

    for needle in [
        "- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。",
        "components/tray/src/view.rs",
        "`render_tray_panel`",
        "overlays_view_macro_complexity_is_bounded_by_semantic_subblocks",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 macro-complexity evidence should include `{needle}`."
        );
    }
}

#[test]
fn overlays_function_first_split_prefers_plain_render_functions_over_extra_components() {
    let overlays_view = load_source("src/view.rs");
    let overlay_view = load_source("../overlay/src/view.rs");
    let popover_view = load_source("../popover/src/view.rs");
    let modal_view = load_source("../modal/src/view.rs");
    let sheet_view = load_source("../sheet/src/view.rs");
    let tray_view = load_source("../tray/src/view.rs");
    let check2_source = load_source("src/overlays/check2.md");

    for (scope, source) in [
        ("OverlaysRoot", overlays_view.as_str()),
        ("Overlay", overlay_view.as_str()),
        ("Popover", popover_view.as_str()),
        ("Modal", modal_view.as_str()),
        ("Sheet", sheet_view.as_str()),
        ("Tray", tray_view.as_str()),
    ] {
        assert_eq!(
            source.matches("#[component]").count(),
            1,
            "{scope} view should expose only one top-level #[component], not fragment components."
        );
        assert!(
            !source.contains("#[component]\nfn render_")
                && !source.contains("#[component]\r\nfn render_"),
            "{scope} view should keep render fragments as plain functions, not #[component] noise."
        );
    }

    for needle in [
        "fn render_modal_title(",
        "fn render_modal_description(",
        "fn render_modal_body(",
        "fn render_modal_sections(",
    ] {
        assert!(
            modal_view.contains(needle),
            "modal should keep function-first render fragment `{needle}`."
        );
    }

    for needle in [
        "fn render_backdrop(",
        "fn render_panel(inputs: SheetPanelInputs) -> impl IntoView {",
    ] {
        assert!(
            sheet_view.contains(needle),
            "sheet should keep function-first render fragment `{needle}`."
        );
    }

    for needle in [
        "fn render_tray_close_slot(",
        "fn render_tray_header_slot(",
        "fn render_tray_body_slot(",
        "fn render_tray_footer_slot(",
        "fn render_tray_panel(inputs: TrayPanelRenderInputs) -> AnyView {",
    ] {
        assert!(
            tray_view.contains(needle),
            "tray should keep function-first render fragment `{needle}`."
        );
    }

    for needle in [
        "- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。",
        "`components/modal/src/view.rs`",
        "`components/sheet/src/view.rs`",
        "`components/tray/src/view.rs`",
        "`render_modal_title/render_modal_description/render_modal_body/render_modal_sections`",
        "`render_backdrop/render_panel`",
        "`render_tray_close_slot/render_tray_header_slot/render_tray_body_slot/render_tray_footer_slot/render_tray_panel`",
        "overlays_function_first_split_prefers_plain_render_functions_over_extra_components",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 function-first split evidence should include `{needle}`."
        );
    }
}

#[test]
fn overlays_static_fragments_are_constantized_with_accessible_svg_template() {
    let tray_view = load_source("../tray/src/view.rs");
    let check2_source = load_source("src/overlays/check2.md");

    for needle in [
        "const TRAY_CLOSE_ICON_VIEWBOX: &str = \"0 0 20 20\";",
        "const TRAY_CLOSE_ICON_PATH: &str = \"M5 5l10 10M15 5L5 15\";",
        "const TRAY_CLOSE_ICON_STROKE_WIDTH: &str = \"1.5\";",
        "fn render_tray_close_icon() -> AnyView {",
        "<svg viewBox=TRAY_CLOSE_ICON_VIEWBOX fill=\"none\" aria-hidden=\"true\">",
        "d=TRAY_CLOSE_ICON_PATH",
        "stroke_width=TRAY_CLOSE_ICON_STROKE_WIDTH",
        "{render_tray_close_icon()}",
    ] {
        assert!(
            tray_view.contains(needle),
            "tray view should keep constantized static fragment token `{needle}`."
        );
    }

    assert_eq!(
        tray_view.matches("TRAY_CLOSE_ICON_PATH").count(),
        2,
        "tray close icon path should be declared once and consumed once."
    );
    assert!(
        !tray_view.contains("d=\"M5 5l10 10M15 5L5 15\""),
        "tray close icon path literal should not be scattered inline after constantization."
    );

    for needle in [
        "- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。",
        "`components/tray/src/view.rs`",
        "`TRAY_CLOSE_ICON_VIEWBOX/TRAY_CLOSE_ICON_PATH/TRAY_CLOSE_ICON_STROKE_WIDTH`",
        "`render_tray_close_icon`",
        "overlays_static_fragments_are_constantized_with_accessible_svg_template",
    ] {
        assert!(
            check2_source.contains(needle),
            "check2 static-fragment evidence should include `{needle}`."
        );
    }
}

#[test]
fn overlays_inner_html_usage_is_forbidden_and_docs_shell_path_is_whitelisted() {
    for rel_path in [
        "src/mod.rs",
        "src/logic.rs",
        "src/view.rs",
        "src/styles.rs",
        "src/motion.rs",
        "src/README.md",
        "../overlay/src/mod.rs",
        "../overlay/src/logic.rs",
        "../overlay/src/view.rs",
        "../overlay/src/styles.rs",
        "../overlay/src/motion.rs",
        "../overlay/src/README.md",
        "../popover/src/mod.rs",
        "../popover/src/logic.rs",
        "../popover/src/view.rs",
        "../popover/src/styles.rs",
        "../popover/src/motion.rs",
        "../popover/src/README.md",
        "../modal/src/mod.rs",
        "../modal/src/logic.rs",
        "../modal/src/view.rs",
        "../modal/src/styles.rs",
        "../modal/src/motion.rs",
        "../modal/src/README.md",
        "../sheet/src/mod.rs",
        "../sheet/src/logic.rs",
        "../sheet/src/view.rs",
        "../sheet/src/styles.rs",
        "../sheet/src/motion.rs",
        "../tray/src/mod.rs",
        "../tray/src/logic.rs",
        "../tray/src/view.rs",
        "../tray/src/styles.rs",
        "../tray/src/motion.rs",
        "../tray/src/README.md",
        "../../apps/docs-app/src/pages/components/pages/overlays.rs",
        "../../apps/docs-app/src/pages/components/pages/overlays_extra.rs",
    ] {
        let source = load_source(rel_path);
        for forbidden in [
            "inner_html",
            "set_inner_html(",
            "dangerously_set_inner_html",
            "<script",
            "javascript:",
            "onerror=",
            "onload=",
        ] {
            assert!(
                !source.contains(forbidden),
                "overlays family path `{rel_path}` must not contain untrusted html injection token `{forbidden}`."
            );
        }
    }

    let shell_source = load_source("../../apps/docs-app/src/pages/components/shell.rs");
    for needle in [
        "fn component_readme_markdown(slug: &str) -> Option<&'static str> {",
        "include_str!(\"../../../../../components/accordion/src/README.md\")",
        "include_str!(\"../../../../../components/modal/src/README.md\")",
        "_ => None,",
        "let readme_html = component_readme_markdown(slug).map(crate::markdown::markdown_to_html);",
        "<div data-slot=\"component-readme\" inner_html=html></div>",
    ] {
        assert!(
            shell_source.contains(needle),
            "docs shell inner_html path should stay trusted/whitelisted via `{needle}`."
        );
    }

    let markdown_source = load_source("../../apps/docs-app/src/markdown.rs");
    for needle in [
        "pub fn markdown_to_html(markdown: &str) -> String {",
        "render_markdown(markdown).html",
    ] {
        assert!(
            markdown_source.contains(needle),
            "markdown conversion contract should keep `{needle}`."
        );
    }

    let check_script = load_source("../../scripts/check-ui-inner-html.sh");
    let script_needle = "cargo test -p ui-overlays overlays_inner_html_usage_is_forbidden_and_docs_shell_path_is_whitelisted";
    assert!(
        check_script.contains(script_needle),
        "inner-html gate script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated() {
    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let crate_root_source = load_source("../../crates/ui/src/lib.rs");
    let docs_app_source = load_source("../../apps/docs-app/src/lib.rs");
    let debug_overlay_source = load_source("../../apps/docs-app/src/debug_overlay.rs");
    let trace_source = load_source("../../crates/ui-headless/src/trace.rs");
    let controllable_state_source =
        load_source("../../crates/ui-headless/src/controllable_state.rs");
    let modal_view_source = load_source("../modal/src/view.rs");
    let modal_logic_source = load_source("../modal/src/logic.rs");
    let overlays_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let debug_overlay_e2e_source = load_source("../../e2e/tests/docs_app_debug_overlay.spec.mjs");

    for needle in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "sheet-wasm-debug = [\"component-sheet\", \"dep:tracing\"]",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui Cargo features should keep shared wasm-debug marker `{needle}`."
        );
    }
    for forbidden in [
        "overlay-wasm-debug",
        "overlays-wasm-debug",
        "popover-wasm-debug",
        "modal-wasm-debug",
        "tray-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "overlays family should not define component-local wasm-debug feature `{forbidden}`."
        );
    }

    for needle in [
        "macro_rules! wasm_debug_proxy",
        "pub(crate) use wasm_debug_proxy;",
    ] {
        assert!(
            crate_root_source.contains(needle),
            "ui root should keep wasm-debug isolation marker `{needle}`."
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
            "docs-app should expose dev-only wasm debug entry via `{needle}`."
        );
    }

    for needle in [
        "pub fn UiDebugOverlay(#[prop(optional)] enabled: bool) -> AnyView",
        "if !enabled {",
        "data-slot=\"ui-debug-overlay\"",
        "data-slot=\"ui-debug-overlay-events\"",
        "data-slot=\"ui-debug-overlay-event\"",
        "ui_headless::UiTraceEventKind::Inspect",
        "ui_headless::UiTraceEventKind::OpenChange { open }",
        "let ts_ms = event.ts_ms;",
        "format!(\"{ts_ms}ms\")",
    ] {
        assert!(
            debug_overlay_source.contains(needle),
            "debug overlay should keep visual/temporal trace marker `{needle}`."
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
            "ui-headless trace should keep typed timestamp/source event marker `{needle}`."
        );
    }

    for needle in [
        "pub fn use_controllable_open_state_traced(",
        "trace.emit(component, UiTraceEventKind::OpenChange { open: next });",
    ] {
        assert!(
            controllable_state_source.contains(needle),
            "ui-headless controllable state should emit open-change trace event via `{needle}`."
        );
    }

    for needle in [
        "ui_headless::use_controllable_open_state_traced(",
        "let open_state = ui_headless::use_controllable_open_state_traced(",
        "\"modal\",",
        "data-open-mode=open_contract.mode.as_attr()",
        "data-open-source=open_contract.open_source.as_attr()",
        "data-open-change-source=open_contract.open_change_source.as_attr()",
        "data-open-prop-source=open_contract.open_prop_source.as_attr()",
    ] {
        assert!(
            modal_view_source.contains(needle),
            "Modal should expose reproducible interaction/state markers for debug tracing via `{needle}`."
        );
    }

    for needle in [
        "title=\"State + Source Markers\"",
        "<Button on_press=open_custom_modal>",
        "open: \" {move || open_custom_raw.get()}",
        "on_open_change=on_controlled_open_change",
        "Inspect data-state / data-open-mode / data-*-source markers.",
    ] {
        assert!(
            overlays_docs_source.contains(needle),
            "overlays docs playground should keep minimal replay path marker `{needle}`."
        );
    }

    for needle in [
        "debug overlay captures traced open/close events",
        "[data-slot=\"ui-debug-overlay-event\"][data-component=\"date-picker\"][data-kind=\"open-change\"]",
        ").toHaveCount(2);",
    ] {
        assert!(
            debug_overlay_e2e_source.contains(needle),
            "debug overlay e2e should keep replay/timeline contract marker `{needle}`."
        );
    }

    let combined = format!("{modal_view_source}\n{modal_logic_source}");
    for forbidden in [
        "wasm_debug_proxy!",
        "observability::",
        "data-debug-source=",
        "data-debug-before=",
        "data-debug-after=",
        "data-debug-timestamp-ms=",
        "request_replay",
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "#[prop(optional)] debug",
    ] {
        assert!(
            !combined.contains(forbidden),
            "overlays component contracts should not leak wasm-debug internals `{forbidden}`."
        );
    }

    let wasm_debug_script = load_source("../../scripts/check-ui-wasm-debug.sh");
    let script_needle = "cargo test -p ui-overlays overlays_wasm_debug_contract_reuses_global_trace_overlay_and_stays_feature_isolated";
    assert!(
        wasm_debug_script.contains(script_needle),
        "wasm-debug check script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench() {
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");
    let overlays_docs_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let dx_script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "fn compose_scoped_css(scope_selector: &str, raw: &str) -> String {",
        "<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>",
        "on:input=move |ev| set_test_css.set(event_target_value(&ev))",
        "\"Show test\"",
        "\"Restore original CSS\"",
        "data-playground-scope=scope_id.clone()",
        "data-slot=\"playground-test\"",
    ] {
        assert!(
            playground_source.contains(needle),
            "playground hot-reload contract should include `{needle}`."
        );
    }

    for needle in [
        "title=\"Workbench (Display + Config + Code + CSS Test)\"",
        "data-slot=\"popover-workbench-controls\"",
        "test_css_source=workbench_test_css",
        "test_config_signal=workbench_actual_config",
        "\"Toggle workbench open\"",
        "\"open: \" {move || workbench_open_raw.get()}",
        "on_exit_complete=workbench_on_exit_complete",
        "title=\"Interactive Playground\"",
        "test_css_source=interactive_test_css",
        "test_config_signal=interactive_config",
        "Inspect root markers in DevTools while toggling config.",
    ] {
        assert!(
            overlays_docs_source.contains(needle),
            "overlays docs should keep DX workbench/context marker `{needle}`."
        );
    }

    for forbidden in [
        "MODAL_WORKBENCH_STORAGE_KEY",
        "load_modal_workbench_state",
        "save_modal_workbench_state",
        "window.localStorage",
    ] {
        assert!(
            !overlays_docs_source.contains(forbidden),
            "overlays docs should keep optional persisted state as N/A without leaking `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_dx_playground_supports_css_hot_reload_and_context_preserving_isolated_workbench";
    assert!(
        dx_script_source.contains(script_needle),
        "dx check script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries() {
    let cargo_source = load_source("../../crates/ui/Cargo.toml");
    let button_view_source = load_source("../../components/button/src/view.rs");
    let engineering_script_source = load_source("../../scripts/check-ui-engineering.sh");

    let protocol_sources = [
        ("Overlays", load_source("src/protocol.rs")),
        ("Overlay", load_source("../overlay/src/protocol.rs")),
        ("Popover", load_source("../popover/src/protocol.rs")),
        ("Modal", load_source("../modal/src/protocol.rs")),
        ("Tray", load_source("../tray/src/protocol.rs")),
        ("Sheet", load_source("../sheet/src/protocol.rs")),
    ];

    for (scope, source) in protocol_sources {
        for needle in [
            "use serde::{Deserialize, Serialize};",
            "#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]",
            "#[serde(rename_all = \"snake_case\")]",
            "#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]",
            "#[serde(default)]",
            "pub schema_version:",
        ] {
            assert!(
                source.contains(needle),
                "{scope} protocol should keep serde/schema contract marker `{needle}`."
            );
        }

        for forbidden in [
            "serde_json::",
            "from_json(",
            "to_json_result(",
            "SchemaError",
        ] {
            assert!(
                !source.contains(forbidden),
                "{scope} protocol should avoid ad-hoc serde drift token `{forbidden}`."
            );
        }
    }

    for required in [
        "button-wasm-debug = [\"component-button\", \"dep:tracing\"]",
        "accordion-wasm-debug = [\"component-accordion\", \"dep:tracing\"]",
        "target: \"ui::button::state_change\"",
    ] {
        assert!(
            cargo_source.contains(required) || button_view_source.contains(required),
            "engineering baseline should keep canonical tracing marker `{required}`."
        );
    }

    for forbidden in [
        "overlay-wasm-debug",
        "overlays-wasm-debug",
        "popover-wasm-debug",
        "modal-wasm-debug",
        "tray-wasm-debug",
    ] {
        assert!(
            !cargo_source.contains(forbidden),
            "overlays family should not define component-local tracing feature alias `{forbidden}`."
        );
    }

    let combined_sources = [
        load_source("src/mod.rs"),
        load_source("src/logic.rs"),
        load_source("src/view.rs"),
        load_source("src/styles.rs"),
        load_source("src/motion.rs"),
        load_source("src/protocol.rs"),
        load_source("../overlay/src/mod.rs"),
        load_source("../overlay/src/logic.rs"),
        load_source("../overlay/src/view.rs"),
        load_source("../overlay/src/styles.rs"),
        load_source("../overlay/src/motion.rs"),
        load_source("../overlay/src/protocol.rs"),
        load_source("../popover/src/mod.rs"),
        load_source("../popover/src/logic.rs"),
        load_source("../popover/src/view.rs"),
        load_source("../popover/src/styles.rs"),
        load_source("../popover/src/motion.rs"),
        load_source("../popover/src/protocol.rs"),
        load_source("../modal/src/mod.rs"),
        load_source("../modal/src/logic.rs"),
        load_source("../modal/src/view.rs"),
        load_source("../modal/src/styles.rs"),
        load_source("../modal/src/motion.rs"),
        load_source("../modal/src/protocol.rs"),
        load_source("../sheet/src/mod.rs"),
        load_source("../sheet/src/logic.rs"),
        load_source("../sheet/src/view.rs"),
        load_source("../sheet/src/styles.rs"),
        load_source("../sheet/src/motion.rs"),
        load_source("../sheet/src/protocol.rs"),
        load_source("../tray/src/mod.rs"),
        load_source("../tray/src/logic.rs"),
        load_source("../tray/src/view.rs"),
        load_source("../tray/src/styles.rs"),
        load_source("../tray/src/motion.rs"),
        load_source("../tray/src/protocol.rs"),
    ]
    .join("\n");

    for forbidden in [
        "tracing::span!(",
        "tracing::event!(",
        "#[tracing::instrument]",
        "target: \"ui::overlay::",
        "target: \"ui::overlays::",
        "target: \"ui::popover::",
        "target: \"ui::modal::",
        "target: \"ui::sheet::",
        "target: \"ui::tray::",
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
            !combined_sources.contains(forbidden),
            "overlays engineering contract should not leak tracing/runtime marker `{forbidden}`."
        );
    }

    let script_needle = "cargo test -p ui-overlays overlays_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries";
    assert!(
        engineering_script_source.contains(script_needle),
        "engineering check script should enforce `{script_needle}`."
    );
}

#[test]
fn overlays_version_deprecation_migration_is_na_without_major_breaking_upgrade() {
    let manifest_source = load_source("src/Component.toml");
    let rbi_source = load_source("src/overlays.rbi");
    let mod_source = load_source("src/mod.rs");
    let logic_source = load_source("src/logic.rs");
    let view_source = load_source("src/view.rs");
    let styles_source = load_source("src/styles.rs");
    let motion_source = load_source("src/motion.rs");
    let protocol_source = load_source("src/protocol.rs");
    let check2_source = load_source("src/overlays/check2.md");

    for needle in [
        "schema_version = \"1\"",
        "name = \"OverlaysRoot\"",
        "crate = \"ui-overlays\"",
    ] {
        assert!(
            manifest_source.contains(needle),
            "overlays manifest should keep stable v1 schema marker `{needle}`."
        );
    }

    for needle in [
        "pub fn OverlaysRoot(",
        "is_open: bool,",
        "is_modal: bool,",
        "aria_label: Option<String>,",
        "class_name: Option<String>,",
    ] {
        assert!(
            rbi_source.contains(needle),
            "overlays RBI should keep stable public API marker `{needle}`."
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
            "overlays should not introduce major-version migration marker `{forbidden}` in current scope."
        );
    }

    for needle in [
        "- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。",
        "N/A：本次 `Overlays` 未发生跨大版本 API 破坏升级",
        "schema_version = \"1\"",
        "overlays_version_deprecation_migration_is_na_without_major_breaking_upgrade",
        "scripts/check-ui-engineering.sh",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2_source.contains(needle),
            "overlays/check2.md should keep version-migration governance marker `{needle}`."
        );
    }
}

#[test]
fn overlays_version_deprecation_migration_script_covers_engineering_gate() {
    let script_source = load_source("../../scripts/check-ui-engineering.sh");

    let marker = "cargo test -p ui-overlays overlays_version_deprecation_migration_is_na_without_major_breaking_upgrade";
    assert!(
        script_source.contains(marker),
        "engineering check script should enforce `{marker}`."
    );
}

#[test]
fn overlays_tree_shaking_contract_is_feature_gated_and_budget_guarded() {
    let cargo_source = load_source("Cargo.toml");
    let lib_source = load_source("src/lib.rs");
    let css_source = load_source("../../crates/ui/src/css.rs");
    let web_demo_manifest = load_source("../../apps/web-demo/Cargo.toml");
    let docs_app_manifest = load_source("../../apps/docs-app/Cargo.toml");
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");
    let ci_workflow = load_source("../../.github/workflows/ci.yml");

    for needle in [
        "component-overlay = []",
        "component-overlays = [",
        "\"component-overlay\"",
        "\"component-popover\"",
        "\"component-modal\"",
        "\"component-tray\"",
        "\"component-sheet\"",
        "\"component-button\"",
    ] {
        assert!(
            cargo_source.contains(needle),
            "ui feature graph should include overlays tree-shaking token `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"component-overlay\")]",
        "pub mod overlay;",
        "#[cfg(feature = \"component-overlays\")]",
        "#[path = \"../../../components/overlays/src/mod.rs\"]",
        "pub mod overlays;",
        "#[cfg(feature = \"component-popover\")]",
        "pub use ui_popover as popover;",
        "#[cfg(feature = \"component-modal\")]",
        "pub mod modal;",
        "#[cfg(feature = \"component-sheet\")]",
        "pub use ui_sheet as sheet;",
        "#[cfg(feature = \"component-tray\")]",
        "pub use ui_tray as tray;",
    ] {
        assert!(
            lib_source.contains(needle),
            "crate root should keep overlays exports feature-gated via `{needle}`."
        );
    }

    for needle in [
        "#[cfg(feature = \"inject-css\")]",
        "#[cfg(feature = \"component-overlay\")]",
        "out.push_str(crate::overlay::styles::CSS);",
        "#[cfg(feature = \"component-overlays\")]",
        "out.push_str(crate::overlays::styles::CSS);",
        "#[cfg(feature = \"component-popover\")]",
        "out.push_str(crate::popover::styles::CSS);",
        "#[cfg(feature = \"component-modal\")]",
        "out.push_str(crate::modal::styles::CSS);",
        "#[cfg(feature = \"component-sheet\")]",
        "out.push_str(crate::sheet::styles::CSS);",
        "#[cfg(feature = \"component-tray\")]",
        "out.push_str(crate::tray::styles::CSS);",
    ] {
        assert!(
            css_source.contains(needle),
            "css aggregation should keep overlays feature gates via `{needle}`."
        );
    }

    for forbidden in [
        "ALL_COMPONENTS_MAP",
        "component_registry",
        "static COMPONENTS:",
    ] {
        assert!(
            !lib_source.contains(forbidden) && !css_source.contains(forbidden),
            "ui should not expose always-reachable registry token `{forbidden}`."
        );
    }

    assert!(
        web_demo_manifest.contains("default-features = false")
            && web_demo_manifest.contains("\"web-demo-components\"")
            && !web_demo_manifest.contains("\"all-components\""),
        "web-demo should keep source-mode feature pruning via web-demo-components without all-components."
    );
    assert!(
        docs_app_manifest.contains("default-features = false")
            && docs_app_manifest.contains("\"all-components\""),
        "docs-app should opt in to all-components explicitly for full catalog coverage."
    );

    for needle in [
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$MIN_FEATURES\"",
        "if grep -q 'all-components' <<<\"$MIN_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p web-demo",
        "if grep -q 'all-components' <<<\"$WEB_DEMO_TREE_OUTPUT\"; then",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$MIN_FEATURES\"",
        "cargo build -p ui --target wasm32-unknown-unknown --release --no-default-features --features \"$MIN_FEATURES\"",
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking verification pipeline should include `{needle}`."
        );
    }

    assert!(
        ci_workflow.contains("name: Tree Shaking Budget")
            && ci_workflow.contains("run: ./scripts/check-ui-tree-shaking.sh"),
        "CI should run the tree-shaking budget guard script."
    );
}

#[test]
fn overlays_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget() {
    let tree_shaking_script = load_source("../../scripts/check-ui-tree-shaking.sh");
    let tree_shaking_budget = load_source("../../scripts/tree_shaking_budget.env");

    for needle in [
        "OVERLAYS_MIN_FEATURES=\"component-overlays,inject-css\"",
        "cargo test -p ui-overlays overlays_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "cargo test -p ui-overlays overlays_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "cargo test -p ui-overlays overlays_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "OVERLAYS_TREE_OUTPUT",
        "if ! grep -q 'feature \"component-overlays\" (command-line)' <<<\"$OVERLAYS_TREE_OUTPUT\"; then",
        "if ! grep -q 'feature \"inject-css\" (command-line)' <<<\"$OVERLAYS_TREE_OUTPUT\"; then",
        "if grep -q 'all-components' <<<\"$OVERLAYS_TREE_OUTPUT\"; then",
        "cargo tree -e features -i ui -p ui --no-default-features --features \"$OVERLAYS_MIN_FEATURES\"",
        "cargo tree -e features -i ui -p web-demo",
        "cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features \"$OVERLAYS_MIN_FEATURES\"",
    ] {
        assert!(
            tree_shaking_script.contains(needle),
            "tree-shaking script should enforce overlays feature-pruning guard `{needle}`."
        );
    }

    for needle in [
        "TREE_SHAKING_BASELINE_RLIB_BYTES",
        "TREE_SHAKING_MAX_RATIO_PERCENT",
    ] {
        assert!(
            tree_shaking_script.contains(needle) || tree_shaking_budget.contains(needle),
            "tree-shaking pipeline should keep budget guard token `{needle}`."
        );
    }
}

#[test]
fn overlays_check2_marks_tree_shaking_feature_pruning_contract_complete() {
    let check2 = load_source("src/overlays/check2.md");

    assert!(
        check2.contains("- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。"),
        "check2 should mark tree-shaking feature-pruning checklist item as completed."
    );

    for needle in [
        "`component-overlays`",
        "`crates/ui/src/lib.rs`",
        "`crates/ui/src/css.rs`",
        "`scripts/check-ui-tree-shaking.sh`",
        "overlays_tree_shaking_contract_is_feature_gated_and_budget_guarded",
        "overlays_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget",
        "overlays_check2_marks_tree_shaking_feature_pruning_contract_complete",
        "cargo tree -e features -i ui -p ui --no-default-features --features component-overlays,inject-css",
        "Invalid cross-device link (os error 18)",
    ] {
        assert!(
            check2.contains(needle),
            "check2 tree-shaking feature-pruning entry should reference `{needle}`."
        );
    }
}

#[test]
fn crate_root_registers_overlays_module() {
    let source = load_source("src/lib.rs");

    assert!(
        source.contains("#[path = \"../../../components/overlays/src/mod.rs\"]")
            && source.contains("pub mod overlays;"),
        "crate root should include path-bound `pub mod overlays;` for @ui-baseline/overlays compatibility."
    );
}

#[test]
fn overlays_compatibility_reuses_overlay_popover_modal_tray_docs_playgrounds() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Overlay\"",
        "slug=\"overlay\"",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "title=\"Modal\"",
        "slug=\"modal\"",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs should contain `{needle}` for overlays compatibility coverage."
        );
    }

    for needle in ["title=\"Tray\"", "slug=\"tray\"", "<Tray"] {
        assert!(
            overlays_extra_source.contains(needle),
            "overlays_extra docs should contain `{needle}` for Tray compatibility coverage."
        );
    }
}

#[test]
#[ignore = "TODO: contract migration follow-up"]
fn overlays_module_docs_page_covers_primary_playgrounds() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");
    let mod_source = load_source("../../apps/docs-app/src/pages/components/mod.rs");

    for needle in [
        "pub(super) fn overlay() -> AnyView",
        "title=\"Overlay\"",
        "slug=\"overlay\"",
        "<Playground title=\"Overlay presence\" code_signal=code>",
        "title=\"State + Source Markers\"",
        "pub(super) fn popover() -> AnyView",
        "title=\"Popover\"",
        "slug=\"popover\"",
        "<Playground title=\"Popover\" code_signal=code>",
        "pub(super) fn modal() -> AnyView",
        "title=\"Modal\"",
        "slug=\"modal\"",
        "<Playground title=\"Label + Description\" code_signal=semantic_code>",
        "<Overlay",
        "<Popover",
        "<Modal",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs should include `{needle}` for overlays module primary playground coverage.",
        );
    }

    for needle in [
        "pub(super) fn tray() -> AnyView",
        "title=\"Tray\"",
        "slug=\"tray\"",
        "<Playground title=\"Tray + Footer Actions\" code_signal=semantic_code>",
        "title=\"State + Source Markers\"",
        "<Tray",
    ] {
        assert!(
            overlays_extra_source.contains(needle),
            "overlays_extra docs should include `{needle}` for overlays module tray primary playground coverage.",
        );
    }

    assert!(
        mod_source.contains("\"overlays\" => &[\"overlay\", \"popover\", \"modal\", \"tray\"]"),
        "components mod mapping should keep `overlays` mapped to overlay/popover/modal/tray slugs.",
    );
}

#[test]
fn overlays_module_docs_playgrounds_lock_state_matrix_contract_values() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let overlays_extra_source =
        load_source("../../apps/docs-app/src/pages/components/pages/overlays_extra.rs");

    for needle in [
        "title=\"Overlay presence\"",
        "<Button on_press=open_overlay>\"Open overlay\"</Button>",
        "<Overlay open=open on_close=on_close on_exit_complete=on_exit_complete>",
        "title=\"State + Source Markers\"",
        "role=\"alertdialog\"",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "motion=marker_motion",
        "class_name=\"docs-overlay-state\".to_string()",
        "aria_labelledby=\"overlay-marker-title\".to_string()",
        "aria_describedby=\"overlay-marker-desc\".to_string()",
        "on_exit_complete=on_marker_exit_complete",
        "title=\"Popover\"",
        "anchor_ref=anchor_ref",
        "on_exit_complete=on_exit_complete",
        "motion=custom_motion",
        "is_modal=false",
        "class_name=\"docs-popover-state\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
        "title=\"Label + Description\"",
        "id_base=\"docs-modal-semantic\".to_string()",
        "description=\"Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.\".to_string()",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-modal-custom\".to_string()",
        "class_name=\"docs-modal-custom\".to_string()",
        "motion=custom_motion",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs playgrounds should contain `{needle}` for overlays module contracts.",
        );
    }

    for needle in [
        "title=\"Tray + Footer Actions\"",
        "id_base=\"docs-tray-semantic\".to_string()",
        "description=\"Tray composes Sheet with title/description wiring and footer action slots.\".to_string()",
        "title=\"State + Source Markers\"",
        "id_base=\"docs-tray-fixed\".to_string()",
        "motion=custom_motion",
        "is_fixed_height=true",
        "is_dismissable=false",
        "is_keyboard_dismiss_disabled=true",
        "show_close_button=false",
        "class_name=\"docs-tray-custom\".to_string()",
        "on_exit_complete=on_custom_exit_complete",
    ] {
        assert!(
            overlays_extra_source.contains(needle),
            "overlays_extra docs playgrounds should contain `{needle}` for overlays module tray contracts.",
        );
    }
}

#[test]
fn overlays_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");

    for needle in [
        "title=\"Hello World (Minimal Path)\"",
        "title=\"Hello World (Minimal API)\"",
        "title=\"State Matrix\"",
        "description=\"State matrix over controlled/uncontrolled + default_open + description branches.\"",
        "data-slot=\"modal-state-matrix\"",
        "data-slot=\"drawer-state-matrix\"",
        "title=\"Controlled vs Uncontrolled\"",
        "data-slot=\"modal-controlled-uncontrolled\"",
        "data-slot=\"drawer-controlled-uncontrolled\"",
        "title=\"Streaming / Snapshot Contract\"",
        "data-slot=\"modal-streaming-contract\"",
        "data-slot=\"drawer-streaming-contract\"",
        "stream_mode_options = vec![",
        "\"Snapshot\".to_string(),",
        "\"Streaming (fallback=snapshot)\".to_string(),",
        "code_imports=MODAL_DOC_IMPORTS.to_string()",
        "code_imports=DRAWER_DOC_IMPORTS.to_string()",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs should keep `{needle}` for copy-paste-ready docs contract."
        );
    }
}

#[test]
fn overlays_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies() {
    let overlays_source = load_source("../../apps/docs-app/src/pages/components/pages/overlays.rs");
    let playground_source = load_source("../../apps/docs-app/src/playground.rs");

    for needle in [
        "data-slot=\"modal-source-first\"",
        "data-slot=\"drawer-source-first\"",
        "<h3>\"Source-first Copy-Paste\"</h3>",
        "<code>\"Show code\"</code>",
        "<code>\"MODAL_DOC_IMPORTS\"</code>",
        "<code>\"DRAWER_DOC_IMPORTS\"</code>",
        "<code>\"compose_copy_ready_code\"</code>",
        "ui = { workspace = true, default-features = false, features = [\"component-modal\", \"inject-css\"] }",
        "ui = { workspace = true, default-features = false, features = [\"component-drawer\", \"inject-css\"] }",
        "components/modal/src/mod.rs",
        "components/modal/src/logic.rs",
        "components/modal/src/view.rs",
        "components/modal/src/styles.rs",
        "components/modal/src/motion.rs",
        "components/drawer/src/mod.rs",
        "components/drawer/src/logic.rs",
        "components/drawer/src/view.rs",
        "components/drawer/src/styles.rs",
        "components/drawer/src/motion.rs",
    ] {
        assert!(
            overlays_source.contains(needle),
            "overlays docs source-first section should include `{needle}`."
        );
    }

    for needle in [
        "fn compose_copy_ready_code(raw: &str, imports: &str) -> String {",
        "code_imports: Option<String>,",
        "compose_copy_ready_code(&dynamic_code.get(), &code_imports.get_value())",
        "compose_copy_ready_code(&snippet, &code_imports.get_value())",
        "{move || if show_code_panel.get() { \"Hide code\" } else { \"Show code\" }}",
    ] {
        assert!(
            playground_source.contains(needle),
            "docs playground should keep import-aware copy-ready path via `{needle}`."
        );
    }
}

#[test]
fn overlays_dx_check_script_covers_docs_product_copy_paste_ready_contract() {
    let script_source = load_source("../../scripts/check-ui-dx.sh");

    for needle in [
        "cargo test -p ui-overlays overlays_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot",
        "cargo test -p ui-overlays overlays_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies",
    ] {
        assert!(
            script_source.contains(needle),
            "dx check script should enforce overlays docs product contract via `{needle}`."
        );
    }
}

#[test]
fn overlays_check2_has_no_unchecked_items_after_verification() {
    let source = load_source("src/overlays/check2.md");

    assert!(
        !source.contains("- [ ]"),
        "overlays/check2.md should not keep unchecked checklist items after completion."
    );
}
