use super::*;

pub(crate) fn link_button() -> AnyView {
    let variant_options = vec![
        "Default".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Outline".to_string(),
    ];
    let (variant_index, set_variant_index) = signal(Some(0_usize));
    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => ButtonVariant::Secondary,
        2 => ButtonVariant::Ghost,
        3 => ButtonVariant::Outline,
        _ => ButtonVariant::Default,
    });

    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        2 => ButtonSize::M,
        3 => ButtonSize::L,
        _ => ButtonSize::Xl,
    });

    let (disabled, set_disabled) = signal(false);
    let (open_in_new_tab, set_open_in_new_tab) = signal(false);
    let (sponsored_rel, set_sponsored_rel) = signal(false);

    let code = Signal::derive(move || {
        let variant = variant.get();
        let size = size.get();
        let disabled = disabled.get();

        let mut snippet = vec![
            "<LinkButton".to_string(),
            "  href=\"https://example.com/docs\".into()".to_string(),
        ];

        if variant != ButtonVariant::Default {
            snippet.push(format!("  variant=ButtonVariant::{variant:?}"));
        }
        if size != ButtonSize::M {
            snippet.push(format!("  size=ButtonSize::{size:?}"));
        }
        if disabled {
            snippet.push("  disabled=true".to_string());
        }
        if open_in_new_tab.get() {
            snippet.push("  target=Some(\"_blank\")".to_string());
        }
        if sponsored_rel.get() {
            snippet.push("  rel=Some(\"sponsored\".into())".to_string());
        }

        snippet.extend([
            ">".to_string(),
            "  \"Open docs\"".to_string(),
            "</LinkButton>".to_string(),
        ]);

        snippet.join("\n")
    });

    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/button/link_button/styles.rs */\n{}",
            ui::link_button::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let rel_value = if sponsored_rel.get() { "sponsored" } else { "" };
        format!(
            "LinkButtonWorkbenchConfig {{\n  href: \"https://example.com/docs\",\n  variant: \"{:?}\",\n  size: \"{:?}\",\n  disabled: {},\n  target: \"{}\",\n  rel: \"{}\",\n  aria_label: {},\n  class_name: Some(\"docs-link-button-workbench\"),\n}}",
            variant.get(),
            size.get(),
            disabled.get(),
            if open_in_new_tab.get() {
                "_blank"
            } else {
                "_self"
            },
            rel_value,
            if open_in_new_tab.get() {
                "Some(\"Open docs in a new tab\")"
            } else {
                "Some(\"Open docs in the same tab\")"
            },
        )
    });

    let showcase_code = Signal::derive(move || {
        r#"<LinkButton href="https://example.com/docs".to_string()>
  "Open docs"
</LinkButton>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<LinkButton href="https://example.com/xs".to_string() size=ButtonSize::Xs>
  "xs"
</LinkButton>
<LinkButton href="https://example.com/s".to_string() size=ButtonSize::S>
  "s"
</LinkButton>
<LinkButton href="https://example.com/m".to_string() size=ButtonSize::M>
  "m"
</LinkButton>
<LinkButton
  href="https://example.com/l".to_string()
  size=ButtonSize::L
  variant=ButtonVariant::Secondary
>
  "l secondary"
</LinkButton>
<LinkButton
  href="https://example.com/xl".to_string()
  size=ButtonSize::Xl
>
  "xl"
</LinkButton>
<LinkButton href="https://example.com/disabled".to_string() disabled=true>
  "Disabled"
</LinkButton>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="LinkButton"
            slug="link-button"
            group="Actions"
            description="Button styling on anchors with baseline-style disabled semantics and secure rel handling for external targets."
        >
            <Playground title="Hello World (Default LinkButton)" code_signal=showcase_code>
                <LinkButton href="https://example.com/docs".to_string()>
                    "Open docs"
                </LinkButton>
            </Playground>

            <Playground
                title="Interactive Playground (Display + Config + Code + CSS Test)"
                code_signal=code
                test_css_source=workbench_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui/src/button/link_button/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-link-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="LinkButton variant".to_string()
                        />

                        <div class="docs-search__label">"Size"</div>
                        <SegmentedControl
                            id_base="docs-link-button-size".to_string()
                            options=size_options.clone()
                            selected_index=size_index
                            set_selected_index=set_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="LinkButton size".to_string()
                        />

                        <Switch checked=disabled set_checked=set_disabled>"Disabled"</Switch>
                        <Switch checked=open_in_new_tab set_checked=set_open_in_new_tab>
                            "Open in new tab (_blank)"
                        </Switch>
                        <Switch checked=sponsored_rel set_checked=set_sponsored_rel>
                            "Add sponsored rel"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    {move || {
                        let variant = variant.get();
                        let size = size.get();
                        let disabled = disabled.get();
                        let rel = if sponsored_rel.get() {
                            "sponsored".to_string()
                        } else {
                            String::new()
                        };

                        view! {
                            <div class="docs-row">
                                {if open_in_new_tab.get() {
                                    view! {
                                        <LinkButton
                                            href="https://example.com/docs".to_string()
                                            target="_blank"
                                            rel=rel.clone()
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            aria_label="Open docs in a new tab".to_string()
                                            class_name="docs-link-button-workbench".to_string()
                                        >
                                            "Open docs"
                                        </LinkButton>
                                    }
                                        .into_any()
                                } else {
                                    view! {
                                        <LinkButton
                                            href="https://example.com/docs".to_string()
                                            rel=rel
                                            variant=variant
                                            size=size
                                            disabled=disabled
                                            aria_label="Open docs in the same tab".to_string()
                                            class_name="docs-link-button-workbench".to_string()
                                        >
                                            "Open docs"
                                        </LinkButton>
                                    }
                                        .into_any()
                                }}
                                <LinkButton href="https://example.com/changelog".to_string()>
                                    "Same tab"
                                </LinkButton>
                                <LinkButton href="   ".to_string() variant=ButtonVariant::Ghost>
                                    "Missing href"
                                </LinkButton>
                            </div>
                        }
                    }}
                    <span class="ui-muted">
                        "_blank links auto-append noopener+noreferrer; blank href is normalized as non-navigable."
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Variant + size + disabled)" code_signal=states_code>
                <div class="docs-stack">
                    <div class="docs-row">
                        <LinkButton href="https://example.com/xs".to_string() size=ButtonSize::Xs>
                            "xs"
                        </LinkButton>
                        <LinkButton href="https://example.com/s".to_string() size=ButtonSize::S>
                            "s"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/m".to_string()
                            size=ButtonSize::M
                        >
                            "m"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/l".to_string()
                            size=ButtonSize::L
                            variant=ButtonVariant::Secondary
                        >
                            "l secondary"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/xl".to_string()
                            size=ButtonSize::Xl
                        >
                            "xl"
                        </LinkButton>
                    </div>
                    <div class="docs-row">
                        <LinkButton href="https://example.com/disabled".to_string() disabled=true>
                            "Disabled"
                        </LinkButton>
                        <LinkButton
                            href="https://example.com/disabled-ghost".to_string()
                            variant=ButtonVariant::Ghost
                            disabled=true
                        >
                            "Disabled ghost"
                        </LinkButton>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
