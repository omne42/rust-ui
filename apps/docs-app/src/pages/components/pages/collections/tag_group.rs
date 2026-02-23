use super::*;

pub(crate) fn tag_group() -> AnyView {
    fn default_workbench_tags() -> Vec<Tag> {
        vec![
            Tag::new("tag-rust", "Rust"),
            Tag::new("tag-leptos", "Leptos"),
            Tag::disabled("tag-a11y", "Accessibility"),
            Tag::new("tag-design", "Design tokens"),
        ]
    }

    let (showcase_tags, _set_showcase_tags) = signal(vec![
        Tag::new("tag-showcase-rust", "Rust"),
        Tag::new("tag-showcase-leptos", "Leptos"),
        Tag::new("tag-showcase-ui", "UI primitives"),
    ]);

    let (workbench_tags, set_workbench_tags) = signal(default_workbench_tags());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_surface_variant, set_workbench_surface_variant) = signal(false);
    let (workbench_large_size, set_workbench_large_size) = signal(false);
    let (workbench_custom_id_base, set_workbench_custom_id_base) = signal(true);
    let (workbench_show_description, set_workbench_show_description) = signal(true);
    let (workbench_show_error, set_workbench_show_error) = signal(true);
    let (workbench_force_invalid, set_workbench_force_invalid) = signal(false);
    let (workbench_required, set_workbench_required) = signal(true);
    let (workbench_external_aria_describedby, set_workbench_external_aria_describedby) =
        signal(false);
    let (workbench_custom_aria_label, set_workbench_custom_aria_label) = signal(false);
    let (workbench_custom_class_name, set_workbench_custom_class_name) = signal(false);
    let (workbench_zh_lang, set_workbench_zh_lang) = signal(false);
    let (workbench_rtl_dir, set_workbench_rtl_dir) = signal(false);
    let (workbench_remove_count, set_workbench_remove_count) = signal(0_u32);
    let (workbench_last_removed, set_workbench_last_removed) = signal(None::<String>);
    let (workbench_next_custom_tag, set_workbench_next_custom_tag) = signal(1_u32);

    let on_workbench_remove = Callback::new(move |tag: Tag| {
        let removed_label = tag.label.clone();
        set_workbench_tags.update(|tags| tags.retain(|item| item.id != tag.id));
        set_workbench_remove_count.update(|count| *count += 1);
        set_workbench_last_removed.set(Some(removed_label));
    });

    let on_add_custom_tag = Callback::new(move |_| {
        let index = workbench_next_custom_tag.get();
        set_workbench_tags.update(|tags| {
            tags.push(Tag::new(
                format!("tag-custom-{index}"),
                format!("Custom tag {index}"),
            ));
        });
        set_workbench_next_custom_tag.update(|next| *next += 1);
    });

    let on_reset_workbench_tags = Callback::new(move |_| {
        set_workbench_tags.set(default_workbench_tags());
        set_workbench_remove_count.set(0);
        set_workbench_last_removed.set(None);
    });

    let on_clear_workbench_tags = Callback::new(move |_| {
        set_workbench_tags.set(Vec::new());
    });

    let workbench_invalid =
        Signal::derive(move || workbench_force_invalid.get() || workbench_tags.get().is_empty());
    let workbench_required_signal = Signal::derive(move || workbench_required.get());
    let workbench_aria_describedby_signal = Signal::derive(move || {
        if workbench_external_aria_describedby.get() {
            Some("tag-group-external-help".to_string())
        } else {
            None
        }
    });

    let hello_code = Signal::derive(move || {
        r#"let (tags, _set_tags) = signal(vec![
  Tag::new("tag-showcase-rust", "Rust"),
  Tag::new("tag-showcase-leptos", "Leptos"),
  Tag::new("tag-showcase-ui", "UI primitives"),
]);

