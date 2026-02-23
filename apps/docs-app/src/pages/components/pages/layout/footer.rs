use super::*;

pub(crate) fn footer() -> AnyView {
    let (workbench_muted, set_workbench_muted) = signal(true);
    let (workbench_bordered, set_workbench_bordered) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let semantic_code = Signal::derive(move || {
        r#"<Footer>
  <p>"Cancel · Save"</p>
</Footer>
<Footer tone=FooterTone::Muted>
  <p>"Secondary action hint"</p>
</Footer>"#
            .to_string()
    });

    let bordered_code = Signal::derive(move || {
        r#"<View border=ViewBorder::Subtle radius=ViewRadius::Md>
  <Header bordered=true>
    <h3>"Profile settings"</h3>
  </Header>
  <Content padded=true>
    <p>"Main settings body"</p>
  </Content>
  <Footer
    tone=FooterTone::Muted
    bordered=true
    aria_label="Settings footer".to_string()
    class_name="docs-footer-custom".to_string()
  >
    <p>"Cancel · Save"</p>
  </Footer>
</View>"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone = if workbench_muted.get() {
            "FooterTone::Muted"
        } else {
            "FooterTone::Default"
        };
        let bordered = workbench_bordered.get();
        let aria = workbench_custom_aria.get();
        let class_name = workbench_custom_class.get();

        let mut lines = vec!["<Footer".to_string()];
        lines.push(format!("  tone={tone}"));
        if bordered {
            lines.push("  bordered=true".to_string());
        }
        if aria {
            lines.push("  aria_label=\"Workbench footer\".into()".to_string());
        }
        if class_name {
            lines.push("  class_name=\"docs-footer-workbench\".into()".to_string());
        }
        lines.extend([
            ">".to_string(),
            "  <p>\"Cancel · Save\"</p>".to_string(),
            "</Footer>".to_string(),
        ]);
        lines.join("\n")
    });

    let footer_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui-layout/src/footer/styles.rs */\n{}",
            ui_layout::footer::styles::CSS
        )
    });

    let footer_actual_config = Signal::derive(move || {
        let tone = if workbench_muted.get() {
            FooterTone::Muted
        } else {
            FooterTone::Default
        };
        let bordered = workbench_bordered.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let aria_label = if custom_aria {
            "Workbench footer"
        } else {
            "Footer"
        };

        let mut classes = vec!["ui-footer".to_string(), tone.class_name().into()];
        if bordered {
            classes.push("ui-footer--bordered".to_string());
        }
        if custom_class {
            classes.push("ui-footer--custom-class".to_string());
            classes.push("docs-footer-workbench".to_string());
        }

        format!(
            "FooterActualConfig {{\n  tone: {tone:?},\n  bordered: {bordered},\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  aria_label: \"{aria_label}\",\n  class_name: {},\n  data_state: \"{}\",\n  class: \"{}\",\n}}",
            if custom_class {
                "\"docs-footer-workbench\""
            } else {
                "\"\""
            },
            if bordered && matches!(tone, FooterTone::Muted) {
                "muted-bordered"
            } else if bordered {
                "bordered"
            } else if matches!(tone, FooterTone::Muted) {
                "muted"
            } else {
                "default"
            },
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="Footer"
            slug="footer"
            group="Layout"
            description="Semantic container footer (`<footer>`) with centralized tone/border/source state contracts."
        >
            <Playground title="Semantic Footer + Tone" code_signal=semantic_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <div class="docs-stack">
                        <Footer>
                            <p>"Cancel · Save"</p>
                        </Footer>
                        <Footer tone=FooterTone::Muted>
                            <p>"Secondary action hint"</p>
                        </Footer>
                    </div>
                </View>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                code_signal=workbench_code
                test_css_source=footer_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/crates/ui-layout/src/footer/styles.rs".to_string()
                test_config_signal=footer_actual_config
                description="Footer workbench: 对比展示 + config 快照 + copy-ready code + scoped CSS test."
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <ui::Switch checked=workbench_muted set_checked=set_workbench_muted>
                            "Muted tone"
                        </ui::Switch>
                        <ui::Switch checked=workbench_bordered set_checked=set_workbench_bordered>
                            "Bordered"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                            "Custom aria_label"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let tone = if workbench_muted.get() {
                        FooterTone::Muted
                    } else {
                        FooterTone::Default
                    };
                    let bordered = workbench_bordered.get();
                    let custom_aria = workbench_custom_aria.get();
                    let custom_class = workbench_custom_class.get();

                    view! {
                        <div class="docs-stack">
                            <div class="docs-row">
                                <View border=ViewBorder::Subtle radius=ViewRadius::Md class_name="docs-footer-workbench-card".to_string()>
                                    <Header bordered=true>
                                        <h3>"Configured Footer"</h3>
                                    </Header>
                                    <Content padded=true>
                                        <p>"State toggles apply to this footer."</p>
                                    </Content>
                                    <Footer
                                        tone=tone
                                        bordered=bordered
                                        aria_label=if custom_aria {
                                            "Workbench footer".to_string()
                                        } else {
                                            String::new()
                                        }
                                        class_name=if custom_class {
                                            "docs-footer-workbench".to_string()
                                        } else {
                                            String::new()
                                        }
                                    >
                                        <p>"Cancel · Save"</p>
                                    </Footer>
                                </View>

                                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                                    <Header bordered=true>
                                        <h3>"Reference Footer"</h3>
                                    </Header>
                                    <Content padded=true>
                                        <p>"Baseline default state for comparison."</p>
                                    </Content>
                                    <Footer>
                                        <p>"Cancel · Save"</p>
                                    </Footer>
                                </View>
                            </div>

                            <div class="ui-muted">
                                {format!(
                                    "comparison: configured(tone={}, bordered={}, custom_aria={}, custom_class={}) vs reference(default)",
                                    if matches!(tone, FooterTone::Muted) { "muted" } else { "default" },
                                    bordered,
                                    custom_aria,
                                    custom_class,
                                )}
                            </div>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Tone / Border / Source Comparison)"
                code_signal=bordered_code
            >
                <div class="docs-stack docs-stack--tight">
                    <Footer>
                        <p>"Default footer"</p>
                    </Footer>
                    <Footer tone=FooterTone::Muted bordered=true>
                        <p>"Muted + bordered"</p>
                    </Footer>
                    <Footer
                        tone=FooterTone::Muted
                        bordered=true
                        aria_label="Settings footer".to_string()
                        class_name="docs-footer-custom".to_string()
                    >
                        <p>"Custom aria + class"</p>
                    </Footer>
                </div>
            </Playground>

            <Playground title="Bordered + Custom Aria/Class" code_signal=bordered_code>
                <View border=ViewBorder::Subtle radius=ViewRadius::Md>
                    <Header bordered=true>
                        <h3>"Profile settings"</h3>
                    </Header>
                    <Content padded=true>
                        <p>"Main settings body"</p>
                    </Content>
                    <Footer
                        tone=FooterTone::Muted
                        bordered=true
                        aria_label="Settings footer".to_string()
                        class_name="docs-footer-custom".to_string()
                    >
                        <p>"Cancel · Save"</p>
                    </Footer>
                </View>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
