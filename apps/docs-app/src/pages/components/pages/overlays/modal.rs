use super::*;

pub(crate) fn modal() -> AnyView {
    let (minimal_present, set_minimal_present) = signal(false);
    let open_minimal_modal: OnPress = Callback::new(move |_| set_minimal_present.set(true));
    let close_minimal_modal: OnPress = Callback::new(move |_| set_minimal_present.set(false));
    let on_minimal_open_change: Callback<bool> = Callback::new(move |next: bool| {
        if !next {
            set_minimal_present.set(false);
        }
    });

    let (interactive_open_raw, set_interactive_open_raw) = signal(false);
    let interactive_open: Signal<bool> = Signal::derive(move || interactive_open_raw.get());
    let open_interactive_modal: OnPress =
        Callback::new(move |_| set_interactive_open_raw.set(true));
    let close_interactive_modal: OnPress =
        Callback::new(move |_| set_interactive_open_raw.set(false));
    let (interactive_with_description, set_interactive_with_description) = signal(true);
    let (interactive_custom_id, set_interactive_custom_id) = signal(true);
    let (interactive_custom_title, set_interactive_custom_title) = signal(true);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);
    let (interactive_custom_motion, set_interactive_custom_motion) = signal(false);
    let (interactive_custom_exit, set_interactive_custom_exit) = signal(false);

    let interactive_code = Signal::derive(move || {
        let with_description = interactive_with_description.get();
        let custom_id = interactive_custom_id.get();
        let custom_title = interactive_custom_title.get();
        let custom_class = interactive_custom_class.get();
        let custom_motion = interactive_custom_motion.get();
        let custom_exit = interactive_custom_exit.get();

        let mut lines = vec![
            "let (open, set_open) = signal(false);".to_string(),
            "let close: OnPress = Callback::new(move |_| set_open.set(false));".to_string(),
            "".to_string(),
            "<Modal".to_string(),
            "  is_open=Signal::derive(move || open.get())".to_string(),
            format!(
                "  id_base={}",
                if custom_id {
                    "\"docs-modal-interactive\".into()"
                } else {
                    "\" \".into()"
                }
            ),
            format!(
                "  title={}",
                if custom_title {
                    "\"Action required\".into()"
                } else {
                    "\" \".into()"
                }
            ),
            "  on_close=close".to_string(),
        ];

        if with_description {
            lines.push("  description=\"Review settings before confirming.\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-modal-custom\".into()".to_string());
        }
        if custom_motion {
            lines.push("  motion=OverlayMotion {".to_string());
            lines.push("    initial_scale: 0.92,".to_string());
            lines.push("    initial_y_px: 18.0,".to_string());
            lines.push("    ..OverlayMotion::default()".to_string());
            lines.push("  }".to_string());
        }
        if custom_exit {
            lines.push("  on_exit_complete=Callback::new(move |_| {})".to_string());
        }
        lines.push("  lang=\"en-US\".into()".to_string());
        lines.push("  dir=A11yDirection::Ltr".to_string());
        lines.push(">".to_string());
        lines.push("  ...".to_string());
        lines.push("</Modal>".to_string());
        lines.join("\n")
    });

    let interactive_test_css = Signal::derive(move || {
        format!(
            "/* components/modal/src/styles.rs */\n{}",
            ui::modal::styles::CSS
        )
    });

    let interactive_config = Signal::derive(move || {
        let id_base = if interactive_custom_id.get() {
            "docs-modal-interactive"
        } else {
            "ui-modal"
        };
        let title = if interactive_custom_title.get() {
            "Action required"
        } else {
            "Modal"
        };
        let description = if interactive_with_description.get() {
            Some("Review settings before confirming.")
        } else {
            None
        };
        format!(
            "ModalActualConfig {{\n  is_open: Some({}),\n  default_open: None,\n  on_open_change: \"none\",\n  id_base: {:?},\n  title: {:?},\n  on_close: \"close_interactive_modal\",\n  description: {:?},\n  motion: {},\n  on_exit_complete: {},\n  lang: Some(\"en-US\"),\n  dir: Some(A11yDirection::Ltr),\n  class_name: {},\n  id_source: {},\n  title_source: {},\n  description_source: {},\n  class_source: {},\n  motion_source: {},\n  exit_source: {},\n}}",
            interactive_open_raw.get(),
            id_base,
            title,
            description,
            if interactive_custom_motion.get() {
                "OverlayMotion::custom"
            } else {
                "OverlayMotion::default"
            },
            if interactive_custom_exit.get() {
                "\"custom\""
            } else {
                "\"none\""
            },
            if interactive_custom_class.get() {
                "Some(\"docs-modal-custom\")"
            } else {
                "None"
            },
            if interactive_custom_id.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_title.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_with_description.get() {
                "\"present\""
            } else {
                "\"absent\""
            },
            if interactive_custom_class.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_motion.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
            if interactive_custom_exit.get() {
                "\"custom\""
            } else {
                "\"default\""
            },
        )
    });

    let (open_semantic_raw, set_open_semantic_raw) = signal(false);
    let open_semantic: Signal<bool> = Signal::derive(move || open_semantic_raw.get());
    let (present_semantic, set_present_semantic) = signal(open_semantic.get_untracked());
    Effect::new(move |_| {
        if open_semantic.get() {
            set_present_semantic.set(true);
        }
    });

    let close_semantic: OnPress = Callback::new(move |_| set_open_semantic_raw.set(false));
    let open_semantic_modal: OnPress = Callback::new(move |_| set_open_semantic_raw.set(true));
    let on_semantic_exit_complete = Callback::new(move |_| set_present_semantic.set(false));

    let (open_custom_raw, set_open_custom_raw) = signal(false);
    let open_custom: Signal<bool> = Signal::derive(move || open_custom_raw.get());
    let (present_custom, set_present_custom) = signal(open_custom.get_untracked());
    Effect::new(move |_| {
        if open_custom.get() {
            set_present_custom.set(true);
        }
    });

    let close_custom: OnPress = Callback::new(move |_| set_open_custom_raw.set(false));
    let open_custom_modal: OnPress = Callback::new(move |_| set_open_custom_raw.set(true));
    let on_custom_exit_complete = Callback::new(move |_| set_present_custom.set(false));

    let custom_motion = OverlayMotion {
        initial_scale: 0.92,
        initial_y_px: 18.0,
        ..OverlayMotion::default()
    };

    let semantic_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let open: Signal<bool> = Signal::derive(move || open_raw.get());
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let on_exit_complete = Callback::new(move |_| {});

