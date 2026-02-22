use crate::pages::components::{ComponentDoc, ComponentPage};

pub(super) const DIRECTION_PROVIDER_DOC: ComponentDoc = ComponentDoc {
    name: "DirectionProvider",
    slug: "direction-provider",
    group: "Layout",
    page: direction_provider,
};
use crate::playground::Playground;
use leptos::prelude::*;
use ui::{DirectionMode, DirectionProvider, Snippet};

const DIRECTION_COPY_IMPORTS: &str =
    "use leptos::prelude::*;\nuse ui::{DirectionMode, DirectionProvider};";
const DIRECTION_SOURCE_FIRST_SNIPPET: &str = r#"use leptos::prelude::*;
use ui::{DirectionMode, DirectionProvider};

view! {
  <DirectionProvider direction=DirectionMode::Ltr>
    <div class="docs-direction-demo">"Name → Value"</div>
  </DirectionProvider>
}"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectionInteractiveSource {
    Direction,
    DirAlias,
    Default,
}

impl DirectionInteractiveSource {
    const fn as_attr(self) -> &'static str {
        match self {
            Self::Direction => "direction",
            Self::DirAlias => "dir-alias",
            Self::Default => "default",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DirectionInteractiveLang {
    None,
    En,
    Ar,
}

impl DirectionInteractiveLang {
    const fn as_attr(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::En => "en",
            Self::Ar => "ar",
        }
    }

    const fn as_option(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::En => Some("en"),
            Self::Ar => Some("ar"),
        }
    }
}

