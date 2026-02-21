use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui_components::{
    Chart, ChartKind, ChartPoint, ColorSwatch, ColorSwatchPicker, ColorSwatchPickerItem,
    ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize, EmptyState, EmptyStateAlign,
    EmptyStateTone, ErrorView, ErrorViewMotion, ErrorViewTone, FlipCard, FlipCardMotion, Icon,
    IconSize, IconTone, Keyboard, KeyboardTone, LabeledValue, LabeledValueOrientation,
    LabeledValueTone, PressableFeedback, PressableFeedbackEffect, PressableFeedbackMotion,
    PressableFeedbackTone, RippleMotion, SegmentedControl, SegmentedControlSize, Skeleton,
    SkeletonGroup, SkeletonGroupDensity, SkeletonGroupLayout, SkeletonGroupVariant,
    SkeletonVariant, Switch, Text, TextAlign, TextElement, TextTone, TextWeight,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ChartWorkbenchState {
    kind_index: usize,
    dataset_index: usize,
    is_disabled: bool,
    is_show_grid: bool,
    custom_class: bool,
    lang: bool,
}

impl Default for ChartWorkbenchState {
    fn default() -> Self {
        Self {
            kind_index: 0,
            dataset_index: 0,
            is_disabled: false,
            is_show_grid: true,
            custom_class: false,
            lang: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ChartWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 6 {
            return None;
        }

        let parse_index = |at: usize, max: usize| {
            parts
                .get(at)?
                .parse::<usize>()
                .ok()
                .map(|value| value.min(max))
        };
        let parse_bool = |at: usize| match *parts.get(at)? {
            "1" => Some(true),
            "0" => Some(false),
            _ => None,
        };

        Some(Self {
            kind_index: parse_index(0, 1)?,
            dataset_index: parse_index(1, 2)?,
            is_disabled: parse_bool(2)?,
            is_show_grid: parse_bool(3)?,
            custom_class: parse_bool(4)?,
            lang: parse_bool(5)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{},{}",
            self.kind_index,
            self.dataset_index,
            bool_digit(self.is_disabled),
            bool_digit(self.is_show_grid),
            bool_digit(self.custom_class),
            bool_digit(self.lang),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const CHART_WORKBENCH_STORAGE_KEY: &str = "docs:chart:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_chart_workbench_state() -> Option<ChartWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(CHART_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    ChartWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_chart_workbench_state() -> Option<ChartWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_chart_workbench_state(state: ChartWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(CHART_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_chart_workbench_state(_state: ChartWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_chart_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(CHART_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_chart_workbench_state() {}

pub(super) fn labeled_value() -> AnyView {
    let orientation_options = vec!["Stacked".to_string(), "Inline".to_string()];
    let tone_options = vec![
        "Default".to_string(),
        "Subtle".to_string(),
        "Strong".to_string(),
    ];
    let labeled_value_imports =
        "use leptos::prelude::*;\nuse ui_components::{LabeledValue, LabeledValueOrientation, LabeledValueTone};"
            .to_string();
    let (orientation_index, set_orientation_index) = signal(Some(0_usize));
    let (tone_index, set_tone_index) = signal(Some(0_usize));
    let (show_description, set_show_description) = signal(true);
    let (custom_aria, set_custom_aria) = signal(false);
    let (custom_class, set_custom_class) = signal(false);

    let hello_world_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />"#.to_string()
    });

    let orientation_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<LabeledValue label="Project".to_string() value="Omne".to_string() />
<LabeledValue
  label="Status".to_string()
  value="Healthy".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>
<LabeledValue
  label="SLA".to_string()
  value="99.95%".to_string()
  orientation=LabeledValueOrientation::Stacked
  tone=LabeledValueTone::Strong
  description="SLA snapshot".to_string()
/>"#
        .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  description="Updated 2 minutes ago".to_string()
  aria_label="Build status".to_string()
  class_name="docs-labeled-value-custom".to_string()
  tone=LabeledValueTone::Strong
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<LabeledValue label="Default path".to_string() value="No controlled axis".to_string() />
<LabeledValue
  label="App state mapped".to_string()
  value="Map upstream state to orientation/tone/class_name".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Output mode".to_string()
  value="Snapshot".to_string()
  description="Inspect data-output-mode=snapshot and data-output-status=validated".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<LabeledValue
  label="Build".to_string()
  value="passing".to_string()
  orientation=LabeledValueOrientation::Inline
  tone=LabeledValueTone::Subtle
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let (orientation, orientation_variant) = match orientation_index.get().unwrap_or(0) {
            0 => ("stacked", "LabeledValueOrientation::Stacked"),
            _ => ("inline", "LabeledValueOrientation::Inline"),
        };
        let tone_variant = match tone_index.get().unwrap_or(0) {
            0 => "LabeledValueTone::Default",
            1 => "LabeledValueTone::Subtle",
            _ => "LabeledValueTone::Strong",
        };
        let description_line = if show_description.get() {
            "  description=\"Updated 2 minutes ago\".into()\n"
        } else {
            ""
        };
        let aria_line = if custom_aria.get() {
            "  aria_label=\"Build status\".into()\n"
        } else {
            ""
        };
        let class_line = if custom_class.get() {
            "  class_name=\"docs-labeled-value-workbench\".into()\n"
        } else {
            ""
        };

        format!(
            "<LabeledValue\n  label=\"Build\".into()\n  value=\"passing\".into()\n  orientation={orientation_variant} // {orientation}\n  tone={tone_variant}\n{description_line}{aria_line}{class_line}/>"
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let (orientation, orientation_variant) = match orientation_index.get().unwrap_or(0) {
            0 => ("stacked", "LabeledValueOrientation::Stacked"),
            _ => ("inline", "LabeledValueOrientation::Inline"),
        };
        let tone = match tone_index.get().unwrap_or(0) {
            0 => "default",
            1 => "subtle",
            _ => "strong",
        };
        let tone_variant = match tone_index.get().unwrap_or(0) {
            0 => "LabeledValueTone::Default",
            1 => "LabeledValueTone::Subtle",
            _ => "LabeledValueTone::Strong",
        };

        format!(
            "LabeledValueActualConfig {{\n  orientation: {orientation_variant} ({orientation}),\n  tone: {tone_variant} ({tone}),\n  has_description: {},\n  custom_aria_label: {},\n  custom_class_name: {},\n}}",
            show_description.get(),
            custom_aria.get(),
            custom_class.get(),
        )
    });

    view! {
        <ComponentPage
            title="LabeledValue"
            slug="labeled-value"
            group="Display"
            description="Label-value pair primitive with centralized orientation/tone/source state contracts and baseline-style data markers."
        >
            <Playground
                title="Hello World (Default API)"
                code_signal=hello_world_code
                code_imports=labeled_value_imports.clone()
            >
                <LabeledValue label="Project".to_string() value="Omne".to_string() />
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Stacked
                        tone=LabeledValueTone::Strong
                        description="SLA snapshot".to_string()
                    />
                </div>
            </Playground>

            <Playground title="Orientation + Tone" code_signal=orientation_code>
                <div class="docs-stack">
                    <LabeledValue label="Project".to_string() value="Omne".to_string() />
                    <LabeledValue
                        label="Status".to_string()
                        value="Healthy".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground title="Description + Custom Aria/Class" code_signal=custom_code>
                <div class="docs-stack">
                    <LabeledValue
                        label="Build".to_string()
                        value="passing".to_string()
                        description="Updated 2 minutes ago".to_string()
                        aria_label="Build status".to_string()
                        class_name="docs-labeled-value-custom".to_string()
                        tone=LabeledValueTone::Strong
                    />
                    <LabeledValue
                        label="SLA".to_string()
                        value="99.95%".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Default
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="LabeledValue has no controlled/uncontrolled runtime axis; compare default usage vs app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <LabeledValue
                        label="Default path".to_string()
                        value="No controlled axis".to_string()
                    />
                    <LabeledValue
                        label="App state mapped".to_string()
                        value="Map upstream state to orientation/tone/class_name".to_string()
                        orientation=LabeledValueOrientation::Inline
                        tone=LabeledValueTone::Subtle
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="LabeledValue is a display leaf: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=labeled_value_imports.clone()
            >
                <div class="docs-stack">
                    <LabeledValue
                        label="Output mode".to_string()
                        value="Snapshot".to_string()
                        description="Inspect data-output-mode=snapshot and data-output-status=validated".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=labeled_value_imports.clone()
            >
                <LabeledValue
                    label="Build".to_string()
                    value="passing".to_string()
                    orientation=LabeledValueOrientation::Inline
                    tone=LabeledValueTone::Subtle
                />
            </Playground>

            <Playground
                title="Interactive Playground"
                description="展示 / Config / Code / CSS Test 集成工作台（含多场景对比）。"
                code_signal=workbench_code
                code_imports=labeled_value_imports
                test_source_path="components/labeled-value/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight">
                        <div class="docs-search__label">"Orientation"</div>
                        <SegmentedControl
                            id_base="docs-labeled-value-workbench-orientation".to_string()
                            options=orientation_options.clone()
                            selected_index=orientation_index
                            set_selected_index=set_orientation_index
                            size=SegmentedControlSize::Sm
                            aria_label="LabeledValue orientation".to_string()
                        />

                        <div class="docs-search__label">"Tone"</div>
                        <SegmentedControl
                            id_base="docs-labeled-value-workbench-tone".to_string()
                            options=tone_options.clone()
                            selected_index=tone_index
                            set_selected_index=set_tone_index
                            size=SegmentedControlSize::Sm
                            aria_label="LabeledValue tone".to_string()
                        />

                        <Switch checked=show_description set_checked=set_show_description>
                            "Description"
                        </Switch>
                        <Switch checked=custom_aria set_checked=set_custom_aria>
                            "Custom aria_label"
                        </Switch>
                        <Switch checked=custom_class set_checked=set_custom_class>
                            "Custom class"
                        </Switch>
                    </div>
                }
            >
                {move || {
                    let orientation = match orientation_index.get().unwrap_or(0) {
                        0 => LabeledValueOrientation::Stacked,
                        _ => LabeledValueOrientation::Inline,
                    };
                    let tone = match tone_index.get().unwrap_or(0) {
                        0 => LabeledValueTone::Default,
                        1 => LabeledValueTone::Subtle,
                        _ => LabeledValueTone::Strong,
                    };
                    let description = if show_description.get() {
                        "Updated 2 minutes ago".to_string()
                    } else {
                        "".to_string()
                    };
                    let aria_label = if custom_aria.get() {
                        "Build status".to_string()
                    } else {
                        "".to_string()
                    };
                    let class_name = if custom_class.get() {
                        "docs-labeled-value-workbench".to_string()
                    } else {
                        "".to_string()
                    };

                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <LabeledValue
                                label="Build".to_string()
                                value="passing".to_string()
                                description=description
                                orientation=orientation
                                tone=tone
                                aria_label=aria_label
                                class_name=class_name
                            />
                            <LabeledValue
                                label="Compare / Inline".to_string()
                                value="Healthy".to_string()
                                orientation=LabeledValueOrientation::Inline
                                tone=LabeledValueTone::Subtle
                            />
                            <LabeledValue
                                label="Compare / Stacked".to_string()
                                value="99.95%".to_string()
                                orientation=LabeledValueOrientation::Stacked
                                tone=LabeledValueTone::Strong
                                description="SLA snapshot".to_string()
                            />
                        </div>
                    }
                }}
            </Playground>

            <section class="docs-card docs-prose" data-slot="labeled-value-streaming-modes">
                <h3>"Streaming / Snapshot"</h3>
                <ul data-slot="labeled-value-streaming-rows">
                    <li><code>"data-output-mode"</code>" = snapshot"</li>
                    <li><code>"data-output-status"</code>" = validated"</li>
                    <li><code>"streaming support"</code>" = optional (fallback=snapshot)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="labeled-value-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn keyboard() -> AnyView {
    let keyboard_imports =
        "use leptos::prelude::*;\nuse ui_components::{Keyboard, KeyboardTone};".to_string();
    let tone_options = vec!["default".to_string(), "muted".to_string()];
    let key_options = vec![
        "⌘K".to_string(),
        "Ctrl+Shift+P".to_string(),
        "⌥⇧P".to_string(),
    ];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0));
    let (workbench_key_index, set_workbench_key_index) = signal(Some(0));
    let (workbench_is_compact, set_workbench_is_compact) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_tone = Signal::derive(move || match workbench_tone_index.get().unwrap_or(0) {
        1 => KeyboardTone::Muted,
        _ => KeyboardTone::Default,
    });
    let workbench_key_text = Signal::derive(move || match workbench_key_index.get().unwrap_or(0) {
        1 => "Ctrl+Shift+P",
        2 => "⌥⇧P",
        _ => "⌘K",
    });

    let workbench_code = Signal::derive(move || {
        let tone = workbench_tone.get();
        let key_text = workbench_key_text.get();
        let is_compact = workbench_is_compact.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();

        let mut snippet = vec!["<Keyboard".to_string()];
        if tone == KeyboardTone::Muted {
            snippet.push("  tone=KeyboardTone::Muted".to_string());
        }
        if is_compact {
            snippet.push("  is_compact=true".to_string());
        }
        if custom_aria {
            snippet.push("  aria_label=\"Open command palette\".into()".to_string());
        }
        if custom_class {
            snippet.push("  class_name=\"docs-keyboard-custom\".into()".to_string());
        }
        snippet.push(">".to_string());
        snippet.push(format!("  \"{key_text}\""));
        snippet.push("</Keyboard>".to_string());
        snippet.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let tone = workbench_tone.get();
        let key_text = workbench_key_text.get();
        let is_compact = workbench_is_compact.get();
        let custom_aria = workbench_custom_aria.get();
        let custom_class = workbench_custom_class.get();
        let mut class_tokens = vec![
            "ui-keyboard".to_string(),
            match tone {
                KeyboardTone::Muted => "ui-keyboard--tone-muted".to_string(),
                KeyboardTone::Default => "ui-keyboard--tone-default".to_string(),
            },
        ];
        if is_compact {
            class_tokens.push("ui-keyboard--compact".to_string());
        }
        if custom_class {
            class_tokens.push("ui-keyboard--custom-class".to_string());
            class_tokens.push("docs-keyboard-custom".to_string());
        }

        format!(
            "KeyboardActualConfig {{\n  tone: {tone:?},\n  key_text: \"{key_text}\",\n  is_compact: {is_compact},\n  custom_aria_label: {custom_aria},\n  custom_class_name: {custom_class},\n  class: \"{}\",\n  marker_expectations: [\"data-tone\", \"data-state\", \"data-compact\", \"data-aria-source\", \"data-class-source\"],\n}}",
            class_tokens.join(" ")
        )
    });

    let keyboard_test_css_source = Signal::derive(move || {
        format!(
            "/* components/keyboard/src/styles.rs */\n{}",
            ui_components::keyboard::styles::CSS
        )
    });

    let hello_world_code = Signal::derive(move || r#"<Keyboard>"⌘K"</Keyboard>"#.to_string());

    let state_matrix_code = Signal::derive(move || {
        r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
<Keyboard is_compact=true>"Ctrl+K"</Keyboard>
<Keyboard
  tone=KeyboardTone::Muted
  is_compact=true
  aria_label="Open command palette".to_string()
  class_name="docs-keyboard-custom".to_string()
>
  "Ctrl+Shift+P"
</Keyboard>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<Keyboard>"⌘K"</Keyboard>
<Keyboard
  tone=KeyboardTone::Muted
  is_compact=true
  class_name="docs-keyboard-custom".to_string()
>
  "Mapped from upstream app state"
</Keyboard>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Keyboard
  tone=KeyboardTone::Muted
  aria_label="Snapshot contract marker".to_string()
>
  "⌘K"
</Keyboard>"#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>"#.to_string()
    });

    view! {
        <ComponentPage
            title="Keyboard"
            slug="keyboard"
            group="Display"
            description="Keyboard command primitive (`<kbd>`) with centralized tone/compact/source state contracts."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_world_code
                code_imports=keyboard_imports.clone()
            >
                <Keyboard>"⌘K"</Keyboard>
            </Playground>

            <Playground
                title="State Matrix (Tone / Compact / Source Markers)"
                code_signal=state_matrix_code
                code_imports=keyboard_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Default"</span>
                        <Keyboard>"⌘K"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Muted"</span>
                        <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Compact"</span>
                        <Keyboard is_compact=true>"Ctrl+K"</Keyboard>
                    </div>
                    <div class="docs-card" style="flex: 1 1 180px;">
                        <span class="ui-muted">"Muted + Compact + Custom"</span>
                        <Keyboard
                            tone=KeyboardTone::Muted
                            is_compact=true
                            aria_label="Open command palette".to_string()
                            class_name="docs-keyboard-custom".to_string()
                        >
                            "Ctrl+Shift+P"
                        </Keyboard>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast (N/A for Keyboard)"
                description="Keyboard has no controllable state axis; compare default rendering with upstream state mapped into plain props."
                code_signal=controlled_contrast_code
                code_imports=keyboard_imports.clone()
            >
                <div class="docs-row">
                    <Keyboard>"⌘K"</Keyboard>
                    <Keyboard
                        tone=KeyboardTone::Muted
                        is_compact=true
                        class_name="docs-keyboard-custom".to_string()
                    >
                        "Mapped from upstream app state"
                    </Keyboard>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Keyboard is a display leaf: streaming is optional and falls back to snapshot (`data-ui-streaming=optional`, `data-ui-streaming-fallback=snapshot`)."
                code_signal=stream_snapshot_code
                code_imports=keyboard_imports.clone()
            >
                <Keyboard
                    tone=KeyboardTone::Muted
                    aria_label="Snapshot contract marker".to_string()
                >
                    "⌘K"
                </Keyboard>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=keyboard_imports.clone()
            >
                <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=keyboard_imports.clone()
                test_css_source=keyboard_test_css_source
                test_source_path="components/keyboard/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调 tone/key/is_compact/aria/class，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Tone"</div>
                            <SegmentedControl
                                id_base="docs-keyboard-tone".to_string()
                                options=tone_options.clone()
                                selected_index=workbench_tone_index
                                set_selected_index=set_workbench_tone_index
                                size=SegmentedControlSize::Sm
                                aria_label="Keyboard tone".to_string()
                            />

                            <div class="docs-search__label">"Key Text"</div>
                            <SegmentedControl
                                id_base="docs-keyboard-key".to_string()
                                options=key_options.clone()
                                selected_index=workbench_key_index
                                set_selected_index=set_workbench_key_index
                                size=SegmentedControlSize::Sm
                                aria_label="Keyboard key text".to_string()
                            />

                            <Switch checked=workbench_is_compact set_checked=set_workbench_is_compact>
                                "is_compact"
                            </Switch>
                            <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                "Custom aria_label"
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
                        let tone = workbench_tone.get();
                        let key_text = workbench_key_text.get();
                        let is_compact = workbench_is_compact.get();
                        let aria_label = if workbench_custom_aria.get() {
                            "Open command palette".to_string()
                        } else {
                            "".to_string()
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-keyboard-custom".to_string()
                        } else {
                            "".to_string()
                        };

                        view! {
                            <Keyboard
                                tone=tone
                                is_compact=is_compact
                                aria_label=aria_label
                                class_name=class_name
                            >
                                {key_text}
                            </Keyboard>
                        }
                    }}

                    <div class="docs-row">
                        <span class="ui-muted">"Compare baseline:"</span>
                        <Keyboard>"⌘K"</Keyboard>
                        <Keyboard tone=KeyboardTone::Muted>"⌥⇧P"</Keyboard>
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="keyboard-parameter-matrix">
                <h3>"Parameter Matrix (API + Defaults)"</h3>
                <ul data-slot="keyboard-parameter-rows">
                    <li><code>"tone"</code>" = KeyboardTone::Default (default)"</li>
                    <li><code>"is_compact"</code>" = false (default)"</li>
                    <li><code>"aria_label"</code>" = \"Keyboard\" fallback after trim/normalize"</li>
                    <li><code>"class_name"</code>" = optional custom class (default none)"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn text() -> AnyView {
    let tone_code = Signal::derive(move || {
        r#"<Text text="Primary body copy".to_string() />
<Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
<Text text="Strong headline".to_string() tone=TextTone::Strong weight=TextWeight::Bold />"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Text
  text="Centered label".to_string()
  align=TextAlign::Center
  element=TextElement::Div
/>
<Text
  text="Long text that truncates when width is constrained by the container around it".to_string()
  truncate=true
  class_name="docs-text-custom".to_string()
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Text"
            slug="text"
            group="Display"
            description="Typography primitive with centralized tone/alignment/weight/source state contracts and baseline-style data markers."
        >
            <Playground title="Tone + Weight Matrix" code_signal=tone_code>
                <div class="docs-stack">
                    <Text text="Primary body copy".to_string() />
                    <Text text="Subtle metadata".to_string() tone=TextTone::Subtle />
                    <Text
                        text="Strong headline".to_string()
                        tone=TextTone::Strong
                        weight=TextWeight::Bold
                    />
                </div>
            </Playground>

            <Playground title="Alignment + Truncate + Element" code_signal=states_code>
                <div class="docs-stack">
                    <Text
                        text="Centered label".to_string()
                        align=TextAlign::Center
                        weight=TextWeight::Semibold
                        element=TextElement::Div
                    />
                    <Text
                        text="Long text that truncates when width is constrained by the container around it".to_string()
                        truncate=true
                        class_name="docs-text-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn icon() -> AnyView {
    let hello_code = Signal::derive(move || r#"<Icon>"✓"</Icon>"#.to_string());
    let icon_code_imports =
        "use leptos::prelude::*;\nuse ui_components::{Icon, IconSize, IconTone};".to_string();

    let matrix_code = Signal::derive(move || {
        r#"<Icon size=IconSize::Sm tone=IconTone::Default is_decorative=true>"✓"</Icon>
<Icon size=IconSize::Md tone=IconTone::Muted is_decorative=true>"⚙"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Accent is_decorative=true>"★"</Icon>
<Icon size=IconSize::Lg tone=IconTone::Danger is_decorative=true>"⚠"</Icon>"#
            .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<Icon
  size=IconSize::Md
  tone=IconTone::Accent
  is_decorative=false
  aria_label="Sync successful".to_string()
>
  "✓"
</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Muted
  is_disabled=true
  class_name="docs-icon-custom".to_string()
  is_decorative=true
>
  "⚙"
</Icon>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<Icon is_decorative=true>"✓"</Icon>
<Icon
  size=IconSize::Lg
  tone=IconTone::Accent
  is_decorative=false
  aria_label="Mapped from upstream app state".to_string()
>
  "★"
</Icon>"#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Icon
  size=IconSize::Md
  tone=IconTone::Muted
  is_decorative=false
  aria_label="Snapshot mode icon".to_string()
>
  "⏺"
</Icon>"#
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Icon size=IconSize::Sm tone=IconTone::Accent is_decorative=true>
  "✓"
</Icon>"#
            .to_string()
    });

    let (workbench_size_key, set_workbench_size_key) = signal("md".to_string());
    let (workbench_tone_key, set_workbench_tone_key) = signal("default".to_string());
    let (workbench_glyph, set_workbench_glyph) = signal("✓".to_string());
    let (workbench_disabled, set_workbench_disabled) = signal(false);
    let (workbench_decorative, set_workbench_decorative) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_label, set_workbench_label) = signal("Status icon".to_string());

    let workbench_code = Signal::derive(move || {
        let size = workbench_size_key.get();
        let tone = workbench_tone_key.get();
        let glyph = workbench_glyph.get();
        let disabled = workbench_disabled.get();
        let decorative = workbench_decorative.get();
        let custom_class = workbench_custom_class.get();
        let class_line = if custom_class {
            "  class_name=\"docs-icon-custom\".into()\n".to_string()
        } else {
            String::new()
        };
        let aria_line = if decorative {
            String::new()
        } else {
            format!("  aria_label=\"{}\".into()\n", workbench_label.get().trim())
        };
        format!(
            "<Icon\n  size=IconSize::{}\n  tone=IconTone::{}\n  is_disabled={disabled}\n  is_decorative={decorative}\n{aria_line}{class_line}>\n  \"{glyph}\"\n</Icon>",
            match size.as_str() {
                "sm" => "Sm",
                "lg" => "Lg",
                _ => "Md",
            },
            match tone.as_str() {
                "muted" => "Muted",
                "accent" => "Accent",
                "danger" => "Danger",
                _ => "Default",
            },
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/icon/styles.rs */\n{}",
            ui_components::icon::styles::CSS
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let size_key = workbench_size_key.get();
        let tone_key = workbench_tone_key.get();
        let size_class = match size_key.as_str() {
            "sm" => "ui-icon--size-sm",
            "lg" => "ui-icon--size-lg",
            _ => "ui-icon--size-md",
        };
        let tone_class = match tone_key.as_str() {
            "muted" => "ui-icon--tone-muted",
            "accent" => "ui-icon--tone-accent",
            "danger" => "ui-icon--tone-danger",
            _ => "ui-icon--tone-default",
        };
        let disabled = workbench_disabled.get();
        let decorative = workbench_decorative.get();
        let custom_class = workbench_custom_class.get();
        let data_state = if disabled {
            "disabled"
        } else if decorative {
            "decorative"
        } else {
            "labeled"
        };

        let mut classes = vec!["ui-icon".to_string(), size_class.into(), tone_class.into()];
        if disabled {
            classes.push("ui-icon--disabled".to_string());
        }
        if decorative {
            classes.push("ui-icon--decorative".to_string());
        }
        if custom_class {
            classes.push("ui-icon--custom-class".to_string());
            classes.push("docs-icon-custom".to_string());
        }

        format!(
            "IconActualConfig {{\n  size: \"{}\",\n  tone: \"{}\",\n  disabled: {},\n  decorative: {},\n  glyph: \"{}\",\n  aria_source: \"{}\",\n  class_source: \"{}\",\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            size_key,
            tone_key,
            disabled,
            decorative,
            workbench_glyph.get(),
            if decorative { "n/a" } else { "custom" },
            if custom_class { "custom" } else { "default" },
            classes.join(" "),
        )
    });

    view! {
        <ComponentPage
            title="Icon"
            slug="icon"
            group="Display"
            description="baseline-style icon primitive with centralized size/tone/accessibility/source state contracts and stable slot/data markers."
        >
            <Playground title="Hello World (Default Path)" code_signal=hello_code>
                <Icon>"✓"</Icon>
            </Playground>

            <Playground title="Size + Tone Matrix" code_signal=matrix_code>
                <div class="docs-row">
                    <Icon size=IconSize::Sm tone=IconTone::Default is_decorative=true>
                        "✓"
                    </Icon>
                    <Icon size=IconSize::Md tone=IconTone::Muted is_decorative=true>
                        "⚙"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Accent is_decorative=true>
                        "★"
                    </Icon>
                    <Icon size=IconSize::Lg tone=IconTone::Danger is_decorative=true>
                        "⚠"
                    </Icon>
                </div>
            </Playground>

            <Playground title="Accessible + Disabled + Custom Class" code_signal=states_code>
                <div class="docs-row">
                    <Icon
                        size=IconSize::Md
                        tone=IconTone::Accent
                        is_decorative=false
                        aria_label="Sync successful".to_string()
                    >
                        "✓"
                    </Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Muted
                        is_disabled=true
                        class_name="docs-icon-custom".to_string()
                        is_decorative=true
                    >
                        "⚙"
                    </Icon>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast (N/A for Icon)"
                description="Icon has no controllable state axis; compare default rendering with upstream state mapped into plain props."
                code_signal=controlled_contrast_code
                code_imports=icon_code_imports.clone()
            >
                <div class="docs-row">
                    <Icon is_decorative=true>"✓"</Icon>
                    <Icon
                        size=IconSize::Lg
                        tone=IconTone::Accent
                        is_decorative=false
                        aria_label="Mapped from upstream app state".to_string()
                    >
                        "★"
                    </Icon>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Icon is a display leaf: streaming is optional and falls back to snapshot (`data-ui-streaming=optional`, `data-ui-streaming-fallback=snapshot`)."
                code_signal=stream_snapshot_code
                code_imports=icon_code_imports.clone()
            >
                <Icon
                    size=IconSize::Md
                    tone=IconTone::Muted
                    is_decorative=false
                    aria_label="Snapshot mode icon".to_string()
                >
                    "⏺"
                </Icon>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run; requires `ui_components` dependency in Cargo.toml."
                code_signal=source_first_code
                code_imports=icon_code_imports.clone()
            >
                <Icon size=IconSize::Sm tone=IconTone::Accent is_decorative=true>
                    "✓"
                </Icon>
            </Playground>

            <Playground
                title="Workbench (Display + Config + Code + CSS Test)"
                description="Button-style playground with display/config/code/css-test panels and live icon state controls."
                code_signal=workbench_code
                test_css_source=workbench_test_css
                test_source_path="components/icon/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="icon-workbench-controls">
                        <label class="docs-search__label">
                            "Size"
                            <select
                                prop:value=move || workbench_size_key.get()
                                on:change=move |ev| set_workbench_size_key.set(event_target_value(&ev))
                            >
                                <option value="sm">"Sm"</option>
                                <option value="md">"Md"</option>
                                <option value="lg">"Lg"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Tone"
                            <select
                                prop:value=move || workbench_tone_key.get()
                                on:change=move |ev| set_workbench_tone_key.set(event_target_value(&ev))
                            >
                                <option value="default">"Default"</option>
                                <option value="muted">"Muted"</option>
                                <option value="accent">"Accent"</option>
                                <option value="danger">"Danger"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            "Glyph"
                            <select
                                prop:value=move || workbench_glyph.get()
                                on:change=move |ev| set_workbench_glyph.set(event_target_value(&ev))
                            >
                                <option value="✓">"Check"</option>
                                <option value="⚙">"Gear"</option>
                                <option value="★">"Star"</option>
                                <option value="⚠">"Alert"</option>
                            </select>
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_disabled.get()
                                on:change=move |ev| set_workbench_disabled.set(event_target_checked(&ev))
                            />
                            " Disabled"
                        </label>
                        <label class="docs-search__label">
                            <input
                                type="checkbox"
                                prop:checked=move || workbench_decorative.get()
                                on:change=move |ev| set_workbench_decorative.set(event_target_checked(&ev))
                            />
                            " Decorative"
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
                            "Aria label"
                            <input
                                type="text"
                                prop:value=move || workbench_label.get()
                                on:input=move |ev| set_workbench_label.set(event_target_value(&ev))
                            />
                        </label>
                    </div>
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="icon-workbench">
                    <span class="ui-muted">
                        "display: baseline vs configured vs disabled contrast"
                    </span>
                    <div class="docs-row">
                        <div class="docs-card">
                            <div class="ui-muted">"Baseline"</div>
                            <Icon size=IconSize::Md tone=IconTone::Default is_decorative=true>
                                "✓"
                            </Icon>
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Configured"</div>
                            {move || {
                                let size = match workbench_size_key.get().as_str() {
                                    "sm" => IconSize::Sm,
                                    "lg" => IconSize::Lg,
                                    _ => IconSize::Md,
                                };
                                let tone = match workbench_tone_key.get().as_str() {
                                    "muted" => IconTone::Muted,
                                    "accent" => IconTone::Accent,
                                    "danger" => IconTone::Danger,
                                    _ => IconTone::Default,
                                };
                                let class_name = if workbench_custom_class.get() {
                                    "docs-icon-custom".to_string()
                                } else {
                                    String::new()
                                };
                                let decorative = workbench_decorative.get();
                                let aria_label = if decorative {
                                    String::new()
                                } else {
                                    workbench_label.get()
                                };
                                view! {
                                    <Icon
                                        size=size
                                        tone=tone
                                        is_disabled=workbench_disabled.get()
                                        is_decorative=decorative
                                        aria_label=aria_label
                                        class_name=class_name
                                    >
                                        {workbench_glyph.get()}
                                    </Icon>
                                }
                            }}
                        </div>
                        <div class="docs-card">
                            <div class="ui-muted">"Disabled contrast"</div>
                            <Icon size=IconSize::Lg tone=IconTone::Danger is_disabled=true is_decorative=true>
                                "⚠"
                            </Icon>
                        </div>
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn empty_state() -> AnyView {
    let empty_state_imports =
        "use leptos::prelude::*;\nuse ui_components::{Button, ButtonVariant, EmptyState, EmptyStateAlign, EmptyStateTone};"
            .to_string();
    let hello_code = Signal::derive(move || r#"<EmptyState />"#.to_string());

    let state_matrix_code = Signal::derive(move || {
        r#"<EmptyState />
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>
<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  is_compact=true
  is_bordered=true
/>"#
        .to_string()
    });

    let tone_code = Signal::derive(move || {
        r#"<EmptyState
  title="No projects yet".to_string()
  description="Create your first project to unlock dashboards and team workflows.".to_string()
  tone=EmptyStateTone::Default
  icon=move || view! { <span>"📁"</span> }
  actions=move || view! {
    <ui_components::Button>"Create project"</ui_components::Button>
  }
/>
<EmptyState
  title="Nothing matched".to_string()
  description="Try a different query or clear filters.".to_string()
  tone=EmptyStateTone::Muted
  align=EmptyStateAlign::Center
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<EmptyState
  title="Deployments paused".to_string()
  description="Approvals are required before resuming this environment.".to_string()
  tone=EmptyStateTone::Accent
  is_compact=true
  is_bordered=true
  class_name="docs-empty-state-custom".to_string()
  icon=move || view! { <span>"⏸"</span> }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Review approvals"
    </ui_components::Button>
  }
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<EmptyState />
<EmptyState
  title="Mapped from parent state".to_string()
  description="EmptyState has no controlled/uncontrolled axis; parent can still map app state into props.".to_string()
  tone=EmptyStateTone::Muted
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<EmptyState
  title="Snapshot baseline".to_string()
  description="Component default path renders complete config in one pass.".to_string()
/>
<EmptyState
  title="Streaming optional fallback".to_string()
  description="Not an LLM body reader surface: optional streaming contracts fallback to snapshot.".to_string()
  tone=EmptyStateTone::Muted
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<EmptyState
  title="No incidents".to_string()
  description="Everything is healthy. If this changes, add actions below.".to_string()
  tone=EmptyStateTone::Default
/>"#
        .to_string()
    });

    let tone_options = vec![
        "Default".to_string(),
        "Muted".to_string(),
        "Accent".to_string(),
    ];
    let align_options = vec!["Start".to_string(), "Center".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_align_index, set_workbench_align_index) = signal(Some(0_usize));
    let (workbench_title, set_workbench_title) = signal("No incidents".to_string());
    let (workbench_description, set_workbench_description) =
        signal("Everything is healthy. If this changes, add actions below.".to_string());
    let (workbench_is_compact, set_workbench_is_compact) = signal(false);
    let (workbench_is_bordered, set_workbench_is_bordered) = signal(false);
    let (workbench_with_icon, set_workbench_with_icon) = signal(false);
    let (workbench_with_actions, set_workbench_with_actions) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);

    let workbench_code = Signal::derive(move || {
        let tone_index = workbench_tone_index.get().unwrap_or(0);
        let tone_variant = match tone_index {
            1 => "EmptyStateTone::Muted",
            2 => "EmptyStateTone::Accent",
            _ => "EmptyStateTone::Default",
        };
        let align_index = workbench_align_index.get().unwrap_or(0);
        let align_variant = match align_index {
            1 => "EmptyStateAlign::Center",
            _ => "EmptyStateAlign::Start",
        };
        let title = workbench_title
            .get()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");
        let description = workbench_description
            .get()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");

        let mut lines = vec![
            "<EmptyState".to_string(),
            format!("  title=\"{title}\".to_string()"),
            format!("  description=\"{description}\".to_string()"),
            format!("  tone={tone_variant}"),
            format!("  align={align_variant}"),
        ];

        if workbench_is_compact.get() {
            lines.push("  is_compact=true".to_string());
        }
        if workbench_is_bordered.get() {
            lines.push("  is_bordered=true".to_string());
        }
        if workbench_custom_class.get() {
            lines.push("  class_name=\"docs-empty-state-workbench\".to_string()".to_string());
        }
        if workbench_with_icon.get() {
            lines.push("  icon=move || view! { <span>\"🧭\"</span> }".to_string());
        }
        if workbench_with_actions.get() {
            lines.push("  actions=move || view! {".to_string());
            lines.push(
                "    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>"
                    .to_string(),
            );
            lines.push("      \"Retry\"".to_string());
            lines.push("    </ui_components::Button>".to_string());
            lines.push("  }".to_string());
        }

        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone_variant = match workbench_tone_index.get().unwrap_or(0) {
            1 => "Muted",
            2 => "Accent",
            _ => "Default",
        };
        let align_variant = match workbench_align_index.get().unwrap_or(0) {
            1 => "Center",
            _ => "Start",
        };
        format!(
            "EmptyStateActualConfig {{\n  tone: {tone_variant},\n  align: {align_variant},\n  is_compact: {},\n  is_bordered: {},\n  with_icon: {},\n  with_actions: {},\n  custom_class: {},\n  title: \"{}\",\n  description: \"{}\",\n  marker_expectations: [\"data-tone\", \"data-align\", \"data-state\", \"data-icon\", \"data-actions\", \"data-title-source\", \"data-description-source\"],\n}}",
            workbench_is_compact.get(),
            workbench_is_bordered.get(),
            workbench_with_icon.get(),
            workbench_with_actions.get(),
            workbench_custom_class.get(),
            workbench_title.get(),
            workbench_description.get(),
        )
    });

    view! {
        <ComponentPage
            title="EmptyState"
            slug="empty-state"
            group="Display"
            description="baseline-style empty-state primitive with centralized tone/align/layout/source contracts and stable slot/data markers."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
                code_imports=empty_state_imports.clone()
            >
                <EmptyState />
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=state_matrix_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                    <EmptyState
                        title="Deployments paused".to_string()
                        description="Approvals are required before resuming this environment.".to_string()
                        tone=EmptyStateTone::Accent
                        is_compact=true
                        is_bordered=true
                    />
                </div>
            </Playground>

            <Playground
                title="Tone + Alignment + Actions"
                code_signal=tone_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState
                        title="No projects yet".to_string()
                        description="Create your first project to unlock dashboards and team workflows.".to_string()
                        tone=EmptyStateTone::Default
                        icon=move || view! { <span>"📁"</span> }
                        actions=move || {
                            view! {
                                <ui_components::Button>
                                    "Create project"
                                </ui_components::Button>
                            }
                        }
                    />
                    <EmptyState
                        title="Nothing matched".to_string()
                        description="Try a different query or clear filters.".to_string()
                        tone=EmptyStateTone::Muted
                        align=EmptyStateAlign::Center
                    />
                </div>
            </Playground>

            <Playground
                title="Compact + Bordered + Custom Class"
                code_signal=state_code
                code_imports=empty_state_imports.clone()
            >
                <EmptyState
                    title="Deployments paused".to_string()
                    description="Approvals are required before resuming this environment.".to_string()
                    tone=EmptyStateTone::Accent
                    is_compact=true
                    is_bordered=true
                    class_name="docs-empty-state-custom".to_string()
                    icon=move || view! { <span>"⏸"</span> }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Review approvals"
                            </ui_components::Button>
                        }
                    }
                />
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="EmptyState has no controlled/uncontrolled runtime axis; compare default usage vs app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState />
                    <EmptyState
                        title="Mapped from parent state".to_string()
                        description="EmptyState has no controlled/uncontrolled axis; parent can still map app state into props.".to_string()
                        tone=EmptyStateTone::Muted
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="EmptyState is not an LLM reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=empty_state_imports.clone()
            >
                <div class="docs-stack">
                    <EmptyState
                        title="Snapshot baseline".to_string()
                        description="Component default path renders complete config in one pass.".to_string()
                    />
                    <EmptyState
                        title="Streaming optional fallback".to_string()
                        description="Not an LLM body reader surface: optional streaming contracts fallback to snapshot.".to_string()
                        tone=EmptyStateTone::Muted
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Interactive acceptance canvas: tune props/state and verify semantic markers in real time."
                code_signal=workbench_code
                code_imports=empty_state_imports.clone()
                test_source_path="components/empty-state/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="empty-state-workbench-controls">
                            <label class="docs-search__label" data-slot="empty-state-workbench-title">
                                "Title"
                                <input
                                    type="text"
                                    prop:value=move || workbench_title.get()
                                    on:input=move |ev| set_workbench_title.set(event_target_value(&ev))
                                />
                            </label>
                            <label class="docs-search__label" data-slot="empty-state-workbench-description">
                                "Description"
                                <input
                                    type="text"
                                    prop:value=move || workbench_description.get()
                                    on:input=move |ev| set_workbench_description.set(event_target_value(&ev))
                                />
                            </label>

                            <div data-slot="empty-state-workbench-tone">
                                <div class="docs-search__label">"Tone"</div>
                                <SegmentedControl
                                    id_base="docs-empty-state-workbench-tone".to_string()
                                    options=tone_options.clone()
                                    selected_index=workbench_tone_index
                                    set_selected_index=set_workbench_tone_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="EmptyState tone".to_string()
                                />
                            </div>

                            <div data-slot="empty-state-workbench-align">
                                <div class="docs-search__label">"Align"</div>
                                <SegmentedControl
                                    id_base="docs-empty-state-workbench-align".to_string()
                                    options=align_options.clone()
                                    selected_index=workbench_align_index
                                    set_selected_index=set_workbench_align_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="EmptyState align".to_string()
                                />
                            </div>

                            <div data-slot="empty-state-workbench-toggle-compact">
                                <Switch checked=workbench_is_compact set_checked=set_workbench_is_compact>
                                    "Compact"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-bordered">
                                <Switch checked=workbench_is_bordered set_checked=set_workbench_is_bordered>
                                    "Bordered"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-icon">
                                <Switch checked=workbench_with_icon set_checked=set_workbench_with_icon>
                                    "Icon"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-actions">
                                <Switch checked=workbench_with_actions set_checked=set_workbench_with_actions>
                                    "Actions"
                                </Switch>
                            </div>
                            <div data-slot="empty-state-workbench-toggle-class">
                                <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                    "Custom class"
                                </Switch>
                            </div>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="empty-state-workbench">
                    {move || {
                        let tone = match workbench_tone_index.get().unwrap_or(0) {
                            1 => EmptyStateTone::Muted,
                            2 => EmptyStateTone::Accent,
                            _ => EmptyStateTone::Default,
                        };
                        let align = match workbench_align_index.get().unwrap_or(0) {
                            1 => EmptyStateAlign::Center,
                            _ => EmptyStateAlign::Start,
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-empty-state-workbench".to_string()
                        } else {
                            String::new()
                        };
                        let title = workbench_title.get();
                        let description = workbench_description.get();
                        let is_compact = workbench_is_compact.get();
                        let is_bordered = workbench_is_bordered.get();

                        if workbench_with_icon.get() && workbench_with_actions.get() {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    class_name=class_name
                                    icon=move || view! { <span>"🧭"</span> }
                                    actions=move || {
                                        view! {
                                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                                "Retry"
                                            </ui_components::Button>
                                        }
                                    }
                                />
                            }
                                .into_any()
                        } else if workbench_with_icon.get() {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    class_name=class_name
                                    icon=move || view! { <span>"🧭"</span> }
                                />
                            }
                                .into_any()
                        } else if workbench_with_actions.get() {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    class_name=class_name
                                    actions=move || {
                                        view! {
                                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                                "Retry"
                                            </ui_components::Button>
                                        }
                                    }
                                />
                            }
                                .into_any()
                        } else {
                            view! {
                                <EmptyState
                                    title=title
                                    description=description
                                    tone=tone
                                    align=align
                                    is_compact=is_compact
                                    is_bordered=is_bordered
                                    class_name=class_name
                                />
                            }
                                .into_any()
                        }
                    }}
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=empty_state_imports
            >
                <EmptyState
                    title="No incidents".to_string()
                    description="Everything is healthy. If this changes, add actions below.".to_string()
                    tone=EmptyStateTone::Default
                />
            </Playground>

            <section class="docs-card docs-prose" data-slot="empty-state-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                <p>
                    "Open "
                    <code>"Show code"</code>
                    " in any playground, then use the code block copy action. Copied snippets are auto-normalized by "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " so required imports are included."
                </p>
                <p>"Real component sources:"</p>
                <ul data-slot="empty-state-source-first-paths">
                    <li><code>"components/empty-state/src/mod.rs"</code></li>
                    <li><code>"components/empty-state/src/logic.rs"</code></li>
                    <li><code>"components/empty-state/src/view.rs"</code></li>
                    <li><code>"components/empty-state/src/styles.rs"</code></li>
                    <li><code>"components/empty-state/src/motion.rs"</code></li>
                </ul>
                <p>"Dependency baseline (Cargo.toml):"</p>
                <pre data-slot="empty-state-source-first-deps">
                    <code>
                        "[dependencies]\nui-components = { default-features = false, features = [\"component-empty_state\", \"inject-css\"] }\n# Mount under UiRoot to inject base/theme/components CSS."
                    </code>
                </pre>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn error_view() -> AnyView {
    let error_view_imports =
        "use leptos::prelude::*;\nuse ui_components::{ErrorView, ErrorViewMotion, ErrorViewTone, Icon, IconSize, IconTone};"
            .to_string();
    let workbench_tone_options = vec!["Negative".to_string(), "Neutral".to_string()];
    let workbench_message_options = vec!["Email".to_string(), "Retry".to_string()];
    let (workbench_tone_index, set_workbench_tone_index) = signal(Some(0_usize));
    let (workbench_message_index, set_workbench_message_index) = signal(Some(0_usize));
    let (workbench_is_invalid, set_workbench_is_invalid) = signal(true);
    let (workbench_is_compact, set_workbench_is_compact) = signal(false);
    let (workbench_is_bordered, set_workbench_is_bordered) = signal(false);

    let hello_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
/>"#
        .to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
/>
<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  is_compact=true
  message="Compact neutral tone contract".to_string()
/>
<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  is_compact=true
  is_bordered=true
  message="Compact + bordered source markers".to_string()
/>"#
        .to_string()
    });

    let basic_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Please enter a valid email address".to_string()
/>
<ErrorView
  is_invalid=false
  message="This error stays hidden until the field becomes invalid.".to_string()
/>"#
        .to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  is_compact=true
  is_bordered=true
  class_name="docs-error-view-custom".to_string()
  motion=ErrorViewMotion {
    hidden_translate_px: 12.0,
    hidden_opacity: 0.0,
    hidden_scale: 0.95,
    ..ErrorViewMotion::default()
  }
  icon=move || view! {
    <Icon size=IconSize::Sm tone=IconTone::Danger is_decorative=true>"⚠"</Icon>
  }
  actions=move || view! {
    <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
      "Retry"
    </ui_components::Button>
  }
>
  <span>"Validation failed. Check highlighted fields and retry."</span>
</ErrorView>"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Default path".to_string()
/>
<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  message="ErrorView has no controllable state axis; parent state maps into props.".to_string()
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  message="Snapshot baseline: complete config renders in one pass.".to_string()
/>
<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  message="Streaming is optional and falls back to snapshot for ErrorView.".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<ErrorView
  is_invalid=true
  tone=ErrorViewTone::Neutral
  message="Copy-ready starter".to_string()
/>"#
        .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let tone_variant = match workbench_tone_index.get().unwrap_or(0) {
            1 => "ErrorViewTone::Neutral",
            _ => "ErrorViewTone::Negative",
        };
        let message = match workbench_message_index.get().unwrap_or(0) {
            1 => "Retry request failed. Try again.",
            _ => "Please enter a valid email address",
        };
        let compact_line = if workbench_is_compact.get() {
            "  is_compact=true\n"
        } else {
            ""
        };
        let bordered_line = if workbench_is_bordered.get() {
            "  is_bordered=true\n"
        } else {
            ""
        };

        format!(
            "<ErrorView\n  is_invalid={}\n  tone={tone_variant}\n{compact_line}{bordered_line}  message=\"{message}\".to_string()\n/>",
            workbench_is_invalid.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let tone = match workbench_tone_index.get().unwrap_or(0) {
            1 => "neutral",
            _ => "negative",
        };
        let message = match workbench_message_index.get().unwrap_or(0) {
            1 => "retry",
            _ => "email",
        };

        format!(
            "ErrorViewWorkbenchConfig {{\n  is_invalid: {},\n  tone: \"{tone}\",\n  is_compact: {},\n  is_bordered: {},\n  message_preset: \"{message}\",\n}}",
            workbench_is_invalid.get(),
            workbench_is_compact.get(),
            workbench_is_bordered.get(),
        )
    });

    view! {
        <ComponentPage
            title="ErrorView"
            slug="error-view"
            group="Display"
            description="baseline-style validation error container with centralized visibility/content/source state contracts and spring-driven motion markers."
        >
            <Playground
                title="Hello World"
                description="Default API sync: tone defaults to negative, is_compact/is_bordered default to false; message shown here is an explicit override."
                code_signal=hello_code
                code_imports=error_view_imports.clone()
            >
                <ErrorView
                    is_invalid=true
                    message="Please enter a valid email address".to_string()
                />
            </Playground>

            <Playground
                title="State Matrix (Tone / Compact / Source Markers)"
                code_signal=state_matrix_code
                code_imports=error_view_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        message="Please enter a valid email address".to_string()
                    />
                    <ErrorView
                        is_invalid=true
                        tone=ErrorViewTone::Neutral
                        is_compact=true
                        message="Compact neutral tone contract".to_string()
                    />
                    <ErrorView
                        is_invalid=true
                        tone=ErrorViewTone::Neutral
                        is_compact=true
                        is_bordered=true
                        message="Compact + bordered source markers".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="在线切换 tone / invalid / compact / bordered / message，并实时预览语义状态与反馈。"
                code_signal=workbench_code
                code_imports=error_view_imports.clone()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="error-view-workbench-controls">
                            <div data-slot="error-view-workbench-tone">
                                <div class="docs-search__label">"Tone"</div>
                                <SegmentedControl
                                    id_base="docs-error-view-workbench-tone".to_string()
                                    options=workbench_tone_options.clone()
                                    selected_index=workbench_tone_index
                                    set_selected_index=set_workbench_tone_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ErrorView workbench tone".to_string()
                                />
                            </div>

                            <div data-slot="error-view-workbench-message">
                                <div class="docs-search__label">"Message preset"</div>
                                <SegmentedControl
                                    id_base="docs-error-view-workbench-message".to_string()
                                    options=workbench_message_options.clone()
                                    selected_index=workbench_message_index
                                    set_selected_index=set_workbench_message_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ErrorView workbench message".to_string()
                                />
                            </div>

                            <div data-slot="error-view-workbench-toggle-invalid">
                                <Switch checked=workbench_is_invalid set_checked=set_workbench_is_invalid>
                                    "Invalid"
                                </Switch>
                            </div>
                            <div data-slot="error-view-workbench-toggle-compact">
                                <Switch checked=workbench_is_compact set_checked=set_workbench_is_compact>
                                    "Compact"
                                </Switch>
                            </div>
                            <div data-slot="error-view-workbench-toggle-bordered">
                                <Switch checked=workbench_is_bordered set_checked=set_workbench_is_bordered>
                                    "Bordered"
                                </Switch>
                            </div>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="error-view-workbench">
                    {move || {
                        let tone = match workbench_tone_index.get().unwrap_or(0) {
                            1 => ErrorViewTone::Neutral,
                            _ => ErrorViewTone::Negative,
                        };
                        let message = match workbench_message_index.get().unwrap_or(0) {
                            1 => "Retry request failed. Try again.".to_string(),
                            _ => "Please enter a valid email address".to_string(),
                        };

                        view! {
                            <ErrorView
                                is_invalid=workbench_is_invalid.get()
                                tone=tone
                                is_compact=workbench_is_compact.get()
                                is_bordered=workbench_is_bordered.get()
                                message=message
                            />
                        }
                        .into_any()
                    }}

                    <div class="ui-muted" data-slot="error-view-workbench-feedback">
                        {move || {
                            let tone = match workbench_tone_index.get().unwrap_or(0) {
                                1 => "neutral",
                                _ => "negative",
                            };
                            let message = match workbench_message_index.get().unwrap_or(0) {
                                1 => "retry",
                                _ => "email",
                            };
                            format!(
                                "feedback: invalid={}, tone={tone}, compact={}, bordered={}, message={message}",
                                workbench_is_invalid.get(),
                                workbench_is_compact.get(),
                                workbench_is_bordered.get(),
                            )
                        }}
                    </div>
                </div>
            </Playground>

            <Playground
                title="Invalid Visibility"
                code_signal=basic_code
                code_imports=error_view_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        message="Please enter a valid email address".to_string()
                    />
                    <ErrorView
                        is_invalid=false
                        message="This error stays hidden until the field becomes invalid.".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Custom Content + Motion + Actions"
                code_signal=state_code
                code_imports=error_view_imports.clone()
            >
                <ErrorView
                    is_invalid=true
                    tone=ErrorViewTone::Neutral
                    is_compact=true
                    is_bordered=true
                    class_name="docs-error-view-custom".to_string()
                    motion=ErrorViewMotion {
                        hidden_translate_px: 12.0,
                        hidden_opacity: 0.0,
                        hidden_scale: 0.95,
                        ..ErrorViewMotion::default()
                    }
                    icon=move || {
                        view! {
                            <Icon size=IconSize::Sm tone=IconTone::Danger is_decorative=true>
                                "⚠"
                            </Icon>
                        }
                    }
                    actions=move || {
                        view! {
                            <ui_components::Button variant=ui_components::ButtonVariant::Secondary>
                                "Retry"
                            </ui_components::Button>
                        }
                    }
                >
                    <span>
                        "Validation failed. Check highlighted fields and retry."
                    </span>
                </ErrorView>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast (N/A for ErrorView)"
                description="ErrorView has no controlled/uncontrolled state axis; compare default usage and app-state mapped props."
                code_signal=controlled_contrast_code
                code_imports=error_view_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        message="Default path".to_string()
                    />
                    <ErrorView
                        is_invalid=true
                        tone=ErrorViewTone::Neutral
                        message="ErrorView has no controllable state axis; parent state maps into props.".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="ErrorView is not an LLM body reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=error_view_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ErrorView
                        is_invalid=true
                        message="Snapshot baseline: complete config renders in one pass.".to_string()
                    />
                    <ErrorView
                        is_invalid=true
                        tone=ErrorViewTone::Neutral
                        message="Streaming is optional and falls back to snapshot for ErrorView.".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=error_view_imports
            >
                <ErrorView
                    is_invalid=true
                    tone=ErrorViewTone::Neutral
                    message="Copy-ready starter".to_string()
                />
            </Playground>

            <section class="docs-card docs-prose" data-slot="error-view-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                <p>
                    "Open "
                    <code>"Show code"</code>
                    " in any playground, then use the code block copy action. Copied snippets are auto-normalized by "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " so required imports are included."
                </p>
                <p>"Real component sources:"</p>
                <ul data-slot="error-view-source-paths">
                    <li><code>"components/error-view/src/mod.rs"</code></li>
                    <li><code>"components/error-view/src/logic.rs"</code></li>
                    <li><code>"components/error-view/src/view.rs"</code></li>
                    <li><code>"components/error-view/src/styles.rs"</code></li>
                    <li><code>"components/error-view/src/motion.rs"</code></li>
                </ul>
                <p>"Dependency baseline (Cargo.toml):"</p>
                <pre data-slot="error-view-source-first-deps">
                    <code>
                        "[dependencies]\nui-components = { default-features = false, features = [\"component-error_view\", \"inject-css\"] }\n# Mount under UiRoot to inject base/theme/components CSS."
                    </code>
                </pre>
            </section>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn pressable_feedback() -> AnyView {
    let (press_count, set_press_count) = signal(0u32);
    let on_press_count = Callback::new(move |_| {
        set_press_count.update(|count| *count += 1);
    });

    let basic_code = Signal::derive(move || {
        r#"<PressableFeedback
  effect=PressableFeedbackEffect::Highlight
  tone=PressableFeedbackTone::Accent
  on_press=on_press_count
>
  <div class="docs-ripple-surface">"Press me"</div>
</PressableFeedback>"#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<PressableFeedback
  effect=PressableFeedbackEffect::HighlightRipple
  tone=PressableFeedbackTone::Neutral
  bounded=false
  motion=PressableFeedbackMotion {
    pressed_scale: 0.94,
    highlight_opacity: 0.2,
    ripple: RippleMotion {
      duration_ms: 720,
      ..RippleMotion::default()
    },
    ..PressableFeedbackMotion::default()
  }
  class_name="docs-pressable-feedback-custom".to_string()
>
  <div class="docs-ripple-surface docs-ripple-surface--accent">"Custom feedback"</div>
</PressableFeedback>

<PressableFeedback is_disabled=true effect=PressableFeedbackEffect::Highlight>
  <div class="docs-ripple-surface docs-ripple-surface--static">"Disabled"</div>
</PressableFeedback>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="PressableFeedback"
            slug="pressable-feedback"
            group="Display"
            description="baseline-style press feedback container with centralized effect/tone/boundary/source contracts, spring-driven scale/highlight motion, and optional ripple composition."
        >
            <Playground title="Scale + Highlight" code_signal=basic_code>
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=PressableFeedbackEffect::Highlight
                        tone=PressableFeedbackTone::Accent
                        on_press=on_press_count
                    >
                        <div class="docs-ripple-surface">
                            "Press me"
                        </div>
                    </PressableFeedback>

                    <div class="ui-muted">
                        {move || format!("Press count: {}", press_count.get())}
                    </div>
                </div>
            </Playground>

            <Playground title="Highlight + Ripple + Custom Motion" code_signal=custom_code>
                <div class="docs-stack docs-stack--tight">
                    <PressableFeedback
                        effect=PressableFeedbackEffect::HighlightRipple
                        tone=PressableFeedbackTone::Neutral
                        bounded=false
                        motion=PressableFeedbackMotion {
                            pressed_scale: 0.94,
                            highlight_opacity: 0.2,
                            ripple: RippleMotion {
                                duration_ms: 720,
                                ..RippleMotion::default()
                            },
                            ..PressableFeedbackMotion::default()
                        }
                        class_name="docs-pressable-feedback-custom".to_string()
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--accent">
                            "Custom feedback"
                        </div>
                    </PressableFeedback>

                    <PressableFeedback
                        is_disabled=true
                        effect=PressableFeedbackEffect::Highlight
                    >
                        <div class="docs-ripple-surface docs-ripple-surface--static">
                            "Disabled"
                        </div>
                    </PressableFeedback>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn color_swatch() -> AnyView {
    let color_swatch_imports = "use leptos::prelude::*;\nuse ui_components::{ColorSwatch, ColorSwatchRounding, ColorSwatchShape, ColorSwatchSize};".to_string();

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
            "ColorSwatchActualConfig {{\n  color: \"{color}\",\n  size: {size:?},\n  rounding: {rounding:?},\n  shape: {shape:?},\n  is_bordered: {is_bordered},\n  is_decorative: {is_decorative},\n  bool_source: \"{}\",\n  custom_aria: {custom_aria},\n  custom_class: {custom_class},\n  lang: {},\n  data_alpha: \"{alpha_attr}\",\n  data_state: \"{data_state}\",\n  class: \"{}\",\n}}",
            "is-prop",
            if custom_lang { "\"zh-CN\"" } else { "None" },
            classes.join(" ")
        )
    });

    let workbench_test_css = Signal::derive(move || {
        format!(
            "/* crates/ui-components/src/color/swatch/styles.rs */\n{}",
            ui_components::color::swatch::styles::CSS
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
                test_source_path="crates/ui-components/src/color/swatch/styles.rs".to_string()
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
                            String::new()
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
                description="Copy action auto-injects missing imports for direct run. Source: components/color-swatch/src/{mod,logic,view,styles,motion}.rs. Dependency baseline: ui-components = { default-features = false, features = [\"component-color_swatch\", \"inject-css\"] } + mount under UiRoot."
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
                            "ui-components = { default-features = false, features = [\"component-color_swatch\", \"inject-css\"] }"
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

pub(super) fn color_swatch_picker() -> AnyView {
    let color_swatch_picker_imports = "use leptos::prelude::*;\nuse ui_components::{ColorSwatchPicker, ColorSwatchPickerItem, ColorSwatchRounding, ColorSwatchShape};".to_string();
    let swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("#f80", "Orange"),
        ColorSwatchPickerItem::named("#080", "Green"),
        ColorSwatchPickerItem::named("#08f", "Blue"),
    ];
    let swatches_for_basic = swatches.clone();
    let swatches_for_matrix = swatches.clone();
    let swatches_for_controlled = swatches.clone();
    let swatches_for_stream = swatches.clone();
    let swatches_for_source = swatches.clone();

    let disabled_swatches = vec![
        ColorSwatchPickerItem::named("#A00", "Red"),
        ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
        ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
        ColorSwatchPickerItem::new("#08f"),
    ];
    let disabled_swatches_for_state = disabled_swatches.clone();
    let disabled_swatches_for_matrix = disabled_swatches.clone();
    let (controlled_selected_color, set_controlled_selected_color) =
        signal(Some("#A00".to_string()));

    let hello_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![ColorSwatchPickerItem::named("#f80", "Orange")]).0
/>"##
            .to_string()
    });

    let basic_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>"##
            .to_string()
    });

    let state_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
  class_name="docs-color-swatch-picker-custom".to_string()
  aria_label="Fill color".to_string()
/>"##
            .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>

<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0
  shape=ColorSwatchShape::Wide
  rounding=ColorSwatchRounding::Default
/>"##
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
/>

<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  selected_color=selected_signal
  on_selected_change=on_selected_change
/>"##
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
  ]).0
  aria_label="Fill color".to_string()
