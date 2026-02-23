use super::*;

pub(crate) fn chip() -> AnyView {
    let variant_options = vec![
        "default".to_string(),
        "accent".to_string(),
        "danger".to_string(),
        "outline".to_string(),
    ];
    let size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(1));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1));
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_dismissible, set_workbench_is_dismissible) = signal(true);
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(1) {
            0 => ChipVariant::Default,
            2 => ChipVariant::Danger,
            3 => ChipVariant::Outline,
            _ => ChipVariant::Accent,
        });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => ChipSize::Sm,
        2 => ChipSize::Lg,
        _ => ChipSize::Md,
    });
    let hello_code = Signal::derive(move || {
        r#"<Chip variant=ChipVariant::Accent size=ChipSize::Md on_dismiss=Callback::new(|_| ())>
  "Reviewer"
</Chip>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let variant = workbench_variant.get();
        let size = workbench_size.get();
        let is_disabled = workbench_is_disabled.get();
        let is_dismissible = workbench_is_dismissible.get();
        let custom_label = workbench_custom_label.get() && is_dismissible;
        let custom_class = workbench_custom_class.get();

        let mut snippet = vec!["<Chip".to_string()];
        if variant != ChipVariant::Default {
            snippet.push(format!("  variant=ChipVariant::{variant:?}"));
        }
        if size != ChipSize::Md {
            snippet.push(format!("  size=ChipSize::{size:?}"));
        }
        if is_disabled {
            snippet.push("  is_disabled=true".to_string());
        }
        if is_dismissible {
            snippet.push("  on_dismiss=Callback::new(|_| ())".to_string());
        }
        if custom_label {
            snippet.push("  dismiss_aria_label=\"Remove reviewer\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-chip-custom\".into()".to_string());
        }
        snippet.push(">".to_string());
        snippet.push("  \"Reviewer\"".to_string());
        snippet.push("</Chip>".to_string());
        snippet.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let variant = workbench_variant.get();
        let size = workbench_size.get();
        let is_disabled = workbench_is_disabled.get();
        let is_dismissible = workbench_is_dismissible.get();
        let custom_label = workbench_custom_label.get() && is_dismissible;
        let custom_class = workbench_custom_class.get();

        let state = if is_disabled {
            "disabled"
        } else if is_dismissible {
            "removable"
        } else {
            "static"
        };

        let mut classes = vec![
            "ui-chip".to_string(),
            variant.class_name().into(),
            size.class_name().into(),
            format!("ui-chip--{state}"),
        ];
        classes.push(if custom_label {
            "ui-chip--dismiss-label-custom".to_string()
        } else {
            "ui-chip--dismiss-label-default".to_string()
        });
        if !is_disabled {
            classes.push("ui-chip--enabled".to_string());
        }
        if custom_class {
            classes.push("ui-chip--custom-class".to_string());
            classes.push("docs-chip-custom".to_string());
        }

        format!(
            "ChipActualConfig {{\n  variant: {variant:?},\n  size: {size:?},\n  is_disabled: {is_disabled},\n  on_dismiss: {:?},\n  motion: {:?},\n  dismiss_aria_label: {:?},\n  class_name: {:?},\n  is_dismissible: {is_dismissible},\n  custom_dismiss_label: {custom_label},\n  custom_class: {custom_class},\n  class: \"{}\",\n  marker_expectations: [\"data-variant\", \"data-size\", \"data-state\", \"data-dismiss-label-source\", \"data-class-source\"],\n}}",
            if is_dismissible {
                Some("Callback<MouseEvent>")
            } else {
                None
            },
            ui::chip::ChipMotion::default(),
            if custom_label {
                Some("Remove reviewer")
            } else {
                None
            },
            if custom_class {
                Some("docs-chip-custom")
            } else {
                None
            },
            classes.join(" ")
        )
    });

    let chip_test_css_source = Signal::derive(move || {
        format!(
            "/* components/chip/src/styles.rs */\n{}",
            ui::chip::styles::CSS
        )
    });

    let matrix_code = Signal::derive(move || {
        r#"<Chip variant=ChipVariant::Default size=ChipSize::Sm>"Default / Static"</Chip>
<Chip variant=ChipVariant::Accent size=ChipSize::Md on_dismiss=Callback::new(|_| ())>
  "Accent / Removable"
</Chip>
<Chip variant=ChipVariant::Danger size=ChipSize::Lg is_disabled=true on_dismiss=Callback::new(|_| ())>
  "Danger / Disabled"
</Chip>
<Chip
  variant=ChipVariant::Outline
  size=ChipSize::Md
  on_dismiss=Callback::new(|_| ())
  dismiss_aria_label="Remove reviewer".to_string()
  class_name="docs-chip-custom".to_string()
>
  "Outline / Custom"
</Chip>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Chip"
            slug="chip"
            group="Display"
            description="Chip / tag pill with centralized variant-size-state attrs, dismiss-label source contracts, and optional custom class semantics."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_code
            >
                <div class="docs-row">
                    <Chip
                        variant=ChipVariant::Accent
                        size=ChipSize::Md
                        on_dismiss=Callback::new(|_| ())
                    >
                        "Reviewer"
                    </Chip>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                test_css_source=chip_test_css_source
                test_source_path="components/chip/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 variant/size/disabled/dismiss/custom，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Variant"</div>
                            <SegmentedControl
                                id_base="docs-chip-variant".to_string()
                                options=variant_options.clone()
                                selected_index=workbench_variant_index
                                set_selected_index=set_workbench_variant_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chip variant".to_string()
                            />

                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-chip-size".to_string()
                                options=size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chip size".to_string()
                            />

                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "is_disabled"
                            </Switch>
                            <Switch checked=workbench_is_dismissible set_checked=set_workbench_is_dismissible>
                                "Dismiss action"
                            </Switch>
                            <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                                "Custom dismiss aria label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class_name"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let variant = workbench_variant.get();
                        let size = workbench_size.get();
                        let is_disabled = workbench_is_disabled.get();
                        let is_dismissible = workbench_is_dismissible.get();
                        let dismiss_aria_label = if workbench_custom_label.get() && is_dismissible {
                            "Remove reviewer".to_string()
                        } else {
                            "".to_string()
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-chip-custom".to_string()
                        } else {
                            "".to_string()
                        };

                        if is_dismissible {
                            view! {
                                <Chip
                                    variant=variant
                                    size=size
                                    is_disabled=is_disabled
                                    on_dismiss=Callback::new(|_| ())
                                    motion=ui::chip::ChipMotion::default()
                                    dismiss_aria_label=dismiss_aria_label
                                    class_name=class_name
                                >
                                    "Reviewer"
                                </Chip>
                            }
                                .into_any()
                        } else {
                            view! {
                                <Chip
                                    variant=variant
                                    size=size
                                    is_disabled=is_disabled
                                    motion=ui::chip::ChipMotion::default()
                                    class_name=class_name
                                >
                                    "Reviewer"
                                </Chip>
                            }
                                .into_any()
                        }
                    }}

                    <div class="docs-row">
                        <span class="ui-muted">"Compare baseline:"</span>
                        <Chip variant=ChipVariant::Default size=ChipSize::Sm>"Default"</Chip>
                        <Chip variant=ChipVariant::Accent size=ChipSize::Md>"Accent"</Chip>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Variant / Size / Disabled / Custom)"
                code_signal=matrix_code
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Default / Static"</span>
                        <Chip variant=ChipVariant::Default size=ChipSize::Sm>
                            "Default / Static"
                        </Chip>
                    </div>
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Accent / Removable"</span>
                        <Chip
                            variant=ChipVariant::Accent
                            size=ChipSize::Md
                            on_dismiss=Callback::new(|_| ())
                        >
                            "Accent / Removable"
                        </Chip>
                    </div>
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Danger / Disabled"</span>
                        <Chip
                            variant=ChipVariant::Danger
                            size=ChipSize::Lg
                            is_disabled=true
                            on_dismiss=Callback::new(|_| ())
                        >
                            "Danger / Disabled"
                        </Chip>
                    </div>
                    <div class="docs-card" style="flex: 1 1 200px;">
                        <span class="ui-muted">"Outline / Custom"</span>
                        <Chip
                            variant=ChipVariant::Outline
                            size=ChipSize::Md
                            on_dismiss=Callback::new(|_| ())
                            dismiss_aria_label="Remove reviewer".to_string()
                            class_name="docs-chip-custom".to_string()
                        >
                            "Outline / Custom"
                        </Chip>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
