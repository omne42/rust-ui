use super::*;

pub(crate) fn search_input_button() -> AnyView {
    let persisted_workbench_state = load_search_input_button_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();

    let (press_count, set_press_count) = signal(0_usize);
    let on_press: OnPress = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let preset_options = vec![
        "Default".to_string(),
        "Docs".to_string(),
        "Command".to_string(),
        "Components".to_string(),
    ];
    let (preset_index, set_preset_index) = signal(Some(initial_workbench_state.preset_index));
    let placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0) {
        1 => "Search docs".to_string(),
        2 => "Command menu".to_string(),
        3 => "Find components".to_string(),
        _ => "Search".to_string(),
    });
    let compact_placeholder = Signal::derive(move || match preset_index.get().unwrap_or(0) {
        1 => "Search".to_string(),
        2 => "Cmd".to_string(),
        3 => "Find".to_string(),
        _ => "Search".to_string(),
    });

    let meta_key_options = vec![
        "None".to_string(),
        "⌘".to_string(),
        "Ctrl".to_string(),
        "Alt".to_string(),
    ];
    let (meta_key_index, set_meta_key_index) = signal(Some(0_usize));
    let meta_key_label = Signal::derive(move || match meta_key_index.get().unwrap_or(0) {
        1 => "⌘".to_string(),
        2 => "Ctrl".to_string(),
        3 => "Alt".to_string(),
        _ => String::new(),
    });

    let key_label_options = vec!["None".to_string(), "K".to_string(), "F".to_string()];
    let (key_label_index, set_key_label_index) = signal(Some(0_usize));
    let key_label = Signal::derive(move || match key_label_index.get().unwrap_or(0) {
        1 => "K".to_string(),
        2 => "F".to_string(),
        _ => String::new(),
    });

    let (disabled, set_disabled) = signal(initial_workbench_state.is_disabled);
    let (custom_aria_label, set_custom_aria_label) =
        signal(initial_workbench_state.custom_aria_label);
    let (custom_motion, set_custom_motion) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (submit_type, set_submit_type) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);
    let (persist_workbench_state, set_persist_workbench_state) =
        signal(has_persisted_workbench_state);
    let workbench_node_ref: NodeRef<html::Button> = NodeRef::new();

    Effect::new(move |_| {
        let state = SearchInputButtonWorkbenchState {
            preset_index: preset_index.get().unwrap_or(0).min(3),
            meta_key_index: meta_key_index.get().unwrap_or(0).min(3),
            key_label_index: key_label_index.get().unwrap_or(0).min(2),
            is_disabled: disabled.get(),
            custom_aria_label: custom_aria_label.get(),
        };
        if persist_workbench_state.get() {
            save_search_input_button_workbench_state(state);
        } else {
            clear_search_input_button_workbench_state();
        }
    });

    let code = Signal::derive(move || {
        let placeholder = placeholder.get();
        let compact_placeholder = compact_placeholder.get();
        let meta_key_label = meta_key_label.get();
        let key_label = key_label.get();
        let disabled = disabled.get();
        let custom_aria_label = custom_aria_label.get();

        let mut snippet = vec!["<SearchInputButton".to_string()];

        if placeholder != "Search" {
            snippet.push(format!("  placeholder=\"{placeholder}\".into()"));
        }
        if compact_placeholder != placeholder {
            snippet.push(format!(
                "  compact_placeholder=\"{compact_placeholder}\".into()"
            ));
        }
        if !meta_key_label.is_empty() {
            snippet.push(format!("  meta_key_label=\"{meta_key_label}\".into()"));
        }
        if !key_label.is_empty() {
            snippet.push(format!("  key_label=\"{key_label}\".into()"));
        }
        if disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if custom_aria_label {
            snippet.push("  aria_label=\"Open command menu\".into()".to_string());
        }
        if custom_motion.get() {
            snippet.push(
                "  motion=SearchInputButtonMotion { hover_scale: 1.04, tap_scale: 0.96, ..SearchInputButtonMotion::default() }"
                    .to_string(),
            );
        }
        if custom_class.get() {
            snippet.push("  class_name=\"docs-search-input-button-custom\".into()".to_string());
        }
        snippet.push(format!(
            "  button_type={}",
            if submit_type.get() {
                "Some(ui::button::ButtonType::Submit)"
            } else {
                "Some(ui::button::ButtonType::Button)"
            }
        ));
        snippet.push(format!(
            "  lang={}",
            if rtl_dir.get() {
                "\"ar\".to_string()"
            } else {
                "\"en-US\".to_string()"
            }
        ));
        snippet.push(format!(
            "  dir={}",
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            }
        ));
        snippet.push("  node_ref=NodeRef::<leptos::html::Button>::new()".to_string());
        snippet.push("  on_press=Some(Callback::new(move |_| {}))".to_string());

        snippet.push("/>".to_string());

        snippet.join("\n")
    });

    let states_code = Signal::derive(move || {
        r#"<SearchInputButton placeholder="Find components".to_string() />
<SearchInputButton
  placeholder="Find components".to_string()
  compact_placeholder="Find".to_string()
/>
<SearchInputButton placeholder="Disabled search".to_string() is_disabled=true />
<SearchInputButton placeholder="Forced disabled".to_string() is_disabled=true />"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<SearchInputButton
  placeholder="Browse components".to_string()
  compact_placeholder="Browse".to_string()
  aria_label="Open component search".to_string()
  class_name="docs-search-input-button-custom".to_string()
/>"#
        .to_string()
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "SearchInputButtonWorkbenchConfig {{\n  placeholder: {:?},\n  compact_placeholder: {:?},\n  meta_key_label: {:?},\n  key_label: {:?},\n  is_disabled: {},\n  motion: {},\n  class_name: {:?},\n  button_type: {},\n  aria_label: {:?},\n  lang: {:?},\n  dir: {},\n  node_ref: \"workbench_node_ref\",\n  on_press: \"increment press_count\",\n  custom_aria_label: {},\n  persist_workbench_state: {},\n  on_press_count: {},\n}}",
            placeholder.get(),
            compact_placeholder.get(),
            meta_key_label.get(),
            key_label.get(),
            disabled.get(),
            if custom_motion.get() {
                "SearchInputButtonMotion(custom)"
            } else {
                "SearchInputButtonMotion::default()"
            },
            if custom_class.get() {
                Some("docs-search-input-button-custom")
            } else {
                None
            },
            if submit_type.get() {
                "Some(ButtonType::Submit)"
            } else {
                "Some(ButtonType::Button)"
            },
            if custom_aria_label.get() {
                Some("Open command menu")
            } else {
                None
            },
            if rtl_dir.get() {
                Some("ar")
            } else {
                Some("en-US")
            },
            if rtl_dir.get() {
                "Some(A11yDirection::Rtl)"
            } else {
                "Some(A11yDirection::Ltr)"
            },
            custom_aria_label.get(),
            persist_workbench_state.get(),
            press_count.get(),
        )
    });

    view! {
        <ComponentPage
            title="SearchInputButton"
            slug="search-input-button"
            group="Actions"
            description="baseline-level spring search trigger button with centralized placeholder/shortcut/aria-label state attrs."
        >
            <Playground title="Hello World (Default SearchInputButton)" code_signal=code>
                <div class="docs-row">
                    <SearchInputButton on_press=on_press />
                    <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Preset"</div>
                        <SegmentedControl
                            id_base="docs-search-input-preset".to_string()
                            options=preset_options.clone()
                            selected_index=preset_index
                            set_selected_index=set_preset_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input preset".to_string()
                        />

                        <div class="docs-search__label">"Meta key"</div>
                        <SegmentedControl
                            id_base="docs-search-input-meta-key".to_string()
                            options=meta_key_options.clone()
                            selected_index=meta_key_index
                            set_selected_index=set_meta_key_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input meta key".to_string()
                        />

                        <div class="docs-search__label">"Shortcut key"</div>
                        <SegmentedControl
                            id_base="docs-search-input-key".to_string()
                            options=key_label_options.clone()
                            selected_index=key_label_index
                            set_selected_index=set_key_label_index
                            size=SegmentedControlSize::Sm
                            aria_label="Search input shortcut key".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=custom_aria_label set_checked=set_custom_aria_label>
                            "Custom aria label"
                        </Switch>
                        <Switch checked=custom_motion set_checked=set_custom_motion>
                            "Custom motion"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class_name"
                        </Switch>
                        <Switch checked=submit_type set_checked=set_submit_type>
                            "button_type submit"
                        </Switch>
                        <Switch checked=rtl_dir set_checked=set_rtl_dir>"RTL + ar"</Switch>
                        <Switch checked=persist_workbench_state set_checked=set_persist_workbench_state>
                            "Persist workbench state"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let placeholder = placeholder.get();
                    let compact_placeholder = compact_placeholder.get();
                    let meta_key_label = meta_key_label.get();
                    let key_label = key_label.get();
                    let disabled = disabled.get();
                    let custom_aria_label = custom_aria_label.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                    {if custom_aria_label {
                                    view! {
                                        <SearchInputButton
                                            placeholder=placeholder.clone()
                                            compact_placeholder=compact_placeholder.clone()
                                            meta_key_label=meta_key_label.clone()
                                            key_label=key_label.clone()
                                            aria_label="Open command menu".to_string()
                                            is_disabled=disabled
                                            motion=if custom_motion.get() {
                                                SearchInputButtonMotion {
                                                    hover_scale: 1.04,
                                                    tap_scale: 0.96,
                                                    ..SearchInputButtonMotion::default()
                                                }
                                            } else {
                                                SearchInputButtonMotion::default()
                                            }
                                            class_name=if custom_class.get() {
                                                "docs-search-input-button-custom".to_string()
                                            } else {
                                                String::new()
                                            }
                                            button_type=if submit_type.get() {
                                                ui::button::ButtonType::Submit
                                            } else {
                                                ui::button::ButtonType::Button
                                            }
                                            lang=if rtl_dir.get() {
                                                "ar".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if rtl_dir.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                            node_ref=workbench_node_ref
                                            on_press=on_press
                                        />
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <SearchInputButton
                                            placeholder=placeholder
                                            compact_placeholder=compact_placeholder
                                            meta_key_label=meta_key_label
                                            key_label=key_label
                                            is_disabled=disabled
                                            motion=if custom_motion.get() {
                                                SearchInputButtonMotion {
                                                    hover_scale: 1.04,
                                                    tap_scale: 0.96,
                                                    ..SearchInputButtonMotion::default()
                                                }
                                            } else {
                                                SearchInputButtonMotion::default()
                                            }
                                            class_name=if custom_class.get() {
                                                "docs-search-input-button-custom".to_string()
                                            } else {
                                                String::new()
                                            }
                                            button_type=if submit_type.get() {
                                                ui::button::ButtonType::Submit
                                            } else {
                                                ui::button::ButtonType::Button
                                            }
                                            lang=if rtl_dir.get() {
                                                "ar".to_string()
                                            } else {
                                                "en-US".to_string()
                                            }
                                            dir=if rtl_dir.get() {
                                                A11yDirection::Rtl
                                            } else {
                                                A11yDirection::Ltr
                                            }
                                            node_ref=workbench_node_ref
                                            on_press=on_press
                                        />
                                    }
                                        .into_any()
                                }}
                            </div>
                            <span class="ui-muted">"presses: " {move || press_count.get().to_string()}</span>
                        </div>
                    }
                }}
            </Playground>

            <Playground title="State Matrix (Placeholder + Disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <SearchInputButton placeholder="Find components".to_string() />
                        <SearchInputButton
                            placeholder="Find components".to_string()
                            compact_placeholder="Find".to_string()
                        />
                    </div>
                    <div class="docs-row">
                        <SearchInputButton
                            placeholder="Disabled search".to_string()
                            is_disabled=true
                        />
                        <SearchInputButton
                            placeholder="Forced disabled".to_string()
                            is_disabled=true
                        />
                    </div>
                </div>
            </Playground>

            <Playground title="Custom Class + Aria Label" code_signal=custom_code>
                <div class="docs-row">
                    <SearchInputButton
                        placeholder="Browse components".to_string()
                        compact_placeholder="Browse".to_string()
                        aria_label="Open component search".to_string()
                        class_name="docs-search-input-button-custom".to_string()
                    />
                    <SearchInputButton
                        placeholder="Search by keyword".to_string()
                        class_name="docs-search-input-button-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