pub(super) fn direction_provider() -> AnyView {
    let hello_world_code = Signal::derive(move || {
        r#"<DirectionProvider direction=DirectionMode::Ltr>
  <div class="docs-direction-demo">"Name → Value"</div>
</DirectionProvider>"#
            .to_string()
    });

    let rtl_code = Signal::derive(move || {
        r##"<DirectionProvider direction=DirectionMode::Rtl class_name="docs-direction-rtl".to_string()>
  <div class="docs-direction-demo">"الاسم ← القيمة"</div>
</DirectionProvider>"##.to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r##"<div class="docs-stack docs-stack--tight" data-slot="direction-state-matrix">
  <DirectionProvider direction=DirectionMode::Ltr>
    <div class="docs-direction-demo">"Name → Value"</div>
  </DirectionProvider>
  <DirectionProvider direction=DirectionMode::Rtl class_name="docs-direction-rtl".to_string()>
    <div class="docs-direction-demo">"الاسم ← القيمة"</div>
  </DirectionProvider>
  <DirectionProvider>
    <div class="docs-direction-demo">"Default direction fallback"</div>
  </DirectionProvider>
</div>"##
            .to_string()
    });

    let controlled_uncontrolled_na_code = Signal::derive(move || {
        r#"<DirectionProvider direction=DirectionMode::Ltr>
  <div class="docs-direction-demo">"External value (direction prop)"</div>
</DirectionProvider>
<DirectionProvider dir=DirectionMode::Rtl>
  <div class="docs-direction-demo">"Legacy alias (dir prop)"</div>
</DirectionProvider>
<div class="ui-muted">"DirectionProvider has no internal mutable axis; controlled/uncontrolled is N/A."</div>"#
            .to_string()
    });

    let snapshot_code = Signal::derive(move || {
        r#"<DirectionProvider direction=DirectionMode::Ltr lang="en".to_string()>
  <div class="docs-direction-demo" data-slot="direction-snapshot-demo">
    "Snapshot render keeps lang/dir/data-direction markers stable."
  </div>
</DirectionProvider>"#
            .to_string()
    });

    let (interactive_source, set_interactive_source) =
        signal(DirectionInteractiveSource::Direction);
    let (interactive_input_direction, set_interactive_input_direction) = signal(DirectionMode::Ltr);
    let (interactive_lang, set_interactive_lang) = signal(DirectionInteractiveLang::None);
    let (interactive_custom_class, set_interactive_custom_class) = signal(false);

    let interactive_code = Signal::derive(move || {
        let source = interactive_source.get();
        let input_direction = interactive_input_direction.get();
        let direction_value = match input_direction {
            DirectionMode::Ltr => "DirectionMode::Ltr",
            DirectionMode::Rtl => "DirectionMode::Rtl",
        };
        let direction_prop = match source {
            DirectionInteractiveSource::Direction => format!(" direction={direction_value}"),
            _ => String::new(),
        };
        let dir_prop = match source {
            DirectionInteractiveSource::DirAlias => format!(" dir={direction_value}"),
            _ => String::new(),
        };
        let lang_prop = interactive_lang
            .get()
            .as_option()
            .map(|lang| format!(" lang=\"{lang}\".to_string()"))
            .unwrap_or_default();
        let class_prop = if interactive_custom_class.get() {
            " class_name=\"docs-direction-rtl\".to_string()".to_string()
        } else {
            String::new()
        };
        format!(
            "<DirectionProvider{direction_prop}{dir_prop}{lang_prop}{class_prop}>\n  <div class=\"docs-direction-demo\">\"Interactive preview\"</div>\n</DirectionProvider>"
        )
    });

    view! {
        <ComponentPage
            title="DirectionProvider"
            slug="direction-provider"
            group="Layout"
            description="baseline/Radix-compatible direction context wrapper with normalized `direction`/`dir` props and stable slot + data-direction contracts."
        >
            <p class="ui-muted" data-slot="direction-docs-beginner-path">
                "Quick Start first: copy Hello World to get a working direction context in 3 lines."
            </p>
            <p class="ui-muted" data-slot="direction-docs-advanced-path">
                "Then move to advanced controls: alias `dir`, `lang`, `class_name`, and matrix contracts."
            </p>
            <p class="ui-muted" data-slot="direction-streaming-policy">
                "Streaming Optional; fallback=snapshot."
            </p>
            <p class="ui-muted" data-slot="direction-streaming-modes">
                "Snapshot: renders complete direction context. Streaming: not required for this provider."
            </p>
            <p class="ui-muted" data-slot="direction-copy-ready-hint">
                "Copy-ready snippets prepend imports automatically; dependency: ui; source: components/direction/src/view.rs."
            </p>

            <Playground
                title="Hello World"
                code_signal=hello_world_code
                code_imports=DIRECTION_COPY_IMPORTS.to_string()
            >
                <DirectionProvider direction=DirectionMode::Ltr>
                    <div class="docs-direction-demo">"Name → Value"</div>
                </DirectionProvider>
            </Playground>

            <Playground
                title="RTL Direction + Class"
                code_signal=rtl_code
                code_imports=DIRECTION_COPY_IMPORTS.to_string()
            >
                <DirectionProvider
                    direction=DirectionMode::Rtl
                    class_name="docs-direction-rtl".to_string()
                >
                    <div class="docs-direction-demo">"الاسم ← القيمة"</div>
                </DirectionProvider>
            </Playground>

            <Playground
                title="State Matrix (LTR / RTL / Default)"
                code_signal=state_matrix_code
                code_imports=DIRECTION_COPY_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="direction-state-matrix">
                    <DirectionProvider direction=DirectionMode::Ltr>
                        <div class="docs-direction-demo">"Name → Value"</div>
                    </DirectionProvider>
                    <DirectionProvider
                        direction=DirectionMode::Rtl
                        class_name="docs-direction-rtl".to_string()
                    >
                        <div class="docs-direction-demo">"الاسم ← القيمة"</div>
                    </DirectionProvider>
                    <DirectionProvider>
                        <div class="docs-direction-demo">"Default direction fallback"</div>
                    </DirectionProvider>
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A for Direction)"
                code_signal=controlled_uncontrolled_na_code
                code_imports=DIRECTION_COPY_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="direction-controlled-uncontrolled-na">
                    <DirectionProvider direction=DirectionMode::Ltr>
                        <div class="docs-direction-demo">"External value (direction prop)"</div>
                    </DirectionProvider>
                    <DirectionProvider dir=DirectionMode::Rtl>
                        <div class="docs-direction-demo">"Legacy alias (dir prop)"</div>
                    </DirectionProvider>
                    <div class="ui-muted">
                        "DirectionProvider has no internal mutable axis; controlled/uncontrolled is N/A."
                    </div>
                </div>
            </Playground>

            <Playground
                title="Snapshot Contract"
                code_signal=snapshot_code
                code_imports=DIRECTION_COPY_IMPORTS.to_string()
            >
                <DirectionProvider direction=DirectionMode::Ltr lang="en".to_string()>
                    <div class="docs-direction-demo" data-slot="direction-snapshot-demo">
                        "Snapshot render keeps lang/dir/data-direction markers stable."
                    </div>
                </DirectionProvider>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State Switch + Feedback)"
                description="Toggle source/value/lang/class controls and inspect semantic markers (`data-direction`, `data-direction-source`, `lang`, `dir`) in real time."
                code_signal=interactive_code
                code_imports=DIRECTION_COPY_IMPORTS.to_string()
            >
                <div class="docs-stack docs-stack--tight" data-slot="direction-interactive-playground">
                    <div class="docs-stack docs-stack--tight" data-slot="direction-interactive-controls">
                        <div class="docs-row docs-row--wrap" data-slot="direction-interactive-source-controls">
                            <button
                                type="button"
                                data-slot="direction-interactive-source-direction"
                                data-selected=move || if interactive_source.get() == DirectionInteractiveSource::Direction { "true" } else { "false" }
                                on:click=move |_| set_interactive_source.set(DirectionInteractiveSource::Direction)
                            >
                                "source: direction"
                            </button>
                            <button
                                type="button"
                                data-slot="direction-interactive-source-dir-alias"
                                data-selected=move || if interactive_source.get() == DirectionInteractiveSource::DirAlias { "true" } else { "false" }
                                on:click=move |_| set_interactive_source.set(DirectionInteractiveSource::DirAlias)
                            >
                                "source: dir alias"
                            </button>
                            <button
                                type="button"
                                data-slot="direction-interactive-source-default"
                                data-selected=move || if interactive_source.get() == DirectionInteractiveSource::Default { "true" } else { "false" }
                                on:click=move |_| set_interactive_source.set(DirectionInteractiveSource::Default)
                            >
                                "source: default"
                            </button>
                        </div>

                        <div class="docs-row docs-row--wrap" data-slot="direction-interactive-value-controls">
                            <button
                                type="button"
                                data-slot="direction-interactive-value-ltr"
                                data-selected=move || if interactive_input_direction.get() == DirectionMode::Ltr { "true" } else { "false" }
                                on:click=move |_| set_interactive_input_direction.set(DirectionMode::Ltr)
                            >
                                "value: ltr"
                            </button>
                            <button
                                type="button"
                                data-slot="direction-interactive-value-rtl"
                                data-selected=move || if interactive_input_direction.get() == DirectionMode::Rtl { "true" } else { "false" }
                                on:click=move |_| set_interactive_input_direction.set(DirectionMode::Rtl)
                            >
                                "value: rtl"
                            </button>
                        </div>

                        <div class="docs-row docs-row--wrap" data-slot="direction-interactive-lang-controls">
                            <button
                                type="button"
                                data-slot="direction-interactive-lang-none"
                                data-selected=move || if interactive_lang.get() == DirectionInteractiveLang::None { "true" } else { "false" }
                                on:click=move |_| set_interactive_lang.set(DirectionInteractiveLang::None)
                            >
                                "lang: none"
                            </button>
                            <button
                                type="button"
                                data-slot="direction-interactive-lang-en"
                                data-selected=move || if interactive_lang.get() == DirectionInteractiveLang::En { "true" } else { "false" }
                                on:click=move |_| set_interactive_lang.set(DirectionInteractiveLang::En)
                            >
                                "lang: en"
                            </button>
                            <button
                                type="button"
                                data-slot="direction-interactive-lang-ar"
                                data-selected=move || if interactive_lang.get() == DirectionInteractiveLang::Ar { "true" } else { "false" }
                                on:click=move |_| set_interactive_lang.set(DirectionInteractiveLang::Ar)
                            >
                                "lang: ar"
                            </button>
                        </div>

                        <div class="docs-row docs-row--wrap" data-slot="direction-interactive-misc-controls">
                            <button
                                type="button"
                                data-slot="direction-interactive-class-toggle"
                                data-selected=move || if interactive_custom_class.get() { "true" } else { "false" }
                                on:click=move |_| {
                                    set_interactive_custom_class.update(|value| *value = !*value)
                                }
                            >
                                {move || if interactive_custom_class.get() {
                                    "class: docs-direction-rtl (on)"
                                } else {
                                    "class: docs-direction-rtl (off)"
                                }}
                            </button>
                            <button
                                type="button"
                                data-slot="direction-interactive-reset"
                                on:click=move |_| {
                                    set_interactive_source.set(DirectionInteractiveSource::Direction);
                                    set_interactive_input_direction.set(DirectionMode::Ltr);
                                    set_interactive_lang.set(DirectionInteractiveLang::None);
                                    set_interactive_custom_class.set(false);
                                }
                            >
                                "reset controls"
                            </button>
                        </div>
                    </div>

                    {move || {
                        let source = interactive_source.get();
                        let input_direction = interactive_input_direction.get();
                        let effective_direction = if source == DirectionInteractiveSource::Default {
                            DirectionMode::default()
                        } else {
                            input_direction
                        };
                        let lang_mode = interactive_lang.get();
                        let lang = lang_mode
                            .as_option()
                            .map_or_else(String::new, ToString::to_string);
                        let class_name = if interactive_custom_class.get() {
                            "docs-direction-rtl".to_string()
                        } else {
                            String::new()
                        };
                        let class_marker = if interactive_custom_class.get() {
                            "docs-direction-rtl"
                        } else {
                            "none"
                        };

                        view! {
                            <div class="docs-stack docs-stack--tight" data-slot="direction-interactive-preview">
                                {match source {
                                    DirectionInteractiveSource::Direction => view! {
                                        <DirectionProvider
                                            direction=input_direction
                                            lang=lang.clone()
                                            class_name=class_name.clone()
                                        >
                                            <div class="docs-direction-demo" data-slot="direction-interactive-render">
                                                "Interactive preview · inspect semantic markers in devtools/tests."
                                            </div>
                                        </DirectionProvider>
                                    }
                                        .into_any(),
                                    DirectionInteractiveSource::DirAlias => view! {
                                        <DirectionProvider
                                            dir=input_direction
                                            lang=lang.clone()
                                            class_name=class_name.clone()
                                        >
                                            <div class="docs-direction-demo" data-slot="direction-interactive-render">
                                                "Interactive preview · inspect semantic markers in devtools/tests."
                                            </div>
                                        </DirectionProvider>
                                    }
                                        .into_any(),
                                    DirectionInteractiveSource::Default => view! {
                                        <DirectionProvider
                                            lang=lang
                                            class_name=class_name
                                        >
                                            <div class="docs-direction-demo" data-slot="direction-interactive-render">
                                                "Interactive preview · inspect semantic markers in devtools/tests."
                                            </div>
                                        </DirectionProvider>
                                    }
                                        .into_any(),
                                }}
                                <div
                                    class="ui-muted"
                                    data-slot="direction-interactive-feedback"
                                    data-current-source=source.as_attr()
                                    data-current-direction=effective_direction.as_attr()
                                    data-current-lang=lang_mode.as_attr()
                                    data-current-class=class_marker
                                >
                                    {format!(
                                        "Expected markers -> data-direction={} · data-direction-source={} · lang={} · class={}",
                                        effective_direction.as_attr(),
                                        source.as_attr(),
                                        lang_mode.as_attr(),
                                        class_marker
                                    )}
                                </div>
                            </div>
                        }
                    }}
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="direction-parameter-matrix">
                <h2>"Parameter Matrix"</h2>
                <ul data-slot="direction-parameter-rows">
                    <li><code>"direction: Option&lt;DirectionMode&gt;"</code>" default = None (primary axis; wins priority)"</li>
                    <li><code>"dir: Option&lt;DirectionMode&gt;"</code>" default = None (historical alias; used only when direction=None)"</li>
                    <li><code>"lang: Option&lt;String&gt;"</code>" default = None (forwarded to semantic attrs)"</li>
                    <li><code>"class_name: Option&lt;String&gt;"</code>" default = None (normalized class extension)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="direction-default-rule">
                <h2>"Default Resolution Rule"</h2>
                <p>
                    <code>"logic::resolve_direction(direction, dir)"</code>
                    " uses "
                    <code>"direction > dir > DirectionMode::default()"</code>
                    " and emits "
                    <code>"data-direction-source"</code>
                    " as "
                    <code>"direction|dir-alias|default"</code>
                    "."
                </p>
            </section>

            <section class="docs-card docs-prose" data-slot="direction-source-first">
                <h2>"Source-first / Copy-Paste Ready"</h2>
                <p data-slot="direction-source-first-copy-hint">
                    "Use "
                    <code>"Show code"</code>
                    " in each playground for one-click copy; snippets are import-completed by "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <Snippet
                    text=DIRECTION_SOURCE_FIRST_SNIPPET.to_string()
                    label="Copy DirectionProvider starter".to_string()
                    copyable=true
                    class_name="docs-direction-source-copy".to_string()
                />
                <ul data-slot="direction-source-paths">
                    <li><code>"components/direction/src/mod.rs"</code></li>
                    <li><code>"components/direction/src/logic.rs"</code></li>
                    <li><code>"components/direction/src/view.rs"</code></li>
                    <li><code>"components/direction/src/styles.rs"</code></li>
                    <li><code>"components/direction/src/protocol.rs"</code></li>
                </ul>
                <ul data-slot="direction-source-prerequisites">
                    <li><code>"component-direction"</code></li>
                    <li><code>"inject-css"</code></li>
                </ul>
                <p class="ui-muted" data-slot="direction-source-sync-note">
                    "Starter snippet mirrors Hello World (`direction=DirectionMode::Ltr`); API changes must update this snippet and playground code in the same PR."
                </p>
            </section>
        </ComponentPage>
    }
    .into_any()
}
