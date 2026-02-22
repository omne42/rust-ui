use crate::pages::components::ComponentPage;
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{NativeSelect, NativeSelectOption, NativeSelectSize};

const NATIVE_SELECT_DOC_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{NativeSelect, NativeSelectOption, NativeSelectSize};";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct NativeSelectWorkbenchState {
    size_index: usize,
    selected_mode_index: usize,
    required: bool,
    invalid: bool,
    disabled: bool,
    placeholder: bool,
    custom_class: bool,
    show_compare: bool,
}

impl Default for NativeSelectWorkbenchState {
    fn default() -> Self {
        Self {
            size_index: 1,
            selected_mode_index: 0,
            required: false,
            invalid: false,
            disabled: false,
            placeholder: true,
            custom_class: false,
            show_compare: true,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl NativeSelectWorkbenchState {
    fn parse(raw: &str) -> Option<Self> {
        let parts = raw.split(',').map(str::trim).collect::<Vec<_>>();
        if parts.len() != 8 {
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
            size_index: parse_index(0, 2)?,
            selected_mode_index: parse_index(1, 2)?,
            required: parse_bool(2)?,
            invalid: parse_bool(3)?,
            disabled: parse_bool(4)?,
            placeholder: parse_bool(5)?,
            custom_class: parse_bool(6)?,
            show_compare: parse_bool(7)?,
        })
    }

    fn encode(self) -> String {
        let bool_digit = |value: bool| if value { '1' } else { '0' };
        format!(
            "{},{},{},{},{},{},{},{}",
            self.size_index,
            self.selected_mode_index,
            bool_digit(self.required),
            bool_digit(self.invalid),
            bool_digit(self.disabled),
            bool_digit(self.placeholder),
            bool_digit(self.custom_class),
            bool_digit(self.show_compare),
        )
    }
}

#[cfg(target_arch = "wasm32")]
const NATIVE_SELECT_WORKBENCH_STORAGE_KEY: &str = "docs:native-select:workbench:state";

#[cfg(target_arch = "wasm32")]
fn load_native_select_workbench_state() -> Option<NativeSelectWorkbenchState> {
    let storage = web_sys::window().and_then(|window| window.local_storage().ok().flatten())?;
    let raw = storage
        .get_item(NATIVE_SELECT_WORKBENCH_STORAGE_KEY)
        .ok()
        .flatten()?;
    NativeSelectWorkbenchState::parse(&raw)
}

#[cfg(not(target_arch = "wasm32"))]
fn load_native_select_workbench_state() -> Option<NativeSelectWorkbenchState> {
    None
}

#[cfg(target_arch = "wasm32")]
fn save_native_select_workbench_state(state: NativeSelectWorkbenchState) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.set_item(NATIVE_SELECT_WORKBENCH_STORAGE_KEY, &state.encode()));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_native_select_workbench_state(_state: NativeSelectWorkbenchState) {}

#[cfg(target_arch = "wasm32")]
fn clear_native_select_workbench_state() {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        drop(storage.remove_item(NATIVE_SELECT_WORKBENCH_STORAGE_KEY));
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn clear_native_select_workbench_state() {}

pub(super) fn native_select() -> AnyView {
    let hello_options = vec![
        NativeSelectOption::new("system", "System"),
        NativeSelectOption::new("manual", "Manual"),
    ];
    let controlled_options = vec![
        NativeSelectOption::new("system", "System"),
        NativeSelectOption::new("manual", "Manual"),
        NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
    ];
    let controlled_options_for_controlled = controlled_options.clone();
    let controlled_options_for_compare = controlled_options.clone();
    let (selected_raw, set_selected_raw) = signal(None::<usize>);
    let selected_signal: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());
    let selected_signal_compare: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());
    let on_selected_change = Callback::new(move |next: Option<usize>| set_selected_raw.set(next));
    let on_selected_change_compare =
        Callback::new(move |next: Option<usize>| set_selected_raw.set(next));

    let required_options = vec![
        NativeSelectOption::new("staging", "Staging"),
        NativeSelectOption::new("production", "Production"),
        NativeSelectOption::new("canary", "Canary"),
    ];
    let required_options_for_states = required_options.clone();
    let required_options_for_matrix = required_options.clone();
    let required_options_for_stream = required_options.clone();

