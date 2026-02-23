use super::*;

pub(crate) fn action_button() -> AnyView {
    let (press_count, set_press_count) = signal(0_u32);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });
    let persisted_workbench_state = load_action_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (workbench_size_index, set_workbench_size_index) =
        signal(Some(initial_workbench_state.size_index));
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ActionButtonSize::Xs,
        1 => ActionButtonSize::S,
        3 => ActionButtonSize::L,
        4 => ActionButtonSize::Xl,
        _ => ActionButtonSize::M,
    });
    let loading_placement_options =
        vec!["Start".to_string(), "End".to_string(), "Center".to_string()];
    let (workbench_loading_placement_index, set_workbench_loading_placement_index) =
        signal(Some(initial_workbench_state.loading_placement_index));
    let workbench_loading_placement = Signal::derive(
        move || match workbench_loading_placement_index.get().unwrap_or(2) {
            0 => ActionButtonLoadingPlacement::Start,
            1 => ActionButtonLoadingPlacement::End,
            _ => ActionButtonLoadingPlacement::Center,
        },
    );
    let (workbench_loading, set_workbench_loading) = signal(initial_workbench_state.is_loading);
    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.is_disabled);
    let (workbench_quiet, set_workbench_quiet) = signal(initial_workbench_state.is_quiet);
    let (workbench_icon_only, set_workbench_icon_only) =
        signal(initial_workbench_state.is_icon_only);
    let (workbench_show_start, set_workbench_show_start) =
        signal(initial_workbench_state.show_start);
    let (workbench_show_end, set_workbench_show_end) = signal(initial_workbench_state.show_end);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let (workbench_popup_expanded_raw, set_workbench_popup_expanded_raw) = signal(false);
    let workbench_popup_expanded: Signal<bool> =
        Signal::derive(move || workbench_popup_expanded_raw.get());
    let workbench_controls_signal: Signal<Option<String>> =
        Signal::derive(move || Some("docs-action-button-workbench-panel".to_string()));
    let (workbench_lang_zh, _set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, _set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let workbench_on_press: OnPress = Callback::new(move |_| {
        set_workbench_press_count.update(|count| *count += 1);
        set_workbench_popup_expanded_raw.update(|value| *value = !*value);
    });
    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_action_button_workbench_state(ActionButtonWorkbenchState {
                size_index: workbench_size_index.get().unwrap_or(2),
                loading_placement_index: workbench_loading_placement_index.get().unwrap_or(2),
                is_loading: workbench_loading.get(),
                is_disabled: workbench_disabled.get(),
                is_quiet: workbench_quiet.get(),
                is_icon_only: workbench_icon_only.get(),
                show_start: workbench_show_start.get(),
                show_end: workbench_show_end.get(),
            });
        } else {
            clear_action_button_workbench_state();
        }
    });

    let code = Signal::derive(move || {
        r#"<ActionButton
  on_press=Callback::new(move |_| {})
>
  "Action"
</ActionButton>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<ActionButton
  is_loading=true
  loading_placement=ActionButtonLoadingPlacement::Start
>
  "Start"
</ActionButton>
<ActionButton
  is_loading=true
  loading_placement=ActionButtonLoadingPlacement::End
>
  "End"
</ActionButton>"#
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<ActionButton id="ab-default".to_string() button_type=ActionButtonType::Button on_press=Callback::new(move |_| {})>"Default"</ActionButton>
<ActionButton id="ab-loading".to_string() is_loading=true loading_placement=ActionButtonLoadingPlacement::Start motion=ActionButtonMotion::default() on_press=Callback::new(move |_| {})>"Loading"</ActionButton>
<ActionButton id="ab-popup".to_string() is_quiet=true aria_haspopup="menu" aria_expanded=Signal::derive(move || true) aria_controls="popup-panel".to_string() aria_controls_signal=Signal::derive(move || Some("popup-panel".to_string())) class_name="docs-action-button-workbench".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl on_press=Callback::new(move |_| {})>"Popup"</ActionButton>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let is_loading = workbench_loading.get();
        let loading_placement = workbench_loading_placement.get();
        let is_disabled = workbench_disabled.get();
        let is_quiet = workbench_quiet.get();
        let is_icon_only = workbench_icon_only.get();
        let show_start = workbench_show_start.get();
        let show_end = workbench_show_end.get();
        let lang = if workbench_lang_zh.get() {
            "zh-CN"
        } else {
            "en-US"
        };
        let dir = if workbench_rtl.get() {
            "A11yDirection::Rtl"
        } else {
            "A11yDirection::Ltr"
        };
        let popup_expanded = workbench_popup_expanded.get();

        let mut snippet = vec![
            "<ActionButton".to_string(),
            "  id=\"docs-action-button-workbench\".to_string()".to_string(),
            format!("  size=ActionButtonSize::{size:?}"),
            format!("  is_loading={is_loading}"),
            format!("  loading_placement=ActionButtonLoadingPlacement::{loading_placement:?}"),
            format!("  is_disabled={is_disabled}"),
            format!("  is_quiet={is_quiet}"),
            format!("  is_icon_only={is_icon_only}"),
            "  motion=ActionButtonMotion::default()".to_string(),
            "  class_name=\"docs-action-button-workbench\".to_string()".to_string(),
            "  button_type=ActionButtonType::Button".to_string(),
            "  aria_haspopup=Some(\"menu\")".to_string(),
            format!("  aria_expanded=Signal::derive(move || {popup_expanded})"),
            "  aria_controls=\"docs-action-button-workbench-panel\".to_string()".to_string(),
            "  aria_controls_signal=Signal::derive(move || Some(\"docs-action-button-workbench-panel\".to_string()))".to_string(),
            format!("  lang=\"{lang}\".to_string()"),
            format!("  dir={dir}"),
            "  node_ref=node_ref".to_string(),
            "  on_press=on_press".to_string(),
        ];

        if is_icon_only {
            snippet.push("  aria_label=\"Action\".into()".to_string());
        }
        if show_start {
            snippet.push("  start_content=move || view! { <span>\"★\"</span> }".to_string());
        }
        if show_end {
            snippet.push("  end_content=move || view! { <span>\"→\"</span> }".to_string());
        }

        snippet.extend([
            ">".to_string(),
            if is_icon_only {
                "  \"★\"".to_string()
            } else {
                "  \"Action\"".to_string()
            },
            "</ActionButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/styles.rs */\n{}",
            ui::button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let is_loading = workbench_loading.get();
        let loading_placement = workbench_loading_placement.get();
        let is_disabled = workbench_disabled.get();
        let is_quiet = workbench_quiet.get();
        let is_icon_only = workbench_icon_only.get();
        let show_start = workbench_show_start.get();
        let show_end = workbench_show_end.get();
        let popup_expanded = workbench_popup_expanded.get();
        format!(
            "ActionButtonActualConfig {{\n  id: Some(\"docs-action-button-workbench\"),\n  is_loading: {is_loading},\n  is_disabled: Some({is_disabled}),\n  size: Some({size:?}),\n  is_quiet: Some({is_quiet}),\n  motion: ActionButtonMotion::default(),\n  loading_placement: {loading_placement:?},\n  class_name: Some(\"docs-action-button-workbench\"),\n  button_type: Some(ActionButtonType::Button),\n  aria_label: {:?},\n  aria_haspopup: Some(\"menu\"),\n  aria_expanded: Some({popup_expanded}),\n  aria_controls: Some(\"docs-action-button-workbench-panel\"),\n  aria_controls_signal: Some(\"docs-action-button-workbench-panel\"),\n  lang: Some({:?}),\n  dir: Some({:?}),\n  node_ref: Some(\"workbench_node_ref\"),\n  on_press: \"count={} toggles_popup=true\",\n  has_start_content: {show_start},\n  has_end_content: {show_end},\n}}",
            if is_icon_only { Some("Action") } else { None },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
            workbench_press_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="ActionButton"
            slug="action-button"
            group="Actions"
            description="baseline-style action trigger with state attrs and baseline-level spring hover/press feedback."
        >
            <Playground title="Default + callback" code_signal=code>
                <div class="docs-row">
                    <ActionButton on_press=on_press>"Action"</ActionButton>
                    <ActionButton is_quiet=true on_press=on_press>"Quiet"</ActionButton>
                    <ActionButton
                        is_loading=true
                        loading_placement=ActionButtonLoadingPlacement::Center
                    >
                        "Loading"
                    </ActionButton>
                    <span class="ui-muted">
                        "pressed: "
                        {move || press_count.get().to_string()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="ActionButton Workbench"
                code_signal=workbench_code
                test_css_source=workbench_test_css_source
                test_source_path="/root/code/personal/omne/rust-ui/crates/ui/src/button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Workbench canvas: action-button reuses button css contract, supports scoped css live-edit, and optional state persistence."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="action-button-workbench-controls">
                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-action-button-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButton size".to_string()
                        />
                        <div class="docs-search__label">"Loading placement"</div>
                        <SegmentedControl
                            id_base="docs-action-button-loading-placement".to_string()
                            options=loading_placement_options.clone()
                            selected_index=workbench_loading_placement_index
                            set_selected_index=set_workbench_loading_placement_index
                            size=SegmentedControlSize::Sm
                            aria_label="ActionButton loading placement".to_string()
                        />
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </Switch>
                        <Switch checked=workbench_loading set_checked=set_workbench_loading>
                            "Loading"
                        </Switch>
                        <Switch checked=workbench_quiet set_checked=set_workbench_quiet>
                            "Quiet"
                        </Switch>
                        <Switch checked=workbench_icon_only set_checked=set_workbench_icon_only>
                            "Icon only"
                        </Switch>
                        <Switch checked=workbench_show_start set_checked=set_workbench_show_start>
                            "Start slot"
                        </Switch>
                        <Switch checked=workbench_show_end set_checked=set_workbench_show_end>
                            "End slot"
                        </Switch>
                        <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let size = workbench_size.get();
                    let is_loading = workbench_loading.get();
                    let loading_placement = workbench_loading_placement.get();
                    let is_disabled = workbench_disabled.get();
                    let is_quiet = workbench_quiet.get();
                    let is_icon_only = workbench_icon_only.get();
                    let _show_start = workbench_show_start.get();
                    let _show_end = workbench_show_end.get();
                    let persist = workbench_persist_state.get();

                    view! {
                        <div class="docs-stack" data-slot="action-button-workbench" style="width: min(100%, 360px);">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>
                            <div class="docs-card docs-stack docs-stack--tight" data-slot="action-button-workbench-canvas">
                                <div class="docs-row" style="justify-content: center;">
                                    <ActionButton
                                        id="docs-action-button-workbench".to_string()
                                        size=size
                                        is_loading=is_loading
                                        loading_placement=loading_placement
                                        is_disabled=is_disabled
                                        is_quiet=is_quiet
                                        motion=ActionButtonMotion::default()
                                        class_name="docs-action-button-workbench".to_string()
                                        button_type=ActionButtonType::Button
                                        aria_label=if is_icon_only { "Action".to_string() } else { String::new() }
                                        aria_haspopup="menu"
                                        aria_expanded=workbench_popup_expanded
                                        aria_controls="docs-action-button-workbench-panel".to_string()
                                        aria_controls_signal=workbench_controls_signal
                                        lang=if workbench_lang_zh.get() { "zh-CN".to_string() } else { "en-US".to_string() }
                                        dir=if workbench_rtl.get() { A11yDirection::Rtl } else { A11yDirection::Ltr }
                                        node_ref=workbench_node_ref
                                        on_press=workbench_on_press
                                    >
                                        {if is_icon_only { "★" } else { "Action" }}
                                    </ActionButton>
                                </div>
                                <div id="docs-action-button-workbench-panel" class="ui-muted">
                                    "popup expanded: " {move || workbench_popup_expanded_raw.get()}
                                </div>
                                <div class="ui-muted">
                                    "workbench on_press count: " {move || workbench_press_count.get()}
                                </div>
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Default / Loading / Popup)" code_signal=matrix_code>
                <div class="docs-row">
                    <ActionButton
                        id="docs-action-button-matrix-default".to_string()
                        button_type=ActionButtonType::Button
                        on_press=on_press
                    >
                        "Default"
                    </ActionButton>
                    <ActionButton
                        id="docs-action-button-matrix-loading".to_string()
                        is_loading=true
                        loading_placement=ActionButtonLoadingPlacement::Start
                        motion=ActionButtonMotion::default()
                        on_press=on_press
                    >
                        "Loading"
                    </ActionButton>
                    <ActionButton
                        id="docs-action-button-matrix-popup".to_string()
                        is_quiet=true
                        aria_haspopup="menu"
                        aria_expanded=Signal::derive(move || true)
                        aria_controls="docs-action-button-matrix-popup-panel".to_string()
                        aria_controls_signal=Signal::derive(move || {
                            Some("docs-action-button-matrix-popup-panel".to_string())
                        })
                        class_name="docs-action-button-workbench".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                        node_ref=NodeRef::new()
                        on_press=on_press
                    >
                        "Popup"
                    </ActionButton>
                </div>
            </Playground>

            <Playground title="Loading placement + icon-only" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <ActionButton
                            size=ActionButtonSize::S
                            is_loading=true
                            loading_placement=ActionButtonLoadingPlacement::Start
                        >
                            "Start"
                        </ActionButton>
                        <ActionButton
                            size=ActionButtonSize::L
                            is_loading=true
                            loading_placement=ActionButtonLoadingPlacement::End
                        >
                            "End"
                        </ActionButton>
                        <ActionButton is_quiet=true aria_label="Settings".to_string()>
                            <svg viewBox="0 0 20 20" fill="none" aria-hidden="true">
                                <path
                                    d="M10 13.3a3.3 3.3 0 1 0 0-6.6a3.3 3.3 0 0 0 0 6.6Z"
                                    stroke="currentColor"
                                    stroke_width="1.5"
                                />
                                <path
                                    d="M3.8 10a6.2 6.2 0 0 1 .1-1l1.6-.9.2-.5-.6-1.8a7.6 7.6 0 0 1 1.5-1.5l1.8.6.5-.2.9-1.6a6.4 6.4 0 0 1 2 0l.9 1.6.5.2 1.8-.6c.6.4 1.1.9 1.5 1.5l-.6 1.8.2.5 1.6.9a6.5 6.5 0 0 1 0 2l-1.6.9-.2.5.6 1.8a7.6 7.6 0 0 1-1.5 1.5l-1.8-.6-.5.2-.9 1.6a6.4 6.4 0 0 1-2 0l-.9-1.6-.5-.2-1.8.6a7.6 7.6 0 0 1-1.5-1.5l.6-1.8-.2-.5-1.6-.9a6.2 6.2 0 0 1-.1-1Z"
                                    stroke="currentColor"
                                    stroke_width="1.2"
                                    stroke_linecap="round"
                                    stroke_linejoin="round"
                                />
                            </svg>
                        </ActionButton>
                    </div>
                    <span class="ui-muted">
                        "Start/end slots, loading placement, and icon-only mode all expose stable data-* attrs."
                    </span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