/>"##
            .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r##"<ColorSwatchPicker
  swatches=signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0
  default_selected_color="#f80".to_string()
  class_name="docs-color-swatch-picker-custom".to_string()
/>"##
            .to_string()
    });

    let workbench_shape_options = vec!["Default".to_string(), "Wide".to_string()];
    let workbench_rounding_options = vec!["Default".to_string(), "Full".to_string()];
    let workbench_selected_options = vec![
        "None".to_string(),
        "Red".to_string(),
        "Orange".to_string(),
        "Green".to_string(),
        "Blue".to_string(),
    ];
    let (workbench_shape_index, set_workbench_shape_index) = signal(Some(0_usize));
    let (workbench_rounding_index, set_workbench_rounding_index) = signal(Some(0_usize));
    let (workbench_selected_index, set_workbench_selected_index) = signal(Some(2_usize));
    let (workbench_use_controlled, set_workbench_use_controlled) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_bordered, set_workbench_is_bordered) = signal(true);
    let (workbench_use_disabled_palette, set_workbench_use_disabled_palette) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_custom_aria, set_workbench_custom_aria) = signal(true);
    let (workbench_lang_zh, set_workbench_lang_zh) = signal(false);
    let (workbench_last_selected, set_workbench_last_selected) = signal(Some("#f80".to_string()));

    let workbench_swatches_base = swatches.clone();
    let workbench_swatches_disabled = disabled_swatches.clone();
    let (workbench_swatches, set_workbench_swatches) =
        signal(if workbench_use_disabled_palette.get_untracked() {
            workbench_swatches_disabled.clone()
        } else {
            workbench_swatches_base.clone()
        });
    Effect::new(move |_| {
        let next = if workbench_use_disabled_palette.get() {
            workbench_swatches_disabled.clone()
        } else {
            workbench_swatches_base.clone()
        };
        set_workbench_swatches.set(next);
    });

    let workbench_selected_color =
        Signal::derive(move || match workbench_selected_index.get().unwrap_or(2) {
            1 => Some("#A00".to_string()),
            2 => Some("#f80".to_string()),
            3 => Some("#080".to_string()),
            4 => Some("#08f".to_string()),
            _ => None,
        });

    let workbench_code = Signal::derive(move || {
        let shape_variant = match workbench_shape_index.get().unwrap_or(0) {
            1 => "ColorSwatchShape::Wide",
            _ => "ColorSwatchShape::Default",
        };
        let rounding_variant = match workbench_rounding_index.get().unwrap_or(0) {
            1 => "ColorSwatchRounding::Full",
            _ => "ColorSwatchRounding::Default",
        };
        let swatch_vector = if workbench_use_disabled_palette.get() {
            r##"signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("rgba(14, 116, 144, 0.4)", "Cyan 40%").disabled(true),
    ColorSwatchPickerItem::named("rgba(255, 0, 0, 0)", "Transparent"),
    ColorSwatchPickerItem::new("#08f"),
  ]).0"##
        } else {
            r##"signal(vec![
    ColorSwatchPickerItem::named("#A00", "Red"),
    ColorSwatchPickerItem::named("#f80", "Orange"),
    ColorSwatchPickerItem::named("#080", "Green"),
    ColorSwatchPickerItem::named("#08f", "Blue"),
  ]).0"##
        };
        let selected_color = match workbench_selected_index.get().unwrap_or(2) {
            1 => Some("#A00"),
            2 => Some("#f80"),
            3 => Some("#080"),
            4 => Some("#08f"),
            _ => None,
        };
        let selection_lines = if workbench_use_controlled.get() {
            "  selected_color=selected_signal\n  on_selected_change=on_selected_change\n"
                .to_string()
        } else {
            selected_color
                .map(|color| format!("  default_selected_color=\"{color}\".to_string()\n"))
                .unwrap_or_default()
        };
        let class_line = if workbench_custom_class.get() {
            "  class_name=\"docs-color-swatch-picker-custom\".to_string()\n"
        } else {
            ""
        };
        let aria_line = if workbench_custom_aria.get() {
            "  aria_label=\"Workbench fill color\".to_string()\n"
        } else {
            ""
        };
        let lang_line = if workbench_lang_zh.get() {
            "  lang=\"zh-CN\".to_string()\n"
        } else {
            ""
        };
        format!(
            "<ColorSwatchPicker\n  swatches={swatch_vector}\n  shape={shape_variant}\n  rounding={rounding_variant}\n  is_disabled={}\n  is_bordered={}\n{selection_lines}{class_line}{aria_line}{lang_line}/>",
            workbench_is_disabled.get(),
            workbench_is_bordered.get(),
        )
    });

    let workbench_actual_config = Signal::derive(move || {
        let selected = workbench_selected_color
            .get()
            .unwrap_or_else(|| "none".to_string());
        format!(
            "mode={}\nshape={}\nrounding={}\npalette={}\nselected={}\nis_disabled={}\nis_bordered={}\ncustom_class={}\ncustom_aria={}\nlang={}",
            if workbench_use_controlled.get() {
                "controlled"
            } else {
                "uncontrolled"
            },
            if workbench_shape_index.get().unwrap_or(0) == 1 {
                "wide"
            } else {
                "default"
            },
            if workbench_rounding_index.get().unwrap_or(0) == 1 {
                "full"
            } else {
                "default"
            },
            if workbench_use_disabled_palette.get() {
                "disabled+transparent"
            } else {
                "base"
            },
            selected,
            workbench_is_disabled.get(),
            workbench_is_bordered.get(),
            workbench_custom_class.get(),
            workbench_custom_aria.get(),
            if workbench_lang_zh.get() {
                "zh-CN"
            } else {
                "inherit"
            },
        )
    });

    view! {
        <ComponentPage
            title="ColorSwatchPicker"
            slug="color-swatch-picker"
            group="Display"
            description="baseline-compatible selectable swatch group with centralized color normalization, single-selection state, keyboard roving, and stable slot/data state markers."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <ColorSwatchPicker
                    swatches=signal(vec![ColorSwatchPickerItem::named("#f80", "Orange")]).0
                />
            </Playground>

            <Playground
                title="Basic Selection"
                code_signal=basic_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <ColorSwatchPicker
                    swatches=signal(swatches_for_basic).0
                    default_selected_color="#f80".to_string()
                />
            </Playground>

            <Playground
                title="Transparency + Disabled + Custom Class"
                code_signal=state_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <ColorSwatchPicker
                    swatches=signal(disabled_swatches_for_state).0
                    shape=ColorSwatchShape::Wide
                    rounding=ColorSwatchRounding::Default
                    class_name="docs-color-swatch-picker-custom".to_string()
                    aria_label="Fill color".to_string()
                />
            </Playground>

            <Playground
                title="State Matrix"
                code_signal=matrix_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_matrix).0
                        default_selected_color="#f80".to_string()
                    />
                    <ColorSwatchPicker
                        swatches=signal(disabled_swatches_for_matrix).0
                        shape=ColorSwatchShape::Wide
                        rounding=ColorSwatchRounding::Default
                        class_name="docs-color-swatch-picker-custom".to_string()
                        aria_label="Fill color".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast"
                code_signal=controlled_contrast_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_controlled.clone()).0
                        default_selected_color="#f80".to_string()
                    />
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_controlled.clone()).0
                        selected_color=controlled_selected_color
                        on_selected_change=Callback::new(move |next| {
                            set_controlled_selected_color.set(next);
                        })
                        aria_label="Controlled swatch picker".to_string()
                    />
                    <span class="ui-muted">
                        {move || {
                            format!(
                                "controlled selected_color={}",
                                controlled_selected_color
                                    .get()
                                    .unwrap_or_else(|| "none".to_string())
                            )
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="ColorSwatchPicker is streaming-optional. Marker contract remains `data-ui-stream-support=unsupported` + `data-ui-stream-fallback=snapshot`."
                code_signal=stream_snapshot_code
                code_imports=color_swatch_picker_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_stream).0
                        default_selected_color="#f80".to_string()
                        aria_label="Fill color".to_string()
                    />
                    <span class="ui-muted">
                        "effective markers: data-ui-stream-support=unsupported data-ui-stream-fallback=snapshot data-ui-output-status=verified"
                    </span>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                description="Interactive acceptance canvas: adjust props/state, observe selection feedback, and replay keyboard flow."
                code_signal=workbench_code
                code_imports=color_swatch_picker_imports.clone()
                test_source_path="components/color-swatch-picker/src/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-workbench-controls">
                            <div data-slot="color-swatch-picker-workbench-shape-control">
                                <div class="docs-search__label">"Shape"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-shape".to_string()
                                    options=workbench_shape_options.clone()
                                    selected_index=workbench_shape_index
                                    set_selected_index=set_workbench_shape_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker shape".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-rounding-control">
                                <div class="docs-search__label">"Rounding"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-rounding".to_string()
                                    options=workbench_rounding_options.clone()
                                    selected_index=workbench_rounding_index
                                    set_selected_index=set_workbench_rounding_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker rounding".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-selection-control">
                                <div class="docs-search__label">"Selected color"</div>
                                <SegmentedControl
                                    id_base="docs-color-swatch-picker-workbench-selection".to_string()
                                    options=workbench_selected_options.clone()
                                    selected_index=workbench_selected_index
                                    set_selected_index=set_workbench_selected_index
                                    size=SegmentedControlSize::Sm
                                    aria_label="ColorSwatchPicker selected color".to_string()
                                />
                            </div>
                            <div data-slot="color-swatch-picker-workbench-mode-switch">
                                <Switch checked=workbench_use_controlled set_checked=set_workbench_use_controlled>
                                    "Controlled mode"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-disabled-switch">
                                <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                    "Disabled"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-bordered-switch">
                                <Switch checked=workbench_is_bordered set_checked=set_workbench_is_bordered>
                                    "Bordered"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-palette-switch">
                                <Switch
                                    checked=workbench_use_disabled_palette
                                    set_checked=set_workbench_use_disabled_palette
                                >
                                    "Use disabled/transparent palette"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-custom-class-switch">
                                <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                    "Custom class"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-custom-aria-switch">
                                <Switch checked=workbench_custom_aria set_checked=set_workbench_custom_aria>
                                    "Custom aria_label"
                                </Switch>
                            </div>
                            <div data-slot="color-swatch-picker-workbench-lang-switch">
                                <Switch checked=workbench_lang_zh set_checked=set_workbench_lang_zh>
                                    "Lang=zh-CN"
                                </Switch>
                            </div>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-workbench-canvas">
                    {move || {
                        let shape = match workbench_shape_index.get().unwrap_or(0) {
                            1 => ColorSwatchShape::Wide,
                            _ => ColorSwatchShape::Square,
                        };
                        let rounding = match workbench_rounding_index.get().unwrap_or(0) {
                            1 => ColorSwatchRounding::Full,
                            _ => ColorSwatchRounding::Default,
                        };
                        let default_selected_color = match workbench_selected_index.get().unwrap_or(2)
                        {
                            1 => "#A00".to_string(),
                            2 => "#f80".to_string(),
                            3 => "#080".to_string(),
                            4 => "#08f".to_string(),
                            _ => String::new(),
                        };
                        let class_name = if workbench_custom_class.get() {
                            "docs-color-swatch-picker-custom".to_string()
                        } else {
                            String::new()
                        };
                        let aria_label = if workbench_custom_aria.get() {
                            "Workbench fill color".to_string()
                        } else {
                            String::new()
                        };
                        let lang = if workbench_lang_zh.get() {
                            "zh-CN".to_string()
                        } else {
                            String::new()
                        };

                        if workbench_use_controlled.get() {
                            view! {
                                <ColorSwatchPicker
                                    swatches=workbench_swatches
                                    selected_color=workbench_selected_color
                                    on_selected_change=Callback::new(move |next: Option<String>| {
                                        set_workbench_last_selected.set(next.clone());
                                        let next_index = match next.as_deref() {
                                            Some("#A00") => 1,
                                            Some("#f80") => 2,
                                            Some("#080") => 3,
                                            Some("#08f") => 4,
                                            _ => 0,
                                        };
                                        set_workbench_selected_index.set(Some(next_index));
                                    })
                                    is_disabled=workbench_is_disabled.get()
                                    is_bordered=workbench_is_bordered.get()
                                    shape=shape
                                    rounding=rounding
                                    class_name=class_name
                                    aria_label=aria_label
                                    lang=lang
                                />
                            }
                            .into_any()
                        } else {
                            view! {
                                <ColorSwatchPicker
                                    swatches=workbench_swatches
                                    default_selected_color=default_selected_color
                                    on_selected_change=Callback::new(move |next| {
                                        set_workbench_last_selected.set(next);
                                    })
                                    is_disabled=workbench_is_disabled.get()
                                    is_bordered=workbench_is_bordered.get()
                                    shape=shape
                                    rounding=rounding
                                    class_name=class_name
                                    aria_label=aria_label
                                    lang=lang
                                />
                            }
                            .into_any()
                        }
                    }}
                    <span class="ui-muted" data-slot="color-swatch-picker-workbench-feedback">
                        {move || {
                            format!(
                                "mode={}, palette={}, last_selected={}, disabled={}, bordered={}",
                                if workbench_use_controlled.get() {
                                    "controlled"
                                } else {
                                    "uncontrolled"
                                },
                                if workbench_use_disabled_palette.get() {
                                    "disabled+transparent"
                                } else {
                                    "base"
                                },
                                workbench_last_selected
                                    .get()
                                    .unwrap_or_else(|| "none".to_string()),
                                workbench_is_disabled.get(),
                                workbench_is_bordered.get(),
                            )
                        }}
                    </span>
                    <ol class="ui-muted" data-slot="color-swatch-picker-workbench-replay">
                        <li>"Replay path: focus Orange swatch, press ArrowRight, observe selected marker change."</li>
                        <li>"Toggle Controlled mode and repeat ArrowRight to verify controlled callback sync."</li>
                        <li>"Enable disabled palette and Disabled switch to verify blocked interaction branch."</li>
                    </ol>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Show code + copy returns runnable snippet with imports injected by apps/docs-app/src/playground.rs::compose_copy_ready_code."
                code_signal=source_first_code
                code_imports=color_swatch_picker_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="color-swatch-picker-copy-ready">
                    <h3>"Source-first / Copy-Paste Ready"</h3>
                    <span class="ui-muted">
                        "Playground copy action injects missing imports through "
                        <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                        "."
                    </span>
                    <span class="ui-muted" data-slot="color-swatch-picker-source-prerequisites">
                        "Dependency baseline (Cargo.toml): "
                        <code>
                            "ui-components = { default-features = false, features = [\"component-color_swatch_picker\", \"inject-css\"] }"
                        </code>
                    </span>
                    <ul class="ui-muted" data-slot="color-swatch-picker-source-paths">
                        <li><code>"components/color-swatch-picker/src/mod.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/logic.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/view.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/styles.rs"</code></li>
                        <li><code>"components/color-swatch-picker/src/motion.rs"</code></li>
                    </ul>
                    <ColorSwatchPicker
                        swatches=signal(swatches_for_source).0
                        default_selected_color="#f80".to_string()
                        class_name="docs-color-swatch-picker-custom".to_string()
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn skeleton_group() -> AnyView {
    let loading_code = Signal::derive(move || {
        r#"<SkeletonGroup
  is_loading=true
  variant=SkeletonGroupVariant::Shimmer
  layout=SkeletonGroupLayout::Vertical
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line docs-skeleton-line--short".to_string() />
</SkeletonGroup>"#.to_string()
    });

    let state_code = Signal::derive(move || {
        r#"<SkeletonGroup
  is_loading=false
  is_skeleton_only=false
  variant=SkeletonGroupVariant::None
>
  <div class="ui-muted">"Loaded content rendered by parent group."</div>
</SkeletonGroup>

<SkeletonGroup
  is_loading=false
  is_skeleton_only=true
  variant=SkeletonGroupVariant::Pulse
  class_name="docs-skeleton-group-custom".to_string()
>
  <Skeleton variant=SkeletonVariant::Rect class_name="docs-skeleton-line".to_string() />
</SkeletonGroup>"#
            .to_string()
    });

    view! {
        <ComponentPage
            title="SkeletonGroup"
            slug="skeleton-group"
            group="Display"
            description="baseline-style skeleton coordination container with centralized loading/layout/variant visibility contracts and stable slot/data-state markers."
        >
            <Playground
                title="Shimmer + Pulse Layout"
                code_signal=loading_code
                test_source_path="crates/ui-components/src/skeleton/group/view.rs".to_string()
            >
                <div class="docs-stack">
                    <SkeletonGroup
                        is_loading=true
                        variant=SkeletonGroupVariant::Shimmer
                        layout=SkeletonGroupLayout::Vertical
                        density=SkeletonGroupDensity::Comfortable
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>

                    <SkeletonGroup
                        is_loading=true
                        variant=SkeletonGroupVariant::Pulse
                        layout=SkeletonGroupLayout::Horizontal
                        density=SkeletonGroupDensity::Compact
                        aria_label="Profile placeholders".to_string()
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Circle
                            is_shimmer=false
                            class_name="docs-skeleton-avatar".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            is_shimmer=false
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            is_shimmer=false
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>
                </div>
            </Playground>

            <Playground
                title="Loaded + Skeleton Only"
                code_signal=state_code
                test_source_path="crates/ui-components/src/skeleton/group/view.rs".to_string()
            >
                <div class="docs-stack">
                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=false
                        variant=SkeletonGroupVariant::None
                    >
                        <div class="ui-muted">
                            "Loaded content rendered by parent group."
                        </div>
                    </SkeletonGroup>

                    <SkeletonGroup
                        is_loading=false
                        is_skeleton_only=true
                        variant=SkeletonGroupVariant::Pulse
                        class_name="docs-skeleton-group-custom".to_string()
                    >
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line".to_string()
                        />
                        <Skeleton
                            variant=SkeletonVariant::Rect
                            class_name="docs-skeleton-line docs-skeleton-line--short".to_string()
                        />
                    </SkeletonGroup>

                    <div class="ui-muted">
                        "When `is_skeleton_only=true` and loading is finished, the skeleton group hides itself."
                    </div>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn flip_card() -> AnyView {
    let motion_options = vec![
        "default".to_string(),
        "gentle".to_string(),
        "dramatic".to_string(),
    ];
    let flip_card_imports =
        "use leptos::prelude::*;\nuse ui_components::{FlipCard, FlipCardMotion};".to_string();
    let (workbench_motion_index, set_workbench_motion_index) = signal(Some(0_usize));
    let (workbench_default_is_flipped, set_workbench_default_is_flipped) = signal(false);
    let (workbench_is_disabled, set_workbench_is_disabled) = signal(false);
    let (workbench_is_flip_on_hover, set_workbench_is_flip_on_hover) = signal(true);
    let (workbench_custom_id, set_workbench_custom_id) = signal(true);
    let (workbench_custom_class, set_workbench_custom_class) = signal(true);
    let (controlled_is_flipped, set_controlled_is_flipped) = signal(false);
    let (controlled_toggle_count, set_controlled_toggle_count) = signal(0_u32);

    let workbench_motion =
        Signal::derive(move || match workbench_motion_index.get().unwrap_or(0) {
            1 => FlipCardMotion {
                hover_scale: 1.01,
                hover_tilt_deg: 2.0,
                ..FlipCardMotion::default()
            },
            2 => FlipCardMotion {
                hover_scale: 1.06,
                hover_tilt_deg: 7.5,
                ..FlipCardMotion::default()
            },
            _ => FlipCardMotion::default(),
        });

    let workbench_code = Signal::derive(move || {
        let default_is_flipped = workbench_default_is_flipped.get();
        let is_disabled = workbench_is_disabled.get();
        let is_flip_on_hover = workbench_is_flip_on_hover.get();
        let custom_id = workbench_custom_id.get();
        let custom_class = workbench_custom_class.get();
        let motion_index = workbench_motion_index.get().unwrap_or(0);
        let motion_name = match motion_index {
            1 => "gentle",
            2 => "dramatic",
            _ => "default",
        };
        let motion = workbench_motion.get();

        let mut lines = vec!["<FlipCard".to_string()];
        if default_is_flipped {
            lines.push("  default_is_flipped=true".to_string());
        }
        if is_disabled {
            lines.push("  is_disabled=true".to_string());
        }
        if is_flip_on_hover {
            lines.push("  is_flip_on_hover=true".to_string());
        }
        if custom_id {
            lines.push("  id=\"docs-flip-card-workbench\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-flip-card-state\".into()".to_string());
        }
        if motion_index != 0 {
            lines.push(format!(
                "  motion=FlipCardMotion {{ hover_scale: {:.2}, hover_tilt_deg: {:.1}, ..FlipCardMotion::default() }}",
                motion.hover_scale,
                motion.hover_tilt_deg
            ));
        }
        lines.push("  front=move || view! { <div>\"Workbench front\"</div> }".to_string());
        lines.push("  back=move || view! { <div>\"Workbench back\"</div> }".to_string());
        lines.push("/>".to_string());
        lines.push(format!("// motion preset: {motion_name}"));

        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let default_is_flipped = workbench_default_is_flipped.get();
        let is_disabled = workbench_is_disabled.get();
        let is_flip_on_hover = workbench_is_flip_on_hover.get();
        let custom_id = workbench_custom_id.get();
        let custom_class = workbench_custom_class.get();
        let motion = workbench_motion.get();

        let mut classes = vec![
            "ui-flip-card".to_string(),
            if is_disabled {
                "ui-flip-card--disabled".to_string()
            } else {
                "ui-flip-card--enabled".to_string()
            },
            if default_is_flipped {
                "ui-flip-card--flipped".to_string()
            } else {
                "ui-flip-card--default".to_string()
            },
            if is_flip_on_hover {
                "ui-flip-card--hover".to_string()
            } else {
                "ui-flip-card--toggle".to_string()
            },
        ];
        if custom_class {
            classes.push("ui-flip-card--custom-class".to_string());
            classes.push("docs-flip-card-state".to_string());
        }
        if custom_id {
            classes.push("ui-flip-card--custom-id".to_string());
        }
        if motion != FlipCardMotion::default() {
            classes.push("ui-flip-card--custom-motion".to_string());
        }

        format!(
            "FlipCardActualConfig {{\n  default_is_flipped: {default_is_flipped},\n  is_disabled: {is_disabled},\n  is_flip_on_hover: {is_flip_on_hover},\n  custom_id: {custom_id},\n  custom_class: {custom_class},\n  motion: {{ hover_scale: {:.2}, hover_tilt_deg: {:.1} }},\n  class: \"{}\",\n  markers: [\"data-state\", \"data-visible\", \"data-flip-mode\", \"data-motion-source\", \"data-id-source\"],\n}}",
            motion.hover_scale,
            motion.hover_tilt_deg,
            classes.join(" ")
        )
    });

    let flip_card_test_css_source = Signal::derive(move || {
        format!(
            "/* components/flip-card/src/styles.rs */\n{}",
            ui_components::flip_card::styles::CSS
        )
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<FlipCard front=... back=... />
<FlipCard is_flip_on_hover=true front=... back=... />
<FlipCard is_disabled=true front=... back=... />
<FlipCard motion=FlipCardMotion { hover_scale: 1.06, hover_tilt_deg: 7.5, ..FlipCardMotion::default() } front=... back=... />"#.to_string()
    });

    let hello_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! { <div>"Front"</div> }
  back=move || view! { <div>"Back"</div> }
/>"#
        .to_string()
    });

    let basic_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card-toggle".to_string()
  front=move || view! {
    <div class="ui-flip-card__title">"Front"</div>
    <div class="ui-flip-card__description">"Click or press Enter/Space to flip."</div>
  }
  back=move || view! {
    <div class="ui-flip-card__title">"Back"</div>
    <div class="ui-flip-card__description">"Stable state/data markers for docs and tests."</div>
  }
/>"#
        .to_string()
    });

    let markers_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card"
  class_name="docs-flip-card-state".to_string()
  is_flip_on_hover=true
  motion=FlipCardMotion {
    hover_scale: 1.03,
    hover_tilt_deg: 4.0,
    ..FlipCardMotion::default()
  }
  front=move || view! { <div>"Inspect markers (front)"</div> }
  back=move || view! { <div>"Inspect markers (back)"</div> }
