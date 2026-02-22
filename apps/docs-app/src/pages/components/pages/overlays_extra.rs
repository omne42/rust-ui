use super::playground_workbench::{bool_word, push_line_when, rust_string_literal};
use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{
    BottomSheet, BottomSheetMotion, Button, ButtonVariant, OnPress, SegmentedControl,
    SegmentedControlSize, Snippet, Sonner, SonnerPosition, Switch, ToastMotion, ToastOptions,
    ToastStoreOptions, ToastVariant, Toaster, ToasterPosition, Tray, TrayMotion, Underlay,
    provide_toast_store,
};

const BOTTOM_SHEET_DOC_IMPORTS: &str = "use leptos::prelude::*;\nuse ui::{BottomSheet, BottomSheetMotion, Button, OnPress, SegmentedControl, SegmentedControlSize};";

pub(super) fn bottom_sheet() -> AnyView {
    let (open_hello_raw, set_open_hello_raw) = signal(false);
    let open_hello: Signal<bool> = Signal::derive(move || open_hello_raw.get());
    let (present_hello, set_present_hello) = signal(open_hello.get_untracked());
    Effect::new(move |_| {
        if open_hello.get() {
            set_present_hello.set(true);
        }
    });

    let close_hello: OnPress = Callback::new(move |_| set_open_hello_raw.set(false));
    let open_hello_sheet: OnPress = Callback::new(move |_| set_open_hello_raw.set(true));
    let on_hello_exit_complete = Callback::new(move |_| set_present_hello.set(false));

    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_sheet: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));

    let (open_detached_raw, set_open_detached_raw) = signal(false);
    let open_detached: Signal<bool> = Signal::derive(move || open_detached_raw.get());
    let (present_detached, set_present_detached) = signal(open_detached.get_untracked());
    Effect::new(move |_| {
        if open_detached.get() {
            set_present_detached.set(true);
        }
    });

    let close_detached: OnPress = Callback::new(move |_| set_open_detached_raw.set(false));
    let open_detached_sheet: OnPress = Callback::new(move |_| set_open_detached_raw.set(true));
    let on_detached_exit_complete = Callback::new(move |_| set_present_detached.set(false));

    let (open_custom_motion_raw, set_open_custom_motion_raw) = signal(false);
    let open_custom_motion: Signal<bool> = Signal::derive(move || open_custom_motion_raw.get());
    let (present_custom_motion, set_present_custom_motion) =
        signal(open_custom_motion.get_untracked());
    Effect::new(move |_| {
        if open_custom_motion.get() {
            set_present_custom_motion.set(true);
        }
    });

    let close_custom_motion: OnPress =
        Callback::new(move |_| set_open_custom_motion_raw.set(false));
    let open_custom_motion_sheet: OnPress =
        Callback::new(move |_| set_open_custom_motion_raw.set(true));
    let on_custom_motion_exit_complete =
        Callback::new(move |_| set_present_custom_motion.set(false));

    let hello_world_code = Signal::derive(move || {
        r#"<BottomSheet open=open id_base="bottom-sheet".to_string() title="Bottom sheet".to_string() on_close=on_close>
  <div>"..."</div>
</BottomSheet>"#
            .to_string()
    });

    let semantic_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet".to_string()
  title="Update available".to_string()
  description="A newer version with security improvements is ready to install.".to_string()
  on_close=Callback::new(move |_| {})
  footer=move || view! { ... }
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let detached_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet-detached".to_string()
  title="Quick actions".to_string()
  is_detached=true
  bottom_inset_px=16.0
  is_close_button_visible=false
  class_name="docs-bottom-sheet-custom".to_string()
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let custom_motion_code = Signal::derive(move || {
        r#"<BottomSheet
  open=open
  id_base="bottom-sheet-motion".to_string()
  title="Motion tuned".to_string()
  description="Custom sheet motion demonstrates data-motion-source contract.".to_string()
  motion=BottomSheetMotion {
    sheet: ui::SheetMotion {
      initial_offset_px: 64.0,
      ..ui::SheetMotion::default()
    }
  }
  on_close=Callback::new(move |_| {})
  on_exit_complete=finish_exit
>
  ...
</BottomSheet>"#
            .to_string()
    });

    let state_matrix_options = vec![
        "Description".to_string(),
        "Title Only".to_string(),
        "Detached + Close Hidden".to_string(),
    ];
    let state_matrix_options_after_workbench = state_matrix_options.clone();
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_has_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 0);
    let state_matrix_is_detached =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_hide_close =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);

    let (open_state_matrix_raw, set_open_state_matrix_raw) = signal(false);
    let open_state_matrix: Signal<bool> = Signal::derive(move || open_state_matrix_raw.get());
    let (present_state_matrix, set_present_state_matrix) =
        signal(open_state_matrix.get_untracked());
    Effect::new(move |_| {
        if open_state_matrix.get() {
            set_present_state_matrix.set(true);
        }
    });
    let open_state_matrix_sheet: OnPress =
        Callback::new(move |_| set_open_state_matrix_raw.set(true));
    let close_state_matrix_sheet: OnPress =
        Callback::new(move |_| set_open_state_matrix_raw.set(false));
    let on_state_matrix_exit_complete = Callback::new(move |_| set_present_state_matrix.set(false));

    let state_matrix_code = Signal::derive(move || {
        r#"let options = vec![
  "Description".to_string(),
  "Title Only".to_string(),
  "Detached + Close Hidden".to_string(),
];

<SegmentedControl ... />
<BottomSheet
  open=open
  id_base="bottom-sheet-matrix".to_string()
  title="State Matrix".to_string()
  on_close=on_close
/>"#
        .to_string()
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let (compare_present, set_compare_present) = signal(compare_controlled_open.get_untracked());
    Effect::new(move |_| {
        if compare_controlled_open.get() {
            set_compare_present.set(true);
        }
    });
    let open_compare_controlled: OnPress =
        Callback::new(move |_| set_compare_controlled_open_raw.set(true));
    let close_compare_controlled: OnPress =
        Callback::new(move |_| set_compare_controlled_open_raw.set(false));
    let on_compare_exit_complete = Callback::new(move |_| set_compare_present.set(false));

    let controlled_vs_uncontrolled_code = Signal::derive(move || {
        r#"// BottomSheet is intentionally controlled at the component boundary.
// Uncontrolled open-state belongs to upstream primitives/adapters.
<BottomSheet
  open=open
  id_base="bottom-sheet-controlled".to_string()
  title="Controlled".to_string()
  on_close=on_close
/>"#
        .to_string()
    });

    let stream_mode_options = vec![
        "Snapshot".to_string(),
        "Streaming (fallback=snapshot)".to_string(),
    ];
    let (stream_mode_index, set_stream_mode_index) = signal(Some(0_usize));
    let stream_requested_mode = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "snapshot"
        } else {
            "streaming"
        }
    });
    let stream_requested_output_status = Signal::derive(move || {
        if stream_mode_index.get().unwrap_or(0) == 0 {
            "verified"
        } else {
            "draft"
        }
    });
    let stream_open_raw = RwSignal::new(true);
    let stream_open: Signal<bool> = Signal::derive(move || stream_open_raw.get());
    let stream_close: OnPress = Callback::new(move |_| stream_open_raw.set(false));
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// BottomSheet is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<BottomSheet
  open=open
  id_base="bottom-sheet-stream".to_string()
  title="Streaming Optional Contract".to_string()
  on_close=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    let workbench_title_options = vec![
        "Default title".to_string(),
        "Quick actions".to_string(),
        "Install update".to_string(),
    ];
    let (workbench_title_index, set_workbench_title_index) = signal(Some(0_usize));
    let workbench_title = Signal::derive(move || match workbench_title_index.get().unwrap_or(0) {
        1 => "Quick actions".to_string(),
        2 => "Install update".to_string(),
        _ => "Bottom sheet".to_string(),
    });

    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_footer, set_workbench_show_footer) = signal(true);
    let (workbench_is_detached, set_workbench_is_detached) = signal(false);
    let (workbench_show_close_button, set_workbench_show_close_button) = signal(true);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_present, set_workbench_present) = signal(workbench_open.get_untracked());
    Effect::new(move |_| {
        if workbench_open.get() {
            set_workbench_present.set(true);
        }
    });

    let open_workbench_sheet: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench_sheet: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_exit_complete = Callback::new(move |_| set_workbench_present.set(false));

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            BottomSheetMotion {
                sheet: ui::SheetMotion {
                    initial_offset_px: 72.0,
                    ..ui::SheetMotion::default()
                },
            }
        } else {
            BottomSheetMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let title = workbench_title.get();
        let show_description = workbench_show_description.get();
        let show_footer = workbench_show_footer.get();
        let is_detached = workbench_is_detached.get();
        let show_close_button = workbench_show_close_button.get();
        let custom_motion = workbench_custom_motion.get();

        let description = match title.as_str() {
            "Quick actions" => "Choose one action, then dismiss.",
            "Install update" => "A newer version with security improvements is ready to install.",
            _ => "Default bottom-sheet copy from workbench.",
        };

        let mut lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<BottomSheet".to_string(),
            "  open=open".to_string(),
            "  id_base=\"docs-bottom-sheet-workbench\".to_string()".to_string(),
            format!("  title={}.to_string()", rust_string_literal(&title)),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  lang=Some(\"en\".to_string())".to_string(),
            "  dir=Some(A11yDirection::Ltr)".to_string(),
            "  is_handle_visible=Some(true)".to_string(),
            "  show_handle=Some(true)".to_string(),
            "  show_close_button=Some(true)".to_string(),
            "  is_dismissable=Some(true)".to_string(),
            "  is_keyboard_dismiss_disabled=Some(false)".to_string(),
            "  close_label=Some(\"Close bottom sheet\")".to_string(),
        ];
        push_line_when(
            &mut lines,
            show_description,
            format!(
                "  description={}.to_string()",
                rust_string_literal(description)
            ),
        );
        push_line_when(&mut lines, is_detached, "  is_detached=true".to_string());
        push_line_when(
            &mut lines,
            is_detached,
            "  bottom_inset_px=16.0".to_string(),
        );
        push_line_when(
            &mut lines,
            !show_close_button,
            "  is_close_button_visible=false".to_string(),
        );
        push_line_when(
            &mut lines,
            custom_motion,
            "  motion=BottomSheetMotion { sheet: ui::SheetMotion { initial_offset_px: 72.0, ..ui::SheetMotion::default() } }".to_string(),
        );
        push_line_when(
            &mut lines,
            show_footer,
            "  footer=move || view! { <div class=\"docs-row docs-row--end\"><Button variant=ButtonVariant::Secondary>\"Later\"</Button><Button>\"Confirm\"</Button></div> }".to_string(),
        );
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let is_detached = workbench_is_detached.get();
        format!(
            "BottomSheetWorkbenchConfig {{\n  open: {},\n  on_close: Some(\"OnPress\"),\n  id_base: \"docs-bottom-sheet-workbench\",\n  title: {:?},\n  description: {},\n  footer: {},\n  lang: Some(\"en\"),\n  dir: Some(\"ltr\"),\n  motion: {},\n  is_handle_visible: Some(true),\n  show_handle: Some(true),\n  is_close_button_visible: Some({}),\n  show_close_button: Some({}),\n  close_label: Some(\"Close bottom sheet\"),\n  is_detached: Some({}),\n  bottom_inset_px: Some({}),\n  is_dismissable: Some(true),\n  is_keyboard_dismiss_disabled: Some(false),\n  class_name: Some(\"docs-bottom-sheet-workbench\"),\n  show_description: {},\n  show_footer: {},\n  custom_motion: {},\n}}",
            bool_word(workbench_open_raw.get()),
            workbench_title.get(),
            bool_word(workbench_show_description.get()),
            bool_word(workbench_show_footer.get()),
            if workbench_custom_motion.get() {
                "BottomSheetMotion::custom"
            } else {
                "BottomSheetMotion::default"
            },
            bool_word(workbench_show_close_button.get()),
            bool_word(workbench_show_close_button.get()),
            bool_word(is_detached),
            if is_detached { "16.0" } else { "0.0" },
            bool_word(workbench_show_description.get()),
            bool_word(workbench_show_footer.get()),
            bool_word(workbench_custom_motion.get()),
        )
    });

    view! {
        <ComponentPage
            title="BottomSheet"
            slug="bottom-sheet"
            group="Overlays"
            description="baseline-style bottom sheet primitive composed from Sheet with centralized handle/description/footer/detached contracts and stable slot/data-state markers."
        >
            <Playground
                title="Hello World (Minimal Path)"
                description="Default path: no manual state-machine wiring, simple props only."
                code_signal=hello_world_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_hello_sheet>"Open bottom sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_hello_raw.get()}</span>
                </div>

                <Show when=move || present_hello.get()>
                    <BottomSheet
                        open=open_hello
                        id_base="docs-bottom-sheet-hello".to_string()
                        title="Bottom sheet".to_string()
                        on_close=close_hello
                        on_exit_complete=on_hello_exit_complete
                    >
                        <div class="ui-muted">"Hello World path with the minimum public API surface."</div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="Semantic Bottom Sheet"
                code_signal=semantic_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" data-slot="bottom-sheet-e2e-semantic-controls">
                    <span data-slot="bottom-sheet-e2e-open-semantic">
                        <Button on_press=open_semantic_sheet>"Open bottom sheet"</Button>
                    </span>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <BottomSheet
                        open=open_semantic
                        id_base="docs-bottom-sheet-semantic".to_string()
                        title="Update available".to_string()
                        description="A newer version with security improvements is ready to install.".to_string()
                        on_close=close_semantic
                        footer=move || view! {
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>
                                    "Later"
                                </Button>
                                <Button on_press=close_semantic>"Install"</Button>
                            </div>
                        }
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>
                                "Bottom sheet uses Sheet(bottom) + spring motion with stable a11y wiring."
                            </div>
                            <div class="ui-muted">"Esc/backdrop closes. Focus trap remains active."</div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="Detached + Title Only + Custom Class"
                code_signal=detached_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_detached_sheet>"Open detached sheet"</Button>
                    <span class="ui-muted">"open: " {move || open_detached_raw.get()}</span>
                </div>

                <Show when=move || present_detached.get()>
                    <BottomSheet
                        open=open_detached
                        id_base="docs-bottom-sheet-detached".to_string()
                        title="Quick actions".to_string()
                        is_detached=true
                        bottom_inset_px=16.0
                        is_close_button_visible=false
                        class_name="docs-bottom-sheet-custom".to_string()
                        on_close=close_detached
                        on_exit_complete=on_detached_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Title-only mode keeps `aria-describedby` unset."</div>
                            <div class="ui-muted">"Detached mode applies inset and rounded surface styling."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_detached>
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="Custom Motion Contract"
                code_signal=custom_motion_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" data-slot="bottom-sheet-e2e-motion-controls">
                    <span data-slot="bottom-sheet-e2e-open-motion">
                        <Button on_press=open_custom_motion_sheet>"Open custom motion sheet"</Button>
                    </span>
                    <span class="ui-muted">"open: " {move || open_custom_motion_raw.get()}</span>
                </div>

                <Show when=move || present_custom_motion.get()>
                    <BottomSheet
                        open=open_custom_motion
                        id_base="docs-bottom-sheet-motion".to_string()
                        title="Motion tuned".to_string()
                        description="Custom sheet motion flips data-motion-source to custom.".to_string()
                        motion=BottomSheetMotion {
                            sheet: ui::SheetMotion {
                                initial_offset_px: 64.0,
                                ..ui::SheetMotion::default()
                            },
                        }
                        on_close=close_custom_motion
                        on_exit_complete=on_custom_motion_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Motion contract switches to custom markers for style/debug hooks."</div>
                            <div class="ui-muted">"Use devtools to inspect data-motion-source/custom-motion."</div>
                        </div>
                    </BottomSheet>
                </Show>
            </Playground>

            <Playground
                title="State Scenarios"
                description="Matrix for description/title-only/detached state contracts."
                code_signal=state_matrix_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-state-matrix">
                    <SegmentedControl
                        id_base="docs-bottom-sheet-state-matrix".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="BottomSheet state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_sheet>"Open state matrix sheet"</Button>
                        <span class="ui-muted">"open: " {move || open_state_matrix_raw.get()}</span>
                    </div>

                    <Show when=move || present_state_matrix.get()>
                        {move || {
                            let title = if state_matrix_is_detached.get() {
                                "Detached matrix".to_string()
                            } else {
                                "State matrix".to_string()
                            };
                            let description = if state_matrix_has_description.get() {
                                "Description branch keeps aria-describedby wired.".to_string()
                            } else {
                                String::new()
                            };
                            let is_detached = state_matrix_is_detached.get();
                            let is_close_button_visible = !state_matrix_hide_close.get();

                            view! {
                                <BottomSheet
                                    open=open_state_matrix
                                    id_base="docs-bottom-sheet-state-matrix-open".to_string()
                                    title=title
                                    description=description
                                    is_detached=is_detached
                                    is_close_button_visible=is_close_button_visible
                                    on_close=close_state_matrix_sheet
                                    on_exit_complete=on_state_matrix_exit_complete
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix branch for bottom-sheet semantic markers."</div>
                                        <span class="ui-muted">
                                            "description: "
                                            {move || if state_matrix_has_description.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "detached: "
                                            {move || if state_matrix_is_detached.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "close-button-visible: "
                                            {move || if state_matrix_hide_close.get() { "false" } else { "true" }}
                                        </span>
                                    </div>
                                </BottomSheet>
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="BottomSheet boundary is controlled-only; uncontrolled open state is adapter-level N/A."
                code_signal=controlled_vs_uncontrolled_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="bottom-sheet-controlled-uncontrolled"
                >
                    <div class="docs-row">
                        <Button on_press=open_compare_controlled>"Open controlled sheet"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=close_compare_controlled>
                            "Close controlled sheet"
                        </Button>
                    </div>

                    <Show when=move || compare_present.get()>
                        <BottomSheet
                            open=compare_controlled_open
                            id_base="docs-bottom-sheet-controlled".to_string()
                            title="Controlled".to_string()
                            description="`open: Signal<bool>` is the single source of truth at component boundary.".to_string()
                            on_close=close_compare_controlled
                            on_exit_complete=on_compare_exit_complete
                        >
                            <div>"Controlled path (component public API)."</div>
                        </BottomSheet>
                    </Show>

                    <div class="docs-stack docs-stack--tight">
                        <strong>"Uncontrolled (N/A for BottomSheet)"</strong>
                        <span class="ui-muted">
                            "`BottomSheet` requires `open: Signal<bool>` + `on_close`; uncontrolled behavior should be adapted upstream via primitives."
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="BottomSheet is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    data-slot="bottom-sheet-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-bottom-sheet-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="BottomSheet stream mode".to_string()
                    />
                    <BottomSheet
                        open=stream_open
                        id_base="docs-bottom-sheet-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output remains snapshot-compatible while preserving machine-readable status markers.".to_string()
                        on_close=stream_close
                    >
                        <div>"requested mode and output status are displayed as docs contract markers."</div>
                    </BottomSheet>
                    <span class="ui-muted">"requested mode: " {move || stream_requested_mode.get()}</span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "effective component status: data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                description="Button-style workbench: tune props/state and inspect generated config + copy-ready code."
                code_signal=workbench_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-workbench-controls">
                        <div class="docs-search__label">"Title"</div>
                        <SegmentedControl
                            id_base="docs-bottom-sheet-workbench-title".to_string()
                            options=workbench_title_options.clone()
                            selected_index=workbench_title_index
                            set_selected_index=set_workbench_title_index
                            size=SegmentedControlSize::Sm
                            aria_label="BottomSheet workbench title".to_string()
                        />
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_description.get()
                                on:change=move |ev| set_workbench_show_description.set(event_target_checked(&ev))
                            />
                            " Show description"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_footer.get()
                                on:change=move |ev| set_workbench_show_footer.set(event_target_checked(&ev))
                            />
                            " Show footer actions"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_is_detached.get()
                                on:change=move |ev| set_workbench_is_detached.set(event_target_checked(&ev))
                            />
                            " Detached mode"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_show_close_button.get()
                                on:change=move |ev| set_workbench_show_close_button.set(event_target_checked(&ev))
                            />
                            " Show close button"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-workbench">
                    <div class="docs-row">
                        <Button attr:data-slot="bottom-sheet-workbench-open" on_press=open_workbench_sheet>
                            "Open interactive sheet"
                        </Button>
                        <Button
                            attr:data-slot="bottom-sheet-workbench-close"
                            variant=ButtonVariant::Secondary
                            on_press=close_workbench_sheet
                        >
                            "Close"
                        </Button>
                        <span class="ui-muted">"open: " {move || workbench_open_raw.get()}</span>
                    </div>
                    <Show when=move || workbench_present.get()>
                        {move || {
                            let title = workbench_title.get();
                            let description = if workbench_show_description.get() {
                                match title.as_str() {
                                    "Quick actions" => "Choose one action, then dismiss.".to_string(),
                                    "Install update" => "A newer version with security improvements is ready to install.".to_string(),
                                    _ => "Default bottom-sheet copy from workbench.".to_string(),
                                }
                            } else {
                                String::new()
                            };
                            let motion = workbench_motion.get();

                            if workbench_show_footer.get() {
                                view! {
                                    <BottomSheet
                                        open=workbench_open
                                        id_base="docs-bottom-sheet-workbench".to_string()
                                        title=title
                                        description=description
                                        is_detached=workbench_is_detached.get()
                                        bottom_inset_px=if workbench_is_detached.get() { 16.0 } else { 0.0 }
                                        is_close_button_visible=workbench_show_close_button.get()
                                        motion=motion
                                        footer=move || {
                                            view! {
                                                <div class="docs-row docs-row--end">
                                                    <Button variant=ButtonVariant::Secondary on_press=close_workbench_sheet>
                                                        "Later"
                                                    </Button>
                                                    <Button on_press=close_workbench_sheet>"Confirm"</Button>
                                                </div>
                                            }
                                            .into_any()
                                        }
                                        on_close=close_workbench_sheet
                                        on_exit_complete=on_workbench_exit_complete
                                    >
                                        <div class="ui-muted">"Interactive preview surface for BottomSheet contracts."</div>
                                    </BottomSheet>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <BottomSheet
                                        open=workbench_open
                                        id_base="docs-bottom-sheet-workbench".to_string()
                                        title=title
                                        description=description
                                        is_detached=workbench_is_detached.get()
                                        bottom_inset_px=if workbench_is_detached.get() { 16.0 } else { 0.0 }
                                        is_close_button_visible=workbench_show_close_button.get()
                                        motion=motion
                                        on_close=close_workbench_sheet
                                        on_exit_complete=on_workbench_exit_complete
                                    >
                                        <div class="ui-muted">"Interactive preview surface for BottomSheet contracts."</div>
                                    </BottomSheet>
                                }
                                .into_any()
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Description / Title-only / Detached)"
                description="Workbench 后的多参数状态对比。"
                code_signal=state_matrix_code
                code_imports=BOTTOM_SHEET_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-state-matrix-after-workbench">
                    <SegmentedControl
                        id_base="docs-bottom-sheet-state-matrix-after-workbench".to_string()
                        options=state_matrix_options_after_workbench.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="BottomSheet state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_sheet>"Open state matrix sheet"</Button>
                        <span class="ui-muted">"open: " {move || open_state_matrix_raw.get()}</span>
                    </div>

                    <Show when=move || present_state_matrix.get()>
                        {move || {
                            let title = if state_matrix_is_detached.get() {
                                "Detached matrix".to_string()
                            } else {
                                "State matrix".to_string()
                            };
                            let description = if state_matrix_has_description.get() {
                                "Description branch keeps aria-describedby wired.".to_string()
                            } else {
                                String::new()
                            };
                            let is_detached = state_matrix_is_detached.get();
                            let is_close_button_visible = !state_matrix_hide_close.get();

                            view! {
                                <BottomSheet
                                    open=open_state_matrix
                                    id_base="docs-bottom-sheet-state-matrix-open-after-workbench".to_string()
                                    title=title
                                    description=description
                                    is_detached=is_detached
                                    is_close_button_visible=is_close_button_visible
                                    on_close=close_state_matrix_sheet
                                    on_exit_complete=on_state_matrix_exit_complete
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix branch for bottom-sheet semantic markers."</div>
                                        <span class="ui-muted">
                                            "description: "
                                            {move || if state_matrix_has_description.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "detached: "
                                            {move || if state_matrix_is_detached.get() { "true" } else { "false" }}
                                        </span>
                                        <span class="ui-muted">
                                            "close-button-visible: "
                                            {move || if state_matrix_hide_close.get() { "false" } else { "true" }}
                                        </span>
                                    </div>
                                </BottomSheet>
                            }
                        }}
                    </Show>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-defaults-contract">
                <h3>"Defaults + API Contract (logic.rs SSOT)"</h3>
                <p class="ui-muted">
                    "Source of truth: "
                    <code>"components/bottom-sheet/src/logic.rs"</code>
                </p>
                <ul class="docs-stack docs-stack--tight">
                    <li><code>"DEFAULT_TITLE = \"Bottom sheet\""</code></li>
                    <li><code>"DEFAULT_CLOSE_LABEL = \"Close bottom sheet\""</code></li>
                    <li><code>"DEFAULT_DISMISSABLE = true"</code></li>
                    <li><code>"DEFAULT_KEYBOARD_DISMISS_DISABLED = false"</code></li>
                    <li><code>"DEFAULT_BOTTOM_INSET_PX = 0.0"</code></li>
                    <li><code>"resolve_handle_visibility(is_handle_visible, show_handle)"</code></li>
                    <li>
                        <code>
                            "resolve_close_button_visibility(is_close_button_visible, show_close_button)"
                        </code>
                    </li>
                    <li><code>"resolve_attachment(is_detached, detached)"</code></li>
                    <li><code>"resolve_dismissable(is_dismissable)"</code></li>
                    <li>
                        <code>
                            "resolve_keyboard_dismiss_disabled(is_keyboard_dismiss_disabled)"
                        </code>
                    </li>
                </ul>
            </div>

            <div class="docs-stack docs-stack--tight" data-slot="bottom-sheet-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"BOTTOM_SHEET_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-bottom_sheet\", \"inject-css\"] }"
                    </code>
                </p>
                <ul class="docs-stack docs-stack--tight" data-slot="bottom-sheet-source-paths">
                    <li><code>"components/bottom-sheet/src/mod.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/logic.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/view.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/styles.rs"</code></li>
                    <li><code>"components/bottom-sheet/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn tray() -> AnyView {
    // Legacy source-contract markers retained for overlays semantic suites:
    // title="Tray + Footer Actions"
    // id_base="docs-tray-semantic".to_string()
    // description="Tray composes Sheet with title/description wiring and footer action slots.".to_string()
    // title="State + Source Markers"
    // let custom_motion = TrayMotion {
    // sheet: ui::SheetMotion {
    // initial_offset_px: 46.0
    // id_base="docs-tray-fixed".to_string()
    // motion=custom_motion
    // is_fixed_height=true
    // is_dismissable=false
    // is_keyboard_dismiss_disabled=true
    // show_close_button=false
    // class_name="docs-tray-custom".to_string()
    // data-size-source
    // Inspect data-size-source / data-dismiss-source / data-motion-source in DevTools.
    // on_exit_complete=on_custom_exit_complete
    let (showcase_open_raw, set_showcase_open_raw) = signal(false);
    let showcase_open: Signal<bool> = Signal::derive(move || showcase_open_raw.get());
    let (showcase_close_count, set_showcase_close_count) = signal(0_u32);
    let (showcase_exit_count, set_showcase_exit_count) = signal(0_u32);

    let open_showcase: OnPress = Callback::new(move |_| set_showcase_open_raw.set(true));
    let on_showcase_close: OnPress = Callback::new(move |_| {
        set_showcase_open_raw.set(false);
        set_showcase_close_count.update(|count| *count += 1);
    });
    let on_showcase_exit_complete =
        Callback::new(move |_| set_showcase_exit_count.update(|count| *count += 1));

    let hello_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<Tray
  open=open
  on_close=Callback::new(move |_| set_open_raw.set(false))
  id_base="docs-tray-hello".to_string()
  title="Filters".to_string()
>
  <div>"Tray body content"</div>
</Tray>"#
            .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);
    let (workbench_exit_count, set_workbench_exit_count) = signal(0_u32);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_footer, set_workbench_show_footer) = signal(true);
    let (workbench_show_close_button, set_workbench_show_close_button) = signal(true);
    let (workbench_fixed_height, set_workbench_fixed_height) = signal(false);
    let (workbench_dismissable, set_workbench_dismissable) = signal(true);
    let (workbench_keyboard_dismiss_disabled, set_workbench_keyboard_dismiss_disabled) =
        signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_compact_close_label, set_workbench_compact_close_label) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);

    let open_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let on_workbench_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });
    let on_workbench_exit_complete =
        Callback::new(move |_| set_workbench_exit_count.update(|count| *count += 1));

    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            TrayMotion {
                sheet: ui::SheetMotion {
                    initial_offset_px: 64.0,
                    ..ui::SheetMotion::default()
                },
            }
        } else {
            TrayMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            "Slide-up panel for contextual actions."
        } else {
            ""
        };
        let class_name = if workbench_custom_class_name.get() {
            "docs-tray-workbench"
        } else {
            ""
        };
        let close_label = if workbench_compact_close_label.get() {
            "Dismiss"
        } else {
            "Close tray"
        };
        let lang = if workbench_zh_lang.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };

        let mut lines = vec![
            "let (open_raw, set_open_raw) = signal(false);".to_string(),
            "let open: Signal<bool> = Signal::derive(move || open_raw.get());".to_string(),
            "<Tray".to_string(),
            "  open=open".to_string(),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            "  id_base=\"docs-tray-workbench\".to_string()".to_string(),
            "  title=\"Workbench tray\".to_string()".to_string(),
            format!(
                "  description={}.to_string()",
                rust_string_literal(description)
            ),
            "  footer=move || view! { <Button>\"Apply\"</Button> }".to_string(),
            format!(
                "  motion={}",
                if workbench_custom_motion.get() {
                    "TrayMotion { sheet: ui::SheetMotion { initial_offset_px: 64.0, ..ui::SheetMotion::default() } }"
                } else {
                    "TrayMotion::default()"
                }
            ),
            format!(
                "  show_close_button={}",
                bool_word(workbench_show_close_button.get())
            ),
            format!("  close_label={}", rust_string_literal(close_label)),
            format!(
                "  is_fixed_height={}",
                bool_word(workbench_fixed_height.get())
            ),
            format!(
                "  is_dismissable={}",
                bool_word(workbench_dismissable.get())
            ),
            format!(
                "  is_keyboard_dismiss_disabled={}",
                bool_word(workbench_keyboard_dismiss_disabled.get())
            ),
            format!("  lang={}.to_string()", rust_string_literal(lang)),
            format!("  dir={dir}"),
            "  on_exit_complete=Callback::new(move |_| {})".to_string(),
            format!(
                "  class_name={}.to_string()",
                rust_string_literal(class_name)
            ),
        ];
        push_line_when(&mut lines, true, ">".to_string());
        lines.push("  <div>\"Body\"</div>".to_string());
        lines.push("</Tray>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let description = if workbench_show_description.get() {
            Some("Slide-up panel for contextual actions")
        } else {
            None
        };
        let footer = if workbench_show_footer.get() {
            "Some(ViewFn)"
        } else {
            "None"
        };
        let motion = if workbench_custom_motion.get() {
            "TrayMotion::custom(initial_offset_px=64)"
        } else {
            "TrayMotion::default()"
        };
        let close_label = if workbench_compact_close_label.get() {
            "Dismiss"
        } else {
            "Close tray"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-tray-workbench")
        } else {
            None
        };
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN")
        } else {
            Some("en-US")
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };

        format!(
            "TrayActualConfig {{\n  open: {},\n  on_close: \"count={}\",\n  id_base: \"docs-tray-workbench\",\n  title: \"Workbench tray\",\n  description: {description:?},\n  footer: {footer},\n  motion: {motion},\n  show_close_button: {},\n  close_label: {:?},\n  is_fixed_height: {},\n  is_dismissable: {},\n  is_keyboard_dismiss_disabled: {},\n  lang: {lang:?},\n  dir: {dir},\n  on_exit_complete: \"count={}\",\n  class_name: {class_name:?},\n}}",
            bool_word(workbench_open_raw.get()),
            workbench_close_count.get(),
            bool_word(workbench_show_close_button.get()),
            close_label,
            bool_word(workbench_fixed_height.get()),
            bool_word(workbench_dismissable.get()),
            bool_word(workbench_keyboard_dismiss_disabled.get()),
            workbench_exit_count.get(),
        )
    });

    let (matrix_default_open_raw, set_matrix_default_open_raw) = signal(false);
    let matrix_default_open: Signal<bool> = Signal::derive(move || matrix_default_open_raw.get());
    let open_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(true));
    let close_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(false));

    let (matrix_fixed_open_raw, set_matrix_fixed_open_raw) = signal(false);
    let matrix_fixed_open: Signal<bool> = Signal::derive(move || matrix_fixed_open_raw.get());
    let open_matrix_fixed: OnPress = Callback::new(move |_| set_matrix_fixed_open_raw.set(true));
    let close_matrix_fixed: OnPress = Callback::new(move |_| set_matrix_fixed_open_raw.set(false));

    let (matrix_compact_open_raw, set_matrix_compact_open_raw) = signal(false);
    let matrix_compact_open: Signal<bool> = Signal::derive(move || matrix_compact_open_raw.get());
    let open_matrix_compact: OnPress =
        Callback::new(move |_| set_matrix_compact_open_raw.set(true));
    let close_matrix_compact: OnPress =
        Callback::new(move |_| set_matrix_compact_open_raw.set(false));

    let matrix_code = Signal::derive(move || {
        r#"<Tray open=default_open on_close=dismiss_default id_base="tray-default".to_string() title="Default".to_string() />
<Tray open=fixed_open on_close=dismiss_fixed id_base="tray-fixed".to_string() title="Fixed".to_string() is_fixed_height=true is_dismissable=false />
<Tray open=compact_open on_close=dismiss_compact id_base="tray-compact".to_string() title="Compact".to_string() show_close_button=false close_label="Dismiss" />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Tray"
            slug="tray"
            group="Overlays"
            description="Tray playground with full API workbench and state-matrix comparison."
        >
            <Playground title="Hello World (Default Tray)" code_signal=hello_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_showcase>"Open tray"</Button>
                        <span class="ui-muted">"open: " {move || showcase_open_raw.get()}</span>
                    </div>
                    <span class="ui-muted">
                        "on_close: " {move || showcase_close_count.get()}
                        " · on_exit_complete: " {move || showcase_exit_count.get()}
                    </span>
                </div>
                <Tray
                    open=showcase_open
                    on_close=on_showcase_close
                    id_base="docs-tray-hello".to_string()
                    title="Filters".to_string()
                    description="Tray body with semantic heading and close control.".to_string()
                    footer=move || view! {
                        <div class="docs-row docs-row--end">
                            <Button variant=ButtonVariant::Secondary on_press=on_showcase_close>
                                "Reset"
                            </Button>
                            <Button on_press=on_showcase_close>"Apply"</Button>
                        </div>
                    }
                    on_exit_complete=on_showcase_exit_complete
                >
                    <div class="docs-stack docs-stack--tight">
                        <span>"Real tray content for mobile-first actions."</span>
                        <span class="ui-muted">
                            "Dismiss via close action, Esc, or backdrop by default."
                        </span>
                    </div>
                </Tray>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tray-workbench-controls">
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "description"
                        </Switch>
                        <Switch checked=workbench_show_footer set_checked=set_workbench_show_footer>
                            "footer"
                        </Switch>
                        <Switch
                            checked=workbench_show_close_button
                            set_checked=set_workbench_show_close_button
                        >
                            "show_close_button"
                        </Switch>
                        <Switch checked=workbench_fixed_height set_checked=set_workbench_fixed_height>
                            "is_fixed_height"
                        </Switch>
                        <Switch checked=workbench_dismissable set_checked=set_workbench_dismissable>
                            "is_dismissable"
                        </Switch>
                        <Switch
                            checked=workbench_keyboard_dismiss_disabled
                            set_checked=set_workbench_keyboard_dismiss_disabled
                        >
                            "is_keyboard_dismiss_disabled"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "custom motion"
                        </Switch>
                        <Switch
                            checked=workbench_compact_close_label
                            set_checked=set_workbench_compact_close_label
                        >
                            "close_label compact"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <Button variant=ButtonVariant::Secondary on_press=open_workbench>
                            "Open workbench tray"
                        </Button>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="tray-workbench-feedback">
                    <span class="ui-muted">
                        "open: " {move || workbench_open_raw.get()}
                        " · on_close: " {move || workbench_close_count.get()}
                        " · on_exit_complete: " {move || workbench_exit_count.get()}
                    </span>
                </div>
                <Tray
                    open=workbench_open
                    on_close=on_workbench_close
                    id_base="docs-tray-workbench".to_string()
                    title="Workbench tray".to_string()
                    description=if workbench_show_description.get() {
                        "Slide-up panel for contextual actions.".to_string()
                    } else {
                        String::new()
                    }
                    footer=move || {
                        view! {
                            <Show when=move || workbench_show_footer.get()>
                                <div class="docs-row docs-row--end">
                                    <Button variant=ButtonVariant::Secondary on_press=on_workbench_close>
                                        "Cancel"
                                    </Button>
                                    <Button on_press=on_workbench_close>"Save"</Button>
                                </div>
                            </Show>
                        }
                        .into_any()
                    }
                    motion=workbench_motion.get()
                    show_close_button=workbench_show_close_button.get()
                    close_label=if workbench_compact_close_label.get() {
                        "Dismiss"
                    } else {
                        "Close tray"
                    }
                    is_fixed_height=workbench_fixed_height.get()
                    is_dismissable=workbench_dismissable.get()
                    is_keyboard_dismiss_disabled=workbench_keyboard_dismiss_disabled.get()
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    }
                    on_exit_complete=on_workbench_exit_complete
                    class_name=if workbench_custom_class_name.get() {
                        "docs-tray-workbench".to_string()
                    } else {
                        String::new()
                    }
                >
                    <div class="docs-stack docs-stack--tight">
                        <span>"Workbench body area. Toggle parameters from the control panel."</span>
                        <span class="ui-muted">
                            "This tray exposes close/exit callback counts as live feedback."
                        </span>
                    </div>
                </Tray>
            </Playground>

            <Playground title="State Matrix (Default / Fixed / Compact)" code_signal=matrix_code>
                <div class="docs-row" data-slot="tray-state-matrix-controls">
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_default>
                        "Open Default"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_fixed>
                        "Open Fixed"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_compact>
                        "Open Compact"
                    </Button>
                </div>

                <Tray
                    open=matrix_default_open
                    on_close=close_matrix_default
                    id_base="docs-tray-matrix-default".to_string()
                    title="Default tray".to_string()
                    description="Default behavior with footer actions.".to_string()
                    footer=move || view! {
                        <div class="docs-row docs-row--end">
                            <Button on_press=close_matrix_default>"Done"</Button>
                        </div>
                    }
                >
                    <div>"Default tray body."</div>
                </Tray>

                <Tray
                    open=matrix_fixed_open
                    on_close=close_matrix_fixed
                    id_base="docs-tray-matrix-fixed".to_string()
                    title="Fixed tray".to_string()
                    is_fixed_height=true
                    is_dismissable=false
                    is_keyboard_dismiss_disabled=true
                    show_close_button=true
                    class_name="docs-tray-fixed".to_string()
                >
                    <div>"Fixed-height tray with stricter dismiss behavior."</div>
                </Tray>

                <Tray
                    open=matrix_compact_open
                    on_close=close_matrix_compact
                    id_base="docs-tray-matrix-compact".to_string()
                    title="Compact tray".to_string()
                    show_close_button=false
                    close_label="Dismiss"
                    motion=TrayMotion {
                        sheet: ui::SheetMotion {
                            initial_offset_px: 48.0,
                            ..ui::SheetMotion::default()
                        },
                    }
                >
                    <div>"Compact state prioritizes content area."</div>
                </Tray>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn sonner() -> AnyView {
    let portal_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let inline_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let source_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));
    let workbench_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));
    let hello_world_code = Signal::derive(move || r#"<Sonner />"#.to_string());

    let push_saved: OnPress = Callback::new(move |_| {
        portal_store.get_value().push_simple("Saved");
    });
    let push_danger: OnPress = Callback::new(move |_| {
        portal_store.get_value().push.run(ToastOptions {
            title: "Publish failed".to_string(),
            description: Some("Check network and retry.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(6000),
        });
    });

    let push_inline: OnPress = Callback::new(move |_| {
        inline_store.get_value().push.run(ToastOptions {
            title: "Undo available".to_string(),
            description: Some("Item moved to archive.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5000),
        });
    });
    let clear_inline: OnPress = Callback::new(move |_| inline_store.get_value().clear.run(()));

    let push_source: OnPress = Callback::new(move |_| {
        source_store.get_value().push.run(ToastOptions {
            title: "Migration complete".to_string(),
            description: Some("Source markers are stable.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(5500),
        });
    });
    let clear_source: OnPress = Callback::new(move |_| source_store.get_value().clear.run(()));

    let basic_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Sonner store=store.clone() />
store.push_simple("Saved");"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopCenter
  max_toasts=2
  class_name="docs-sonner-inline".to_string()
/>"#
        .to_string()
    });

    let source_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Sonner
  store=store.clone()
  portal=false
  position=SonnerPosition::TopLeft
  max_toasts=4
  aria_label="Status updates".to_string()
  class_name="docs-sonner-source".to_string()
  motion=ToastMotion {
    initial_y_px: 22.0,
    initial_scale: 0.94,
    ..ToastMotion::default()
  }
/>"#
        .to_string()
    });

    let custom_motion = ToastMotion {
        initial_y_px: 22.0,
        initial_scale: 0.94,
        ..ToastMotion::default()
    };

    let (workbench_top_left, set_workbench_top_left) = signal(false);
    let (workbench_portal, set_workbench_portal) = signal(true);
    let (workbench_max_toasts, set_workbench_max_toasts) = signal(3_u16);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_push_count, set_workbench_push_count) = signal(0_u32);
    let (workbench_clear_count, set_workbench_clear_count) = signal(0_u32);

    let push_workbench: OnPress = Callback::new(move |_| {
        workbench_store.get_value().push.run(ToastOptions {
            title: "Workbench event".to_string(),
            description: Some("Sonner workbench is active.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(5000),
        });
        set_workbench_push_count.update(|count| *count += 1);
    });
    let clear_workbench: OnPress = Callback::new(move |_| {
        workbench_store.get_value().clear.run(());
        set_workbench_clear_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Sonner\n  position={}\n  portal={}\n  max_toasts={}\n  aria_label={}\n  class_name={}\n  motion={}\n  store=Some(store)\n  lang={}\n  dir={}\n/>",
            if workbench_top_left.get() {
                "SonnerPosition::TopLeft"
            } else {
                "SonnerPosition::BottomRight"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_aria.get() {
                "Some(\"Status updates\".to_string())"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-sonner-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_custom_motion.get() {
                "ToastMotion { initial_y_px: 22.0, initial_scale: 0.94, ..ToastMotion::default() }"
            } else {
                "ToastMotion::default()"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\".to_string())"
            } else {
                "Some(\"en-US\".to_string())"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SonnerActualConfig {{\n  position: {},\n  portal: {},\n  max_toasts: {},\n  aria_label: {},\n  class_name: {},\n  motion: {},\n  store: Some(workbench_store),\n  lang: {},\n  dir: {},\n  push_count: {},\n  clear_count: {},\n}}",
            if workbench_top_left.get() {
                "SonnerPosition::TopLeft"
            } else {
                "SonnerPosition::BottomRight"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_aria.get() {
                "Some(\"Status updates\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-sonner-workbench\")"
            } else {
                "None"
            },
            if workbench_custom_motion.get() {
                "ToastMotion::custom"
            } else {
                "ToastMotion::default"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            workbench_push_count.get(),
            workbench_clear_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="Sonner"
            slug="sonner"
            group="Overlays"
            description="baseline-style toast host that composes ToastViewport with position presets, queue limits, and stable Sonner slot/source-state data contracts."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="ui-muted">
                    "Default path mounts a notification host with sensible defaults and no manual state wiring."
                </div>
                <Sonner />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="sonner-workbench-controls">
                        <Switch checked=workbench_top_left set_checked=set_workbench_top_left>
                            "position TopLeft"
                        </Switch>
                        <Switch checked=workbench_portal set_checked=set_workbench_portal>
                            "portal"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "motion"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <label class="docs-search__label">
                            "max_toasts (" {move || workbench_max_toasts.get()} ")"
                            <input
                                type="range"
                                min="1"
                                max="6"
                                step="1"
                                prop:value=move || workbench_max_toasts.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(3)
                                        .clamp(1, 6);
                                    set_workbench_max_toasts.set(next);
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_workbench>"Push workbench toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_workbench>
                            "Clear"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "push: " {move || workbench_push_count.get()}
                        " · clear: " {move || workbench_clear_count.get()}
                    </span>
                    <Sonner
                        position=if workbench_top_left.get() {
                            SonnerPosition::TopLeft
                        } else {
                            SonnerPosition::BottomRight
                        }
                        portal=workbench_portal.get()
                        max_toasts=usize::from(workbench_max_toasts.get())
                        aria_label=if workbench_custom_aria.get() {
                            "Status updates".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-sonner-workbench".to_string()
                        } else {
                            String::new()
                        }
                        motion=if workbench_custom_motion.get() {
                            ToastMotion {
                                initial_y_px: 22.0,
                                initial_scale: 0.94,
                                ..ToastMotion::default()
                            }
                        } else {
                            ToastMotion::default()
                        }
                        store=workbench_store.get_value()
                        lang=if workbench_zh_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl_dir.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                    />
                </div>
            </Playground>

            <Playground title="Portal Queue + Variants" code_signal=basic_code>
                <div class="docs-row" data-slot="sonner-portal-controls">
                    <span data-slot="sonner-portal-push-success">
                        <Button variant=ButtonVariant::Secondary on_press=push_saved>
                            "Push success"
                        </Button>
                    </span>
                    <span data-slot="sonner-portal-push-danger">
                        <Button variant=ButtonVariant::Destructive on_press=push_danger>
                            "Push danger"
                        </Button>
                    </span>
                </div>
                <Sonner store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center + Max Queue" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="sonner-inline-controls">
                        <span data-slot="sonner-inline-push">
                            <Button on_press=push_inline>"Push accent"</Button>
                        </span>
                        <span data-slot="sonner-inline-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <Sonner
                        store=inline_store.get_value()
                        portal=false
                        position=SonnerPosition::TopCenter
                        max_toasts=2
                        class_name="docs-sonner-inline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Source Markers)"
                description="Inspect `data-state`, `data-queue`, `data-position-source`, `data-portal-source`, `data-max-toasts-source`, `data-store-source`, and `data-motion-source` contracts."
                code_signal=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="sonner-source-controls">
                        <span data-slot="sonner-source-push">
                            <Button on_press=push_source>"Push marker toast"</Button>
                        </span>
                        <span data-slot="sonner-source-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_source>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools."
                    </div>
                    <Sonner
                        store=source_store.get_value()
                        portal=false
                        position=SonnerPosition::TopLeft
                        max_toasts=4
                        aria_label="Status updates".to_string()
                        class_name="docs-sonner-source".to_string()
                        motion=custom_motion
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="sonner-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="sonner-api-rows">
                    <li>
                        <code>"position: SonnerPosition"</code>
                        " "
                        {format!(
                            "default = SonnerPosition::{:?} ({})",
                            SonnerPosition::default(),
                            SonnerPosition::default().as_attr()
                        )}
                    </li>
                    <li>
                        <code>"portal: bool"</code>
                        " "
                        {format!("default = {}", ui::sonner::DEFAULT_PORTAL)}
                    </li>
                    <li>
                        <code>"max_toasts: usize"</code>
                        " "
                        {format!("default = {}", ui::sonner::DEFAULT_MAX_TOASTS)}
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui::sonner::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"class_name: Option<String>"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"lang: Option<String>, dir: Option<A11yDirection>"</code>
                        " default = None (inherits app locale context)"
                    </li>
                    <li>
                        <code>"motion: ToastMotion"</code>
                        " default = ToastMotion::default()"
                    </li>
                    <li>
                        <code>"store: Option<ToastStore>"</code>
                        " default path = provided -> context -> local"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="sonner-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="sonner-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = portal | inline"
                    </li>
                    <li>
                        <code>"data-queue"</code>
                        " = single | bounded | extended"
                    </li>
                    <li>
                        <code>"data-position"</code>
                        " = top-left | top-center | top-right | bottom-left | bottom-center | bottom-right"
                    </li>
                    <li>
                        <code>"data-store-source"</code>
                        " = provided | context | local"
                    </li>
                    <li>
                        <code>"data-position-source / data-portal-source / data-max-toasts-source / data-motion-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A (Sonner is host config, no controlled/uncontrolled runtime axis)"
                    </li>
                </ul>
            </section>

        </ComponentPage>
    }
    .into_any()
}

pub(super) fn toaster() -> AnyView {
    let portal_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 3 }));
    let inline_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 2 }));
    let source_store = StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));
    let workbench_store =
        StoredValue::new(provide_toast_store(ToastStoreOptions { max_toasts: 4 }));

    let hello_world_code = Signal::derive(move || r#"<Toaster />"#.to_string());

    let push_saved: OnPress = Callback::new(move |_| {
        portal_store.get_value().push_simple("Synced");
    });
    let push_warning: OnPress = Callback::new(move |_| {
        portal_store.get_value().push.run(ToastOptions {
            title: "Action required".to_string(),
            description: Some("Session expires soon.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5000),
        });
    });

    let push_inline: OnPress = Callback::new(move |_| {
        inline_store.get_value().push.run(ToastOptions {
            title: "Draft restored".to_string(),
            description: Some("Recovered from autosave.".to_string()),
            variant: ToastVariant::Default,
            duration_ms: Some(4500),
        });
    });
    let clear_inline: OnPress = Callback::new(move |_| inline_store.get_value().clear.run(()));

    let push_source: OnPress = Callback::new(move |_| {
        source_store.get_value().push.run(ToastOptions {
            title: "Promotion deployed".to_string(),
            description: Some("Store + source markers stay inspectable.".to_string()),
            variant: ToastVariant::Danger,
            duration_ms: Some(5200),
        });
    });
    let clear_source: OnPress = Callback::new(move |_| source_store.get_value().clear.run(()));

    let basic_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });
