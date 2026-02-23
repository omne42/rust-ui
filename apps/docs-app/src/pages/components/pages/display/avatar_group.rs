use super::*;

pub(crate) fn avatar_group() -> AnyView {
    let src_a = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27%232b5cff%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EA%3C/text%3E%3C/svg%3E";
    let src_b = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27%23ff4bd8%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EG%3C/text%3E%3C/svg%3E";
    let src_c = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27%2312b981%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3ET%3C/text%3E%3C/svg%3E";

    let items = vec![
        AvatarGroupItem {
            name: Some("Ada Lovelace".to_string()),
            src: Some(src_a.into()),
            alt: Some("Ada".to_string()),
        },
        AvatarGroupItem {
            name: Some("Grace Hopper".to_string()),
            src: Some(src_b.into()),
            alt: Some("Grace".to_string()),
        },
        AvatarGroupItem {
            name: Some("Alan Turing".to_string()),
            src: Some(src_c.into()),
            alt: Some("Alan".to_string()),
        },
        AvatarGroupItem {
            name: Some("Katherine Johnson".to_string()),
            src: None,
            alt: Some("Katherine".to_string()),
        },
        AvatarGroupItem {
            name: Some("Annie Easley".to_string()),
            src: None,
            alt: Some("Annie".to_string()),
        },
    ];

    let empty_items: Vec<AvatarGroupItem> = Vec::new();
    let empty_items_for_hello = empty_items.clone();
    let empty_items_for_state_matrix = empty_items.clone();
    let empty_items_for_controlled = empty_items.clone();
    let empty_items_custom: Vec<AvatarGroupItem> = Vec::new();
    let overflow_items = items.clone();
    let size_items = items.clone();
    let custom_items = items.clone();
    let state_matrix_items = items.clone();
    let controlled_items = items.clone();
    let stream_snapshot_items = items.clone();
    let workbench_items_overflow = items.clone();
    let workbench_items_stable = items.iter().take(2).cloned().collect::<Vec<_>>();
    let workbench_items_empty: Vec<AvatarGroupItem> = Vec::new();
    let workbench_items_overflow_for_state_matrix = workbench_items_overflow.clone();
    let workbench_items_stable_for_state_matrix = workbench_items_stable.clone();
    let workbench_items_empty_for_state_matrix = workbench_items_empty.clone();
    let source_first_items = items;
    let code_imports =
        "use leptos::prelude::*;\nuse ui::{AvatarGroup, AvatarGroupItem, AvatarSize};".to_string();

    let hello_code =
        Signal::derive(move || r#"<AvatarGroup items=empty_items.clone() />"#.to_string());

    let overflow_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=3
  size=AvatarSize::Md
/>"#
        .to_string()
    });

    let sizes_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=6
  size=AvatarSize::Sm
/>
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=6
  size=AvatarSize::Lg
/>"#
        .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=Vec::<AvatarGroupItem>::new()
  size=AvatarSize::Md
  aria_label="No collaborators".to_string()
  class_name="docs-avatar-group-custom".to_string()
/>
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=3
  size=AvatarSize::Md
  aria_label="Core collaborators".to_string()
  class_name="docs-avatar-group-custom".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<AvatarGroup items=empty_items.clone() />
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
  ]
  max=4
  size=AvatarSize::Md
/>
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=2
  size=AvatarSize::Md
  aria_label="Core collaborators".to_string()
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"// AvatarGroup has no controlled/uncontrolled runtime axis (`value/on_value_change/default_value`).
// Contrast default props with app-state mapped props.
let upstream_max = 2_usize;

<AvatarGroup items=empty_items.clone() />
<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=upstream_max
  aria_label="Upstream mapped".to_string()
/>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
  ]
  max=2
  aria_label="Snapshot baseline".to_string()
