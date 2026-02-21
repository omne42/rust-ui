use crate::button::{Button, ButtonSize, ButtonVariant};
use crate::{CodeBlockMotion, CodeBlockStrings, logic, motion, protocol};
use leptos::{html, prelude::*};
use ui_headless::{a11y::A11yDirection, i18n};

const COPY_ICON_VIEWBOX: &str = "0 0 20 20";
const COPIED_ICON_PATH_D: &str = "M5 10.5l3 3 7-7";
const DEFAULT_COPY_ICON_RECT_X: i32 = 7;
const DEFAULT_COPY_ICON_RECT_Y: i32 = 7;
const DEFAULT_COPY_ICON_RECT_SIZE: i32 = 10;
const DEFAULT_COPY_ICON_RECT_RADIUS: i32 = 2;
const DEFAULT_COPY_ICON_BODY_PATH_D: &str = "M5 13V5a2 2 0 0 1 2-2h8";
const COPY_ICON_STROKE: &str = "currentColor";
const COPY_ICON_STROKE_LINECAP: &str = "round";
const COPY_ICON_STROKE_LINEJOIN: &str = "round";
const COPIED_ICON_STROKE_WIDTH: f64 = 1.8;
const DEFAULT_COPY_ICON_STROKE_WIDTH: f64 = 1.5;

#[cfg(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32"))]
mod wasm_debug {
    use leptos::prelude::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CodeBlockDebugSource {
        CopyButtonPress,
        Replay,
    }