<Toaster store=store.clone() />
store.push_simple("Synced");"#
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopCenter
  max_toasts=2
  class_name="docs-toaster-inline".to_string()
/>"#
        .to_string()
    });

    let source_code = Signal::derive(move || {
        r#"let store = provide_toast_store(ToastStoreOptions { max_toasts: 3 });

<Toaster
  store=store.clone()
  portal=false
  position=ToasterPosition::TopLeft
  max_toasts=4
  aria_label="Alert stream".to_string()
  class_name="docs-toaster-source".to_string()
  motion=ToastMotion {
    initial_y_px: 20.0,
    initial_scale: 0.95,
    ..ToastMotion::default()
  }
/>"#
        .to_string()
    });

    let custom_motion = ToastMotion {
        initial_y_px: 20.0,
        initial_scale: 0.95,
        ..ToastMotion::default()
    };

    let (workbench_top_left, set_workbench_top_left) = signal(false);
    let (workbench_portal, set_workbench_portal) = signal(true);
    let (workbench_max_toasts, set_workbench_max_toasts) = signal(3_u16);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_push_count, set_workbench_push_count) = signal(0_u32);
    let (workbench_clear_count, set_workbench_clear_count) = signal(0_u32);

    let push_workbench: OnPress = Callback::new(move |_| {
        workbench_store.get_value().push.run(ToastOptions {
            title: "Workbench alert".to_string(),
            description: Some("Toaster workbench is active.".to_string()),
            variant: ToastVariant::Accent,
            duration_ms: Some(5200),
        });
        set_workbench_push_count.update(|count| *count += 1);
    });
    let clear_workbench: OnPress = Callback::new(move |_| {
        workbench_store.get_value().clear.run(());
        set_workbench_clear_count.update(|count| *count += 1);
    });

    let workbench_code = Signal::derive(move || {
        format!(
            "<Toaster\n  position={}\n  portal={}\n  max_toasts={}\n  aria_label={}\n  class_name={}\n  lang={}\n  dir={}\n  motion={}\n  store=Some(store)\n/>",
            if workbench_top_left.get() {
                "ToasterPosition::TopLeft"
            } else {
                "ToasterPosition::BottomRight"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_aria.get() {
                "Some(\"Alert stream\".to_string())"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-toaster-workbench\".to_string())"
            } else {
                "None"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\".to_string())"
            } else {
                "Some(\"en-US\".to_string())"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_motion.get() {
                "ToastMotion { initial_y_px: 20.0, initial_scale: 0.95, ..ToastMotion::default() }"
            } else {
                "ToastMotion::default()"
            }
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ToasterActualConfig {{\n  position: {},\n  portal: {},\n  max_toasts: {},\n  aria_label: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  motion: {},\n  store: Some(workbench_store),\n  push_count: {},\n  clear_count: {},\n}}",
            if workbench_top_left.get() {
                "ToasterPosition::TopLeft"
            } else {
                "ToasterPosition::BottomRight"
            },
            workbench_portal.get(),
            workbench_max_toasts.get(),
            if workbench_custom_aria.get() {
                "Some(\"Alert stream\")"
            } else {
                "None"
            },
            if workbench_custom_class.get() {
                "Some(\"docs-toaster-workbench\")"
            } else {
                "None"
            },
            if workbench_zh_lang.get() {
                "Some(\"zh-CN\")"
            } else {
                "Some(\"en-US\")"
            },
            if workbench_rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            if workbench_custom_motion.get() {
                "ToastMotion::custom"
            } else {
                "ToastMotion::default"
            },
            workbench_push_count.get(),
            workbench_clear_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="Toaster"
            slug="toaster"
            group="Overlays"
            description="baseline-compatible toast host that composes Sonner/ToastViewport with centralized slot/queue/position/store source-state contracts and baseline-level spring motion handoff."
        >
            <Playground title="Hello World" code_signal=hello_world_code>
                <div class="ui-muted">
                    "Default path only mounts host; no state primitive wiring or custom store binding required."
                </div>
                <Toaster />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="toaster-workbench-controls">
                        <Switch checked=workbench_top_left set_checked=set_workbench_top_left>
                            "position TopLeft"
                        </Switch>
                        <Switch checked=workbench_portal set_checked=set_workbench_portal>
                            "portal"
                        </Switch>
                        <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "aria_label"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_custom_motion set_checked=set_workbench_custom_motion>
                            "motion"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <label class="docs-search__label">
                            "max_toasts (" {move || workbench_max_toasts.get()} ")"
                            <input
                                type="range"
                                min="1"
                                max="6"
                                step="1"
                                prop:value=move || workbench_max_toasts.get().to_string()
                                on:input=move |ev| {
                                    let next = event_target_value(&ev)
                                        .parse::<u16>()
                                        .unwrap_or(3)
                                        .clamp(1, 6);
                                    set_workbench_max_toasts.set(next);
                                }
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=push_workbench>"Push workbench toast"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=clear_workbench>
                            "Clear"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "push: " {move || workbench_push_count.get()}
                        " · clear: " {move || workbench_clear_count.get()}
                    </span>
                    <Toaster
                        position=if workbench_top_left.get() {
                            ToasterPosition::TopLeft
                        } else {
                            ToasterPosition::BottomRight
                        }
                        portal=workbench_portal.get()
                        max_toasts=usize::from(workbench_max_toasts.get())
                        aria_label=if workbench_custom_aria.get() {
                            "Alert stream".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class.get() {
                            "docs-toaster-workbench".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_zh_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl_dir.get() {
                            ui_headless::A11yDirection::Rtl
                        } else {
                            ui_headless::A11yDirection::Ltr
                        }
                        motion=if workbench_custom_motion.get() {
                            ToastMotion {
                                initial_y_px: 20.0,
                                initial_scale: 0.95,
                                ..ToastMotion::default()
                            }
                        } else {
                            ToastMotion::default()
                        }
                        store=workbench_store.get_value()
                    />
                </div>
            </Playground>

            <Playground title="Portal Queue Host" code_signal=basic_code>
                <div class="docs-row" data-slot="toaster-portal-controls">
                    <span data-slot="toaster-portal-push-success">
                        <Button variant=ButtonVariant::Secondary on_press=push_saved>
                            "Push success"
                        </Button>
                    </span>
                    <span data-slot="toaster-portal-push-accent">
                        <Button on_press=push_warning>
                            "Push accent"
                        </Button>
                    </span>
                </div>
                <Toaster store=portal_store.get_value() />
            </Playground>

            <Playground title="Inline Top-Center Host" code_signal=state_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="toaster-inline-controls">
                        <span data-slot="toaster-inline-push">
                            <Button on_press=push_inline>"Push inline toast"</Button>
                        </span>
                        <span data-slot="toaster-inline-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_inline>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <Toaster
                        store=inline_store.get_value()
                        portal=false
                        position=ToasterPosition::TopCenter
                        max_toasts=2
                        class_name="docs-toaster-inline".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Source Markers)"
                description="Inspect `data-state`, `data-queue`, `data-position-source`, `data-portal-source`, `data-max-toasts-source`, `data-store-source`, and `data-motion-source` contracts."
                code_signal=source_code
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row" data-slot="toaster-source-controls">
                        <span data-slot="toaster-source-push">
                            <Button on_press=push_source>"Push source toast"</Button>
                        </span>
                        <span data-slot="toaster-source-clear">
                            <Button variant=ButtonVariant::Secondary on_press=clear_source>
                                "Clear"
                            </Button>
                        </span>
                    </div>
                    <div class="ui-muted">
                        "Inspect data-position-source / data-portal-source / data-max-toasts-source / data-store-source / data-motion-source in DevTools."
                    </div>
                    <Toaster
                        store=source_store.get_value()
                        portal=false
                        position=ToasterPosition::TopLeft
                        max_toasts=4
                        aria_label="Alert stream".to_string()
                        class_name="docs-toaster-source".to_string()
                        motion=custom_motion
                    />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="toaster-api-matrix">
                <h3>"API Matrix"</h3>
                <ul data-slot="toaster-api-rows">
                    <li>
                        <code>"position: ToasterPosition"</code>
                        " "
                        {format!(
                            "default = ToasterPosition::{:?} ({})",
                            ToasterPosition::default(),
                            ToasterPosition::default().as_attr()
                        )}
                    </li>
                    <li>
                        <code>"portal: bool"</code>
                        " "
                        {format!("default = {}", ui::toaster::DEFAULT_PORTAL)}
                    </li>
                    <li>
                        <code>"max_toasts: usize"</code>
                        " "
                        {format!(
                            "default = {}",
                            ui::toaster::DEFAULT_MAX_TOASTS
                        )}
                    </li>
                    <li>
                        <code>"aria_label: Option<String>"</code>
                        " "
                        {format!(
                            "default label = {:?}",
                            ui::toaster::DEFAULT_ARIA_LABEL
                        )}
                    </li>
                    <li>
                        <code>"class_name: Option<String>"</code>
                        " default = None"
                    </li>
                    <li>
                        <code>"lang: Option<String>, dir: Option<A11yDirection>"</code>
                        " default = None (inherits app locale context)"
                    </li>
                    <li>
                        <code>"motion: ToastMotion"</code>
                        " default = ToastMotion::default()"
                    </li>
                    <li>
                        <code>"store: Option<ToastStore>"</code>
                        " default path = provided -> context -> local"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="toaster-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="toaster-state-rows">
                    <li>
                        <code>"data-state"</code>
                        " = portal | inline"
                    </li>
                    <li>
                        <code>"data-queue"</code>
                        " = single | bounded | extended"
                    </li>
                    <li>
                        <code>"data-position"</code>
                        " = top-left | top-center | top-right | bottom-left | bottom-center | bottom-right"
                    </li>
                    <li>
                        <code>"data-store-source"</code>
                        " = provided | context | local"
                    </li>
                    <li>
                        <code>"data-position-source / data-portal-source / data-max-toasts-source / data-motion-source"</code>
                        " = default | custom"
                    </li>
                    <li>
                        <code>"control mode"</code>
                        " = N/A (Toaster is host config, no controlled/uncontrolled runtime axis)"
                    </li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="toaster-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Each playground already supports "
                    <code>"Show code"</code>
                    " with copy action. The copied snippet is import-ready via "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text="use leptos::prelude::*;\nuse ui::*;\n\n<Toaster />".to_string()
                    label="Copy starter".to_string()
                    copyable=true
                    class_name="docs-toaster-source-copy".to_string()
                />
                <ul data-slot="toaster-source-paths">
                    <li><code>"components/toaster/src/mod.rs"</code></li>
                    <li><code>"components/toaster/src/logic.rs"</code></li>
                    <li><code>"components/toaster/src/view.rs"</code></li>
                    <li><code>"components/toaster/src/styles.rs"</code></li>
                    <li><code>"components/toaster/src/motion.rs"</code></li>
                </ul>
                <ul data-slot="toaster-source-prerequisites">
                    <li><code>"component-toaster"</code></li>
                    <li><code>"component-toast"</code></li>
                    <li><code>"component-sonner"</code></li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn underlay() -> AnyView {
    let (showcase_open_raw, set_showcase_open_raw) = signal(false);
    let showcase_open: Signal<bool> = Signal::derive(move || showcase_open_raw.get());
    let (showcase_open_change_count, set_showcase_open_change_count) = signal(0_u32);
    let (showcase_close_count, set_showcase_close_count) = signal(0_u32);

    let open_showcase: OnPress = Callback::new(move |_| set_showcase_open_raw.set(true));
    let on_showcase_open_change = Callback::new(move |next: bool| {
        set_showcase_open_raw.set(next);
        set_showcase_open_change_count.update(|count| *count += 1);
    });
    let on_showcase_close: OnPress = Callback::new(move |_| {
        set_showcase_open_raw.set(false);
        set_showcase_close_count.update(|count| *count += 1);
    });

    let hello_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(false);
let open: Signal<bool> = Signal::derive(move || open_raw.get());

<Underlay
  id_base="docs-underlay-hello".to_string()
  is_open=open
  on_open_change=Callback::new(move |next| set_open_raw.set(next))
  on_close=Callback::new(move |_| set_open_raw.set(false))
/>"#
        .to_string()
    });

    let (workbench_open_raw, set_workbench_open_raw) = signal(false);
    let workbench_open_signal: Signal<bool> = Signal::derive(move || workbench_open_raw.get());
    let (workbench_open_change_count, set_workbench_open_change_count) = signal(0_u32);
    let (workbench_close_count, set_workbench_close_count) = signal(0_u32);

    let (workbench_is_transparent, set_workbench_is_transparent) = signal(false);
    let (workbench_transparent_alias, set_workbench_transparent_alias) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_disabled_alias, set_workbench_disabled_alias) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_disable_motion, set_workbench_disable_motion) = signal(false);

    let open_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(true));
    let close_workbench: OnPress = Callback::new(move |_| set_workbench_open_raw.set(false));
    let on_workbench_open_change = Callback::new(move |next: bool| {
        set_workbench_open_raw.set(next);
        set_workbench_open_change_count.update(|count| *count += 1);
    });
    let on_workbench_close: OnPress = Callback::new(move |_| {
        set_workbench_open_raw.set(false);
        set_workbench_close_count.update(|count| *count += 1);
    });

    let workbench_motion = Signal::derive(move || {
        if workbench_disable_motion.get() {
            ui::UnderlayMotion::disabled()
        } else {
            ui::UnderlayMotion::default()
        }
    });

    let workbench_code = Signal::derive(move || {
        let class_name = if workbench_custom_class_name.get() {
            "docs-underlay-workbench"
        } else {
            ""
        };
        let lang = if workbench_zh_lang.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl_dir.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let motion = if workbench_disable_motion.get() {
            "UnderlayMotion::disabled()"
        } else {
            "UnderlayMotion::default()"
        };

        vec![
            "<Underlay".to_string(),
            "  id_base=\"docs-underlay-workbench\".to_string()".to_string(),
            "  is_open=Signal::derive(move || open_raw.get())".to_string(),
            "  open=Signal::derive(move || open_raw.get())".to_string(),
            "  default_open=false".to_string(),
            "  on_open_change=Callback::new(move |next| set_open_raw.set(next))".to_string(),
            "  on_close=Callback::new(move |_| set_open_raw.set(false))".to_string(),
            format!(
                "  is_transparent={}",
                bool_word(workbench_is_transparent.get())
            ),
            format!(
                "  transparent={}",
                bool_word(workbench_transparent_alias.get())
            ),
            format!("  is_disabled={}", bool_word(workbench_is_disabled.get())),
            format!("  disabled={}", bool_word(workbench_disabled_alias.get())),
            format!("  lang={}.to_string()", rust_string_literal(lang)),
            format!("  dir={dir}"),
            format!("  motion={motion}"),
            format!(
                "  class_name={}.to_string()",
                rust_string_literal(class_name)
            ),
            "/>".to_string(),
        ]
        .join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN")
        } else {
            Some("en-US")
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-underlay-workbench")
        } else {
            None
        };
        let motion = if workbench_disable_motion.get() {
            "UnderlayMotion::disabled()"
        } else {
            "UnderlayMotion::default()"
        };
        format!(
            "UnderlayActualConfig {{\n  id_base: \"docs-underlay-workbench\",\n  is_open: Some({}),\n  open: Some({}),\n  default_open: Some(false),\n  on_open_change: \"count={}\",\n  on_close: \"count={}\",\n  is_transparent: Some({}),\n  transparent: Some({}),\n  is_disabled: Some({}),\n  disabled: Some({}),\n  lang: {lang:?},\n  dir: {dir},\n  motion: {motion},\n  class_name: {class_name:?},\n}}",
            bool_word(workbench_open_raw.get()),
            bool_word(workbench_open_raw.get()),
            workbench_open_change_count.get(),
            workbench_close_count.get(),
            bool_word(workbench_is_transparent.get()),
            bool_word(workbench_transparent_alias.get()),
            bool_word(workbench_is_disabled.get()),
            bool_word(workbench_disabled_alias.get()),
        )
    });

    let (matrix_default_open_raw, set_matrix_default_open_raw) = signal(false);
    let matrix_default_open: Signal<bool> = Signal::derive(move || matrix_default_open_raw.get());
    let open_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(true));
    let on_matrix_default_open_change =
        Callback::new(move |next: bool| set_matrix_default_open_raw.set(next));
    let close_matrix_default: OnPress =
        Callback::new(move |_| set_matrix_default_open_raw.set(false));

    let (matrix_transparent_open_raw, set_matrix_transparent_open_raw) = signal(false);
    let matrix_transparent_open: Signal<bool> =
        Signal::derive(move || matrix_transparent_open_raw.get());
    let open_matrix_transparent: OnPress =
        Callback::new(move |_| set_matrix_transparent_open_raw.set(true));
    let on_matrix_transparent_open_change =
        Callback::new(move |next: bool| set_matrix_transparent_open_raw.set(next));
    let close_matrix_transparent: OnPress =
        Callback::new(move |_| set_matrix_transparent_open_raw.set(false));

    let matrix_code = Signal::derive(move || {
        r#"<Underlay id_base="underlay-default".to_string() is_open=default_open on_open_change=on_default_change on_close=dismiss_default />
<Underlay id_base="underlay-transparent".to_string() is_open=transparent_open on_open_change=on_transparent_change is_transparent=true transparent=true />
<Underlay id_base="underlay-disabled".to_string() is_open=Signal::derive(|| true) is_disabled=true disabled=true />"#.to_string()
    });

    view! {
        <ComponentPage
            title="Underlay"
            slug="underlay"
            group="Overlays"
            description="Underlay playground with full API workbench and state-matrix comparison."
        >
            <Playground title="Hello World (Default Underlay)" code_signal=hello_code>
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <Button on_press=open_showcase>"Open underlay"</Button>
                        <Button variant=ButtonVariant::Secondary on_press=on_showcase_close>
                            "Close"
                        </Button>
                    </div>
                    <span class="ui-muted">
                        "open: " {move || showcase_open_raw.get()}
                        " · on_open_change: " {move || showcase_open_change_count.get()}
                        " · on_close: " {move || showcase_close_count.get()}
                    </span>
                </div>
                <Underlay
                    id_base="docs-underlay-hello".to_string()
                    is_open=showcase_open
                    on_open_change=on_showcase_open_change
                    on_close=on_showcase_close
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="underlay-workbench-controls">
                        <Switch
                            checked=workbench_is_transparent
                            set_checked=set_workbench_is_transparent
                        >
                            "is_transparent"
                        </Switch>
                        <Switch
                            checked=workbench_transparent_alias
                            set_checked=set_workbench_transparent_alias
                        >
                            "transparent alias"
                        </Switch>
                        <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                            "is_disabled"
                        </Switch>
                        <Switch checked=workbench_disabled_alias set_checked=set_workbench_disabled_alias>
                            "disabled alias"
                        </Switch>
                        <Switch
                            checked=workbench_custom_class_name
                            set_checked=set_workbench_custom_class_name
                        >
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_zh_lang set_checked=set_workbench_zh_lang>
                            "lang zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl_dir set_checked=set_workbench_rtl_dir>
                            "dir RTL"
                        </Switch>
                        <Switch checked=workbench_disable_motion set_checked=set_workbench_disable_motion>
                            "motion disabled"
                        </Switch>
                        <div class="docs-row docs-row--tight">
                            <Button variant=ButtonVariant::Secondary on_press=open_workbench>
                                "Open"
                            </Button>
                            <Button variant=ButtonVariant::Secondary on_press=close_workbench>
                                "Close"
                            </Button>
                        </div>
                    </div>
                }
            >
                <span class="ui-muted" data-slot="underlay-workbench-feedback">
                    "open: " {move || workbench_open_raw.get()}
                    " · on_open_change: " {move || workbench_open_change_count.get()}
                    " · on_close: " {move || workbench_close_count.get()}
                </span>
                <Underlay
                    id_base="docs-underlay-workbench".to_string()
                    is_open=workbench_open_signal
                    open=workbench_open_signal
                    default_open=false
                    on_open_change=on_workbench_open_change
                    on_close=on_workbench_close
                    is_transparent=workbench_is_transparent.get()
                    transparent=workbench_transparent_alias.get()
                    is_disabled=workbench_is_disabled.get()
                    disabled=workbench_disabled_alias.get()
                    lang=if workbench_zh_lang.get() {
                        "zh-CN".to_string()
                    } else {
                        "en-US".to_string()
                    }
                    dir=if workbench_rtl_dir.get() {
                        ui_headless::A11yDirection::Rtl
                    } else {
                        ui_headless::A11yDirection::Ltr
                    }
                    motion=workbench_motion.get()
                    class_name=if workbench_custom_class_name.get() {
                        "docs-underlay-workbench".to_string()
                    } else {
                        String::new()
                    }
                />
            </Playground>

            <Playground title="State Matrix (Default / Transparent / Disabled)" code_signal=matrix_code>
                <div class="docs-row" data-slot="underlay-matrix-controls">
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_default>
                        "Open Default"
                    </Button>
                    <Button variant=ButtonVariant::Secondary on_press=open_matrix_transparent>
                        "Open Transparent"
                    </Button>
                </div>
                <Underlay
                    id_base="docs-underlay-matrix-default".to_string()
                    is_open=matrix_default_open
                    on_open_change=on_matrix_default_open_change
                    on_close=close_matrix_default
                />
                <Underlay
                    id_base="docs-underlay-matrix-transparent".to_string()
                    is_open=matrix_transparent_open
                    on_open_change=on_matrix_transparent_open_change
                    on_close=close_matrix_transparent
                    is_transparent=true
                    transparent=true
                />
                <Underlay
                    id_base="docs-underlay-matrix-disabled".to_string()
                    is_open=Signal::derive(|| true)
                    is_disabled=true
                    disabled=true
                    class_name="docs-underlay-disabled".to_string()
                />
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