/>
// Streaming Optional; fallback=snapshot.
// Inspect markers: data-ui-stream-support=optional data-ui-stream-fallback=snapshot data-ui-output-status=verified."#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<AvatarGroup
  items=vec![
    AvatarGroupItem {
      name: Some("Ada Lovelace".to_string()),
      src: None,
      alt: Some("Ada".to_string()),
    },
    AvatarGroupItem {
      name: Some("Grace Hopper".to_string()),
      src: None,
      alt: Some("Grace".to_string()),
    },
    AvatarGroupItem {
      name: Some("Alan Turing".to_string()),
      src: None,
      alt: Some("Alan".to_string()),
    },
    AvatarGroupItem {
      name: Some("Katherine Johnson".to_string()),
      src: None,
      alt: Some("Katherine".to_string()),
    },
  ]
  max=3
  size=AvatarSize::Md
  aria_label="Copy-ready collaborators".to_string()
/>"#
        .to_string()
    });

    let workbench_roster_options = vec![
        "empty".to_string(),
        "stable".to_string(),
        "overflow".to_string(),
    ];
    let workbench_size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let workbench_max_options = vec!["2".to_string(), "3".to_string(), "4".to_string()];
    let (workbench_roster_index, set_workbench_roster_index) = signal(Some(2_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_max_index, set_workbench_max_index) = signal(Some(1_usize));
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_roster =
        Signal::derive(move || match workbench_roster_index.get().unwrap_or(2) {
            0 => "empty",
            1 => "stable",
            _ => "overflow",
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => AvatarSize::Sm,
        2 => AvatarSize::Lg,
        _ => AvatarSize::Md,
    });
    let workbench_max = Signal::derive(move || match workbench_max_index.get().unwrap_or(1) {
        0 => 2_usize,
        2 => 4_usize,
        _ => 3_usize,
    });

    let workbench_code = Signal::derive(move || {
        let roster = workbench_roster.get();
        let size = match workbench_size.get() {
            AvatarSize::Sm => "AvatarSize::Sm",
            AvatarSize::Md => "AvatarSize::Md",
            AvatarSize::Lg => "AvatarSize::Lg",
        };
        let max = workbench_max.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let roster_comment = match roster {
            "empty" => "// empty roster",
            "stable" => "// stable roster (2 items, no overflow)",
            _ => "// overflow roster",
        };

        let mut lines = vec![
            roster_comment.to_string(),
            "<AvatarGroup".to_string(),
            "  items=your_items".to_string(),
            format!("  max={max}"),
            format!("  size={size}"),
        ];
        if custom_aria {
            lines.push("  aria_label=\"Interactive collaborators\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-avatar-group-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let roster = workbench_roster.get();
        let size = match workbench_size.get() {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
        };
        let max = workbench_max.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let items_desc = match roster {
            "empty" => "Vec::<AvatarGroupItem>::new()",
            "stable" => "stable_roster(2)",
            _ => "overflow_roster(5)",
        };

        format!(
            "AvatarGroupWorkbenchConfig {{\n  items: \"{items_desc}\",\n  max: {max},\n  size: \"{size}\",\n  aria_label: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  roster: \"{roster}\",\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  rtl: {rtl},\n}}",
            if custom_aria {
                "Some(\"Interactive collaborators\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-avatar-group-custom\")"
            } else {
                "None"
            },
            if rtl { "Some(\"ar\")" } else { "None" },
            if rtl { "Some(\"rtl\")" } else { "None" },
        )
    });

    view! {
        <ComponentPage
            title="AvatarGroup"
            slug="avatar-group"
            group="Display"
            description="Stacked avatars with centralized overflow/empty/aria-label-source state attrs and baseline-style root contracts."
        >
            <Playground title="Hello World" code_signal=hello_code code_imports=code_imports.clone()>
                <div class="docs-row">
                    <AvatarGroup items=empty_items_for_hello />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State + Preview)"
                description="Adjust roster/size/max and semantic sources in real time. Use this as repeatable acceptance surface."
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{AvatarGroup, AvatarGroupItem, AvatarSize};\nuse ui::color::area::A11yDirection;".to_string()
                test_source_path="components/avatar-group/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="avatar-group-workbench-controls">
                            <div class="docs-search__label">"Roster state"</div>
                            <SegmentedControl
                                id_base="docs-avatar-group-workbench-roster".to_string()
                                options=workbench_roster_options.clone()
                                selected_index=workbench_roster_index
                                set_selected_index=set_workbench_roster_index
                                size=SegmentedControlSize::Sm
                                aria_label="AvatarGroup roster mode".to_string()
                            />

                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-avatar-group-workbench-size".to_string()
                                options=workbench_size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="AvatarGroup size".to_string()
                            />

                            <div class="docs-search__label">"Max visible"</div>
                            <SegmentedControl
                                id_base="docs-avatar-group-workbench-max".to_string()
                                options=workbench_max_options.clone()
                                selected_index=workbench_max_index
                                set_selected_index=set_workbench_max_index
                                size=SegmentedControlSize::Sm
                                aria_label="AvatarGroup max visible".to_string()
                            />

                            <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                "Custom aria label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                "RTL direction"
                            </Switch>
                        </div>
                    }
                }
            >
                {move || {
                    let roster = workbench_roster.get();
                    let size = workbench_size.get();
                    let max = workbench_max.get();
                    let custom_aria = workbench_custom_aria.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let configured_items = match roster {
                        "empty" => workbench_items_empty.clone(),
                        "stable" => workbench_items_stable.clone(),
                        _ => workbench_items_overflow.clone(),
                    };
                    let configured_total = configured_items.len();
                    let visible = configured_total.min(max);
                    let overflow = configured_total.saturating_sub(visible);
                    let expected_state = if configured_total == 0 {
                        "empty"
                    } else if overflow > 0 {
                        "overflow"
                    } else {
                        "stable"
                    };
                    let size_attr = match size {
                        AvatarSize::Sm => "sm",
                        AvatarSize::Md => "md",
                        AvatarSize::Lg => "lg",
                    };

                    let aria_label = if custom_aria {
                        "Interactive collaborators".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-avatar-group-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if rtl { "ar".to_string() } else { String::new() };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    view! {
                        <div class="docs-stack" data-slot="avatar-group-workbench-preview">
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Baseline"</div>
                                    <AvatarGroup items=workbench_items_overflow.clone() max=3 size=AvatarSize::Md />
                                </div>
                                <div
                                    class="docs-stack docs-stack--tight"
                                    data-slot="avatar-group-workbench-configured"
                                >
                                    <div class="docs-search__label">"Configured"</div>
                                    <AvatarGroup
                                        items=configured_items
                                        max=max
                                        size=size
                                        aria_label=aria_label
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    />
                                </div>
                            </div>
                            <p class="ui-muted" data-slot="avatar-group-workbench-state">
                                {format!(
                                    "expected: state={expected_state}, size={size_attr}, total={configured_total}, overflow={overflow}"
                                )}
                            </p>
                            <p class="ui-muted" data-slot="avatar-group-spec-preview-na">
                                "AI Spec input/preview linkage: N/A for AvatarGroup (non-spec component)."
                            </p>
                        </div>
                    }
                }}
            </Playground>



            <Playground title="Overflow Stack" code_signal=overflow_code code_imports=code_imports.clone()>
                <div class="docs-row">
                    <AvatarGroup items=overflow_items.clone() max=3 size=AvatarSize::Md />
                    <AvatarGroup
                        items=overflow_items.clone()
                        max=2
                        size=AvatarSize::Lg
                        aria_label="Core collaborators".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Sizes Without Overflow"
                code_signal=sizes_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Sm />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Md />
                    <AvatarGroup items=size_items.clone() max=6 size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground title="Custom Aria + Class" code_signal=custom_code code_imports=code_imports.clone()>
                <div class="docs-row">
                    <AvatarGroup
                        items=empty_items_custom
                        max=4
                        size=AvatarSize::Md
                        aria_label="No collaborators".to_string()
                        class_name="docs-avatar-group-custom".to_string()
                    />
                    <AvatarGroup
                        items=custom_items
                        max=3
                        size=AvatarSize::Md
                        aria_label="Core collaborators".to_string()
                        class_name="docs-avatar-group-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Roster Scenarios"
                description="Covers empty/stable/overflow and custom aria-label contracts in one matrix."
                code_signal=state_matrix_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=empty_items_for_state_matrix />
                    <AvatarGroup items=state_matrix_items.clone() max=6 size=AvatarSize::Md />
                    <AvatarGroup
                        items=state_matrix_items.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Core collaborators".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="AvatarGroup has no controlled/uncontrolled state machine. Compare default props with app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=empty_items_for_controlled />
                    <AvatarGroup
                        items=controlled_items.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Upstream mapped".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="AvatarGroup is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=code_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="avatar-group-streaming-preview">
                    <AvatarGroup
                        items=stream_snapshot_items.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Snapshot baseline".to_string()
                    />
                    <p class="ui-muted" data-slot="avatar-group-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                </div>
            </Playground>



            <Playground
                title="State Matrix (Empty / Stable / Overflow)"
                description="Workbench 后的多参数对比展示。"
                code_signal=state_matrix_code
                code_imports=code_imports.clone()
            >
                <div class="docs-row">
                    <AvatarGroup items=workbench_items_empty_for_state_matrix.clone() />
                    <AvatarGroup
                        items=workbench_items_stable_for_state_matrix.clone()
                        max=6
                        size=AvatarSize::Md
                    />
                    <AvatarGroup
                        items=workbench_items_overflow_for_state_matrix.clone()
                        max=2
                        size=AvatarSize::Md
                        aria_label="Core collaborators".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=code_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="avatar-group-source-first-preview">
                    <AvatarGroup
                        items=source_first_items.clone()
                        max=3
                        size=AvatarSize::Md
                        aria_label="Copy-ready collaborators".to_string()
                    />
                    <p class="ui-muted" data-slot="avatar-group-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/avatar-group/src/view.rs."
                    </p>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="avatar-group-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p data-slot="avatar-group-source-first-contract">
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="avatar-group-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-avatar-group"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        " path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text=source_first_code.get()
                    label="Copy avatar-group starter".to_string()
                    copyable=true
                    class_name="docs-avatar-group-source-copy".to_string()
                />
                <ul data-slot="avatar-group-source-paths">
                    <li><code>"components/avatar-group/src/mod.rs"</code></li>
                    <li><code>"components/avatar-group/src/logic.rs"</code></li>
                    <li><code>"components/avatar-group/src/view.rs"</code></li>
                    <li><code>"components/avatar-group/src/styles.rs"</code></li>
                </ul>
                <p class="ui-muted" data-slot="avatar-group-source-sync-note">
                    "Sync note: snippet text is sourced from "
                    <code>"source_first_code"</code>
                    " and mirrors "
                    <code>"components/avatar-group/src/view.rs"</code>
                    " API usage; update docs snippet and source implementation together to avoid drift."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-group-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="avatar-group-state-rows">
                    <li><code>"data-state"</code>" = empty | stable | overflow"</li>
                    <li><code>"data-aria-label-source / data-class-source"</code>" = default | custom"</li>
                    <li><code>"data-ui-state / data-ui-action"</code>" = empty/stable/overflow with render-stable-roster | render-overflow-summary"</li>
                    <li><code>"controlled/uncontrolled axis"</code>" = N/A (AvatarGroup has no runtime controllable state machine)"</li>
                    <li><code>"disabled axis"</code>" = N/A (AvatarGroup has no is_disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-group-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="avatar-group-parameter-rows">
                    <li><code>"items: Vec&lt;AvatarGroupItem&gt;"</code>" required roster input (Hello World uses empty vec)"</li>
                    <li><code>"max: Option&lt;usize&gt;"</code>" default = None -> normalize to 4 in logic (`normalize_avatar_group_max_visible`)"</li>
                    <li><code>"size: AvatarSize"</code>" default = AvatarSize::Md"</li>
                    <li><code>"aria_label: Option&lt;String&gt;"</code>" default = None -> i18n default aria label via logic fallback"</li>
                    <li><code>"class_name: Option&lt;String&gt;, lang: Option&lt;String&gt;"</code>" default = None; blank strings are normalized away in logic"</li>
                    <li><code>"dir: Option&lt;A11yDirection&gt;"</code>" default = None (inherits locale direction context)"</li>
                    <li><code>"AvatarGroupItem{name/src/alt}: Option&lt;String&gt;"</code>" empty/blank values normalize to empty strings in logic output fields"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