<Modal
  is_open=open
  id_base="m".to_string()
  title="Confirm".to_string()
  description="Modal composes Overlay and wires aria attributes.".to_string()
  on_close=close
  on_exit_complete=on_exit_complete
>
  ...
</Modal>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"let (open_raw, set_open_raw) = signal(true);
let close: OnPress = Callback::new(move |_| set_open_raw.set(false));
let on_exit_complete = Callback::new(move |_| {});
let custom_motion = OverlayMotion {
  initial_scale: 0.92,
  initial_y_px: 18.0,
  ..OverlayMotion::default()
};

<Modal
  is_open=Signal::derive(move || open_raw.get())
  id_base="m-custom".to_string()
  title="Title only".to_string()
  class_name="docs-modal-custom".to_string()
  motion=custom_motion
  on_close=close
  on_exit_complete=on_exit_complete
>
  ...
</Modal>"#
            .to_string()
    });

    let state_matrix_options = vec![
        "Uncontrolled + default_open=true".to_string(),
        "Uncontrolled + no description".to_string(),
        "Controlled + custom title".to_string(),
    ];
    let (state_matrix_index, set_state_matrix_index) = signal(Some(0_usize));
    let state_matrix_is_controlled =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 2);
    let state_matrix_default_open =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) == 0);
    let state_matrix_with_description =
        Signal::derive(move || state_matrix_index.get().unwrap_or(0) != 1);
    let (state_matrix_open_raw, set_state_matrix_open_raw) = signal(false);
    let state_matrix_open: Signal<bool> = Signal::derive(move || state_matrix_open_raw.get());
    let on_state_matrix_open_change =
        Callback::new(move |next: bool| set_state_matrix_open_raw.set(next));
    let close_state_matrix_modal: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(false));
    let open_state_matrix_modal: OnPress =
        Callback::new(move |_| set_state_matrix_open_raw.set(true));
    let state_matrix_code = Signal::derive(move || {
        let scenario = state_matrix_index.get().unwrap_or(0);
        let mut lines = vec![
            "<Modal".to_string(),
            "  id_base=\"docs-modal-state-matrix\".into()".to_string(),
            "  title=\"State Matrix\".into()".to_string(),
            "  on_close=Callback::new(|_| {})".to_string(),
        ];

        match scenario {
            0 => {
                lines.push("  default_open=true".to_string());
                lines.push("  description=\"Uncontrolled baseline branch\".into()".to_string());
            }
            1 => {
                lines.push("  default_open=false".to_string());
            }
            _ => {
                lines.push("  is_open=Signal::derive(move || open_raw.get())".to_string());
                lines.push(
                    "  on_open_change=Callback::new(move |next: bool| set_open_raw.set(next))"
                        .to_string(),
                );
                lines.push("  title=\"Controlled Matrix\".into()".to_string());
                lines.push("  description=\"Controlled branch\".into()".to_string());
            }
        }
        lines.push(">".to_string());
        lines.push("  <div>\"Matrix content\"</div>".to_string());
        lines.push("</Modal>".to_string());
        lines.join("\n")
    });

    let (compare_controlled_open_raw, set_compare_controlled_open_raw) = signal(false);
    let compare_controlled_open: Signal<bool> =
        Signal::derive(move || compare_controlled_open_raw.get());
    let on_compare_controlled_open_change =
        Callback::new(move |next: bool| set_compare_controlled_open_raw.set(next));
    let (compare_uncontrolled_open_raw, set_compare_uncontrolled_open_raw) = signal(true);
    let on_compare_uncontrolled_open_change =
        Callback::new(move |next: bool| set_compare_uncontrolled_open_raw.set(next));
    let compare_code = Signal::derive(move || {
        r#"let (controlled_open_raw, set_controlled_open_raw) = signal(false);
let controlled_open: Signal<bool> = Signal::derive(move || controlled_open_raw.get());
let (uncontrolled_open_raw, set_uncontrolled_open_raw) = signal(true);

<Modal
  id_base="docs-modal-compare-controlled".into()
  title="Controlled".into()
  is_open=controlled_open
  on_open_change=Callback::new(move |next: bool| set_controlled_open_raw.set(next))
  on_close=Callback::new(move |_| set_controlled_open_raw.set(false))
/>

<Modal
  id_base="docs-modal-compare-uncontrolled".into()
  title="Uncontrolled".into()
  default_open=true
  on_open_change=Callback::new(move |next: bool| set_uncontrolled_open_raw.set(next))
  on_close=Callback::new(move |_| {})
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
    let streaming_snapshot_code = Signal::derive(move || {
        r#"// Modal is not an LLM body reader surface.
// Streaming is optional; fallback stays snapshot.
<Modal
  id_base="docs-modal-stream".into()
  title="Streaming Optional Contract".into()
  default_open=true
  on_close=Callback::new(move |_| {})
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Modal"
            slug="modal"
            group="Overlays"
            description="Overlay composition with centralized title/description/class state attrs and stable modal slots."
        >
            <Playground
                title="Hello World (Minimal Path)"
                description="Default path: no manual state-machine wiring, simple props only."
                code_signal=Signal::derive(move || MODAL_MINIMAL_PLAYGROUND_CODE.to_string())
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row">
                    <Button on_press=open_minimal_modal>"Open minimal modal"</Button>
                </div>

                <Show when=move || minimal_present.get()>
                    <Modal
                        default_open=true
                        id_base="docs-modal-minimal".to_string()
                        title="Hello".to_string()
                        on_close=close_minimal_modal
                        on_open_change=on_minimal_open_change
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Minimal modal content"</div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Display + Config + Code + CSS Test: toggle source contracts and inspect actual normalized config."
                code_signal=interactive_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
                test_css_source=interactive_test_css
                test_source_path="components/modal/src/styles.rs".to_string()
                test_config_signal=interactive_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <Switch
                            checked=interactive_with_description
                            set_checked=set_interactive_with_description
                        >
                            "Description"
                        </Switch>
                        <Switch checked=interactive_custom_id set_checked=set_interactive_custom_id>
                            "Custom id_base"
                        </Switch>
                        <Switch
                            checked=interactive_custom_title
                            set_checked=set_interactive_custom_title
                        >
                            "Custom title"
                        </Switch>
                        <Switch
                            checked=interactive_custom_class
                            set_checked=set_interactive_custom_class
                        >
                            "Custom class"
                        </Switch>
                        <Switch
                            checked=interactive_custom_motion
                            set_checked=set_interactive_custom_motion
                        >
                            "Custom motion"
                        </Switch>
                        <Switch
                            checked=interactive_custom_exit
                            set_checked=set_interactive_custom_exit
                        >
                            "Custom exit callback"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let motion = if interactive_custom_motion.get() {
                        OverlayMotion {
                            initial_scale: 0.92,
                            initial_y_px: 18.0,
                            ..OverlayMotion::default()
                        }
                    } else {
                        OverlayMotion::default()
                    };
                    let class_name = if interactive_custom_class.get() {
                        "docs-modal-custom".to_string()
                    } else {
                        String::new()
                    };
                    let description = if interactive_with_description.get() {
                        "Review settings before confirming.".to_string()
                    } else {
                        String::new()
                    };
                    let id_base = if interactive_custom_id.get() {
                        "docs-modal-interactive".to_string()
                    } else {
                        " ".to_string()
                    };
                    let title = if interactive_custom_title.get() {
                        "Action required".to_string()
                    } else {
                        " ".to_string()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-row" attr:data-slot="modal-interactive-controls">
                                <Button attr:data-slot="modal-interactive-open" on_press=open_interactive_modal>
                                    "Open interactive modal"
                                </Button>
                                <span class="ui-muted">
                                    "open: " {move || interactive_open_raw.get()}
                                </span>
                            </div>

                            {if interactive_custom_exit.get() {
                                view! {
                                    <Modal
                                        is_open=interactive_open
                                        id_base=id_base.clone()
                                        title=title.clone()
                                        on_close=close_interactive_modal
                                        description=description.clone()
                                        motion=motion
                                        on_exit_complete=Callback::new(move |_| {})
                                        lang="en-US".to_string()
                                        dir=ui_headless::A11yDirection::Ltr
                                        class_name=class_name.clone()
                                    >
                                        <div class="docs-stack docs-stack--tight">
                                            <div>"Inspect root markers in DevTools while toggling config."</div>
                                            <div class="docs-row docs-row--end">
                                                <Button
                                                    attr:data-slot="modal-interactive-close"
                                                    variant=ButtonVariant::Secondary
                                                    on_press=close_interactive_modal
                                                >
                                                    "Close"
                                                </Button>
                                            </div>
                                        </div>
                                    </Modal>
                                }
                                    .into_any()
                            } else {
                                view! {
                                    <Modal
                                        is_open=interactive_open
                                        id_base=id_base
                                        title=title
                                        on_close=close_interactive_modal
                                        description=description
                                        motion=motion
                                        lang="en-US".to_string()
                                        dir=ui_headless::A11yDirection::Ltr
                                        class_name=class_name
                                    >
                                        <div class="docs-stack docs-stack--tight">
                                            <div>"Inspect root markers in DevTools while toggling config."</div>
                                            <div class="docs-row docs-row--end">
                                                <Button
                                                    attr:data-slot="modal-interactive-close"
                                                    variant=ButtonVariant::Secondary
                                                    on_press=close_interactive_modal
                                                >
                                                    "Close"
                                                </Button>
                                            </div>
                                        </div>
                                    </Modal>
                                }
                                    .into_any()
                            }}
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="Label + Description"
                code_signal=semantic_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="modal-e2e-described-controls">
                    <Button attr:data-slot="modal-e2e-open-described" on_press=open_semantic_modal>
                        "Open described modal"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_semantic_raw.get()}</span>
                </div>

                <Show when=move || present_semantic.get()>
                    <Modal
                        is_open=open_semantic
                        id_base="docs-modal-semantic".to_string()
                        title="Confirm".to_string()
                        description="Modal composes Overlay with stable aria-labelledby + aria-describedby wiring.".to_string()
                        on_close=close_semantic
                        on_exit_complete=on_semantic_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"Described modal content"</div>
                            <div class="ui-muted">"Esc/backdrop closes, focus remains trapped in panel."</div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_semantic>
                                    "Cancel"
                                </Button>
                                <Button on_press=close_semantic>"Confirm"</Button>
                            </div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-id-source`, `data-title-source`, `data-description-source`, `data-motion-source`, and `data-exit-source` contracts."
                code_signal=custom_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-row" attr:data-slot="modal-e2e-custom-controls">
                    <Button attr:data-slot="modal-e2e-open-custom" on_press=open_custom_modal>
                        "Open custom modal"
                    </Button>
                    <span class="ui-muted">"open: " {move || open_custom_raw.get()}</span>
                </div>

                <Show when=move || present_custom.get()>
                    <Modal
                        is_open=open_custom
                        id_base="docs-modal-custom".to_string()
                        title="Title only".to_string()
                        class_name="docs-modal-custom".to_string()
                        motion=custom_motion
                        on_close=close_custom
                        on_exit_complete=on_custom_exit_complete
                    >
                        <div class="docs-stack docs-stack--tight">
                            <div>"No description path keeps aria-describedby unset."</div>
                            <div class="ui-muted">
                                "Inspect data-id-source / data-title-source / data-motion-source in DevTools."
                            </div>
                            <div class="docs-row docs-row--end">
                                <Button variant=ButtonVariant::Secondary on_press=close_custom>
                                    "Dismiss"
                                </Button>
                            </div>
                        </div>
                    </Modal>
                </Show>
            </Playground>

            <Playground
                title="State Matrix"
                description="State matrix over controlled/uncontrolled + default_open + description branches."
                code_signal=state_matrix_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="modal-state-matrix">
                    <SegmentedControl
                        id_base="docs-modal-state-matrix-scenario".to_string()
                        options=state_matrix_options.clone()
                        selected_index=state_matrix_index
                        set_selected_index=set_state_matrix_index
                        size=SegmentedControlSize::Sm
                        aria_label="Modal state matrix scenario".to_string()
                    />
                    <div class="docs-row">
                        <Button on_press=open_state_matrix_modal>"Open matrix modal"</Button>
                        <span class="ui-muted">
                            "controlled_open: " {move || state_matrix_open_raw.get().to_string()}
                        </span>
                    </div>
                    {move || {
                        if state_matrix_is_controlled.get() {
                            view! {
                                <Modal
                                    id_base="docs-modal-state-matrix".to_string()
                                    title="Controlled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Controlled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    is_open=state_matrix_open
                                    on_open_change=on_state_matrix_open_change
                                    on_close=close_state_matrix_modal
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix controlled branch"</div>
                                    </div>
                                </Modal>
                            }
                            .into_any()
                        } else {
                            view! {
                                <Modal
                                    id_base="docs-modal-state-matrix".to_string()
                                    title="Uncontrolled Matrix".to_string()
                                    description=if state_matrix_with_description.get() {
                                        "Uncontrolled matrix path".to_string()
                                    } else {
                                        String::new()
                                    }
                                    default_open=state_matrix_default_open.get()
                                    on_open_change=on_state_matrix_open_change
                                    on_close=Callback::new(|_| {})
                                >
                                    <div class="docs-stack docs-stack--tight">
                                        <div>"State matrix uncontrolled branch"</div>
                                    </div>
                                </Modal>
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted">
                        "mode: "
                        {move || if state_matrix_is_controlled.get() { "controlled" } else { "uncontrolled" }}
                    </span>
                    <span class="ui-muted">
                        "default_open: " {move || state_matrix_default_open.get()}
                    </span>
                    <span class="ui-muted">
                        "with_description: " {move || state_matrix_with_description.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                description="Side-by-side contrast of parent-controlled open state vs default-driven internal state."
                code_signal=compare_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" attr:data-slot="modal-controlled-uncontrolled">
                    <div class="docs-row">
                        <Button on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(true))>
                            "Open controlled modal"
                        </Button>
                        <Button
                            variant=ButtonVariant::Secondary
                            on_press=Callback::new(move |_| set_compare_controlled_open_raw.set(false))
                        >
                            "Close controlled modal"
                        </Button>
                    </div>
                    <div class="docs-row">
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Controlled"</strong>
                            <Modal
                                id_base="docs-modal-compare-controlled".to_string()
                                title="Controlled".to_string()
                                description="open + on_open_change are owned by parent signal.".to_string()
                                is_open=compare_controlled_open
                                on_open_change=on_compare_controlled_open_change
                                on_close=Callback::new(move |_| set_compare_controlled_open_raw.set(false))
                            >
                                <div>"Controlled content"</div>
                            </Modal>
                            <span class="ui-muted">
                                "open: "
                                {move || if compare_controlled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                        <div class="docs-stack docs-stack--tight">
                            <strong>"Uncontrolled"</strong>
                            <Modal
                                id_base="docs-modal-compare-uncontrolled".to_string()
                                title="Uncontrolled".to_string()
                                description="default_open initializes once; primitive owns later transitions.".to_string()
                                default_open=true
                                on_open_change=on_compare_uncontrolled_open_change
                                on_close=Callback::new(|_| {})
                            >
                                <div>"Uncontrolled content"</div>
                            </Modal>
                            <span class="ui-muted">
                                "open (reported by on_open_change): "
                                {move || if compare_uncontrolled_open_raw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Modal is streaming-optional and snapshot-first (`fallback=snapshot`)."
                code_signal=streaming_snapshot_code
                code_imports=MODAL_DOC_IMPORTS.to_string()
            >
                <div
                    class="docs-stack docs-stack--tight"
                    attr:data-slot="modal-streaming-contract"
                    data-requested-stream-mode=move || stream_requested_mode.get()
                    data-requested-output-status=move || stream_requested_output_status.get()
                >
                    <SegmentedControl
                        id_base="docs-modal-stream-mode".to_string()
                        options=stream_mode_options.clone()
                        selected_index=stream_mode_index
                        set_selected_index=set_stream_mode_index
                        size=SegmentedControlSize::Sm
                        aria_label="Modal stream mode".to_string()
                    />
                    <Modal
                        id_base="docs-modal-stream".to_string()
                        title="Streaming Optional Contract".to_string()
                        description="Component output stays snapshot while keeping output status machine-readable.".to_string()
                        default_open=true
                        on_close=Callback::new(|_| {})
                    >
                        <div>"This component defaults to snapshot rendering."</div>
                    </Modal>
                    <span class="ui-muted">
                        "requested mode: " {move || stream_requested_mode.get()}
                    </span>
                    <span class="ui-muted">
                        "requested output status: " {move || stream_requested_output_status.get()}
                    </span>
                    <span class="ui-muted">
                        "effective component status: data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <div class="docs-stack docs-stack--tight" attr:data-slot="modal-source-first">
                <h3>"Source-first Copy-Paste"</h3>
                <p class="ui-muted">
                    "Use "
                    <code>"Show code"</code>
                    " in any playground to copy import-ready snippets."
                </p>
                <p class="ui-muted">
                    "Imports are auto-completed via "
                    <code>"MODAL_DOC_IMPORTS"</code>
                    " + "
                    <code>"compose_copy_ready_code"</code>
                    "."
                </p>
                <p class="ui-muted">
                    "Dependency prerequisites: "
                    <code>
                        "ui = { workspace = true, default-features = false, features = [\"component-modal\", \"inject-css\"] }"
                    </code>
                </p>
                <p class="ui-muted" attr:data-slot="modal-defaults-contract">
                    "Logic defaults stay synchronized with "
                    <code>"components/modal/src/logic.rs"</code>
                    ": "
                    <code>"id_base=\"ui-modal\""</code>
                    ", "
                    <code>"title=\"Modal\""</code>
                    ", "
                    <code>"default_open=false"</code>
                    "."
                </p>
                <ul class="docs-stack docs-stack--tight" attr:data-slot="modal-source-paths">
                    <li><code>"components/modal/src/mod.rs"</code></li>
                    <li><code>"components/modal/src/logic.rs"</code></li>
                    <li><code>"components/modal/src/view.rs"</code></li>
                    <li><code>"components/modal/src/styles.rs"</code></li>
                    <li><code>"components/modal/src/motion.rs"</code></li>
                </ul>
            </div>
        </ComponentPage>
    }
    .into_any()
}
