{
    let (open_multi, set_open_multi) = signal(open_set([0]));
    let open_multi_0 = derive_item_open(open_multi, 0);
    let open_multi_1 = derive_item_open(open_multi, 1);
    let open_multi_2 = derive_item_open(open_multi, 2);
    let on_multi_0_change = on_item_open_change(set_open_multi, 0);
    let on_multi_1_change = on_item_open_change(set_open_multi, 1);
    let on_multi_2_change = on_item_open_change(set_open_multi, 2);

    let (open_single, set_open_single) = signal(open_set([1]));
    let open_single_0 = derive_item_open(open_single, 0);
    let open_single_1 = derive_item_open(open_single, 1);
    let open_single_2 = derive_item_open(open_single, 2);
    let on_single_0_change = on_item_open_change(set_open_single, 0);
    let on_single_1_change = on_item_open_change(set_open_single, 1);
    let on_single_2_change = on_item_open_change(set_open_single, 2);

    let persisted_workbench_open = load_workbench_open();
    let (workbench_open, set_workbench_open) = signal(
        persisted_workbench_open
            .clone()
            .unwrap_or_else(|| open_set([0])),
    );
    let workbench_open_0 = derive_item_open(workbench_open, 0);
    let workbench_open_1 = derive_item_open(workbench_open, 1);
    let workbench_open_2 = derive_item_open(workbench_open, 2);
    let on_workbench_0_change = on_item_open_change(set_workbench_open, 0);
    let on_workbench_1_change = on_item_open_change(set_workbench_open, 1);
    let on_workbench_2_change = on_item_open_change(set_workbench_open, 2);

    let workbench_variant_options = vec![
        "Light".to_string(),
        "Shadow".to_string(),
        "Bordered".to_string(),
        "Splitted".to_string(),
    ];
    let workbench_slot_projection_options = vec![
        "KeepAlive".to_string(),
        "Lazy".to_string(),
        "Eager".to_string(),
    ];
    let workbench_lang_options = vec![
        "auto".to_string(),
        "en".to_string(),
        "zh-CN".to_string(),
        "ar".to_string(),
    ];
    let workbench_dir_options = vec!["auto".to_string(), "ltr".to_string(), "rtl".to_string()];

    let (workbench_multiple_mode, set_workbench_multiple_mode) = signal(true);
    let (workbench_disallow_empty_selection, set_workbench_disallow_empty_selection) = signal(true);
    let (workbench_root_disabled, set_workbench_root_disabled) = signal(false);
    let (workbench_disable_second, set_workbench_disable_second) = signal(false);
    let (workbench_disable_third, set_workbench_disable_third) = signal(false);
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(3_usize));
    let (workbench_slot_projection_index, set_workbench_slot_projection_index) =
        signal(Some(0_usize));
    let (workbench_lang_index, set_workbench_lang_index) = signal(Some(0_usize));
    let (workbench_dir_index, set_workbench_dir_index) = signal(Some(0_usize));
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_motion, set_workbench_custom_motion) = signal(false);
    let (workbench_id_base, set_workbench_id_base) = signal("docs-accordion-workbench".to_string());
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(persisted_workbench_open.is_some());
    let (item_api_first_default_open, set_item_api_first_default_open) = signal(true);
    let (item_api_second_default_open, set_item_api_second_default_open) = signal(false);
    let (item_api_third_default_open, set_item_api_third_default_open) = signal(false);
    let item_api_default_open = Signal::derive(move || {
        derive_default_open_set(
            item_api_first_default_open.get(),
            item_api_second_default_open.get(),
            item_api_third_default_open.get(),
        )
    });
    let (item_api_open, set_item_api_open) = signal(item_api_default_open.get_untracked());
    let (item_api_attach_panel_lifecycle, set_item_api_attach_panel_lifecycle) = signal(true);
    let (item_api_lifecycle_events, set_item_api_lifecycle_events) = signal(Vec::<String>::new());
    let (item_api_remount_key, set_item_api_remount_key) = signal(0_u64);
    let on_item_api_0_change = on_item_open_change(set_item_api_open, 0);
    let on_item_api_1_change = on_item_open_change(set_item_api_open, 1);
    let on_item_api_2_change = on_item_open_change(set_item_api_open, 2);

    let on_item_api_panel_lifecycle_0 =
        Callback::new(move |event: AccordionPanelLifecycleEvent| {
            set_item_api_lifecycle_events.update(|events| {
                events.push(format!("key=0 {}", event.as_str()));
                if events.len() > 12 {
                    let stale = events.len() - 12;
                    events.drain(0..stale);
                }
            });
        });
    let on_item_api_panel_lifecycle_1 =
        Callback::new(move |event: AccordionPanelLifecycleEvent| {
            set_item_api_lifecycle_events.update(|events| {
                events.push(format!("key=1 {}", event.as_str()));
                if events.len() > 12 {
                    let stale = events.len() - 12;
                    events.drain(0..stale);
                }
            });
        });
    let on_item_api_panel_lifecycle_2 =
        Callback::new(move |event: AccordionPanelLifecycleEvent| {
            set_item_api_lifecycle_events.update(|events| {
                events.push(format!("key=2 {}", event.as_str()));
                if events.len() > 12 {
                    let stale = events.len() - 12;
                    events.drain(0..stale);
                }
            });
        });

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(3) {
            0 => AccordionVariant::Light,
            1 => AccordionVariant::Shadow,
            2 => AccordionVariant::Bordered,
            _ => AccordionVariant::Splitted,
        });
    let workbench_slot_projection =
        Signal::derive(
            move || match workbench_slot_projection_index.get().unwrap_or(0) {
                1 => AccordionSlotProjection::Lazy,
                2 => AccordionSlotProjection::Eager,
                _ => AccordionSlotProjection::KeepAlive,
            },
        );
    let workbench_lang = Signal::derive(move || match workbench_lang_index.get().unwrap_or(0) {
        1 => Some("en".to_string()),
        2 => Some("zh-CN".to_string()),
        3 => Some("ar".to_string()),
        _ => None,
    });
    let workbench_dir = Signal::derive(move || match workbench_dir_index.get().unwrap_or(0) {
        1 => Some(ui::color::area::A11yDirection::Ltr),
        2 => Some(ui::color::area::A11yDirection::Rtl),
        _ => None,
    });
    let workbench_motion = Signal::derive(move || {
        if workbench_custom_motion.get() {
            AccordionMotion {
                indicator_open_rotation_deg: 108.0,
                panel_offset_y_px: 24.0,
                ..AccordionMotion::default()
            }
        } else {
            AccordionMotion::default()
        }
    });

    let snapshot_mode = Signal::derive(|| AiRenderMode::Snapshot);
    let verified_output = Signal::derive(|| AiOutputStatus::Verified);

    let streaming_total_items = ACCORDION_STREAMING_TOTAL_ITEMS;
    let streaming_total_chars = count_chars(ACCORDION_STREAMING_FULL_CODE);
    let (streaming_chars, set_streaming_chars) = signal(0_usize);
    let (streaming_auto, set_streaming_auto) = signal(false);
    let (streaming_open_set, set_streaming_open_set) = signal(open_set([]));
    let streaming_open_0 = derive_item_open(streaming_open_set, 0);
    let streaming_open_1 = derive_item_open(streaming_open_set, 1);
    let streaming_open_2 = derive_item_open(streaming_open_set, 2);
    let on_streaming_0_change = on_item_open_change(set_streaming_open_set, 0);
    let on_streaming_1_change = on_item_open_change(set_streaming_open_set, 1);
    let on_streaming_2_change = on_item_open_change(set_streaming_open_set, 2);

    let streaming_chars_clamped =
        Signal::derive(move || streaming_chars.get().min(streaming_total_chars));
    let streaming_input_code = Signal::derive(move || {
        take_chars(ACCORDION_STREAMING_FULL_CODE, streaming_chars_clamped.get())
    });
    let streaming_projection =
        Signal::derive(move || project_streaming_accordion_markup(&streaming_input_code.get()));
    let streaming_is_complete = Signal::derive(move || streaming_projection.get().is_complete());
    let streaming_mode = Signal::derive(move || {
        if streaming_is_complete.get() {
            AiRenderMode::Snapshot
        } else {
            AiRenderMode::Streaming
        }
    });
    let streaming_output_status = Signal::derive(move || {
        if streaming_is_complete.get() {
            AiOutputStatus::Verified
        } else {
            AiOutputStatus::Draft
        }
    });
    let streaming_has_root_open = Signal::derive(move || streaming_projection.get().has_root_open);
    let streaming_rendered_items = Signal::derive(move || streaming_projection.get().items.len());
    let streaming_visible_item_count = Signal::derive(move || {
        streaming_rendered_items
            .get()
            .min(ACCORDION_STREAMING_TOTAL_ITEMS)
    });
    let streaming_item_1_text = Signal::derive(move || {
        streaming_projection
            .get()
            .items
            .first()
            .map(|item| item.text.clone())
            .unwrap_or_default()
    });
    let streaming_item_2_text = Signal::derive(move || {
        streaming_projection
            .get()
            .items
            .get(1)
            .map(|item| item.text.clone())
            .unwrap_or_default()
    });
    let streaming_item_3_text = Signal::derive(move || {
        streaming_projection
            .get()
            .items
            .get(2)
            .map(|item| item.text.clone())
            .unwrap_or_default()
    });
    let streaming_update_disabled =
        Signal::derive(move || streaming_chars_clamped.get() >= streaming_total_chars);
    let streaming_code = Signal::derive(move || {
        compose_streaming_demo_code(
            &streaming_input_code.get(),
            &streaming_projection.get(),
            &streaming_open_set.get(),
            streaming_mode.get(),
            streaming_output_status.get(),
        )
    });

    Effect::new(move |_| {
        let visible = streaming_visible_item_count.get();
        set_streaming_open_set.update(|open_set| {
            open_set.retain(|index| *index < visible);
        });
    });

    let auto_timeout = StoredValue::new_local(None::<TimeoutHandle>);
    Effect::new(move |_| {
        if let Some(handle) = auto_timeout.get_value() {
            handle.clear();
        }
        auto_timeout.set_value(None);

        if !streaming_auto.get() {
            return;
        }

        let current = streaming_chars_clamped.get();
        let delay_ms = if current >= streaming_total_chars {
            ACCORDION_STREAMING_AUTO_RESET_DELAY_MS
        } else {
            ACCORDION_STREAMING_AUTO_INTERVAL_MS
        };

        if let Ok(handle) = set_timeout_with_handle(
            move || {
                if current >= streaming_total_chars {
                    set_streaming_chars.set(0);
                    set_streaming_open_set.set(open_set([]));
                } else {
                    set_streaming_chars.update(|value| {
                        *value = (*value + ACCORDION_STREAMING_AUTO_STEP).min(streaming_total_chars)
                    });
                }
            },
            std::time::Duration::from_millis(delay_ms),
        ) {
            auto_timeout.set_value(Some(handle));
        }
    });

    on_cleanup(move || {
        if let Some(handle) = auto_timeout.get_value() {
            handle.clear();
        }
    });

    Effect::new(move |_| {
        let open = workbench_open.get();
        if workbench_persist_state.get() {
            save_workbench_open(&open);
        } else {
            clear_workbench_open();
        }
    });

    Effect::new(move |_| {
        let multiple_mode = workbench_multiple_mode.get();
        let disallow_empty = workbench_disallow_empty_selection.get();
        let disable_second = workbench_disable_second.get();
        let disable_third = workbench_disable_third.get();

        set_workbench_open.update(|open| {
            open.retain(|index| *index < 3);
            if disable_second {
                open.remove(&1);
            }
            if disable_third {
                open.remove(&2);
            }

            if !multiple_mode && open.len() > 1 {
                let keep = open.iter().next().copied();
                open.clear();
                if let Some(keep) = keep {
                    open.insert(keep);
                }
            }

            if disallow_empty && open.is_empty() {
                if !disable_second {
                    open.insert(1);
                } else if !disable_third {
                    open.insert(2);
                } else {
                    open.insert(0);
                }
            }
        });
    });

    let hello_code = Signal::derive(move || {
        r#"<Accordion variant=AccordionVariant::Light>
  <AccordionItem label="First">"Panel 1"</AccordionItem>
  <AccordionItem label="Second">"Panel 2"</AccordionItem>
</Accordion>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        let open_literal = open_set_literal(&open_multi.get());
        format!(
            "let (open, set_open) = signal({open_literal});\n\
let item_0_open = Signal::derive(move || open.get().contains(&0));\n\
let item_1_open = Signal::derive(move || open.get().contains(&1));\n\
let item_2_open = Signal::derive(move || open.get().contains(&2));\n\
\n\
<Accordion\n\
  id_base=\"accordion\".to_string()\n\
  selection_mode=AccordionSelectionMode::Multiple\n\
  variant=AccordionVariant::Shadow\n\
>\n\
  <AccordionItem key=0 label=\"First\" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>\"Panel 1\"</div></AccordionItem>\n\
  <AccordionItem key=1 label=\"Second\" open=item_1_open on_open_change=on_item_open_change(set_open, 1)><div>\"Panel 2\"</div></AccordionItem>\n\
  <AccordionItem key=2 label=\"Third\" open=item_2_open on_open_change=on_item_open_change(set_open, 2)><div>\"Panel 3\"</div></AccordionItem>\n\
</Accordion>"
        )
    });

    let states_code = Signal::derive(move || {
        r#"let (open, set_open) = signal(open_set([1]));
let item_0_open = Signal::derive(move || open.get().contains(&0));
let item_1_open = Signal::derive(move || open.get().contains(&1));
let item_2_open = Signal::derive(move || open.get().contains(&2));

<Accordion
  id_base="accordion-single".to_string()
  selection_mode=AccordionSelectionMode::Single
  variant=AccordionVariant::Bordered
  disallow_empty_selection=true
>
  <AccordionItem key=0 label="Overview" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>"Overview"</div></AccordionItem>
  <AccordionItem key=1 label="Details" open=item_1_open on_open_change=on_item_open_change(set_open, 1)><div>"Details"</div></AccordionItem>
  <AccordionItem key=2 label="History" open=item_2_open on_open_change=on_item_open_change(set_open, 2) is_disabled=true><div>"History"</div></AccordionItem>
</Accordion>"#.to_string()
    });

    let controlled_preview_actual_config = Signal::derive(move || {
        let open = open_multi.get().iter().copied().collect::<Vec<_>>();
        format!(
            "AccordionControlledPreviewConfig {{\n  id_base: Some(\"docs-accordion\"),\n  selection_mode: \"multiple\",\n  variant: \"shadow\",\n  open: {open:?},\n  on_open_change: \"per-item callback updates open set\",\n  on_panel_lifecycle: \"events=0 (not attached in controlled preview)\",\n}}"
        )
    });

    let workbench_code = Signal::derive(move || {
        let multiple_mode = workbench_multiple_mode.get();
        let disallow_empty_selection = workbench_disallow_empty_selection.get();
        let root_disabled = workbench_root_disabled.get();
        let disable_security = workbench_disable_second.get();
        let disable_third = workbench_disable_third.get();
        let variant = workbench_variant.get();
        let slot_projection = workbench_slot_projection.get();
        let lang = workbench_lang.get();
        let dir = workbench_dir.get();
        let custom_class = workbench_custom_class.get();
        let custom_motion = workbench_custom_motion.get();
        let id_base = workbench_id_base.get();
        let persist_state = workbench_persist_state.get();
        let open_literal = open_set_literal(&workbench_open.get());
        let selection_mode = if multiple_mode {
            "AccordionSelectionMode::Multiple"
        } else {
            "AccordionSelectionMode::Single"
        };
        let variant_literal = match variant {
            AccordionVariant::Light => "AccordionVariant::Light",
            AccordionVariant::Shadow => "AccordionVariant::Shadow",
            AccordionVariant::Bordered => "AccordionVariant::Bordered",
            AccordionVariant::Splitted => "AccordionVariant::Splitted",
        };
        let slot_projection_literal = match slot_projection {
            AccordionSlotProjection::Lazy => "AccordionSlotProjection::Lazy",
            AccordionSlotProjection::KeepAlive => "AccordionSlotProjection::KeepAlive",
            AccordionSlotProjection::Eager => "AccordionSlotProjection::Eager",
        };

        let mut lines = vec![
            format!("// persist_state={}", bool_word(persist_state)),
            if persist_state {
                "let saved = load_workbench_open();".to_string()
            } else {
                "// persist disabled; use current runtime snapshot".to_string()
            },
            if persist_state {
                format!("let (open, set_open) = signal(saved.unwrap_or_else(|| {open_literal}));")
            } else {
                format!("let (open, set_open) = signal({open_literal});")
            },
            "let item_0_open = Signal::derive(move || open.get().contains(&0));".to_string(),
            "let item_1_open = Signal::derive(move || open.get().contains(&1));".to_string(),
            "let item_2_open = Signal::derive(move || open.get().contains(&2));".to_string(),
            "<Accordion".to_string(),
            format!("  id_base={}.to_string()", rust_string_literal(&id_base)),
            format!("  selection_mode={selection_mode}"),
            format!("  variant={variant_literal}"),
            format!("  slot_projection={slot_projection_literal}"),
        ];
        push_line_when(
            &mut lines,
            disallow_empty_selection,
            "  disallow_empty_selection=true".to_string(),
        );
        push_line_when(&mut lines, root_disabled, "  is_disabled=true".to_string());
        if let Some(lang) = lang {
            lines.push(format!("  lang={}.to_string()", rust_string_literal(&lang)));
        }
        if let Some(dir) = dir {
            let dir_literal = match dir {
                ui::color::area::A11yDirection::Ltr => "ui::color::area::A11yDirection::Ltr",
                ui::color::area::A11yDirection::Rtl => "ui::color::area::A11yDirection::Rtl",
            };
            lines.push(format!("  dir={dir_literal}"));
        }
        push_line_when(
            &mut lines,
            custom_class,
            "  class_name=\"docs-accordion-workbench--custom\".to_string()".to_string(),
        );
        if custom_motion {
            lines.push(
                "  motion=AccordionMotion { indicator_open_rotation_deg: 108.0, panel_offset_y_px: 24.0, ..AccordionMotion::default() }"
                    .to_string(),
            );
        }
        lines.push(">".to_string());
        lines.push(
            "  <AccordionItem key=0 label=\"Profile\" open=item_0_open on_open_change=on_item_open_change(set_open, 0)><div>\"Profile panel\"</div></AccordionItem>".to_string(),
        );
        lines.push(format!(
            "  <AccordionItem key=1 label=\"Security\" open=item_1_open on_open_change=on_item_open_change(set_open, 1){}><div>\"Security panel\"</div></AccordionItem>",
            if disable_security { " is_disabled=true" } else { "" }
        ));
        lines.push(format!(
            "  <AccordionItem key=2 label=\"Notifications\" open=item_2_open on_open_change=on_item_open_change(set_open, 2){}><div>\"Notifications panel\"</div></AccordionItem>",
            if disable_third { " is_disabled=true" } else { "" }
        ));
        lines.push("</Accordion>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let open = workbench_open.get().iter().copied().collect::<Vec<_>>();
        let selection_mode = if workbench_multiple_mode.get() {
            "multiple"
        } else {
            "single"
        };
        let variant = workbench_variant.get().as_str();
        let slot_projection = workbench_slot_projection.get().as_str();
        let dir = match workbench_dir.get() {
            Some(ui::color::area::A11yDirection::Ltr) => "ltr",
            Some(ui::color::area::A11yDirection::Rtl) => "rtl",
            _ => "auto",
        };
        let lang = workbench_lang.get();
        let class_name = if workbench_custom_class.get() {
            "docs-accordion-workbench--custom"
        } else {
            ""
        };
        let id_base = workbench_id_base.get();
        let second_open = open.contains(&1);
        let third_open = open.contains(&2);

        format!(
            "AccordionWorkbenchConfig {{\n  id_base: {id_base:?},\n  selection_mode: \"{selection_mode}\",\n  variant: \"{variant}\",\n  slot_projection: \"{slot_projection}\",\n  disallow_empty_selection: {},\n  is_disabled: {},\n  lang: {lang:?},\n  dir: \"{dir}\",\n  class_name: \"{class_name}\",\n  motion: \"{}\",\n  custom_motion: {},\n  persist_state: {},\n  open: {open:?},\n  items: [\n    {{ key: 0, label: \"Profile\", is_disabled: false, open: {}, default_open: false, on_open_change: true, on_panel_lifecycle: false }},\n    {{ key: 1, label: \"Security\", is_disabled: {}, open: {}, default_open: false, on_open_change: true, on_panel_lifecycle: false }},\n    {{ key: 2, label: \"Notifications\", is_disabled: {}, open: {}, default_open: false, on_open_change: true, on_panel_lifecycle: false }},\n  ],\n}}",
            if workbench_custom_motion.get() {
                "custom"
            } else {
                "default"
            },
            bool_word(workbench_disallow_empty_selection.get()),
            bool_word(workbench_root_disabled.get()),
            bool_word(workbench_custom_motion.get()),
            bool_word(workbench_persist_state.get()),
            bool_word(open.contains(&0)),
            bool_word(workbench_disable_second.get()),
            bool_word(second_open),
            bool_word(workbench_disable_third.get()),
            bool_word(third_open),
        )
    });

    let item_api_code = Signal::derive(move || {
        let default_open = item_api_default_open.get();
        let attach_panel_lifecycle = item_api_attach_panel_lifecycle.get();
        let first_default_open = default_open.contains(&0);
        let second_default_open = default_open.contains(&1);
        let third_default_open = default_open.contains(&2);

        let mut lines = vec![
            "let (observed_open, set_observed_open) = signal(open_set([]));".to_string(),
            "let on_item_0_change = on_item_open_change(set_observed_open, 0);".to_string(),
            "let on_item_1_change = on_item_open_change(set_observed_open, 1);".to_string(),
            "let on_item_2_change = on_item_open_change(set_observed_open, 2);".to_string(),
            "<Accordion".to_string(),
            "  id_base=\"docs-accordion-item-api\".to_string()".to_string(),
            "  selection_mode=AccordionSelectionMode::Multiple".to_string(),
            "  variant=AccordionVariant::Splitted".to_string(),
            ">".to_string(),
        ];

        lines.push(format!(
            "  <AccordionItem key=0 label=\"Profile\" default_open={first_default_open} on_open_change=on_item_0_change{}><div>\"Profile panel content\"</div></AccordionItem>",
            if attach_panel_lifecycle {
                " on_panel_lifecycle=on_panel_lifecycle_0"
            } else {
                ""
            }
        ));
        lines.push(format!(
            "  <AccordionItem key=1 label=\"Security\" default_open={second_default_open} on_open_change=on_item_1_change{}><div>\"Security panel content\"</div></AccordionItem>",
            if attach_panel_lifecycle {
                " on_panel_lifecycle=on_panel_lifecycle_1"
            } else {
                ""
            }
        ));
        lines.push(format!(
            "  <AccordionItem key=2 label=\"Notifications\" default_open={third_default_open} on_open_change=on_item_2_change{}><div>\"Notifications panel content\"</div></AccordionItem>",
            if attach_panel_lifecycle {
                " on_panel_lifecycle=on_panel_lifecycle_2"
            } else {
                ""
            }
        ));
        lines.push("</Accordion>".to_string());
        lines.join("\n")
    });

    let item_api_actual_config = Signal::derive(move || {
        let default_open = item_api_default_open
            .get()
            .iter()
            .copied()
            .collect::<Vec<_>>();
        let observed_open = item_api_open.get().iter().copied().collect::<Vec<_>>();
        let attach_panel_lifecycle = item_api_attach_panel_lifecycle.get();
        let lifecycle_events = item_api_lifecycle_events.get();
        let lifecycle_count = lifecycle_events.len();
        let remount_key = item_api_remount_key.get();

        format!(
            "AccordionItemApiConfig {{\n  remount_key: {remount_key},\n  selection_mode: \"multiple\",\n  default_open: {default_open:?},\n  observed_open: {observed_open:?},\n  attach_on_panel_lifecycle: {},\n  lifecycle_event_count: {lifecycle_count},\n  items: [\n    {{ key: 0, label: \"Profile\", is_disabled: false, open: \"uncontrolled\", default_open: {}, on_open_change: true, on_panel_lifecycle: {} }},\n    {{ key: 1, label: \"Security\", is_disabled: false, open: \"uncontrolled\", default_open: {}, on_open_change: true, on_panel_lifecycle: {} }},\n    {{ key: 2, label: \"Notifications\", is_disabled: false, open: \"uncontrolled\", default_open: {}, on_open_change: true, on_panel_lifecycle: {} }},\n  ],\n  lifecycle_events: {lifecycle_events:?},\n}}",
            bool_word(attach_panel_lifecycle),
            bool_word(default_open.contains(&0)),
            bool_word(attach_panel_lifecycle),
            bool_word(default_open.contains(&1)),
            bool_word(attach_panel_lifecycle),
            bool_word(default_open.contains(&2)),
            bool_word(attach_panel_lifecycle),
        )
    });

    view! {
        <ComponentPage
            title="Accordion"
            slug="accordion"
            group="Collections"
            description="Multi-panel disclosure with roving tabindex, baseline-level spring motion, and baseline-style root state attrs."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                description="Zero wiring path: no controlled state and no headless/state-primitives setup needed."
                code_signal=hello_code
                >
                    <div class="docs-stack">
                        <AiSpace mode=snapshot_mode output_status=verified_output>
                            <Accordion variant=AccordionVariant::Light>
                                <AccordionItem label="First">"Panel 1 content"</AccordionItem>
                                <AccordionItem label="Second">"Panel 2 content"</AccordionItem>
                            </Accordion>
                        </AiSpace>
                        <span class="ui-muted">"minimal default path"</span>
                    </div>
            </Playground>

            <Playground
                title="Multiple + Controlled"
                code_signal=code
                test_config_signal=controlled_preview_actual_config
                controls=move || view! {
                    <div class="docs-row" data-slot="accordion-controlled-preview-controls">
                        <button
                            type="button"
                            on:click=move |_| set_open_multi.set(open_set([0, 1, 2]))
                        >
                            "Open all"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_open_multi.set(open_set([0]))
                        >
                            "Only first"
                        </button>
                        <button
                            type="button"
                            on:click=move |_| set_open_multi.set(open_set([]))
                        >
                            "Close all"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack">
                    <AiSpace mode=snapshot_mode output_status=verified_output>
                        <Accordion
                            id_base="docs-accordion".to_string()
                            selection_mode=AccordionSelectionMode::Multiple
                            variant=AccordionVariant::Shadow
                        >
                            <AccordionItem key=0 label="First" open=open_multi_0 on_open_change=on_multi_0_change>
                                <div class="docs-stack">
                                    <div>"Panel 1 content"</div>
                                    <div class="ui-muted">"Press Enter/Space or click the triggers."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=1 label="Second" open=open_multi_1 on_open_change=on_multi_1_change>
                                <div class="docs-stack">
                                    <div>"Panel 2 content"</div>
                                    <div class="ui-muted">"Arrow keys move focus between triggers."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=2 label="Third" open=open_multi_2 on_open_change=on_multi_2_change>
                                <div class="docs-stack">
                                    <div>"Panel 3 content"</div>
                                    <div class="ui-muted">"Multiple mode allows multiple open panels."</div>
                                </div>
                            </AccordionItem>
                        </Accordion>
                    </AiSpace>
                    <span class="ui-muted">
                        "open: "
                        {move || {
                            let open = open_multi.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Single + Disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <AiSpace mode=snapshot_mode output_status=verified_output>
                        <Accordion
                            id_base="docs-accordion-single".to_string()
                            selection_mode=AccordionSelectionMode::Single
                            variant=AccordionVariant::Bordered
                            disallow_empty_selection=true
                        >
                            <AccordionItem key=0 label="Overview" open=open_single_0 on_open_change=on_single_0_change>
                                <div class="docs-stack">
                                    <div>"Overview content"</div>
                                    <div class="ui-muted">"Single mode keeps at most one panel open."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=1 label="Details" open=open_single_1 on_open_change=on_single_1_change>
                                <div class="docs-stack">
                                    <div>"Details content"</div>
                                    <div class="ui-muted">"Selection is fully controlled by `open`."</div>
                                </div>
                            </AccordionItem>
                            <AccordionItem key=2 label="History" open=open_single_2 on_open_change=on_single_2_change is_disabled=true>
                                <div class="docs-stack">
                                    <div>"History content"</div>
                                    <div class="ui-muted">"This trigger is disabled and skipped by roving focus."</div>
                                </div>
                            </AccordionItem>
                        </Accordion>
                    </AiSpace>
                    <span class="ui-muted">
                        "single open: "
                        {move || {
                            let open = open_single.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    <span class="ui-muted">"disabled index: 2"</span>
                </div>
            </Playground>

            <Playground
                title="Streaming Output (AI Space)"
                code_signal=streaming_code
                description="Character streaming starts from <Accordion>. Update adds +20 chars each time; Auto adds +2 chars every 0.1s. After completion, wait 3s, reset, and loop."
                code_imports=String::new()
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-streaming-controls">
                        <div class="docs-row">
                            <button
                                type="button"
                                on:click=move |_| {
                                    set_streaming_chars.set(0);
                                    set_streaming_open_set.set(open_set([]));
                                }
                            >
                                "Reset"
                            </button>
                            <button
                                type="button"
                                prop:disabled=streaming_update_disabled
                                on:click=move |_| {
                                    set_streaming_chars.update(|value| {
                                        *value = (*value + ACCORDION_STREAMING_MANUAL_STEP)
                                            .min(streaming_total_chars)
                                    });
                                }
                            >
                                "Update"
                            </button>
                            <label class="docs-search__label">
                                <input
                                    type="checkbox"
                                    prop:checked=move || streaming_auto.get()
                                    on:change=move |ev| set_streaming_auto.set(event_target_checked(&ev))
                                />
                                " Auto"
                            </label>
                        </div>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="accordion-streaming-demo">
                    <span class="ui-muted">
                        "mode: "
                        {move || streaming_mode.get().as_str()}
                        " | status: "
                        {move || streaming_output_status.get().as_str()}
                        " | chars: "
                        {move || streaming_chars_clamped.get()}
                        "/"
                        {streaming_total_chars}
                        " | items rendered: "
                        {move || streaming_rendered_items.get()}
                        "/"
                        {streaming_total_items}
                    </span>
                    <AiSpace mode=streaming_mode output_status=streaming_output_status>
                        <div class="docs-card" data-slot="accordion-streaming-canvas">
                            <Show when=move || streaming_has_root_open.get()>
                                <Show when=move || streaming_visible_item_count.get() == 1>
                                    <Accordion
                                        id_base="docs-accordion-ai-stream".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        <AccordionItem
                                            key=0
                                            label=ACCORDION_STREAMING_ITEM_LABELS[0].to_string()
                                            open=streaming_open_0
                                            on_open_change=on_streaming_0_change
                                        >
                                            <div>{move || streaming_item_1_text.get()}</div>
                                        </AccordionItem>
                                    </Accordion>
                                </Show>
                                <Show when=move || streaming_visible_item_count.get() == 2>
                                    <Accordion
                                        id_base="docs-accordion-ai-stream".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        <AccordionItem
                                            key=0
                                            label=ACCORDION_STREAMING_ITEM_LABELS[0].to_string()
                                            open=streaming_open_0
                                            on_open_change=on_streaming_0_change
                                        >
                                            <div>{move || streaming_item_1_text.get()}</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=1
                                            label=ACCORDION_STREAMING_ITEM_LABELS[1].to_string()
                                            open=streaming_open_1
                                            on_open_change=on_streaming_1_change
                                        >
                                            <div>{move || streaming_item_2_text.get()}</div>
                                        </AccordionItem>
                                    </Accordion>
                                </Show>
                                <Show when=move || streaming_visible_item_count.get() == 3>
                                    <Accordion
                                        id_base="docs-accordion-ai-stream".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        <AccordionItem
                                            key=0
                                            label=ACCORDION_STREAMING_ITEM_LABELS[0].to_string()
                                            open=streaming_open_0
                                            on_open_change=on_streaming_0_change
                                        >
                                            <div>{move || streaming_item_1_text.get()}</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=1
                                            label=ACCORDION_STREAMING_ITEM_LABELS[1].to_string()
                                            open=streaming_open_1
                                            on_open_change=on_streaming_1_change
                                        >
                                            <div>{move || streaming_item_2_text.get()}</div>
                                        </AccordionItem>
                                        <AccordionItem
                                            key=2
                                            label=ACCORDION_STREAMING_ITEM_LABELS[2].to_string()
                                            open=streaming_open_2
                                            on_open_change=on_streaming_2_change
                                        >
                                            <div>{move || streaming_item_3_text.get()}</div>
                                        </AccordionItem>
                                    </Accordion>
                                </Show>
                            </Show>
                        </div>
                    </AiSpace>
                </div>
            </Playground>

            <Playground
                title="Workbench (Isolated Canvas + Optional Persist)"
                description="Accordion API workbench: adjust root/item props and inspect real-time config/code feedback."
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-workbench-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_multiple_mode.get()
                                on:change=move |ev| set_workbench_multiple_mode.set(event_target_checked(&ev))
                            />
                            " Multiple mode"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disallow_empty_selection.get()
                                on:change=move |ev| set_workbench_disallow_empty_selection.set(event_target_checked(&ev))
                            />
                            " Disallow empty selection"
                        </label>
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-variant".to_string()
                            options=workbench_variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench variant".to_string()
                        />
                        <div class="docs-search__label">"Slot projection"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-slot-projection".to_string()
                            options=workbench_slot_projection_options.clone()
                            selected_index=workbench_slot_projection_index
                            set_selected_index=set_workbench_slot_projection_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench slot projection".to_string()
                        />
                        <div class="docs-search__label">"Lang"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-lang".to_string()
                            options=workbench_lang_options.clone()
                            selected_index=workbench_lang_index
                            set_selected_index=set_workbench_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench language".to_string()
                        />
                        <div class="docs-search__label">"Direction"</div>
                        <SegmentedControl
                            id_base="docs-accordion-workbench-dir".to_string()
                            options=workbench_dir_options.clone()
                            selected_index=workbench_dir_index
                            set_selected_index=set_workbench_dir_index
                            size=SegmentedControlSize::Sm
                            aria_label="Accordion workbench direction".to_string()
                        />
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_root_disabled.get()
                                on:change=move |ev| set_workbench_root_disabled.set(event_target_checked(&ev))
                            />
                            " Disable root"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_second.get()
                                on:change=move |ev| set_workbench_disable_second.set(event_target_checked(&ev))
                            />
                            " Disable item #1"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disable_third.get()
                                on:change=move |ev| set_workbench_disable_third.set(event_target_checked(&ev))
                            />
                            " Disable item #2"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_class.get()
                                on:change=move |ev| set_workbench_custom_class.set(event_target_checked(&ev))
                            />
                            " Custom class"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_custom_motion.get()
                                on:change=move |ev| set_workbench_custom_motion.set(event_target_checked(&ev))
                            />
                            " Custom motion"
                        </label>
                        <label class="docs-search__label">
                            "id_base "
                            <input
                                type="text"
                                prop:value=move || workbench_id_base.get()
                                on:input=move |ev| set_workbench_id_base.set(event_target_value(&ev))
                            />
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_persist_state.get()
                                on:change=move |ev| set_workbench_persist_state.set(event_target_checked(&ev))
                            />
                            " Persist open state (optional)"
                        </label>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="accordion-workbench">
                    <span class="ui-muted">
                        "persist: "
                        {move || if workbench_persist_state.get() { "on" } else { "off" }}
                        " | open: "
                        {move || {
                            let open = workbench_open.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    <span class="ui-muted" data-slot="accordion-workbench-summary">
                        "mode: "
                        {move || if workbench_multiple_mode.get() { "multiple" } else { "single" }}
                        " | variant: "
                        {move || workbench_variant.get().as_str()}
                        " | slot: "
                        {move || workbench_slot_projection.get().as_str()}
                        " | root_disabled: "
                        {move || workbench_root_disabled.get()}
                        " | disable_second: "
                        {move || workbench_disable_second.get()}
                        " | disable_third: "
                        {move || workbench_disable_third.get()}
                    </span>
                    <pre class="docs-code" data-slot="accordion-workbench-config-preview">
                        {move || workbench_actual_config.get()}
                    </pre>
                    {move || {
                        let selection_mode = if workbench_multiple_mode.get() {
                            AccordionSelectionMode::Multiple
                        } else {
                            AccordionSelectionMode::Single
                        };
                        let disallow_empty_selection = workbench_disallow_empty_selection.get();
                        let disable_security = workbench_disable_second.get();
                        let disable_notifications = workbench_disable_third.get();
                        let variant = workbench_variant.get();
                        let slot_projection = workbench_slot_projection.get();
                        let is_disabled = workbench_root_disabled.get();
                        let lang = workbench_lang.get().unwrap_or_default();
                        let dir = workbench_dir.get();
                        let motion = workbench_motion.get();
                        let class_name = if workbench_custom_class.get() {
                            "docs-accordion-workbench--custom".to_string()
                        } else {
                            String::new()
                        };
                        let id_base = workbench_id_base.get();
                        view! {
                            <div
                                class="docs-card"
                                data-slot="accordion-workbench-canvas"
                            >
                                <AiSpace mode=snapshot_mode output_status=verified_output>
                                    {match dir {
                                        Some(dir) => view! {
                                            <Accordion
                                                id_base=id_base.clone()
                                                selection_mode=selection_mode
                                                variant=variant
                                                disallow_empty_selection=disallow_empty_selection
                                                is_disabled=is_disabled
                                                lang=lang.clone()
                                                dir=dir
                                                slot_projection=slot_projection
                                                motion=motion
                                                class_name=class_name.clone()
                                            >
                                                <AccordionItem
                                                    key=0
                                                    label="Profile"
                                                    open=workbench_open_0
                                                    on_open_change=on_workbench_0_change
                                                >
                                                    <div>"Profile panel content"</div>
                                                </AccordionItem>
                                                <AccordionItem
                                                    key=1
                                                    label="Security".to_string()
                                                    open=workbench_open_1
                                                    on_open_change=on_workbench_1_change
                                                    is_disabled=disable_security
                                                >
                                                    <div>"Security panel content"</div>
                                                </AccordionItem>
                                                <AccordionItem
                                                    key=2
                                                    label="Notifications"
                                                    open=workbench_open_2
                                                    on_open_change=on_workbench_2_change
                                                    is_disabled=disable_notifications
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            </Accordion>
                                        }
                                            .into_any(),
                                        None => view! {
                                            <Accordion
                                                id_base=id_base
                                                selection_mode=selection_mode
                                                variant=variant
                                                disallow_empty_selection=disallow_empty_selection
                                                is_disabled=is_disabled
                                                lang=lang
                                                slot_projection=slot_projection
                                                motion=motion
                                                class_name=class_name
                                            >
                                                <AccordionItem
                                                    key=0
                                                    label="Profile"
                                                    open=workbench_open_0
                                                    on_open_change=on_workbench_0_change
                                                >
                                                    <div>"Profile panel content"</div>
                                                </AccordionItem>
                                                <AccordionItem
                                                    key=1
                                                    label="Security".to_string()
                                                    open=workbench_open_1
                                                    on_open_change=on_workbench_1_change
                                                    is_disabled=disable_security
                                                >
                                                    <div>"Security panel content"</div>
                                                </AccordionItem>
                                                <AccordionItem
                                                    key=2
                                                    label="Notifications"
                                                    open=workbench_open_2
                                                    on_open_change=on_workbench_2_change
                                                    is_disabled=disable_notifications
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            </Accordion>
                                        }
                                            .into_any(),
                                    }}
                                </AiSpace>
                            </div>
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="Item API Workbench (default_open + lifecycle)"
                description="Focused API coverage for AccordionItem defaults and lifecycle callbacks. Update config then remount to replay default_open initialization."
                code_signal=item_api_code
                test_config_signal=item_api_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-item-api-controls">
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_first_default_open.get()
                                on:change=move |ev| set_item_api_first_default_open.set(event_target_checked(&ev))
                            />
                            " Item #0 default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_second_default_open.get()
                                on:change=move |ev| set_item_api_second_default_open.set(event_target_checked(&ev))
                            />
                            " Item #1 default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_third_default_open.get()
                                on:change=move |ev| set_item_api_third_default_open.set(event_target_checked(&ev))
                            />
                            " Item #2 default_open"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || item_api_attach_panel_lifecycle.get()
                                on:change=move |ev| set_item_api_attach_panel_lifecycle.set(event_target_checked(&ev))
                            />
                            " Attach on_panel_lifecycle"
                        </label>
                        <button
                            type="button"
                            data-slot="accordion-item-api-remount-button"
                            on:click=move |_| {
                                set_item_api_open.set(item_api_default_open.get_untracked());
                                set_item_api_lifecycle_events.set(Vec::new());
                                set_item_api_remount_key.update(|key| *key = key.saturating_add(1));
                            }
                        >
                            "Apply default_open + remount"
                        </button>
                    </div>
                }
            >
                <div class="docs-stack" data-slot="accordion-item-api-workbench">
                    <span class="ui-muted">
                        "remount_key: "
                        {move || item_api_remount_key.get()}
                        " | default_open: "
                        {move || {
                            let open = item_api_default_open.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                        " | observed_open: "
                        {move || {
                            let open = item_api_open.get().iter().copied().collect::<Vec<_>>();
                            format!("{open:?}")
                        }}
                    </span>
                    <pre class="docs-code" data-slot="accordion-item-api-config-preview">
                        {move || item_api_actual_config.get()}
                    </pre>
                    <div class="docs-card" data-slot="accordion-item-api-canvas">
                        <For
                            each=move || vec![item_api_remount_key.get()]
                            key=|mount_key| *mount_key
                            children=move |_| {
                                let default_open = item_api_default_open.get();
                                let attach_panel_lifecycle = item_api_attach_panel_lifecycle.get();
                                let first_default_open = default_open.contains(&0);
                                let second_default_open = default_open.contains(&1);
                                let third_default_open = default_open.contains(&2);
                                view! {
                                    <Accordion
                                        id_base="docs-accordion-item-api".to_string()
                                        selection_mode=AccordionSelectionMode::Multiple
                                        variant=AccordionVariant::Splitted
                                    >
                                        {if attach_panel_lifecycle {
                                            view! {
                                                <AccordionItem
                                                    key=0
                                                    label="Profile"
                                                    default_open=first_default_open
                                                    on_open_change=on_item_api_0_change
                                                    on_panel_lifecycle=on_item_api_panel_lifecycle_0
                                                >
                                                    <div>"Profile panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <AccordionItem
                                                    key=0
                                                    label="Profile"
                                                    default_open=first_default_open
                                                    on_open_change=on_item_api_0_change
                                                >
                                                    <div>"Profile panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        }}
                                        {if attach_panel_lifecycle {
                                            view! {
                                                <AccordionItem
                                                    key=1
                                                    label="Security"
                                                    default_open=second_default_open
                                                    on_open_change=on_item_api_1_change
                                                    on_panel_lifecycle=on_item_api_panel_lifecycle_1
                                                >
                                                    <div>"Security panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <AccordionItem
                                                    key=1
                                                    label="Security"
                                                    default_open=second_default_open
                                                    on_open_change=on_item_api_1_change
                                                >
                                                    <div>"Security panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        }}
                                        {if attach_panel_lifecycle {
                                            view! {
                                                <AccordionItem
                                                    key=2
                                                    label="Notifications"
                                                    default_open=third_default_open
                                                    on_open_change=on_item_api_2_change
                                                    on_panel_lifecycle=on_item_api_panel_lifecycle_2
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        } else {
                                            view! {
                                                <AccordionItem
                                                    key=2
                                                    label="Notifications"
                                                    default_open=third_default_open
                                                    on_open_change=on_item_api_2_change
                                                >
                                                    <div>"Notifications panel content"</div>
                                                </AccordionItem>
                                            }
                                                .into_any()
                                        }}
                                    </Accordion>
                                }
                            }
                        />
                    </div>
                    <div class="docs-stack docs-stack--tight" data-slot="accordion-item-api-events">
                        <span class="docs-search__label">"on_panel_lifecycle feed"</span>
                        <Show
                            when=move || !item_api_lifecycle_events.get().is_empty()
                            fallback=move || view! {
                                <span class="ui-muted">"no lifecycle events yet"</span>
                            }
                        >
                            <ul class="docs-stack docs-stack--tight">
                                {move || {
                                    item_api_lifecycle_events
                                        .get()
                                        .into_iter()
                                        .map(|event| view! { <li>{event}</li> })
                                        .collect_view()
                                }}
                            </ul>
                        </Show>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