    impl CodeBlockDebugSource {
        pub const fn as_str(self) -> &'static str {
            match self {
                Self::CopyButtonPress => "copy-button-press",
                Self::Replay => "replay",
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct CodeBlockDebugState {
        pub copied: bool,
        pub is_loading: bool,
        pub has_error: bool,
    }

    #[derive(Clone)]
    pub struct CodeBlockDebugEvent {
        pub sequence: usize,
        pub logical_time: u64,
        pub source: CodeBlockDebugSource,
        pub before: CodeBlockDebugState,
        pub after: CodeBlockDebugState,
    }

    #[derive(Clone, Copy)]
    pub struct CodeBlockDebugStore {
        sequence: RwSignal<usize>,
        logical_time: RwSignal<u64>,
        pub events: RwSignal<Vec<CodeBlockDebugEvent>>,
    }

    impl CodeBlockDebugStore {
        pub fn new() -> Self {
            Self {
                sequence: RwSignal::new(0),
                logical_time: RwSignal::new(0),
                events: RwSignal::new(Vec::new()),
            }
        }

        pub fn record(
            self,
            source: CodeBlockDebugSource,
            before: CodeBlockDebugState,
            after: CodeBlockDebugState,
        ) {
            let sequence = self.sequence.get_untracked().saturating_add(1);
            self.sequence.set(sequence);
            let logical_time = self.logical_time.get_untracked().saturating_add(1);
            self.logical_time.set(logical_time);

            self.events.update(|events| {
                events.push(CodeBlockDebugEvent {
                    sequence,
                    logical_time,
                    source,
                    before,
                    after,
                });
                if events.len() > 200 {
                    let overflow = events.len().saturating_sub(200);
                    events.drain(0..overflow);
                }
            });

            tracing::event!(
                target: "ui_components::code_block::state_change",
                tracing::Level::DEBUG,
                sequence,
                logical_time,
                source = source.as_str(),
                before = %format_state(before),
                after = %format_state(after),
                "code-block state transition"
            );
        }
    }

    pub fn format_state(state: CodeBlockDebugState) -> String {
        format!(
            "copied={} loading={} error={}",
            state.copied, state.is_loading, state.has_error
        )
    }
}

#[cfg(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32"))]
fn snapshot_debug_state(
    copy_logic: &crate::snippet::SnippetLogic,
) -> wasm_debug::CodeBlockDebugState {
    wasm_debug::CodeBlockDebugState {
        copied: copy_logic.copied.get_untracked(),
        is_loading: copy_logic.is_loading.get_untracked(),
        has_error: copy_logic.has_error.get_untracked(),
    }
}

fn copy_icon(copied: bool) -> impl IntoView {
    if copied {
        view! {
            <svg viewBox=COPY_ICON_VIEWBOX fill="none" aria-hidden="true">
                <path
                    d=COPIED_ICON_PATH_D
                    stroke=COPY_ICON_STROKE
                    stroke_width=COPIED_ICON_STROKE_WIDTH
                    stroke_linecap=COPY_ICON_STROKE_LINECAP
                    stroke_linejoin=COPY_ICON_STROKE_LINEJOIN
                />
            </svg>
        }
        .into_any()
    } else {
        view! {
            <svg viewBox=COPY_ICON_VIEWBOX fill="none" aria-hidden="true">
                <rect
                    x=DEFAULT_COPY_ICON_RECT_X
                    y=DEFAULT_COPY_ICON_RECT_Y
                    width=DEFAULT_COPY_ICON_RECT_SIZE
                    height=DEFAULT_COPY_ICON_RECT_SIZE
                    rx=DEFAULT_COPY_ICON_RECT_RADIUS
                    stroke=COPY_ICON_STROKE
                    stroke_width=DEFAULT_COPY_ICON_STROKE_WIDTH
                />
                <path
                    d=DEFAULT_COPY_ICON_BODY_PATH_D
                    stroke=COPY_ICON_STROKE
                    stroke_width=DEFAULT_COPY_ICON_STROKE_WIDTH
                    stroke_linecap=COPY_ICON_STROKE_LINECAP
                />
            </svg>
        }
        .into_any()
    }
}

fn code_block_meta(
    label: StoredValue<Option<String>>,
    language: StoredValue<Option<String>>,
) -> impl IntoView {
    view! {
        <div class="ui-code-block__meta" data-slot="code-block-meta">
            {move || label.get_value().map(|label| view! {
                <span class="ui-code-block__label" data-slot="code-block-label">{label}</span>
            })}
            {move || language.get_value().map(|language| view! {
                <span class="ui-code-block__language" data-slot="code-block-language">{language}</span>
            })}
        </div>
    }
}

fn code_block_header(
    show_header: bool,
    copyable: bool,
    label: StoredValue<Option<String>>,
    language: StoredValue<Option<String>>,
    copy_logic: crate::snippet::SnippetLogic,
    on_copy_press: Callback<()>,
    copy_to_clipboard_aria_label: StoredValue<String>,
) -> impl IntoView {
    view! {
        <Show when=move || show_header>
            <div class="ui-code-block__header" data-slot="code-block-header">
                {code_block_meta(label, language)}

                <Show when=move || copyable>
                    <Button
                        class_name="ui-code-block__copy-button"
                        variant=ButtonVariant::Ghost
                        size=ButtonSize::IconSm
                        aria_label=copy_to_clipboard_aria_label.get_value()
                        on_press=on_copy_press
                    >
                        {move || copy_icon(copy_logic.copied.get())}
                    </Button>
                </Show>
            </div>
        </Show>
    }
}

fn code_block_code_content(code_value: StoredValue<String>) -> impl IntoView {
    view! {
        <pre class="ui-code-block__pre" data-slot="code-block-pre">
            <code class="ui-code-block__code" data-slot="code-block-code">
                {move || code_value.get_value()}
            </code>
        </pre>
    }
}

fn code_block_status(
    copy_logic: crate::snippet::SnippetLogic,
    copied_label: StoredValue<String>,
) -> impl IntoView {
    view! {
        <span
            class="ui-code-block__a11y-status"
            data-slot="code-block-status"
            aria-live="polite"
            aria-atomic="true"
        >
            {move || if copy_logic.copied.get() {
                copied_label.get_value()
            } else {
                String::new()
            }}
        </span>
    }
}

#[cfg(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32"))]
fn render_debug_panel(
    debug_store: wasm_debug::CodeBlockDebugStore,
    request_replay: Callback<wasm_debug::CodeBlockDebugSource>,
) -> impl IntoView {
    let events = debug_store.events;

    view! {
        <details class="ui-code-block__debug" data-slot="code-block-debug" open>
            <summary data-slot="code-block-debug-summary">
                {move || format!("CodeBlock Debug ({})", events.get().len())}
            </summary>
            <ol data-slot="code-block-debug-list">
                <For
                    each=move || events.get()
                    key=|event| event.sequence
                    children=move |event| {
                        let source = event.source.as_str();
                        let before_text = wasm_debug::format_state(event.before);
                        let after_text = wasm_debug::format_state(event.after);
                        let before_attr = before_text.clone();
                        let after_attr = after_text.clone();
                        let request_replay = request_replay.clone();

                        view! {
                            <li
                                data-slot="code-block-debug-event"
                                data-debug-sequence=event.sequence
                                data-debug-logical-time=event.logical_time
                                data-debug-source=source
                                data-debug-before=before_attr
                                data-debug-after=after_attr
                            >
                                <button
                                    type="button"
                                    data-slot="code-block-debug-replay"
                                    on:click=move |_| request_replay.run(event.source)
                                >
                                    "Replay"
                                </button>
                                <code data-slot="code-block-debug-source">{source}</code>
                                <code data-slot="code-block-debug-before">{before_text}</code>
                                <code data-slot="code-block-debug-after">{after_text}</code>
                            </li>
                        }
                    }
                />
            </ol>
        </details>
    }
}

#[component]
pub fn CodeBlock(
    #[prop(into)] code: String,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] language: Option<String>,
    #[prop(optional, into)] lang: Option<String>,
    #[prop(optional)] dir: Option<A11yDirection>,
    #[prop(optional)] is_copyable: Option<bool>,
    #[prop(optional)] copyable: Option<bool>,
    #[prop(optional)] is_copied: Option<Signal<bool>>,
    #[prop(optional)] copied: Option<Signal<bool>>,
    #[prop(optional)] default_copied: Option<bool>,
    #[prop(optional)] on_copied_change: Option<Callback<bool>>,
    #[prop(optional)] output_mode: Option<protocol::CodeBlockAgentOutputMode>,
    #[prop(optional)] output_status: Option<protocol::CodeBlockAgentOutputStatus>,
    #[prop(optional)] motion: CodeBlockMotion,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    let i18n = i18n::use_ui_i18n();
    let strings = i18n.strings::<CodeBlockStrings>();
    let copy_to_clipboard_aria_label: String = strings.copy_to_clipboard_aria_label.as_ref().into();
    let copy_to_clipboard_aria_label = StoredValue::new(copy_to_clipboard_aria_label);
    let copied_status_text = strings.copied_status_text.as_ref().into();
    let motion = crate::motion::sanitize_motion(motion);
    let copyable_contract = logic::resolve_copyable_contract(is_copyable, copyable);
    let copied_contract =
        logic::resolve_copied_contract(is_copied, copied, default_copied, on_copied_change);
    let model = logic::resolve_render_model(logic::CodeBlockLogicInput {
        code,
        label,
        language,
        is_copyable: copyable_contract.is_copyable,
        class_name,
        has_custom_motion: motion != CodeBlockMotion::default(),
    });

    let state = model.state;
    let class_name = model.class_name;

    let code_value = StoredValue::new(model.code);
    let label = StoredValue::new(model.label);
    let language = StoredValue::new(model.language);

    let copy_logic =
        crate::snippet::use_snippet_logic_with_options(crate::snippet::SnippetLogicOptions {
            text: code_value.get_value(),
            copied: copied_contract.copied,
            default_copied: copied_contract.default_copied,
            on_copied_change: copied_contract.on_copied_change,
            copied_source: copied_contract.source,
            lang,
            dir,
        });
    let copied_label = StoredValue::new(copied_status_text);
    let output_mode = output_mode.unwrap_or(protocol::CodeBlockAgentOutputMode::Snapshot);
    let render_policy = protocol::render_policy();
    let output_status = output_status.unwrap_or(render_policy.output_status);
    debug_assert!(!render_policy.allow_inner_html);
    debug_assert!(!render_policy.allow_script_injection);

    let copy_logic_for_agent = copy_logic.clone();
    let agent_data = Memo::new(move |_| {
        protocol::resolve_agent_data_attrs(protocol::CodeBlockAgentInput {
            copied: copy_logic_for_agent.copied.get(),
            is_loading: copy_logic_for_agent.is_loading.get(),
            has_error: copy_logic_for_agent.has_error.get(),
            output_mode,
            output_status,
            copyable_source: protocol::CodeBlockAgentCopyableSource::from_attr(
                copyable_contract.source.as_attr(),
            ),
            copied_source: protocol::CodeBlockAgentCopiedSource::from_attr(
                copy_logic_for_agent.copied_source.as_attr(),
            ),
            motion_source: protocol::CodeBlockAgentMotionSource::from_attr(
                state.motion_source_attr,
            ),
        })
    });

    let root_ref: NodeRef<html::Div> = NodeRef::new();
    motion::attach_motion(root_ref, copy_logic.copied, motion);

    #[cfg(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32"))]
    let debug_store = wasm_debug::CodeBlockDebugStore::new();

    #[cfg(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32"))]
    let on_copy_press = {
        let copy_logic = copy_logic.clone();
        let debug_store = debug_store;
        Callback::new(move |_| {
            let before = snapshot_debug_state(&copy_logic);
            copy_logic.copy.run(());
            let after = snapshot_debug_state(&copy_logic);
            debug_store.record(
                wasm_debug::CodeBlockDebugSource::CopyButtonPress,
                before,
                after,
            );
        })
    };

    #[cfg(not(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32")))]
    let on_copy_press = copy_logic.copy;

    let copy_logic_for_header = copy_logic.clone();
    let copy_logic_for_status = copy_logic.clone();
    let on_copy_press_for_header = on_copy_press;

    let debug_panel: Option<AnyView> = {
        #[cfg(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32"))]
        {
            let copy_logic = copy_logic.clone();
            let debug_store_for_replay = debug_store;
            let request_replay = Callback::new(move |_source: wasm_debug::CodeBlockDebugSource| {
                let before = snapshot_debug_state(&copy_logic);
                copy_logic.copy.run(());
                let after = snapshot_debug_state(&copy_logic);
                debug_store_for_replay.record(
                    wasm_debug::CodeBlockDebugSource::Replay,
                    before,
                    after,
                );
            });
            Some(render_debug_panel(debug_store, request_replay).into_any())
        }
        #[cfg(not(all(feature = "wasm-debug", debug_assertions, target_arch = "wasm32")))]
        {
            None
        }
    };

    view! {
        <div
            class=class_name
            data-slot="code-block"
            data-state=state.state_attr
            data-header=state.header_attr
            data-multiline=state.is_multiline.then_some("true")
            data-empty=state.is_empty.then_some("true")
            data-label=state.has_label.then_some("true")
            data-language=state.has_language.then_some("true")
            data-copyable=state.copyable.then_some("true")
            data-copied=move || copy_logic.copied.get().then_some("true")
            data-copy-loading=move || copy_logic.is_loading.get().then_some("true")
            data-copy-error=move || copy_logic.has_error.get().then_some("true")
            data-copyable-source=copyable_contract.source.as_attr()
            data-copied-source=copy_logic.copied_source.as_attr()
            data-motion-source=state.motion_source_attr
            data-custom-class=state.has_custom_class_name.then_some("true")
            data-ui-schema=move || agent_data.get().schema.as_attr()
            data-ui-intent=move || agent_data.get().intent.as_attr()
            data-ui-action=move || agent_data.get().action.as_attr()
            data-ui-state=move || agent_data.get().state.as_attr()
            data-ui-source=move || agent_data.get().source.as_attr()
            data-ui-source-copyable=move || agent_data.get().source_copyable.as_attr()
            data-ui-source-copied=move || agent_data.get().source_copied.as_attr()
            data-ui-source-motion=move || agent_data.get().source_motion.as_attr()
            data-ui-output-mode=move || agent_data.get().output_mode.as_attr()
            data-ui-output-status=move || agent_data.get().output_status.as_attr()
            aria-busy=move || copy_logic.aria_busy.get()
            lang=copy_logic.lang.clone()
            dir=copy_logic.dir
            node_ref=root_ref
        >
            {code_block_header(
                state.show_header,
                state.copyable,
                label,
                language,
                copy_logic_for_header,
                on_copy_press_for_header,
                copy_to_clipboard_aria_label,
            )}

            {code_block_code_content(code_value)}

            {code_block_status(copy_logic_for_status, copied_label)}
            {debug_panel}
        </div>
    }
}
