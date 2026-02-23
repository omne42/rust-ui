use super::*;

pub(crate) fn tabs() -> AnyView {
    let manual_labels = vec!["Profile", "Billing", "Team"];
    let workbench_labels = vec!["Overview", "Details", "Settings"];

    let (selected_auto, set_selected_auto) = signal(0_usize);
    let on_auto_change = Callback::new(move |index: usize| set_selected_auto.set(index));

    let (selected_manual, set_selected_manual) = signal(1_usize);
    let on_manual_change = Callback::new(move |index: usize| set_selected_manual.set(index));

    let persisted_tabs_workbench_selected = load_tabs_workbench_selected();
    let (tabs_workbench_selected, set_tabs_workbench_selected) =
        signal(persisted_tabs_workbench_selected.unwrap_or(0_usize));
    let on_tabs_workbench_change =
        Callback::new(move |index: usize| set_tabs_workbench_selected.set(index));
    let (tabs_workbench_manual_mode, set_tabs_workbench_manual_mode) = signal(false);
    let (tabs_workbench_disable_settings, set_tabs_workbench_disable_settings) = signal(false);
    let (tabs_workbench_persist_state, set_tabs_workbench_persist_state) =
        signal(persisted_tabs_workbench_selected.is_some());

    Effect::new(move |_| {
        let selected_index = tabs_workbench_selected.get();
        if tabs_workbench_persist_state.get() {
            save_tabs_workbench_selected(selected_index);
        } else {
            clear_tabs_workbench_selected();
        }
    });

    let hello_world_code = Signal::derive(move || {
        r#"<Tabs labels=vec!["Overview", "Details", "Settings"] id_base="tabs".to_string()>
  <div>"Overview panel"</div>
  <div>"Details panel"</div>
  <div>"Settings panel"</div>
</Tabs>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(0_usize);
let on_change = Callback::new(move |next: usize| set_selected.set(next));
<Tabs
  labels=vec!["Overview", "Details", "Settings"]
  id_base="tabs".to_string()
  selected_index=selected
  on_selection_change=on_change
  keyboard_activation=TabsKeyboardActivation::Automatic
>
  <div>"Overview panel"</div>
  <div>"Details panel"</div>
  <div>"Settings panel"</div>
</Tabs>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"let (selected, set_selected) = signal(1_usize);
let on_change = Callback::new(move |next: usize| set_selected.set(next));
<Tabs
  labels=vec!["Profile", "Billing", "Team"]
  id_base="tabs-manual".to_string()
  keyboard_activation=TabsKeyboardActivation::Manual
  selected_index=selected
  on_selection_change=on_change
  disabled_indices=vec![2]
>
  <div>"Profile panel"</div>
  <div>"Billing panel"</div>
  <div>"Team panel"</div>
</Tabs>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let selected_index = tabs_workbench_selected.get();
        let keyboard_activation = if tabs_workbench_manual_mode.get() {
            "TabsKeyboardActivation::Manual"
        } else {
            "TabsKeyboardActivation::Automatic"
        };
        let disabled_indices = if tabs_workbench_disable_settings.get() {
            "vec![2]"
        } else {
            "Vec::<usize>::new()"
        };
        let persist_selected_index = bool_word(tabs_workbench_persist_state.get());

        format!(
            "let saved = load_tabs_workbench_selected();\n\
let (selected, set_selected) = signal(saved.unwrap_or({selected_index}_usize));\n\
let on_change = Callback::new(move |next: usize| set_selected.set(next));\n\
// Workbench keeps interaction context and can optionally persist selected index.\n\
<Tabs\n\
  labels=vec![\"Overview\", \"Details\", \"Settings\"]\n\
  id_base=\"tabs-workbench\".to_string()\n\
  selected_index=selected\n\
  on_selection_change=on_change\n\
  keyboard_activation={keyboard_activation}\n\
  disabled_indices={disabled_indices}\n\
>\n\
  <div>\"Overview panel\"</div>\n\
  <div>\"Details panel\"</div>\n\
  <div>\"Settings panel\"</div>\n\
</Tabs>\n\
// persist_selected_index={persist_selected_index}"
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let keyboard_activation = if tabs_workbench_manual_mode.get() {
            "manual"
        } else {
            "automatic"
        };
        let disabled_indices = if tabs_workbench_disable_settings.get() {
            vec![2_usize]
        } else {
            Vec::new()
        };
        format!(
            "TabsWorkbenchConfig {{\n  id_base: \"docs-tabs-workbench\",\n  selected_index: {},\n  keyboard_activation: \"{keyboard_activation}\",\n  disabled_indices: {:?},\n  persist_selected_index: {},\n}}",
            tabs_workbench_selected.get(),
            disabled_indices,
            tabs_workbench_persist_state.get(),
        )
    });

    view! {
        <ComponentPage
            title="Tabs"
            slug="tabs"
            group="Collections"
            description="Tabs with roving tabindex, spring indicator motion, and default-theme visual baseline hierarchy."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                description="Zero-wiring default path for beginners: no controlled state setup required."
                code_signal=hello_world_code
            >
                <div class="docs-stack">
                    <Tabs
                        labels=vec!["Overview", "Details", "Settings"]
                        id_base="docs-tabs-hello".to_string()
                    >
                        <div class="docs-stack">
                            <div>"Overview"</div>
                            <div class="ui-muted">"Start here: default selection is managed internally."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Details"</div>
                            <div class="ui-muted">"No state machine wiring required for common usage."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Settings"</div>
                            <div class="ui-muted">"Upgrade to controlled mode only when needed."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "Beginner path first; advanced controls follow below."
                    </span>
                </div>
            </Playground>

            <Playground title="Automatic + Controlled" code_signal=code>
                <div class="docs-stack">
                    <Tabs
                        labels=vec!["Overview", "Details", "Settings"]
                        id_base="docs-tabs".to_string()
                        selected_index=selected_auto
                        on_selection_change=on_auto_change
                        keyboard_activation=TabsKeyboardActivation::Automatic
                    >
                        <div class="docs-stack">
                            <div>"Overview"</div>
                            <div class="ui-muted">"Arrow keys move + select in automatic mode."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Details"</div>
                            <div class="ui-muted">"Selection change is controlled by signal callback."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Settings"</div>
                            <div class="ui-muted">"Indicator motion stays spring-driven."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "selected: "
                        {move || selected_auto.get()}
                    </span>
                    <span class="ui-muted">
                        "Default theme baseline: clear hierarchy, layered contrast, and explicit hover/focus feedback."
                    </span>
                </div>
            </Playground>

            <Playground title="Manual + Disabled" code_signal=states_code>
                <div class="docs-stack">
                    <Tabs
                        labels=manual_labels
                        id_base="docs-tabs-manual".to_string()
                        selected_index=selected_manual
                        on_selection_change=on_manual_change
                        keyboard_activation=TabsKeyboardActivation::Manual
                        disabled_indices=vec![2]
                    >
                        <div class="docs-stack">
                            <div>"Profile"</div>
                            <div class="ui-muted">"Manual mode: focus moves first, Enter/Space commits."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Billing"</div>
                            <div class="ui-muted">"Current selected index reflects committed tab."</div>
                        </div>
                        <div class="docs-stack">
                            <div>"Team"</div>
                            <div class="ui-muted">"This tab is disabled and skipped by roving focus."</div>
                        </div>
                    </Tabs>
                    <span class="ui-muted">
                        "manual selected: "
                        {move || selected_manual.get()}
                    </span>
                    <span class="ui-muted">"disabled tab index: 2"</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                description="Tune keyboard/disabled semantics while preserving context, with optional selected-index persistence."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tabs-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || tabs_workbench_manual_mode.get()
                                on:change=move |ev| set_tabs_workbench_manual_mode.set(event_target_checked(&ev))
                            />
                            " Manual keyboard activation"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || tabs_workbench_disable_settings.get()
                                on:change=move |ev| set_tabs_workbench_disable_settings.set(event_target_checked(&ev))
                            />
                            " Disable \"Settings\" tab"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || tabs_workbench_persist_state.get()
                                on:change=move |ev| set_tabs_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist selected index (optional)"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="tabs-workbench">
                    <span class="ui-muted">
                        "persist selected: "
                        {move || if tabs_workbench_persist_state.get() { "on" } else { "off" }}
                    </span>
                    <span class="ui-muted">
                        "workbench selected: "
                        {move || tabs_workbench_selected.get()}
                    </span>
                    <div class="docs-card" data-slot="tabs-workbench-canvas">
                        {move || {
                            let disabled_indices = if tabs_workbench_disable_settings.get() {
                                vec![2]
                            } else {
                                Vec::new()
                            };
                            let selected_index = tabs_workbench_selected;
                            let on_selection_change = on_tabs_workbench_change;

                            if tabs_workbench_manual_mode.get() {
                                view! {
                                    <Tabs
                                        labels=workbench_labels.clone()
                                        id_base="docs-tabs-workbench".to_string()
                                        selected_index=selected_index
                                        on_selection_change=on_selection_change
                                        keyboard_activation=TabsKeyboardActivation::Manual
                                        disabled_indices=disabled_indices
                                    >
                                        <div class="docs-stack">
                                            <div>"Overview"</div>
                                            <div class="ui-muted">"Keep context while toggling keyboard mode."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Details"</div>
                                            <div class="ui-muted">"Selection stays controlled by workbench signal."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Settings"</div>
                                            <div class="ui-muted">"Optional disabled state stays inspectable via markers."</div>
                                        </div>
                                    </Tabs>
                                }
                                .into_any()
                            } else {
                                view! {
                                    <Tabs
                                        labels=workbench_labels.clone()
                                        id_base="docs-tabs-workbench".to_string()
                                        selected_index=selected_index
                                        on_selection_change=on_selection_change
                                        keyboard_activation=TabsKeyboardActivation::Automatic
                                        disabled_indices=disabled_indices
                                    >
                                        <div class="docs-stack">
                                            <div>"Overview"</div>
                                            <div class="ui-muted">"Keep context while toggling keyboard mode."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Details"</div>
                                            <div class="ui-muted">"Selection stays controlled by workbench signal."</div>
                                        </div>
                                        <div class="docs-stack">
                                            <div>"Settings"</div>
                                            <div class="ui-muted">"Optional disabled state stays inspectable via markers."</div>
                                        </div>
                                    </Tabs>
                                }
                                .into_any()
                            }
                        }}
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
