use super::*;

pub(crate) fn share_button() -> AnyView {
    let custom_items = vec![
        ShareButtonItem::new(SharePlatform::Github, "Repository"),
        ShareButtonItem::new(SharePlatform::X, "Post"),
        ShareButtonItem::new(SharePlatform::Facebook, "Facebook"),
    ];
    let custom_items_for_matrix = custom_items.clone();
    let custom_items_for_workbench = custom_items.clone();

    let (showcase_last, set_showcase_last) = signal(None::<SharePlatform>);
    let on_showcase_press =
        Callback::new(move |platform: SharePlatform| set_showcase_last.set(Some(platform)));

    let icon_options = vec![
        "Suffix".to_string(),
        "Prefix".to_string(),
        "None".to_string(),
    ];
    let from_options = vec!["Up".to_string(), "Left".to_string(), "Right".to_string()];
    let size_options = vec![
        "xs".to_string(),
        "s".to_string(),
        "m".to_string(),
        "l".to_string(),
        "xl".to_string(),
    ];
    let variant_options = vec![
        "Default".to_string(),
        "Secondary".to_string(),
        "Ghost".to_string(),
        "Outline".to_string(),
    ];
    let (workbench_icon_index, set_workbench_icon_index) = signal(Some(0_usize));
    let (workbench_from_index, set_workbench_from_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(2_usize));
    let (workbench_variant_index, set_workbench_variant_index) = signal(Some(0_usize));
    let (workbench_custom_label, set_workbench_custom_label) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_use_items, set_workbench_use_items) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);
    let (workbench_press_count, set_workbench_press_count) = signal(0_u32);
    let (workbench_last, set_workbench_last) = signal(None::<SharePlatform>);
    let on_workbench_press = Callback::new(move |platform: SharePlatform| {
        set_workbench_press_count.update(|count| *count += 1);
        set_workbench_last.set(Some(platform));
    });

    let workbench_icon = Signal::derive(move || match workbench_icon_index.get().unwrap_or(0) {
        1 => ShareButtonIconPlacement::Prefix,
        2 => ShareButtonIconPlacement::None,
        _ => ShareButtonIconPlacement::Suffix,
    });
    let workbench_from = Signal::derive(move || match workbench_from_index.get().unwrap_or(0) {
        1 => FlipDirection::Left,
        2 => FlipDirection::Right,
        _ => FlipDirection::Top,
    });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(2) {
        0 => ButtonSize::Xs,
        1 => ButtonSize::S,
        3 => ButtonSize::L,
        4 => ButtonSize::Xl,
        _ => ButtonSize::M,
    });
    let workbench_variant =
        Signal::derive(move || match workbench_variant_index.get().unwrap_or(0) {
            1 => ButtonVariant::Secondary,
            2 => ButtonVariant::Ghost,
            3 => ButtonVariant::Outline,
            _ => ButtonVariant::Default,
        });

    let hello_code = Signal::derive(move || r#"<ShareButton />"#.to_string());
    let workbench_code = Signal::derive(move || {
        format!(
            "<ShareButton\n  label={}\n  aria_label=\"Share this page\".to_string()\n  icon={:?}\n  from={:?}\n  size={:?}\n  variant={:?}\n  items={}\n  on_icon_press=on_icon_press\n  motion=ShareButtonMotion::default()\n  class_name={}\n  lang={}.to_string()\n  dir={}\n/>",
            if workbench_custom_label.get() {
                "\"Share docs\".to_string()"
            } else {
                "String::new()"
            },
            workbench_icon.get(),
            workbench_from.get(),
            workbench_size.get(),
            workbench_variant.get(),
            if workbench_use_items.get() {
                "custom_items.clone()"
            } else {
                "Vec::<ShareButtonItem>::new()"
            },
            if workbench_custom_class.get() {
                "\"docs-share-button-custom\".to_string()"
            } else {
                "String::new()"
            },
            if workbench_lang_zh.get() {
                "\"zh-CN\""
            } else {
                "\"en-US\""
            },
            if workbench_rtl.get() {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            },
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        format!(
            "ShareButtonActualConfig {{\n  label: {:?},\n  aria_label: Some(\"Share this page\"),\n  icon: {:?},\n  from: {:?},\n  size: {:?},\n  variant: {:?},\n  items: {:?},\n  on_icon_press: \"count={} last={:?}\",\n  motion: ShareButtonMotion::default(),\n  class_name: {:?},\n  lang: {:?},\n  dir: {:?},\n}}",
            if workbench_custom_label.get() {
                Some("Share docs")
            } else {
                None
            },
            workbench_icon.get(),
            workbench_from.get(),
            workbench_size.get(),
            workbench_variant.get(),
            if workbench_use_items.get() {
                vec!["Github", "X", "Facebook"]
            } else {
                vec![]
            },
            workbench_press_count.get(),
            workbench_last.get(),
            if workbench_custom_class.get() {
                Some("docs-share-button-custom")
            } else {
                None
            },
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "en-US"
            },
            if workbench_rtl.get() { "rtl" } else { "ltr" },
        )
    });
    let matrix_code = Signal::derive(move || {
        r#"<ShareButton label="Default".to_string() size=ButtonSize::M variant=ButtonVariant::Default on_icon_press=Callback::new(move |_| {}) />
<ShareButton icon=ShareButtonIconPlacement::Prefix from=FlipDirection::Left label="Prefix".to_string() items=custom_items_for_matrix.clone() variant=ButtonVariant::Secondary motion=ShareButtonMotion::default() />
<ShareButton icon=ShareButtonIconPlacement::None label="Iconless".to_string() class_name="docs-share-button-custom".to_string() aria_label="Share without icon".to_string() lang="zh-CN".to_string() dir=A11yDirection::Rtl />"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="ShareButton"
            slug="share-button"
            group="Actions"
            description="Flip-based share surface with full API workbench and callback feedback."
        >
            <Playground title="Hello World (Default ShareButton)" code_signal=hello_code>
                <div class="docs-row">
                    <ShareButton on_icon_press=on_showcase_press />
                    <span class="ui-muted">
                        "last: "
                        {move || {
                            showcase_last
                                .get()
                                .map(|platform| format!("{platform:?}"))
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Workbench (All API + Actual Config)"
                code_signal=workbench_code
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="share-button-workbench-controls">
                        <SegmentedControl
                            id_base="docs-share-button-icon".to_string()
                            options=icon_options.clone()
                            selected_index=workbench_icon_index
                            set_selected_index=set_workbench_icon_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton icon placement".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-share-button-from".to_string()
                            options=from_options.clone()
                            selected_index=workbench_from_index
                            set_selected_index=set_workbench_from_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton flip direction".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-share-button-size".to_string()
                            options=size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton size".to_string()
                        />
                        <SegmentedControl
                            id_base="docs-share-button-variant".to_string()
                            options=variant_options.clone()
                            selected_index=workbench_variant_index
                            set_selected_index=set_workbench_variant_index
                            size=SegmentedControlSize::Sm
                            aria_label="ShareButton variant".to_string()
                        />
                        <Switch checked=workbench_custom_label set_checked=set_workbench_custom_label>
                            "label"
                        </Switch>
                        <Switch checked=workbench_use_items set_checked=set_workbench_use_items>
                            "items"
                        </Switch>
                        <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "class_name"
                        </Switch>
                        <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                            "lang=zh-CN"
                        </Switch>
                        <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                            "dir=rtl"
                        </Switch>
                    </div>
                }
            >
                <div class="docs-stack">
                    <ShareButton
                        label=if workbench_custom_label.get() {
                            "Share docs".to_string()
                        } else {
                            String::new()
                        }
                        aria_label="Share this page".to_string()
                        icon=workbench_icon.get()
                        from=workbench_from.get()
                        size=workbench_size.get()
                        variant=workbench_variant.get()
                        items=if workbench_use_items.get() {
                            custom_items_for_workbench.clone()
                        } else {
                            Vec::new()
                        }
                        on_icon_press=on_workbench_press
                        motion=ShareButtonMotion::default()
                        class_name=if workbench_custom_class.get() {
                            "docs-share-button-custom".to_string()
                        } else {
                            String::new()
                        }
                        lang=if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        }
                        dir=if workbench_rtl.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        }
                    />
                    <span class="ui-muted">
                        "on_icon_press count: " {move || workbench_press_count.get()}
                        " · last: "
                        {move || {
                            workbench_last
                                .get()
                                .map(|platform| format!("{platform:?}"))
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground title="State Matrix (Default / Prefix / Iconless)" code_signal=matrix_code>
                <div class="docs-row">
                    <ShareButton
                        label="Default".to_string()
                        size=ButtonSize::M
                        variant=ButtonVariant::Default
                        on_icon_press=on_showcase_press
                    />
                    <ShareButton
                        icon=ShareButtonIconPlacement::Prefix
                        from=FlipDirection::Left
                        label="Prefix".to_string()
                        items=custom_items_for_matrix
                        variant=ButtonVariant::Secondary
                        motion=ShareButtonMotion::default()
                    />
                    <ShareButton
                        icon=ShareButtonIconPlacement::None
                        label="Iconless".to_string()
                        class_name="docs-share-button-custom".to_string()
                        aria_label="Share without icon".to_string()
                        lang="zh-CN".to_string()
                        dir=A11yDirection::Rtl
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
