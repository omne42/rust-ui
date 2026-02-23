use super::*;

pub(crate) fn color_swatch() -> AnyView {
    let color_swatch_imports = "use leptos::prelude::*;\nuse ui::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize};".to_string();

    let size_options = vec![
        "xs".to_string(),
        "sm".to_string(),
        "md".to_string(),
        "lg".to_string(),
    ];
    let (size_index, set_size_index) = signal(Some(2_usize));
    let size = Signal::derive(move || match size_index.get().unwrap_or(2) {
        0 => ColorSwatchSize::Xs,
        1 => ColorSwatchSize::Sm,
        3 => ColorSwatchSize::Lg,
        _ => ColorSwatchSize::Md,
    });

    let shape_options = vec!["square".to_string(), "wide".to_string()];
    let (shape_index, set_shape_index) = signal(Some(0_usize));
    let shape = Signal::derive(move || match shape_index.get().unwrap_or(0) {
        1 => ColorSwatchShape::Wide,
        _ => ColorSwatchShape::Square,
    });

    let rounding_options = vec![
        "default".to_string(),
        "full".to_string(),
        "none".to_string(),
    ];
    let (rounding_index, set_rounding_index) = signal(Some(0_usize));
    let rounding = Signal::derive(move || match rounding_index.get().unwrap_or(0) {
        1 => ColorSwatchRounding::Full,
        2 => ColorSwatchRounding::None,
        _ => ColorSwatchRounding::Default,
    });

    let alpha_options = vec![
        "opaque".to_string(),
        "translucent".to_string(),
        "transparent".to_string(),
        "none".to_string(),
    ];
    let (alpha_index, set_alpha_index) = signal(Some(0_usize));
    let color = Signal::derive(move || match alpha_index.get().unwrap_or(0) {
        1 => "rgba(38, 99, 235, 0.35)".to_string(),
        2 => "rgba(255, 0, 0, 0)".to_string(),
        3 => "".to_string(),
        _ => "#2663eb".to_string(),
    });
    let color_name = Signal::derive(move || match alpha_index.get().unwrap_or(0) {
        1 => Some("Brand blue / 35%".to_string()),
        2 => Some("No fill".to_string()),
        3 => None,
        _ => Some("Brand blue".to_string()),
    });

    let (is_bordered, set_is_bordered) = signal(true);
    let (is_decorative, set_is_decorative) = signal(false);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);
    let (custom_lang, set_custom_lang) = signal(false);
    let (rtl_dir, set_rtl_dir) = signal(false);
    let (custom_motion, set_custom_motion) = signal(false);

    let workbench_code = Signal::derive(move || {
        let color = color.get();
        let color_name = color_name.get();
        let size = size.get();
        let shape = shape.get();
        let rounding = rounding.get();
        let is_bordered = is_bordered.get();
        let is_decorative = is_decorative.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();
        let rtl = rtl_dir.get();
        let motion = custom_motion.get();

        let mut out = vec![
            "<ColorSwatch".to_string(),
            format!("  color=\"{color}\".into()"),
        ];
        if let Some(color_name) = color_name {
            out.push(format!("  color_name=\"{color_name}\".into()"));
        }
        if size != ColorSwatchSize::Md {
            out.push(format!("  size=ColorSwatchSize::{size:?}"));
        }
        if rounding != ColorSwatchRounding::Default {
            out.push(format!("  rounding=ColorSwatchRounding::{rounding:?}"));
        }
        if shape != ColorSwatchShape::Square {
            out.push(format!("  shape=ColorSwatchShape::{shape:?}"));
        }
        out.push(format!("  is_bordered={is_bordered}"));
        if is_decorative {
            out.push("  is_decorative=true".to_string());
        }
        if custom_aria {
            out.push("  aria_label=\"Background color\".into()".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-color-swatch-custom\".into()".to_string());
        }
        if custom_lang {
            out.push("  lang=\"zh-CN\".into()".to_string());
        }
        out.push(format!(
            "  dir={}",
            if rtl {
                "A11yDirection::Rtl"
            } else {
                "A11yDirection::Ltr"
            }
        ));
        if motion {
            out.push(
                "  motion=ui::ColorSwatchMotion { spring: ui::ColorSwatchMotion::default().spring }"
                    .to_string(),
            );
        } else {
            out.push("  motion=ui::ColorSwatchMotion::default()".to_string());
        }
        out.push("/>".to_string());
        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let color = color.get();
        let size = size.get();
        let shape = shape.get();
        let rounding = rounding.get();
        let is_bordered = is_bordered.get();
        let is_decorative = is_decorative.get();
        let custom_aria = custom_aria.get();
        let custom_class = custom_class.get();
        let custom_lang = custom_lang.get();
        let dir = if rtl_dir.get() {
            A11yDirection::Rtl
        } else {
            A11yDirection::Ltr
        };
        let motion = if custom_motion.get() {
            ui::color_swatch::ColorSwatchMotion {
                spring: ui::color_swatch::ColorSwatchMotion::default().spring,
                ..ui::color_swatch::ColorSwatchMotion::default()
            }
        } else {
            ui::color_swatch::ColorSwatchMotion::default()
        };
        let alpha_index = alpha_index.get().unwrap_or(0);
        let alpha_attr = match alpha_index {
            1 => "translucent",
            2 => "transparent",
            3 => "none",
            _ => "opaque",
        };
        let data_state = match alpha_index {
            3 => "empty",
            2 => "transparent",
            1 => "translucent",
            _ if is_bordered => "framed",
            _ => "default",
        };

        let mut classes = vec![
            "ui-color-swatch".to_string(),
            size.class_name().into(),
            rounding.class_name().into(),
            shape.class_name().into(),
            format!("ui-color-swatch--alpha-{alpha_attr}"),
        ];
        if is_bordered {
            classes.push("ui-color-swatch--bordered".to_string());
        }
        if custom_class {
            classes.push("ui-color-swatch--custom-class".to_string());
            classes.push("docs-color-swatch-custom".to_string());
        }

        format!(
            "ColorSwatchActualConfig {{\n  color: \"{color}\",\n  color_name: {:?},\n  size: {size:?},\n  rounding: {rounding:?},\n  shape: {shape:?},\n  is_bordered: {is_bordered},\n  is_decorative: {is_decorative},\n  motion: {:?},\n  bool_source: \"{}\",\n  aria_label: {:?},\n  class_name: {:?},\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  lang: {},\n  dir: {:?},\n  data_alpha: \"{alpha_attr}\",\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            color_name.get(),
            motion,
            "is-prop",
            if custom_aria {
                Some("Background color")
            } else {
                None
            },
            if custom_class {
                Some("docs-color-swatch-custom")
            } else {
                None
            },
            if custom_lang { "\"zh-CN\"" } else { "None" },
            dir,
            classes.join(" ")
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui/src/color/swatch/styles.rs */\n{}",
            ui::color::swatch::styles::CSS
        )
    });

    let hello_code =
        Signal::derive(move || r##"<ColorSwatch color="#2663eb".to_string() />"##.to_string());

    let matrix_code = Signal::derive(move || {
        r##"<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
<ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Sm />
<ColorSwatch color="rgba(38, 99, 235, 0.35)".to_string() color_name="Brand blue / 35%".to_string() shape=ColorSwatchShape::Wide />
<ColorSwatch color="rgba(255, 0, 0, 0)".to_string() color_name="No fill".to_string() is_bordered=true />
<ColorSwatch color="".to_string() is_bordered=true />"##.to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r##"<ColorSwatch color="#2663eb".to_string() />
<ColorSwatch
  color="#2663eb".to_string()
  color_name="Mapped from upstream app state".to_string()
  size=ColorSwatchSize::Lg
  shape=ColorSwatchShape::Wide
  is_bordered=true
/>"##
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r##"<ColorSwatch
  color="#2663eb".to_string()
  aria_label="Snapshot contract marker".to_string()
/>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"<ColorSwatch
  color="#ffcc00".to_string()
  color_name="Accent yellow".to_string()
  size=ColorSwatchSize::Lg
  rounding=ColorSwatchRounding::Full
/>"##
            .to_string()
    });

    view! {
        <ComponentPage
            title="ColorSwatch"
            slug="color-swatch"
            group="Display"
            description="baseline-compatible color preview primitive with centralized size/rounding/shape/transparency/source contracts and stable slot/data markers."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
                code_imports=color_swatch_imports.clone()
            >
                <div class="docs-row">
                    <ColorSwatch color="#2663eb".to_string() />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=color_swatch_imports.clone()
                test_css_source=workbench_test_css
                test_source_path="crates/ui/src/color/swatch/styles.rs".to_string()
                test_config_signal=workbench_config
                description="切换尺寸/形状/圆角/透明度/边框/装饰模式，并实时查看 config + code + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-swatch-workbench-controls">
                            <div data-slot="color-swatch-workbench-size-control">
                                <div class="docs-search__label">"Size"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-workbench-size".to_string()
                                    options=size_options.clone()
                                    selected_index=size_index
                                    set_selected_index=set_size_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatch size".to_string()
                                />
                            </div>

                            <div data-slot="color-swatch-workbench-shape-control">
                                <div class="docs-search__label">"Shape"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-workbench-shape".to_string()
                                    options=shape_options.clone()
                                    selected_index=shape_index
                                    set_selected_index=set_shape_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatch shape".to_string()
                                />
                            </div>

                            <div data-slot="color-swatch-workbench-rounding-control">
                                <div class="docs-search__label">"Rounding"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-workbench-rounding".to_string()
                                    options=rounding_options.clone()
                                    selected_index=rounding_index
                                    set_selected_index=set_rounding_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatch rounding".to_string()
                                />
                            </div>

                            <div data-slot="color-swatch-workbench-alpha-control">
                                <div class="docs-search__label">"Alpha"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-workbench-alpha".to_string()
                                    options=alpha_options.clone()
                                    selected_index=alpha_index
                                    set_selected_index=set_alpha_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatch alpha".to_string()
                                />
                            </div>

                            <div data-slot="color-swatch-workbench-bordered-switch">
                                <Switch checked=is_bordered set_checked=set_is_bordered>"Bordered"</Switch>
                            </div>
                            <div data-slot="color-swatch-workbench-decorative-switch">
                                <Switch checked=is_decorative set_checked=set_is_decorative>
                                    "Decorative"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-workbench-custom-aria-switch">
                                <Switch checked=custom_aria set_checked=set_custom_aria>
                                    "Custom aria_label"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-workbench-custom-class-switch">
                                <Switch checked=custom_class set_checked=set_custom_class>
                                    "Custom class"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-workbench-lang-switch">
                                <Switch checked=custom_lang set_checked=set_custom_lang>"Lang=zh-CN"</Switch>
                            </div>
                            <div data-slot="color-swatch-workbench-dir-switch">
                                <Switch checked=rtl_dir set_checked=set_rtl_dir>"dir=rtl"</Switch>
                            </div>
                            <div data-slot="color-swatch-workbench-motion-switch">
                                <Switch checked=custom_motion set_checked=set_custom_motion>
                                    "Custom motion"
                                </Switch>
                            </div>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-workbench-canvas">
                    {move || {
                        let color = color.get();
                        let color_name = color_name.get().unwrap_or_default();
                        let size = size.get();
                        let shape = shape.get();
                        let rounding = rounding.get();
                        let is_bordered = is_bordered.get();
                        let is_decorative = is_decorative.get();
                        let aria_label = if custom_aria.get() {
                            "Background color".to_string()
                        } else {
                            String::new()
                        };
                        let class_name = if custom_class.get() {
                            "docs-color-swatch-custom".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if custom_lang.get() {
                            "zh-CN".to_string()
                        } else {
                            "en-US".to_string()
                        };
                        let dir = if rtl_dir.get() {
                            A11yDirection::Rtl
                        } else {
                            A11yDirection::Ltr
                        };
                        let motion = if custom_motion.get() {
                            ui::color_swatch::ColorSwatchMotion {
                                spring: ui::color_swatch::ColorSwatchMotion::default().spring,
                                ..ui::color_swatch::ColorSwatchMotion::default()
                            }
                        } else {
                            ui::color_swatch::ColorSwatchMotion::default()
                        };

                        view! {
                            <ColorSwatch
                                color=color
                                color_name=color_name
                                size=size
                                shape=shape
                                rounding=rounding
                                is_bordered=is_bordered
                                is_decorative=is_decorative
                                aria_label=aria_label
                                class_name=class_name
                                lang=lang
                                dir=dir
                                motion=motion
                            />
                        }
                        .into_any()
                    }}
                    <span class="ui-muted">
                        {move || format!(
                            "alpha={}, bordered={}, is_decorative={}",
                            match alpha_index.get().unwrap_or(0) {
                                1 => "translucent",
                                2 => "transparent",
                                3 => "none",
                                _ => "opaque",
                            },
                            is_bordered.get(),
                            is_decorative.get(),
                        )}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Size / Alpha / Shape / Empty)"
                code_signal=matrix_code
                code_imports=color_swatch_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"XS / Opaque"</span>
                        <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Xs />
                    </div>
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"SM / Opaque"</span>
                        <ColorSwatch color="#ffcc00".to_string() size=ColorSwatchSize::Sm />
                    </div>
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"Wide / Translucent"</span>
                        <ColorSwatch
                            color="rgba(38, 99, 235, 0.35)".to_string()
                            color_name="Brand blue / 35%".to_string()
                            shape=ColorSwatchShape::Wide
                            rounding=ColorSwatchRounding::Default
                        />
                    </div>
                    <div class="docs-card" style="display: grid; gap: 6px;">
                        <span class="ui-muted">"Transparent / Empty"</span>
                        <div class="docs-row">
                            <ColorSwatch
                                color="rgba(255, 0, 0, 0)".to_string()
                                color_name="No fill".to_string()
                                is_bordered=true
                            />
                            <ColorSwatch color="".to_string() is_bordered=true />
                        </div>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast (N/A for ColorSwatch)"
                description="ColorSwatch has no controllable state axis; compare default rendering with upstream state mapped into plain props."
                code_signal=controlled_contrast_code
                code_imports=color_swatch_imports.clone()
            >
                <div class="docs-row">
                    <ColorSwatch color="#2663eb".to_string() />
                    <ColorSwatch
                        color="#2663eb".to_string()
                        color_name="Mapped from upstream app state".to_string()
                        size=ColorSwatchSize::Lg
                        shape=ColorSwatchShape::Wide
                        is_bordered=true
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="ColorSwatch is a display leaf: streaming is optional and falls back to snapshot (`data-ui-stream-support=optional`, `data-ui-stream-fallback=snapshot`)."
                code_signal=stream_snapshot_code
                code_imports=color_swatch_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatch
                        color="#2663eb".to_string()
                        aria_label="Snapshot contract marker".to_string()
                    />
                    <span class="ui-muted">
                        "effective component markers: data-ui-stream-support=optional data-ui-stream-fallback=snapshot data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run. Source: components/color-swatch/src/{mod,logic,view,styles,motion}.rs. Dependency baseline: ui = { default-features = false, features = [\"component-color_swatch\", \"inject-css\"] } + mount under UiRoot."
                code_signal=source_first_code
                code_imports=color_swatch_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-source-first-contract">
                    <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                    <span class="ui-muted">
                        <code>"Show code"</code>
                        " + copy should output runnable snippet with imports."
                    </span>
                    <span class="ui-muted">
                        "Dependency baseline (Cargo.toml): "
                        <code>
                            "ui = { default-features = false, features = [\"component-color_swatch\", \"inject-css\"] }"
                        </code>
                    </span>
                    <ul class="ui-muted" data-slot="color-swatch-source-paths">
                        <li><code>"components/color-swatch/src/mod.rs"</code></li>
                        <li><code>"components/color-swatch/src/logic.rs"</code></li>
                        <li><code>"components/color-swatch/src/view.rs"</code></li>
                        <li><code>"components/color-swatch/src/styles.rs"</code></li>
                        <li><code>"components/color-swatch/src/motion.rs"</code></li>
                    </ul>
                    <ColorSwatch
                        color="#ffcc00".to_string()
                        color_name="Accent yellow".to_string()
                        size=ColorSwatchSize::Lg
                        rounding=ColorSwatchRounding::Full
                    />
                </div>
            </Playground>

            <Playground title="Rounded Large + Custom Label/Class" code_signal=Signal::derive(move || {
                r##"<ColorSwatch
  color="#ffcc00".to_string()
  color_name="Accent yellow".to_string()
  size=ColorSwatchSize::Lg
  rounding=ColorSwatchRounding::Full
  aria_label="Accent token".to_string()
  class_name="docs-color-swatch-custom".to_string()
/>"##.to_string()
            }) code_imports=color_swatch_imports>
                <div class="docs-row">
                    <ColorSwatch
                        color="#ffcc00".to_string()
                        size=ColorSwatchSize::Lg
                        rounding=ColorSwatchRounding::Full
                        color_name="Accent yellow".to_string()
                        aria_label="Accent token".to_string()
                        class_name="docs-color-swatch-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