    let disabled_options = vec![
        NativeSelectOption::new("legacy", "Legacy").disabled(true),
        NativeSelectOption::new("frozen", "Frozen").disabled(true),
    ];
    let disabled_options_for_states = disabled_options.clone();
    let disabled_options_for_matrix = disabled_options.clone();

    let hello_code = Signal::derive(move || {
        r#"<NativeSelect
  id_base="docs-native-select-hello".to_string()
  options=vec![NativeSelectOption::new("system", "System"), NativeSelectOption::new("manual", "Manual")]
/>"#
            .to_string()
    });

    let code = Signal::derive(move || {
        r#"let (selected_raw, set_selected_raw) = signal(None::<usize>);
let selected: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());

<NativeSelect
  id_base="mode".to_string()
  options=vec![
    NativeSelectOption::new("system", "System"),
    NativeSelectOption::new("manual", "Manual"),
    NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
  ]
  selected_index=selected
  on_selected_index_change=Callback::new(move |next| set_selected_raw.set(next))
  placeholder="Choose mode".to_string()
  name="mode".to_string()
/>"#
        .to_string()
    });

    let states_code = Signal::derive(move || {
        r#"<NativeSelect
  id_base="deployment".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  default_selected_index=1
  is_required=true
  is_invalid=true
  size=NativeSelectSize::Lg
  aria_label="Deployment strategy".to_string()
/>
<NativeSelect
  id_base="disabled".to_string()
  options=vec![
    NativeSelectOption::new("legacy", "Legacy").disabled(true),
    NativeSelectOption::new("frozen", "Frozen").disabled(true),
  ]
  is_disabled=true
  placeholder="Disabled select".to_string()
  size=NativeSelectSize::Sm
/>"#
        .to_string()
    });
    let controlled_uncontrolled_code = Signal::derive(move || {
        r#"let (selected_raw, set_selected_raw) = signal(None::<usize>);
let selected: Signal<Option<usize>> = Signal::derive(move || selected_raw.get());

<NativeSelect
  id_base="docs-native-select-uncontrolled".to_string()
  options=vec![
    NativeSelectOption::new("system", "System"),
    NativeSelectOption::new("manual", "Manual"),
    NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
  ]
  default_selected_index=1
  aria_label="Uncontrolled native select".to_string()
/>
<NativeSelect
  id_base="docs-native-select-controlled-compare".to_string()
  options=vec![
    NativeSelectOption::new("system", "System"),
    NativeSelectOption::new("manual", "Manual"),
    NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
  ]
  selected_index=selected
  on_selected_index_change=Callback::new(move |next| set_selected_raw.set(next))
  aria_label="Controlled native select".to_string()
/>"#
        .to_string()
    });
    let state_matrix_code = Signal::derive(move || {
        r#"<NativeSelect
  id_base="docs-native-select-matrix-default".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  default_selected_index=0
  aria_label="Default matrix".to_string()
/>
<NativeSelect
  id_base="docs-native-select-matrix-controlled".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  selected_index=Signal::derive(|| Some(2usize))
  aria_label="Controlled matrix".to_string()
/>
<NativeSelect
  id_base="docs-native-select-matrix-disabled".to_string()
  options=vec![
    NativeSelectOption::new("legacy", "Legacy").disabled(true),
    NativeSelectOption::new("frozen", "Frozen").disabled(true),
  ]
  is_disabled=true
  placeholder="Disabled matrix".to_string()
  aria_label="Disabled matrix".to_string()
/>"#
        .to_string()
    });
    let stream_snapshot_code = Signal::derive(move || {
        r#"// NativeSelect is Streaming Optional. fallback=snapshot remains stable.
<NativeSelect
  id_base="docs-native-select-stream-snapshot".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  default_selected_index=1
  aria_label="Snapshot baseline".to_string()
/>
<NativeSelect
  id_base="docs-native-select-stream-draft".to_string()
  options=vec![
    NativeSelectOption::new("staging", "Staging"),
    NativeSelectOption::new("production", "Production"),
    NativeSelectOption::new("canary", "Canary"),
  ]
  selected_index=Signal::derive(|| Some(0usize))
  is_invalid=true
  aria_label="Draft fallback snapshot".to_string()
/>"#
        .to_string()
    });
    let workbench_options = StoredValue::new(vec![
        NativeSelectOption::new("system", "System"),
        NativeSelectOption::new("manual", "Manual"),
        NativeSelectOption::new("hybrid", "Hybrid").disabled(true),
    ]);
    let workbench_size_options = vec!["Sm".to_string(), "Md".to_string(), "Lg".to_string()];
    let workbench_selected_options = vec![
        "None".to_string(),
        "System".to_string(),
        "Manual".to_string(),
    ];
    let persisted_workbench_state = load_native_select_workbench_state();
    let has_persisted_workbench_state = persisted_workbench_state.is_some();
    let initial_workbench_state = persisted_workbench_state.unwrap_or_default();
    let (workbench_size_index, set_workbench_size_index) =
        signal(Some(initial_workbench_state.size_index));
    let (workbench_selected_mode_index, set_workbench_selected_mode_index) =
        signal(Some(initial_workbench_state.selected_mode_index));
    let (workbench_required, set_workbench_required) = signal(initial_workbench_state.required);
    let (workbench_invalid, set_workbench_invalid) = signal(initial_workbench_state.invalid);
    let (workbench_disabled, set_workbench_disabled) = signal(initial_workbench_state.disabled);
    let (workbench_placeholder, set_workbench_placeholder) =
        signal(initial_workbench_state.placeholder);
    let (workbench_custom_class, set_workbench_custom_class) =
        signal(initial_workbench_state.custom_class);
    let (workbench_show_compare, set_workbench_show_compare) =
        signal(initial_workbench_state.show_compare);
    let (workbench_persist_state, set_workbench_persist_state) =
        signal(has_persisted_workbench_state);

    Effect::new(move || {
        if workbench_persist_state.get() {
            save_native_select_workbench_state(NativeSelectWorkbenchState {
                size_index: workbench_size_index.get().unwrap_or(1).min(2),
                selected_mode_index: workbench_selected_mode_index.get().unwrap_or(0).min(2),
                required: workbench_required.get(),
                invalid: workbench_invalid.get(),
                disabled: workbench_disabled.get(),
                placeholder: workbench_placeholder.get(),
                custom_class: workbench_custom_class.get(),
                show_compare: workbench_show_compare.get(),
            });
        } else {
            clear_native_select_workbench_state();
        }
    });

    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => NativeSelectSize::Sm,
        2 => NativeSelectSize::Lg,
        _ => NativeSelectSize::Md,
    });
    let workbench_selected_index =
        Signal::derive(
            move || match workbench_selected_mode_index.get().unwrap_or(0) {
                1 => Some(0),
                2 => Some(1),
                _ => None,
            },
        );
    let on_workbench_selected_index_change = Callback::new(move |next: Option<usize>| {
        let mapped = match next {
            Some(0) => Some(1),
            Some(1) => Some(2),
            _ => Some(0),
        };
        set_workbench_selected_mode_index.set(mapped);
    });
    let workbench_code = Signal::derive(move || {
        let size = workbench_size.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let placeholder = workbench_placeholder.get();
        let custom_class = workbench_custom_class.get();
        let selected_mode = workbench_selected_mode_index.get().unwrap_or(0);
        let selected_line = match selected_mode {
            1 => "selected_index=Signal::derive(|| Some(0usize))".to_string(),
            2 => "selected_index=Signal::derive(|| Some(1usize))".to_string(),
            _ => "selected_index=Signal::derive(|| None::<usize>)".to_string(),
        };

        let mut lines = vec![
            "<NativeSelect".to_string(),
            "  id_base=\"docs-native-select-workbench\".into()".to_string(),
            "  options=vec![".to_string(),
            "    NativeSelectOption::new(\"system\", \"System\"),".to_string(),
            "    NativeSelectOption::new(\"manual\", \"Manual\"),".to_string(),
            "    NativeSelectOption::new(\"hybrid\", \"Hybrid\").disabled(true),".to_string(),
            "  ]".to_string(),
            format!("  {selected_line}"),
            format!("  is_required={required}"),
            format!("  is_invalid={invalid}"),
            format!("  is_disabled={disabled}"),
            format!("  size=NativeSelectSize::{size:?}"),
        ];
        if placeholder {
            lines.push("  placeholder=\"Choose mode\".into()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-native-select-custom\".into()".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });
    let workbench_test_css_source = Signal::derive(move || {
        format!(
            "/* crates/ui/src/native_select/styles.rs */\n{}",
            ui::native_select::styles::CSS
        )
    });
    let workbench_actual_config = Signal::derive(move || {
        let size = workbench_size.get();
        let selected_index = workbench_selected_index.get();
        let required = workbench_required.get();
        let invalid = workbench_invalid.get();
        let disabled = workbench_disabled.get();
        let placeholder = workbench_placeholder.get();
        let custom_class = workbench_custom_class.get();
        format!(
            "NativeSelectActualConfig {{\n  id_base: \"docs-native-select-workbench-primary\",\n  options: [\"system\", \"manual\", \"hybrid(disabled)\"],\n  default_selected_index: Some(1),\n  on_selected_index_change: \"callback:on_workbench_selected_index_change\",\n  aria_label: Some(\"Native select workbench\"),\n  lang: Some(\"en-US\"),\n  dir: Some(\"ltr\"),\n  size: NativeSelectSize::{size:?},\n  selected_index: {selected_index:?},\n  is_required: {required},\n  is_invalid: {invalid},\n  is_disabled: {disabled},\n  has_placeholder: {placeholder},\n  has_custom_class_name: {custom_class},\n}}"
        )
    });
    // NativeSelect docs contract markers:
    // <Playground title="Hello World (Uncontrolled)" code_signal=hello_code>
    // <Playground title="Controlled + Placeholder" code_signal=code>
    // <Playground title="Required + Invalid + Disabled" code_signal=states_code>
    // <Playground title="Controlled vs Uncontrolled" code_signal=controlled_uncontrolled_code>
    // <Playground title="State Matrix (Controlled / Uncontrolled / Disabled)" code_signal=state_matrix_code>

    view! {
        <ComponentPage
            title="NativeSelect"
            slug="native-select"
            group="Forms"
            description="baseline-style native `<select>` wrapper with controllable selection, root `data-*` contracts, and stable option normalization."
        >
            <Playground
                title="Hello World (Uncontrolled)"
                code_signal=hello_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <NativeSelect
                    id_base="docs-native-select-hello".to_string()
                    options=hello_options
                />
            </Playground>

            <Playground
                title="Controlled + Placeholder"
                code_signal=code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <NativeSelect
                        id_base="docs-native-select-controlled".to_string()
                        options=controlled_options_for_controlled
                        selected_index=selected_signal
                        on_selected_index_change=on_selected_change
                        placeholder="Choose mode".to_string()
                        name="mode".to_string()
                        aria_label="Mode".to_string()
                    />
                    <span class="ui-muted">
                        "selected index: "
                        {move || {
                            selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Required + Invalid + Disabled"
                code_signal=states_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <NativeSelect
                        id_base="docs-native-select-required".to_string()
                        options=required_options_for_states
                        default_selected_index=1
                        is_required=true
                        is_invalid=true
                        size=NativeSelectSize::Lg
                        aria_label="Deployment strategy".to_string()
                        class_name="docs-native-select-custom".to_string()
                    />

                    <NativeSelect
                        id_base="docs-native-select-disabled".to_string()
                        options=disabled_options_for_states
                        is_disabled=true
                        placeholder="Disabled select".to_string()
                        size=NativeSelectSize::Sm
                        aria_label="Disabled native select".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled"
                code_signal=controlled_uncontrolled_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="native-select-controlled-uncontrolled">
                    <div class="docs-search__label">"Uncontrolled (default_selected_index)"</div>
                    <NativeSelect
                        id_base="docs-native-select-uncontrolled".to_string()
                        options=controlled_options_for_compare.clone()
                        default_selected_index=1
                        aria_label="Uncontrolled native select".to_string()
                    />
                    <div class="docs-search__label">"Controlled (selected_index + on_selected_index_change)"</div>
                    <NativeSelect
                        id_base="docs-native-select-controlled-compare".to_string()
                        options=controlled_options_for_compare
                        selected_index=selected_signal_compare
                        on_selected_index_change=on_selected_change_compare
                        aria_label="Controlled native select".to_string()
                    />
                    <span class="ui-muted">
                        "controlled selected index: "
                        {move || {
                            selected_raw
                                .get()
                                .map(|index| index.to_string())
                                .unwrap_or_else(|| "None".to_string())
                        }}
                    </span>
                </div>
            </Playground>

            <Playground
                title="Selection Modes (Controlled / Uncontrolled / Disabled)"
                code_signal=state_matrix_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="native-select-state-matrix">
                    <NativeSelect
                        id_base="docs-native-select-matrix-default".to_string()
                        options=required_options_for_matrix.clone()
                        default_selected_index=0
                        aria_label="Default matrix".to_string()
                    />
                    <NativeSelect
                        id_base="docs-native-select-matrix-controlled".to_string()
                        options=required_options_for_matrix
                        selected_index=Signal::derive(|| Some(2usize))
                        aria_label="Controlled matrix".to_string()
                    />
                    <NativeSelect
                        id_base="docs-native-select-matrix-disabled".to_string()
                        options=disabled_options_for_matrix
                        is_disabled=true
                        placeholder="Disabled matrix".to_string()
                        aria_label="Disabled matrix".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional (fallback=snapshot)"
                description="NativeSelect 不是正文阅读面；这里展示 snapshot baseline 与 fallback=snapshot 语义标记。"
                code_signal=stream_snapshot_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="native-select-streaming-snapshot">
                    <NativeSelect
                        id_base="docs-native-select-stream-snapshot".to_string()
                        options=required_options_for_stream.clone()
                        default_selected_index=1
                        aria_label="Snapshot baseline".to_string()
                    />
                    <NativeSelect
                        id_base="docs-native-select-stream-draft".to_string()
                        options=required_options_for_stream
                        selected_index=Signal::derive(|| Some(0usize))
                        is_invalid=true
                        aria_label="Draft fallback snapshot".to_string()
                    />
                    <div class="ui-muted" data-slot="native-select-streaming-contract-hint">
                        "Inspect `data-streaming-mode/data-streaming-fallback/data-output-status`."
                    </div>
                </div>
            </Playground>

            <Playground
                title="Interactive Playground"
                code_signal=workbench_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
                test_css_source=workbench_test_css_source
                test_source_path="crates/ui/src/native_select/styles.rs".to_string()
                test_config_signal=workbench_actual_config
                description="Workbench canvas: scoped CSS live-edit（CSS Test）+ optional state persistence across reload；支持 controlled selection / required / invalid / disabled 的对比展示。"
                controls=move || view! {
                    <div class="docs-stack docs-stack--tight" data-slot="native-select-workbench-controls">
                        <div class="docs-search__label">"配置区 · Size"</div>
                        <ui::SegmentedControl
                            id_base="docs-native-select-workbench-size".to_string()
                            options=workbench_size_options.clone()
                            selected_index=workbench_size_index
                            set_selected_index=set_workbench_size_index
                            size=ui::SegmentedControlSize::Sm
                            aria_label="NativeSelect size".to_string()
                        />
                        <div class="docs-search__label">"配置区 · Selected"</div>
                        <ui::SegmentedControl
                            id_base="docs-native-select-workbench-selected".to_string()
                            options=workbench_selected_options.clone()
                            selected_index=workbench_selected_mode_index
                            set_selected_index=set_workbench_selected_mode_index
                            size=ui::SegmentedControlSize::Sm
                            aria_label="NativeSelect selected index".to_string()
                        />
                        <ui::Switch checked=workbench_required set_checked=set_workbench_required>
                            "Required"
                        </ui::Switch>
                        <ui::Switch checked=workbench_invalid set_checked=set_workbench_invalid>
                            "Invalid"
                        </ui::Switch>
                        <ui::Switch checked=workbench_disabled set_checked=set_workbench_disabled>
                            "Disabled"
                        </ui::Switch>
                        <ui::Switch checked=workbench_placeholder set_checked=set_workbench_placeholder>
                            "Placeholder"
                        </ui::Switch>
                        <ui::Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                            "Custom class"
                        </ui::Switch>
                        <ui::Switch checked=workbench_show_compare set_checked=set_workbench_show_compare>
                            "Show compare matrix"
                        </ui::Switch>
                        <ui::Switch checked=workbench_persist_state set_checked=set_workbench_persist_state>
                            "Persist workbench state"
                        </ui::Switch>
                    </div>
                }
            >
                {move || {
                    let size = workbench_size.get();
                    let selected = workbench_selected_index.get();
                    let required = workbench_required.get();
                    let invalid = workbench_invalid.get();
                    let disabled = workbench_disabled.get();
                    let placeholder = if workbench_placeholder.get() {
                        "Choose mode".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if workbench_custom_class.get() {
                        "docs-native-select-custom".to_string()
                    } else {
                        String::new()
                    };
                    let show_compare = workbench_show_compare.get();
                    let persist = workbench_persist_state.get();
                    let on_selected_index_change = on_workbench_selected_index_change;

                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="native-select-workbench-canvas">
                            <span class="ui-muted">
                                "persist: "
                                {if persist { "on" } else { "off" }}
                            </span>
                            <div class="docs-search__label">"展示区 · Primary"</div>
                            <NativeSelect
                                id_base="docs-native-select-workbench-primary".to_string()
                                options=workbench_options.get_value()
                                selected_index=Signal::derive(move || selected)
                                on_selected_index_change=on_selected_index_change
                                is_required=required
                                is_invalid=invalid
                                is_disabled=disabled
                                size=size
                                placeholder=placeholder
                                aria_label="Native select workbench".to_string()
                                class_name=class_name
                                lang="en-US".to_string()
                                dir=ui::A11yDirection::Ltr
                            />

                            <Show when=move || show_compare>
                                <div class="docs-search__label">"展示区 · 对比矩阵"</div>
                                <div class="docs-stack docs-stack--tight">
                                    <NativeSelect
                                        id_base="docs-native-select-workbench-compare-required".to_string()
                                        options=workbench_options.get_value()
                                        default_selected_index=1
                                        is_required=true
                                        is_invalid=true
                                        size=NativeSelectSize::Lg
                                        placeholder="Required + invalid".to_string()
                                        aria_label="Required invalid compare".to_string()
                                    />
                                    <NativeSelect
                                        id_base="docs-native-select-workbench-compare-disabled".to_string()
                                        options=vec![
                                            NativeSelectOption::new("legacy", "Legacy").disabled(true),
                                            NativeSelectOption::new("frozen", "Frozen").disabled(true),
                                        ]
                                        is_disabled=true
                                        size=NativeSelectSize::Sm
                                        placeholder="Disabled select".to_string()
                                        aria_label="Disabled compare".to_string()
                                    />
                                </div>
                            </Show>
                        </div>
                    }
                }}
            </Playground>

            <Playground
                title="State Matrix (Controlled / Uncontrolled / Disabled)"
                code_signal=state_matrix_code
                code_imports=NATIVE_SELECT_DOC_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="native-select-state-matrix-after-workbench">
                    <NativeSelect
                        id_base="docs-native-select-matrix-default-after-workbench".to_string()
                        options=vec![
                            NativeSelectOption::new("staging", "Staging"),
                            NativeSelectOption::new("production", "Production"),
                            NativeSelectOption::new("canary", "Canary"),
                        ]
                        default_selected_index=0
                        aria_label="Default matrix".to_string()
                        lang="en-US".to_string()
                        dir=ui::A11yDirection::Ltr
                    />
                    <NativeSelect
                        id_base="docs-native-select-matrix-controlled-after-workbench".to_string()
                        options=vec![
                            NativeSelectOption::new("staging", "Staging"),
                            NativeSelectOption::new("production", "Production"),
                            NativeSelectOption::new("canary", "Canary"),
                        ]
                        selected_index=Signal::derive(|| Some(2usize))
                        aria_label="Controlled matrix".to_string()
                        lang="ar".to_string()
                        dir=ui::A11yDirection::Rtl
                    />
                    <NativeSelect
                        id_base="docs-native-select-matrix-disabled-after-workbench".to_string()
                        options=vec![
                            NativeSelectOption::new("legacy", "Legacy").disabled(true),
                            NativeSelectOption::new("frozen", "Frozen").disabled(true),
                        ]
                        is_disabled=true
                        placeholder="Disabled matrix".to_string()
                        aria_label="Disabled matrix".to_string()
                        lang="en-US".to_string()
                        dir=ui::A11yDirection::Ltr
                    />
                </div>
            </Playground>
            <div class="ui-muted" data-slot="native-select-source-first">
                "Source-first / Copy-Paste Ready: open any playground code panel, copy once, missing imports are auto-completed."
            </div>
            <div class="ui-muted" data-slot="native-select-source-paths">
                "Source path: crates/ui/src/native_select (feature: component-native_select + inject-css)."
            </div>
        </ComponentPage>
    }
    .into_any()
}