<TagGroup tags=tags label=Some("Project labels".to_string()) />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = if workbench_surface_variant.get() {
            "TagVariant::Surface"
        } else {
            "TagVariant::Default"
        };
        let size = if workbench_large_size.get() {
            "TagSize::Lg"
        } else {
            "TagSize::Md"
        };
        let id_base = if workbench_custom_id_base.get() {
            "Some(\"docs-tag-group-workbench\".to_string())"
        } else {
            "None"
        };
        let label = "Some(\"Framework tags\".to_string())";
        let description = if workbench_show_description.get() {
            "Some(\"Remove chips and observe feedback\".to_string())"
        } else {
            "None"
        };
        let error = if workbench_show_error.get() {
            "Some(\"At least one tag is required\".to_string())"
        } else {
            "None"
        };
        let aria_describedby = if workbench_external_aria_describedby.get() {
            "Signal::derive(|| Some(\"tag-group-external-help\".to_string()))"
        } else {
            "Signal::derive(|| None::<String>)"
        };
        let aria_label = if workbench_custom_aria_label.get() {
            "Some(\"Selected framework tags\".to_string())"
        } else {
            "None"
        };
        let class_name = if workbench_custom_class_name.get() {
            rust_string_literal("docs-tag-group-workbench")
        } else {
            "None".to_string()
        };
        let lang = if workbench_zh_lang.get() {
            "Some(\"zh-CN\".to_string())"
        } else {
            "Some(\"en-US\".to_string())"
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(ui_headless::A11yDirection::Rtl)"
        } else {
            "Some(ui_headless::A11yDirection::Ltr)"
        };

        let mut lines = vec![
            "let (tags, set_tags) = signal(vec![ ... ]);".to_string(),
            "let on_remove = Callback::new(move |tag: Tag| {".to_string(),
            "  set_tags.update(|items| items.retain(|item| item.id != tag.id));".to_string(),
            "});".to_string(),
            "<TagGroup".to_string(),
            "  tags=tags".to_string(),
            format!("  disabled={}", bool_word(workbench_disabled.get())),
            "  on_remove=on_remove".to_string(),
            format!("  variant={variant}"),
            format!("  size={size}"),
            format!("  id_base={id_base}"),
            format!("  label={label}"),
            format!("  description={description}"),
            format!("  error={error}"),
            format!(
                "  invalid=Signal::derive(|| {})",
                bool_word(workbench_invalid.get())
            ),
            format!(
                "  required=Signal::derive(|| {})",
                bool_word(workbench_required_signal.get())
            ),
            format!("  aria_describedby={aria_describedby}"),
            format!("  aria_label={aria_label}"),
            format!("  class_name={class_name}"),
            format!("  lang={lang}"),
            format!("  dir={dir}"),
        ];
        push_line_when(&mut lines, true, "/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tags_repr = {
            let tags = workbench_tags.get();
            if tags.is_empty() {
                "[]".to_string()
            } else {
                format!(
                    "[{}]",
                    tags.into_iter()
                        .map(|tag| format!(
                            "{{ id: {:?}, label: {:?}, disabled: {} }}",
                            tag.id,
                            tag.label,
                            bool_word(tag.disabled)
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        };

        let id_base = if workbench_custom_id_base.get() {
            Some("docs-tag-group-workbench".to_string())
        } else {
            None
        };
        let label = Some("Framework tags".to_string());
        let description = if workbench_show_description.get() {
            Some("Remove chips and observe feedback".to_string())
        } else {
            None
        };
        let error = if workbench_show_error.get() {
            Some("At least one tag is required".to_string())
        } else {
            None
        };
        let aria_describedby = if workbench_external_aria_describedby.get() {
            Some("tag-group-external-help".to_string())
        } else {
            None
        };
        let aria_label = if workbench_custom_aria_label.get() {
            Some("Selected framework tags".to_string())
        } else {
            None
        };
        let class_name = if workbench_custom_class_name.get() {
            Some("docs-tag-group-workbench".to_string())
        } else {
            None
        };
        let lang = if workbench_zh_lang.get() {
            Some("zh-CN".to_string())
        } else {
            Some("en-US".to_string())
        };
        let dir = if workbench_rtl_dir.get() {
            "Some(A11yDirection::Rtl)"
        } else {
            "Some(A11yDirection::Ltr)"
        };
        let variant = if workbench_surface_variant.get() {
            "TagVariant::Surface"
        } else {
            "TagVariant::Default"
        };
        let size = if workbench_large_size.get() {
            "TagSize::Lg"
        } else {
            "TagSize::Md"
        };
        let last_removed = workbench_last_removed
            .get()
            .unwrap_or_else(|| "None".to_string());

        format!(
            "TagGroupActualConfig {{\n  tags: {tags_repr},\n  disabled: {},\n  on_remove: \"count={}, last={}\",\n  variant: {variant},\n  size: {size},\n  id_base: {id_base:?},\n  label: {label:?},\n  description: {description:?},\n  error: {error:?},\n  invalid: {},\n  required: {},\n  aria_describedby: {aria_describedby:?},\n  aria_label: {aria_label:?},\n  class_name: {class_name:?},\n  lang: {lang:?},\n  dir: {dir},\n}}",
            bool_word(workbench_disabled.get()),
            workbench_remove_count.get(),
            last_removed,
            bool_word(workbench_invalid.get()),
            bool_word(workbench_required_signal.get()),
        )
    });

    let (matrix_default_tags, _set_matrix_default_tags) = signal(vec![
        Tag::new("tag-matrix-rust", "Rust"),
        Tag::new("tag-matrix-wasm", "WASM"),
        Tag::new("tag-matrix-a11y", "A11y"),
    ]);
    let (matrix_surface_tags, _set_matrix_surface_tags) = signal(vec![
        Tag::new("tag-matrix-design", "Design"),
        Tag::new("tag-matrix-theme", "Theme"),
        Tag::disabled("tag-matrix-tokens", "Tokens"),
    ]);
    let (matrix_invalid_tags, _set_matrix_invalid_tags) = signal(Vec::<Tag>::new());

    let matrix_code = Signal::derive(move || {
        r#"<TagGroup
  tags=default_tags
  variant=TagVariant::Default
  size=TagSize::Md
  label=Some("Default".to_string())
/>
<TagGroup
  tags=surface_tags
  disabled=true
  variant=TagVariant::Surface
  size=TagSize::Lg
  label=Some("Disabled Surface".to_string())
/>
<TagGroup
  tags=invalid_tags
  variant=TagVariant::Default
  size=TagSize::Md
  label=Some("Required".to_string())
  error=Some("At least one tag is required".to_string())
  invalid=Signal::derive(|| true)
  required=Signal::derive(|| true)
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="TagGroup"
            slug="tag-group"
            group="Collections"
            description="TagGroup playground follows showcase/workbench/matrix with full API coverage and callback feedback."
        >
            <Playground
                title="Hello World (Default TagGroup)"
                code_signal=hello_code
                test_source_path="components/tag/src/group/view.rs".to_string()
            >
                <TagGroup
                    tags=showcase_tags
                    label="Project labels".to_string()
                />
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="tag-group-workbench-controls">
                        <Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "disabled"
                        </Switch>
                        <Switch
                            checked=workbench_surface_variant
                            set_checked=set_workbench_surface_variant
                        >
                            "Surface variant"
                        </Switch>
                        <Switch checked=workbench_large_size set_checked=set_workbench_large_size>
                            "Large size"
                        </Switch>
                        <Switch
                            checked=workbench_custom_id_base
                            set_checked=set_workbench_custom_id_base
                        >
                            "Custom id_base"
                        </Switch>
                        <Switch
                            checked=workbench_show_description
                            set_checked=set_workbench_show_description
                        >
                            "description"
                        </Switch>
                        <Switch checked=workbench_show_error set_checked=set_workbench_show_error>
                            "error"
                        </Switch>
                        <Switch checked=workbench_force_invalid set_checked=set_workbench_force_invalid>
                            "force invalid"
                        </Switch>
                        <Switch checked=workbench_required set_checked=set_workbench_required>
                            "required"
                        </Switch>
                        <Switch
                            checked=workbench_external_aria_describedby
                            set_checked=set_workbench_external_aria_describedby
                        >
                            "aria_describedby"
                        </Switch>
                        <Switch
                            checked=workbench_custom_aria_label
                            set_checked=set_workbench_custom_aria_label
                        >
                            "aria_label"
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
                        <div class="docs-row docs-row--tight">
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_add_custom_tag
                            >
                                "Add tag"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_reset_workbench_tags
                            >
                                "Reset tags"
                            </ui::Button>
                            <ui::Button
                                variant=ui::ButtonVariant::Secondary
                                on_press=on_clear_workbench_tags
                            >
                                "Clear tags"
                            </ui::Button>
                        </div>
                    </div>
                }
                test_source_path="components/tag/src/group/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="tag-group-workbench-display">
                    <Show when=move || workbench_external_aria_describedby.get()>
                        <p id="tag-group-external-help" class="ui-muted">
                            "external help text wired by aria_describedby"
                        </p>
                    </Show>
                    <TagGroup
                        tags=workbench_tags
                        disabled=workbench_disabled.get()
                        on_remove=on_workbench_remove
                        variant=if workbench_surface_variant.get() {
                            ui::TagVariant::Surface
                        } else {
                            ui::TagVariant::Default
                        }
                        size=if workbench_large_size.get() {
                            ui::TagSize::Lg
                        } else {
                            ui::TagSize::Md
                        }
                        id_base=if workbench_custom_id_base.get() {
                            "docs-tag-group-workbench".to_string()
                        } else {
                            String::new()
                        }
                        label="Framework tags".to_string()
                        description=if workbench_show_description.get() {
                            "Remove chips and observe feedback".to_string()
                        } else {
                            String::new()
                        }
                        error=if workbench_show_error.get() {
                            "At least one tag is required".to_string()
                        } else {
                            String::new()
                        }
                        invalid=workbench_invalid
                        required=workbench_required_signal
                        aria_describedby=workbench_aria_describedby_signal
                        aria_label=if workbench_custom_aria_label.get() {
                            "Selected framework tags".to_string()
                        } else {
                            String::new()
                        }
                        class_name=if workbench_custom_class_name.get() {
                            "docs-tag-group-workbench".to_string()
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
                    />
                    <span class="ui-muted" data-slot="tag-group-workbench-feedback">
                        "on_remove count: " {move || workbench_remove_count.get()}
                        " · last removed: "
                        {move || {
                            workbench_last_removed
                                .get()
                                .unwrap_or_else(|| "None".to_string())
                        }}
                        " · remaining: " {move || workbench_tags.get().len()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Surface / Required)"
                code_signal=matrix_code
                test_source_path="components/tag/src/group/view.rs".to_string()
            >
                <div class="docs-row" data-slot="tag-group-state-matrix">
                    <div class="docs-stack">
                        <TagGroup
                            tags=matrix_default_tags
                            variant=ui::TagVariant::Default
                            size=ui::TagSize::Md
                            label="Default".to_string()
                            description="Removable in normal state".to_string()
                            on_remove=Callback::new(move |_| {})
                        />
                    </div>
                    <div class="docs-stack">
                        <TagGroup
                            tags=matrix_surface_tags
                            disabled=true
                            variant=ui::TagVariant::Surface
                            size=ui::TagSize::Lg
                            label="Disabled Surface".to_string()
                            description="Large + disabled visual variant".to_string()
                        />
                    </div>
                    <div class="docs-stack">
                        <TagGroup
                            tags=matrix_invalid_tags
                            variant=ui::TagVariant::Default
                            size=ui::TagSize::Md
                            label="Required".to_string()
                            error="At least one tag is required".to_string()
                            invalid=Signal::derive(|| true)
                            required=Signal::derive(|| true)
                        />
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