/>"#
        .to_string()
    });

    let disabled_code = Signal::derive(move || {
        r#"<FlipCard
  id="docs-flip-card-disabled".to_string()
  is_disabled=true
  front=move || view! { <div>"Disabled front"</div> }
  back=move || view! { <div>"Disabled back"</div> }
/>"#
        .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! { <div>"Uncontrolled front"</div> }
  back=move || view! { <div>"Uncontrolled back"</div> }
/>

let (is_flipped, set_is_flipped) = signal(false);
<FlipCard
  is_flipped=Signal::derive(move || is_flipped.get())
  on_is_flipped_change=Callback::new(move |next| set_is_flipped.set(next))
  front=move || view! { <div>"Controlled front"</div> }
  back=move || view! { <div>"Controlled back"</div> }
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<FlipCard
  front=move || view! { <div>"Snapshot baseline"</div> }
  back=move || view! { <div>"Complete config renders in one pass."</div> }
/>
<FlipCard
  is_flip_on_hover=true
  front=move || view! { <div>"Streaming optional"</div> }
  back=move || view! { <div>"Fallback stays snapshot for FlipCard."</div> }
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<FlipCard
  is_flip_on_hover=true
  front=move || view! { <div>"Copy-ready front"</div> }
  back=move || view! { <div>"Copy-ready back"</div> }
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="FlipCard"
            slug="flip-card"
            group="Display"
            description="3D front/back card with baseline-style state/source markers and baseline-level spring motion for flip/hover interactions."
        >
            <Playground
                title="Hello World (Default Path)"
                code_signal=hello_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        front=move || view! { <div class="ui-flip-card__title">"Front"</div> }
                        back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                    />
                </div>
            </Playground>

            <Playground
                title="Click + Keyboard Flip"
                code_signal=basic_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card-toggle".to_string()
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Front"</div>
                                    <div class="ui-flip-card__description">
                                        "Click or press Enter/Space to flip."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Back"</div>
                                    <div class="ui-flip-card__description">
                                        "Back face stays keyboard reachable with the same button semantics."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=flip_card_imports.clone()
                test_css_source=flip_card_test_css_source
                test_source_path="components/flip-card/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="可调翻转初始态/hover/disabled/id/class/motion，并在同一面板查看 code + config + scoped css test。"
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Motion preset"</div>
                            <SegmentedControl
                                id_base="docs-flip-card-motion".to_string()
                                options=motion_options.clone()
                                selected_index=workbench_motion_index
                                set_selected_index=set_workbench_motion_index
                                size=SegmentedControlSize::Sm
                                aria_label="FlipCard motion preset".to_string()
                            />
                            <Switch checked=workbench_default_is_flipped set_checked=set_workbench_default_is_flipped>
                                "Default Flipped"
                            </Switch>
                            <Switch checked=workbench_is_flip_on_hover set_checked=set_workbench_is_flip_on_hover>
                                "Flip On Hover"
                            </Switch>
                            <div data-slot="chart-workbench-toggle-disabled">
                                <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                    "Disabled"
                                </Switch>
                            </div>
                            <Switch checked=workbench_custom_id set_checked=set_workbench_custom_id>
                                "Custom ID"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom Class"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight">
                    {move || {
                        let default_is_flipped = workbench_default_is_flipped.get();
                        let is_disabled = workbench_is_disabled.get();
                        let is_flip_on_hover = workbench_is_flip_on_hover.get();
                        let with_custom_class = workbench_custom_class.get();
                        let with_custom_id = workbench_custom_id.get();
                        let motion = workbench_motion.get();

                        match (with_custom_class, with_custom_id) {
                            (true, true) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    class_name="docs-flip-card-state".to_string()
                                    id="docs-flip-card-workbench".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (true, false) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    class_name="docs-flip-card-state".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (false, true) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    id="docs-flip-card-workbench".to_string()
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                            (false, false) => view! {
                                <FlipCard
                                    default_is_flipped=default_is_flipped
                                    is_disabled=is_disabled
                                    is_flip_on_hover=is_flip_on_hover
                                    motion=motion
                                    front=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench front"</div>
                                                <div class="ui-flip-card__description">
                                                    "展示区：实时预览当前 config + motion。"
                                                </div>
                                            </>
                                        }
                                    }
                                    back=move || {
                                        view! {
                                            <>
                                                <div class="ui-flip-card__title">"Workbench back"</div>
                                                <div class="ui-flip-card__description">
                                                    "Code/CSS Test 区可直接验证 data-* 契约。"
                                                </div>
                                            </>
                                        }
                                    }
                                />
                            }
                            .into_any(),
                        }
                    }}
                    <div class="ui-muted">
                        "切换 settings 后，使用 Code / Test 面板查看实际配置与 scoped CSS 影响。"
                    </div>
                </div>
            </Playground>

            <Playground
                title="State + Source Markers"
                description="Inspect `data-state`, `data-flip-mode`, `data-class-source`, `data-motion-source`, `data-id-source`, and face-level visibility markers (`data-visible`/`data-hidden`)."
                code_signal=markers_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card".to_string()
                        class_name="docs-flip-card-state".to_string()
                        is_flip_on_hover=true
                        motion=FlipCardMotion {
                            hover_scale: 1.03,
                            hover_tilt_deg: 4.0,
                            ..FlipCardMotion::default()
                        }
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Inspect markers (front)"</div>
                                    <div class="ui-flip-card__description">
                                        "Hover enters flipped mode source = custom."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Inspect markers (back)"</div>
                                    <div class="ui-flip-card__description">
                                        "Front/back visibility markers stay explicit for regression tests."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>

            <Playground
                title="State Matrix (Default / Hover / Disabled / Dramatic Motion)"
                code_signal=state_matrix_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-stack">
                    <div class="docs-row">
                        <FlipCard
                            front=move || view! { <div class="ui-flip-card__title">"Default"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                        <FlipCard
                            is_flip_on_hover=true
                            front=move || view! { <div class="ui-flip-card__title">"Hover flip"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                    </div>
                    <div class="docs-row">
                        <FlipCard
                            is_disabled=true
                            front=move || view! { <div class="ui-flip-card__title">"Disabled"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                        <FlipCard
                            motion=FlipCardMotion {
                                hover_scale: 1.06,
                                hover_tilt_deg: 7.5,
                                ..FlipCardMotion::default()
                            }
                            is_flip_on_hover=true
                            front=move || view! { <div class="ui-flip-card__title">"Dramatic motion"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Back"</div> }
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast"
                description="Compare default uncontrolled usage against externally managed `is_flipped + on_is_flipped_change`."
                code_signal=controlled_contrast_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <div class="docs-row">
                        <FlipCard
                            front=move || view! { <div class="ui-flip-card__title">"Uncontrolled front"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Uncontrolled back"</div> }
                        />
                        <FlipCard
                            is_flipped=Signal::derive(move || controlled_is_flipped.get())
                            on_is_flipped_change=Callback::new(move |next| {
                                set_controlled_is_flipped.set(next);
                                set_controlled_toggle_count.update(|count| *count += 1);
                            })
                            front=move || view! { <div class="ui-flip-card__title">"Controlled front"</div> }
                            back=move || view! { <div class="ui-flip-card__title">"Controlled back"</div> }
                        />
                    </div>
                    <div class="ui-muted">
                        {move || {
                            format!(
                                "Controlled state: is_flipped={}, on_is_flipped_change calls={}",
                                controlled_is_flipped.get(),
                                controlled_toggle_count.get(),
                            )
                        }}
                    </div>
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="flip-card-parameter-matrix">
                <h3>"Parameter Matrix (API + Defaults)"</h3>
                <ul data-slot="flip-card-parameter-rows">
                    <li>
                        <code>"is_flipped"</code>
                        " = controlled axis (default: uncontrolled when omitted)"
                    </li>
                    <li>
                        <code>"default_is_flipped"</code>
                        " / "
                        <code>"default_flipped"</code>
                        " = default priority "
                        <code>"default_is_flipped > default_flipped > DEFAULT_FLIPPED(false)"</code>
                    </li>
                    <li>
                        <code>"on_is_flipped_change"</code>
                        " = optional callback (default: none)"
                    </li>
                    <li>
                        <code>"is_disabled"</code>
                        " / "
                        <code>"disabled"</code>
                        " = default priority "
                        <code>"is_disabled > disabled > DEFAULT_DISABLED(false)"</code>
                    </li>
                    <li>
                        <code>"flip_mode"</code>
                        " / "
                        <code>"is_flip_on_hover"</code>
                        " / "
                        <code>"flip_on_hover"</code>
                        " = default priority "
                        <code>"flip_mode > is_flip_on_hover > flip_on_hover > DEFAULT_HOVER_FLIP(false)"</code>
                    </li>
                    <li>
                        <code>"id"</code>
                        " = custom id or "
                        <code>"IdProvider::next_prefixed_id(DEFAULT_ID_PREFIX)"</code>
                    </li>
                    <li>
                        <code>"class_name"</code>
                        " = optional custom class (default: none)"
                    </li>
                    <li>
                        <code>"motion"</code>
                        " = "
                        <code>"FlipCardMotion::default()"</code>
                    </li>
                </ul>
            </section>

            <Playground
                title="Streaming / Snapshot Contract"
                description="FlipCard is not an LLM body reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports=flip_card_imports.clone()
            >
                <div class="docs-row">
                    <FlipCard
                        front=move || view! { <div class="ui-flip-card__title">"Snapshot baseline"</div> }
                        back=move || view! { <div class="ui-flip-card__description">"Complete config renders in one pass."</div> }
                    />
                    <FlipCard
                        is_flip_on_hover=true
                        front=move || view! { <div class="ui-flip-card__title">"Streaming optional"</div> }
                        back=move || view! { <div class="ui-flip-card__description">"Fallback stays snapshot for FlipCard."</div> }
                    />
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects missing imports for direct run."
                code_signal=source_first_code
                code_imports=flip_card_imports.clone()
            >
                <FlipCard
                    is_flip_on_hover=true
                    front=move || view! { <div class="ui-flip-card__title">"Copy-ready front"</div> }
                    back=move || view! { <div class="ui-flip-card__title">"Copy-ready back"</div> }
                />
            </Playground>

            <section class="docs-card docs-prose" data-slot="flip-card-source-first-contract">
                <h3>"Source-first / Copy-Paste Ready Contract"</h3>
                <p>
                    "Open "
                    <code>"Show code"</code>
                    " in any playground, then use the code block copy action. Copied snippets are auto-normalized by "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    " so required imports are included."
                </p>
                <p>"Real component sources:"</p>
                <ul data-slot="flip-card-source-paths">
                    <li><code>"components/flip-card/src/mod.rs"</code></li>
                    <li><code>"components/flip-card/src/logic.rs"</code></li>
                    <li><code>"components/flip-card/src/view.rs"</code></li>
                    <li><code>"components/flip-card/src/styles.rs"</code></li>
                    <li><code>"components/flip-card/src/motion.rs"</code></li>
                </ul>
                <p>"Dependency baseline (Cargo.toml):"</p>
                <pre data-slot="flip-card-source-first-deps">
                    <code>
                        "[dependencies]\nui-components = { default-features = false, features = [\"component-flip_card\", \"inject-css\"] }\n# Mount under UiRoot to inject base/theme/components CSS."
                    </code>
                </pre>
            </section>

            <Playground
                title="Disabled"
                code_signal=disabled_code
                code_imports=flip_card_imports
            >
                <div class="docs-row">
                    <FlipCard
                        id="docs-flip-card-disabled".to_string()
                        is_disabled=true
                        front=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Disabled front"</div>
                                    <div class="ui-flip-card__description">
                                        "No click/keyboard toggle while disabled."
                                    </div>
                                </>
                            }
                        }
                        back=move || {
                            view! {
                                <>
                                    <div class="ui-flip-card__title">"Disabled back"</div>
                                    <div class="ui-flip-card__description">
                                        "aria-disabled and disabled markers remain consistent."
                                    </div>
                                </>
                            }
                        }
                    />
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}

pub(super) fn chart() -> AnyView {
    let revenue_points = vec![
        ChartPoint::new("jan", "Jan", 12.0),
        ChartPoint::new("feb", "Feb", 18.5),
        ChartPoint::new("mar", "Mar", 17.2),
        ChartPoint::new("apr", "Apr", 24.7),
        ChartPoint::new("may", "May", 28.1),
    ];

    let line_points = vec![
        ChartPoint::new("q1", "Q1", 42.0),
        ChartPoint::new("q2", "Q2", 56.0),
        ChartPoint::new("q3", "Q3", 51.0),
        ChartPoint::new("q4", "Q4", 63.0),
    ];
    let flat_points = vec![
        ChartPoint::new("alpha", "Alpha", 20.0),
        ChartPoint::new("beta", "Beta", 20.0),
        ChartPoint::new("gamma", "Gamma", 20.0),
    ];
    let revenue_points_for_workbench = revenue_points.clone();
    let line_points_for_workbench = line_points.clone();
    let flat_points_for_workbench = flat_points.clone();
    let revenue_points_for_matrix = revenue_points.clone();
    let line_points_for_matrix = line_points.clone();
    let flat_points_for_matrix = flat_points.clone();
    let revenue_points_for_bar = revenue_points.clone();
    let line_points_for_controlled = line_points.clone();
    let revenue_points_for_contrast = revenue_points_for_matrix.clone();
    let line_points_for_contrast = line_points_for_controlled.clone();
    let line_points_for_line = line_points_for_controlled.clone();
    let line_points_for_stream = line_points_for_controlled.clone();

    let (last_action, set_last_action) = signal("none".to_string());
    let on_action = Callback::new(move |id: String| set_last_action.set(id));

    let (controlled_active_raw, set_controlled_active_raw) = signal(1_usize);
    let controlled_active: Signal<usize> = Signal::derive(move || controlled_active_raw.get());
    let on_controlled_active_change =
        Callback::new(move |next: usize| set_controlled_active_raw.set(next));

    let kind_options = vec!["bar".to_string(), "line".to_string()];
    let dataset_options = vec![
        "revenue".to_string(),
        "growth".to_string(),
        "flat".to_string(),
    ];
    let persisted_workbench_state = load_chart_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let (workbench_kind_index, set_workbench_kind_index) =
        signal(Some(initial_workbench_state.kind_index));
    let (workbench_dataset_index, set_workbench_dataset_index) =
        signal(Some(initial_workbench_state.dataset_index));
    let (workbench_is_disabled, set_workbench_is_disabled) =
        signal(initial_workbench_state.is_disabled);
    let (workbench_is_show_grid, set_workbench_is_show_grid) =
        signal(initial_workbench_state.is_show_grid);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.custom_class);
    let (workbench_lang, set_workbench_lang) = signal(initial_workbench_state.lang);
    let (workbench_last_action, set_workbench_last_action) = signal("none".to_string());
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);
    let workbench_on_action = Callback::new(move |id: String| set_workbench_last_action.set(id));

    Effect::new(move |_| {
        if workbench_persist_state.get() {
            save_chart_workbench_state(ChartWorkbenchState {
                kind_index: workbench_kind_index.get().unwrap_or(0),
                dataset_index: workbench_dataset_index.get().unwrap_or(0),
                is_disabled: workbench_is_disabled.get(),
                is_show_grid: workbench_is_show_grid.get(),
                custom_class: workbench_custom_class.get(),
                lang: workbench_lang.get(),
            });
        } else {
            clear_chart_workbench_state();
        }
    });

    let workbench_kind = Signal::derive(move || match workbench_kind_index.get().unwrap_or(0) {
        1 => ChartKind::Line,
        _ => ChartKind::Bar,
    });
    let workbench_dataset_name: Signal<&'static str> =
        Signal::derive(move || match workbench_dataset_index.get().unwrap_or(0) {
            1 => "growth",
            2 => "flat",
            _ => "revenue",
        });
    let workbench_points =
        Signal::derive(move || match workbench_dataset_index.get().unwrap_or(0) {
            1 => line_points_for_workbench.clone(),
            2 => flat_points_for_workbench.clone(),
            _ => revenue_points_for_workbench.clone(),
        });

    let hello_code = Signal::derive(move || {
        r#"<Chart points=vec![ChartPoint::new("jan", "Jan", 12.0), ChartPoint::new("feb", "Feb", 18.5), ChartPoint::new("mar", "Mar", 17.2)] />"#
            .to_string()
    });

    let workbench_code = Signal::derive(move || {
        let kind = workbench_kind.get();
        let dataset = workbench_dataset_name.get();
        let is_disabled = workbench_is_disabled.get();
        let is_show_grid = workbench_is_show_grid.get();
        let custom_class = workbench_custom_class.get();
        let lang = workbench_lang.get();

        let mut out = vec![
            "<Chart".to_string(),
            "  id_base=\"docs-chart-workbench\".into()".to_string(),
            format!("  // dataset: {dataset}"),
            "  points=/* see preview dataset */".to_string(),
            format!("  kind=ChartKind::{kind:?}"),
        ];

        if is_disabled {
            out.push("  is_disabled=true".to_string());
        }
        if !is_show_grid {
            out.push("  is_show_grid=false".to_string());
        }
        if custom_class {
            out.push("  class_name=\"docs-chart-custom\".into()".to_string());
        }
        if lang {
            out.push("  lang=\"en-US\".into()".to_string());
        }
        out.push("  on_action=Callback::new(move |id: String| { /* ... */ })".to_string());
        out.push("/>".to_string());

        out.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let kind = workbench_kind.get();
        let dataset = workbench_dataset_name.get();
        let is_disabled = workbench_is_disabled.get();
        let is_show_grid = workbench_is_show_grid.get();
        let custom_class = workbench_custom_class.get();
        let lang = workbench_lang.get();

        let mut class_tokens = vec![
            "ui-chart".to_string(),
            match kind {
                ChartKind::Bar => "ui-chart--bar".to_string(),
                ChartKind::Line => "ui-chart--line".to_string(),
            },
            if is_disabled {
                "ui-chart--disabled".to_string()
            } else {
                "ui-chart--uncontrolled".to_string()
            },
        ];
        if is_show_grid {
            class_tokens.push("ui-chart--grid".to_string());
        }
        if custom_class {
            class_tokens.push("ui-chart--custom-class".to_string());
            class_tokens.push("docs-chart-custom".to_string());
        }

        format!(
            "ChartActualConfig {{\n  dataset: \"{dataset}\",\n  kind: {kind:?},\n  is_disabled: {is_disabled},\n  is_show_grid: {is_show_grid},\n  custom_class: {custom_class},\n  lang: {},\n  class: \"{}\",\n  marker_expectations: [\"data-kind\", \"data-state\", \"data-active-index\", \"data-motion-source\"],\n}}",
            if lang { "\"en-US\"" } else { "None" },
            class_tokens.join(" ")
        )
    });

    let chart_test_css_source = Signal::derive(move || {
        format!(
            "/* components/chart/src/styles.rs */\n{}",
            ui_components::chart::styles::CSS
        )
    });

    let bar_code = Signal::derive(move || {
        r#"let (last_action, set_last_action) = signal("none".to_string());

<Chart
  id_base="docs-chart-bar".to_string()
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
    ChartPoint::new("apr", "Apr", 24.7),
    ChartPoint::new("may", "May", 28.1),
  ]
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| set_last_action.set(id))
/>
<span class="ui-muted">"last action: " {move || last_action.get()}</span>"#
            .to_string()
    });

    let line_code = Signal::derive(move || {
        r#"let (active_raw, set_active_raw) = signal(1_usize);

<Chart
  id_base="docs-chart-line".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  active_index=Signal::derive(move || active_raw.get())
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
  class_name="docs-chart-custom".to_string()
/>"#
        .to_string()
    });

    let matrix_code = Signal::derive(move || {
        r#"<Chart id_base="docs-chart-matrix-bar".to_string() kind=ChartKind::Bar points=vec![...] />
<Chart id_base="docs-chart-matrix-line".to_string() kind=ChartKind::Line points=vec![...] />
<Chart id_base="docs-chart-matrix-disabled".to_string() kind=ChartKind::Bar is_disabled=true points=vec![...] />
<Chart id_base="docs-chart-matrix-empty".to_string() kind=ChartKind::Line points=vec![] />"#.to_string()
    });

    let chart_imports =
        "use leptos::prelude::*;\nuse ui_components::{Chart, ChartKind, ChartPoint};".to_string();

    let controlled_contrast_code = Signal::derive(move || {
        r#"let (active_raw, set_active_raw) = signal(1_usize);

<Chart
  id_base="docs-chart-uncontrolled-contrast".to_string()
  points=vec![
    ChartPoint::new("jan", "Jan", 12.0),
    ChartPoint::new("feb", "Feb", 18.5),
    ChartPoint::new("mar", "Mar", 17.2),
  ]
  kind=ChartKind::Bar
/>

<Chart
  id_base="docs-chart-controlled-contrast".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  active_index=Signal::derive(move || active_raw.get())
  on_active_index_change=Callback::new(move |next| set_active_raw.set(next))
/>"#
        .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Chart
  id_base="docs-chart-streaming-snapshot".to_string()
  points=vec![
    ChartPoint::new("q1", "Q1", 42.0),
    ChartPoint::new("q2", "Q2", 56.0),
    ChartPoint::new("q3", "Q3", 51.0),
    ChartPoint::new("q4", "Q4", 63.0),
  ]
  kind=ChartKind::Line
  aria_label="Snapshot contract marker".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"<Chart
  id_base="docs-chart-source-first".to_string()
  points=vec![
    ChartPoint::new("apr", "Apr", 24.7),
    ChartPoint::new("may", "May", 28.1),
    ChartPoint::new("jun", "Jun", 31.6),
  ]
  kind=ChartKind::Bar
  on_action=Callback::new(move |id: String| {
    logging::log!("clicked point: {id}");
  })
/>"#
        .to_string()
    });

    view! {
        <ComponentPage
            title="Chart"
            slug="chart"
            group="Display"
            description="baseline-compatible chart primitive with bar/line modes, controlled active-index state, baseline-style data contracts, and baseline-level spring highlight motion for legends."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-row">
                    <Chart id_base="docs-chart-hello".to_string() points=vec![ChartPoint::new("jan", "Jan", 12.0), ChartPoint::new("feb", "Feb", 18.5), ChartPoint::new("mar", "Mar", 17.2)] />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (展示 / Config / Code / CSS Test)"
                code_signal=workbench_code
                code_imports=chart_imports.clone()
                test_css_source=chart_test_css_source
                test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/chart/src/styles.rs".to_string()
                test_config_signal=workbench_config
                description="Workbench canvas: scoped CSS live-edit + optional state persistence across reload."
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight">
                            <div class="docs-search__label">"Kind"</div>
                            <SegmentedControl
                                id_base="docs-chart-kind".to_string()
                                options=kind_options.clone()
                                selected_index=workbench_kind_index
                                set_selected_index=set_workbench_kind_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chart kind".to_string()
                            />

                            <div class="docs-search__label">"Dataset"</div>
                            <SegmentedControl
                                id_base="docs-chart-dataset".to_string()
                                options=dataset_options.clone()
                                selected_index=workbench_dataset_index
                                set_selected_index=set_workbench_dataset_index
                                size=SegmentedControlSize::Sm
                                aria_label="Chart dataset".to_string()
                            />

                            <Switch checked=workbench_is_disabled set_checked=set_workbench_is_disabled>
                                "Disabled"
                            </Switch>
                            <Switch checked=workbench_is_show_grid set_checked=set_workbench_is_show_grid>
                                "Show Grid"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom Class"
                            </Switch>
                            <div data-slot="chart-workbench-toggle-lang">
                                <Switch checked=workbench_lang set_checked=set_workbench_lang>
                                    "Lang=en-US"
                                </Switch>
                            </div>
                            <Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                                "Persist workbench state"
                            </Switch>
                        </div>
                    }
                }
            >
                <div class="docs-stack docs-stack--tight" data-slot="chart-workbench">
                    {move || {
                        let points = workbench_points.get();
                        let kind = workbench_kind.get();
                        let disabled = workbench_is_disabled.get();
                        let is_show_grid = workbench_is_show_grid.get();
                        let class_name = workbench_custom_class
                            .get()
                            .then_some("docs-chart-custom".to_string());
                        let lang = workbench_lang.get().then_some("en-US".to_string());
                        let persist = workbench_persist_state.get();

                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="chart-workbench-canvas">
                                <span class="ui-muted">
                                    "persist: "
                                    {if persist { "on" } else { "off" }}
                                </span>
                                <Chart
                                    id_base="docs-chart-workbench".to_string()
                                    points=points
                                    kind=kind
                                    is_disabled=disabled
                                    is_show_grid=is_show_grid
                                    class_name=class_name.unwrap_or_default()
                                    lang=lang.unwrap_or_default()
                                    on_action=workbench_on_action
                                />
                            </div>
                        }
                    }}
                    <span class="ui-muted">
                        "workbench last action: "
                        {move || workbench_last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Comparison Matrix (Bar / Line / Disabled / Empty)"
                code_signal=matrix_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Bar / Revenue"</span>
                        <Chart
                            id_base="docs-chart-matrix-bar".to_string()
                            points=revenue_points_for_matrix.clone()
                            kind=ChartKind::Bar
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Line / Growth"</span>
                        <Chart
                            id_base="docs-chart-matrix-line".to_string()
                            points=line_points_for_matrix.clone()
                            kind=ChartKind::Line
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Disabled"</span>
                        <div data-slot="chart-e2e-state-disabled">
                            <Chart
                                id_base="docs-chart-matrix-disabled".to_string()
                                points=flat_points_for_matrix.clone()
                                kind=ChartKind::Bar
                                is_disabled=true
                            />
                        </div>
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Empty"</span>
                        <Chart
                            id_base="docs-chart-matrix-empty".to_string()
                            points=vec![]
                            kind=ChartKind::Line
                        />
                    </div>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled Contrast"
                description="Compare default uncontrolled behavior with external active-index control."
                code_signal=controlled_contrast_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-row">
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Uncontrolled / internal state"</span>
                        <Chart
                            id_base="docs-chart-uncontrolled-contrast".to_string()
                            points=revenue_points_for_contrast
                            kind=ChartKind::Bar
                        />
                    </div>
                    <div class="docs-card" style="flex: 1 1 260px;">
                        <span class="ui-muted">"Controlled / external signal"</span>
                        <Chart
                            id_base="docs-chart-controlled-contrast".to_string()
                            points=line_points_for_contrast
                            kind=ChartKind::Line
                            active_index=controlled_active
                            on_active_index_change=on_controlled_active_change
                        />
                        <span class="ui-muted">
                            "active index: "
                            {move || controlled_active_raw.get()}
                        </span>
                    </div>
                </div>
            </Playground>

            <Playground
                title="Bar + Hover/Keyboard + Action"
                code_signal=bar_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <Chart
                        id_base="docs-chart-bar".to_string()
                        points=revenue_points_for_bar.clone()
                        kind=ChartKind::Bar
                        on_action=on_action
                    />
                    <span class="ui-muted">
                        "last action: "
                        {move || last_action.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Controlled Line + Active Index"
                code_signal=line_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-stack docs-stack--tight">
                    <div data-slot="chart-e2e-controlled-line">
                        <Chart
                            id_base="docs-chart-line".to_string()
                            points=line_points_for_line
                            kind=ChartKind::Line
                            active_index=controlled_active
                            on_active_index_change=on_controlled_active_change
                            aria_label="Quarterly growth line chart".to_string()
                            class_name="docs-chart-custom".to_string()
                        />
                    </div>
                    <span class="ui-muted">
                        "controlled active index: "
                        {move || controlled_active_raw.get()}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Streaming / Snapshot Contract"
                description="Chart is `Streaming Optional`; component rendering remains snapshot-based with `fallback=snapshot`."
                code_signal=stream_snapshot_code
                code_imports=chart_imports.clone()
            >
                <div class="docs-stack docs-stack--tight" data-slot="chart-streaming-policy">
                    <Chart
                        id_base="docs-chart-streaming-snapshot".to_string()
                        points=line_points_for_stream
                        kind=ChartKind::Line
                        aria_label="Snapshot contract marker".to_string()
                    />
                    <span class="ui-muted" data-slot="chart-streaming-policy-note">
                        "Streaming Optional; fallback=snapshot."
                    </span>
                </div>
            </Playground>

            <Playground
                title="Source-first Starter (Copy-Paste Ready)"
                description="Copy action auto-injects imports for direct run. Source: components/chart/src/{mod,logic,view,styles,motion}.rs. Dependency baseline: ui-components = { default-features = false, features = [\"component-chart\", \"inject-css\"] } + mount under UiRoot."
                code_signal=source_first_code
                code_imports=chart_imports
            >
                <div class="docs-stack docs-stack--tight" data-slot="chart-source-first">
                    <h3>"Source-first / Copy-Paste Ready"</h3>
                    <span class="ui-muted" data-slot="chart-copy-ready-hint">
                        "Playground copy action injects missing imports through "
                        <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                        "."
                    </span>
                    <span class="ui-muted">
                        "Dependency baseline (Cargo.toml): "
                        <code>
                            "ui-components = { default-features = false, features = [\"component-chart\", \"inject-css\"] }"
                        </code>
                    </span>
                    <ul class="ui-muted" data-slot="chart-source-paths">
                        <li><code>"components/chart/src/mod.rs"</code></li>
                        <li><code>"components/chart/src/logic.rs"</code></li>
                        <li><code>"components/chart/src/view.rs"</code></li>
                        <li><code>"components/chart/src/styles.rs"</code></li>
                        <li><code>"components/chart/src/motion.rs"</code></li>
                    </ul>
                    <div class="docs-stack docs-stack--tight" data-slot="chart-parameter-matrix">
                        <h4>"Parameter Matrix (API Names + Defaults)"</h4>
                        <ul class="ui-muted">
                            <li><code>"kind"</code>" -> default "<code>"ChartKind::Bar"</code></li>
                            <li><code>"default_active_index"</code>" -> default "<code>"0 (clamped)"</code></li>
                            <li><code>"is_disabled"</code>" -> default "<code>"false"</code></li>
                            <li><code>"is_show_grid"</code>" -> default "<code>"true"</code></li>
                            <li><code>"id_base"</code>" -> default "<code>"\"ui-chart\""</code></li>
                            <li><code>"aria_label"</code>" -> default "<code>"\"Chart\""</code></li>
                        </ul>
                    </div>
                    <div class="docs-stack docs-stack--tight" data-slot="chart-state-matrix-summary">
                        <h4>"State Matrix Coverage"</h4>
                        <ul class="ui-muted">
                            <li>"Controlled vs uncontrolled: explicit side-by-side playground."</li>
                            <li>"Disabled/empty/kind branches: covered by Comparison Matrix."</li>
                            <li>"Size/variant: N/A for Chart (not part of current public API axis)."</li>
                        </ul>
                    </div>
                    <span class="ui-muted">"Mount under UiRoot to ensure theme vars and css injection."</span>
                </div>
            </Playground>
        </ComponentPage>
    }
    .into_any()
}
