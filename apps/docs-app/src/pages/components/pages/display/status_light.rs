use super::*;

pub(crate) fn status_light() -> AnyView {
    let variant_options = vec![
        "Default".to_string(),
        "Accent".to_string(),
        "Danger".to_string(),
    ];
    let role_options = vec!["None".to_string(), "status".to_string()];
    let lang_options = vec!["en-US".to_string(), "zh-CN".to_string()];

    let (variant_index, set_variant_index) = signal(Some(0usize));
    let (role_index, set_role_index) = signal(Some(1usize));
    let (lang_index, set_lang_index) = signal(Some(0usize));
    let (custom_class, set_custom_class) = signal(false);
    let (rtl, set_rtl) = signal(false);

    let variant = Signal::derive(move || match variant_index.get().unwrap_or(0) {
        1 => StatusLightVariant::Accent,
        2 => StatusLightVariant::Danger,
        _ => StatusLightVariant::Default,
    });
    let role = Signal::derive(move || match role_index.get().unwrap_or(1) {
        1 => Some(StatusLightRole::Status),
        _ => None,
    });
    let lang = Signal::derive(move || match lang_index.get().unwrap_or(0) {
        1 => "zh-CN".to_string(),
        _ => "en-US".to_string(),
    });
    let dir = Signal::derive(move || {
        if rtl.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        }
    });
    let class_name = Signal::derive(move || {
        if custom_class.get() {
            "docs-status-light-custom".to_string()
        } else {
            String::new()
        }
    });

    let showcase_code = Signal::derive(move || {
        r#"<StatusLight
  variant=StatusLightVariant::Accent
  role=StatusLightRole::Status
>
  "Syncing invoices"
</StatusLight>"#
            .to_string()
    });
    let workbench_code = Signal::derive(move || {
        let mut lines = vec!["<StatusLight".to_string()];
        if variant.get() != StatusLightVariant::Default {
            lines.push(format!("  variant=StatusLightVariant::{:?}", variant.get()));
        }
        if let Some(role) = role.get() {
            lines.push(format!("  role=StatusLightRole::{role:?}"));
        }
        let class_name = class_name.get();
        if !class_name.is_empty() {
            lines.push(format!("  class_name={}", rust_string_literal(&class_name)));
        }
        lines.push(format!("  lang={}", rust_string_literal(&lang.get())));
        lines.push(format!("  dir=A11yDirection::{:?}", dir.get()));
        lines.push(">".to_string());
        lines.push("  \"Syncing invoices\"".to_string());
        lines.push("</StatusLight>".to_string());
        lines.join("\n")
    });
    let workbench_config = Signal::derive(move || {
        format!(
            "StatusLightWorkbenchConfig {{\n  variant: {:?},\n  role: {:?},\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            variant.get(),
            role.get(),
            if class_name.get().is_empty() {
                None::<String>
            } else {
                Some(class_name.get())
            },
            Some(lang.get()),
            Some(dir.get()),
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<StatusLight variant=StatusLightVariant::Default role=StatusLightRole::Status lang="en-US".to_string() dir=A11yDirection::Ltr>
  "Idle"
</StatusLight>
<StatusLight variant=StatusLightVariant::Accent role=StatusLightRole::Status lang="en-US".to_string() dir=A11yDirection::Ltr>
  "Deploying"
</StatusLight>
<StatusLight variant=StatusLightVariant::Danger role=StatusLightRole::Status class_name="docs-status-light-custom".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl>
  "失败"
</StatusLight>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="StatusLight"
            slug="status-light"
            group="Display"
            description="Status indicator + label with centralized variant/live/role-source state attrs and optional custom-class contract."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=showcase_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <StatusLight
                        variant=StatusLightVariant::Accent
                        role=StatusLightRole::Status
                    >
                        "Syncing invoices"
                    </StatusLight>
                </div>
            </Playground>

            <Playground
                title="Workbench (Variant + Role + Locale)"
                code_signal=workbench_code
                test_source_path="components/status-light/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="status-light-workbench-controls">
                        <div class="docs-search__label">"Variant"</div>
                        <SegmentedControl
                            id_base="docs-status-light-variant".to_string()
                            options=variant_options.clone()
                            selected_index=variant_index
                            set_selected_index=set_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="StatusLight variant".to_string()
                        />

                        <div class="docs-search__label">"Role"</div>
                        <SegmentedControl
                            id_base="docs-status-light-role".to_string()
                            options=role_options.clone()
                            selected_index=role_index
                            set_selected_index=set_role_index
                            size=SegmentedControlSize::Sm
                            aria_label="StatusLight role".to_string()
                        />

                        <div class="docs-search__label">"Language"</div>
                        <SegmentedControl
                            id_base="docs-status-light-lang".to_string()
                            options=lang_options.clone()
                            selected_index=lang_index
                            set_selected_index=set_lang_index
                            size=SegmentedControlSize::Sm
                            aria_label="StatusLight language".to_string()
                        />

                        <Switch checked=rtl set_checked=set_rtl>"RTL direction"</Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>"Custom class"</Switch>
                    </div>
                }
            >
                {move || {
                    let variant = variant.get();
                    let role = role.get();
                    let class_name = class_name.get();
                    let lang = lang.get();
                    let dir = dir.get();

                    let content = if let Some(role) = role {
                        view! {
                            <StatusLight
                                variant=variant
                                role=role
                                class_name=class_name
                                lang=lang
                                dir=dir
                            >
                                "Syncing invoices"
                            </StatusLight>
                        }
                        .into_any()
                    } else {
                        view! {
                            <StatusLight
                                variant=variant
                                class_name=class_name
                                lang=lang
                                dir=dir
                            >
                                "Syncing invoices"
                            </StatusLight>
                        }
                        .into_any()
                    };

                    view! { <div class="docs-row">{content}</div> }
                }}
            </Playground>

            <Playground
                title="State Matrix (Variant + Role + Locale)"
                code_signal=matrix_code
                test_source_path="components/status-light/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <StatusLight
                        variant=StatusLightVariant::Default
                        role=StatusLightRole::Status
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "Idle"
                    </StatusLight>
                    <StatusLight
                        variant=StatusLightVariant::Accent
                        role=StatusLightRole::Status
                        lang="en-US".to_string()
                        dir=A11yDirection::Ltr
                    >
                        "Deploying"
                    </StatusLight>
                    <StatusLight
                        variant=StatusLightVariant::Danger
                        role=StatusLightRole::Status
                        class_name="docs-status-light-custom".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    >
                        "失败"
                    </StatusLight>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
