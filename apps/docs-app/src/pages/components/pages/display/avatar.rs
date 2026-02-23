use super::*;

pub(crate) fn avatar() -> AnyView {
    let src = "data:image/svg+xml,%3Csvg%20xmlns%3D%27http%3A//www.w3.org/2000/svg%27%20width%3D%2764%27%20height%3D%2764%27%20viewBox%3D%270%200%2064%2064%27%3E%3Cdefs%3E%3CradialGradient%20id%3D%27g%27%20cx%3D%2732%27%20cy%3D%2732%27%20r%3D%2732%27%3E%3Cstop%20offset%3D%270%27%20stop-color%3D%27%23ff4bd8%27/%3E%3Cstop%20offset%3D%271%27%20stop-color%3D%27%232b5cff%27/%3E%3C/radialGradient%3E%3C/defs%3E%3Crect%20width%3D%2764%27%20height%3D%2764%27%20rx%3D%2732%27%20fill%3D%27url(%23g)%27/%3E%3Ctext%20x%3D%2750%25%27%20y%3D%2752%25%27%20text-anchor%3D%27middle%27%20dominant-baseline%3D%27middle%27%20font-size%3D%2724%27%20font-family%3D%27system-ui%27%20fill%3D%27white%27%3EA%3C/text%3E%3C/svg%3E";

    let hello_code = Signal::derive(move || r#"<Avatar />"#.to_string());

    let image_code = Signal::derive(move || {
        r#"<Avatar name="Ada Lovelace".to_string() src=Some(src.into()) />"#.to_string()
    });

    let state_matrix_code = Signal::derive(move || {
        r#"<Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
<Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
<Avatar size=AvatarSize::Lg />"#
            .to_string()
    });

    let controlled_contrast_code = Signal::derive(move || {
        r#"let avatar_name = Some("Ada Lovelace".to_string());

<Avatar />
<Avatar name=avatar_name />
// Avatar has no controlled/uncontrolled state axis.
// App state maps directly to props; no value/on_change/default triplet."#
            .to_string()
    });

    let stream_snapshot_code = Signal::derive(move || {
        r#"<Avatar name="Snapshot User".to_string() size=AvatarSize::Md />
// Streaming Optional; fallback=snapshot.
// Avatar consumes complete props snapshots and stays render-stable."#
            .to_string()
    });

    let custom_code = Signal::derive(move || {
        r#"<Avatar
  name="  Ada Lovelace  ".to_string()
  alt="  Team lead  ".to_string()
  size=AvatarSize::Lg
  class_name="docs-avatar-custom".to_string()
/>
<Avatar
  alt="  Anonymous collaborator  ".to_string()
  src="   ".to_string()
  class_name="docs-avatar-custom".to_string()
/>"#
        .to_string()
    });

    let source_first_code = Signal::derive(move || {
        r#"use leptos::prelude::*;
use ui::{Avatar, AvatarSize};

<Avatar name="Ada Lovelace".to_string() size=AvatarSize::Md />"#
            .to_string()
    });

    let workbench_mode_options = vec![
        "image".to_string(),
        "name-only".to_string(),
        "fallback".to_string(),
    ];
    let workbench_size_options = vec!["sm".to_string(), "md".to_string(), "lg".to_string()];
    let (workbench_mode_index, set_workbench_mode_index) = signal(Some(0_usize));
    let (workbench_size_index, set_workbench_size_index) = signal(Some(1_usize));
    let (workbench_use_alt, set_workbench_use_alt) = signal(false);
    let (workbench_custom_class, set_workbench_custom_class) = signal(false);
    let (workbench_rtl, set_workbench_rtl) = signal(false);

    let workbench_mode = Signal::derive(move || match workbench_mode_index.get().unwrap_or(0) {
        1 => "name-only",
        2 => "fallback",
        _ => "image",
    });
    let workbench_size = Signal::derive(move || match workbench_size_index.get().unwrap_or(1) {
        0 => AvatarSize::Sm,
        2 => AvatarSize::Lg,
        _ => AvatarSize::Md,
    });

    let workbench_code = Signal::derive(move || {
        let mode = workbench_mode.get();
        let size = match workbench_size.get() {
            AvatarSize::Sm => "AvatarSize::Sm",
            AvatarSize::Md => "AvatarSize::Md",
            AvatarSize::Lg => "AvatarSize::Lg",
        };
        let use_alt = workbench_use_alt.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();

        let mut lines = vec!["<Avatar".to_string(), format!("  size={size}")];
        match mode {
            "image" => {
                lines.push("  name=\"Ada Lovelace\".to_string()".to_string());
                lines.push("  src=Some(src.into())".to_string());
            }
            "name-only" => {
                lines.push("  name=\"Ada Lovelace\".to_string()".to_string());
            }
            _ => {}
        }
        if use_alt {
            lines.push("  alt=\"Team collaborator\".to_string()".to_string());
        }
        if custom_class {
            lines.push("  class_name=\"docs-avatar-custom\".to_string()".to_string());
        }
        if rtl {
            lines.push("  lang=\"ar\".to_string()".to_string());
            lines.push("  dir=A11yDirection::Rtl".to_string());
        }
        lines.push("/>".to_string());
        lines.join("\n")
    });

    let workbench_config = Signal::derive(move || {
        let mode = workbench_mode.get();
        let use_alt = workbench_use_alt.get();
        let custom_class = workbench_custom_class.get();
        let rtl = workbench_rtl.get();
        let size = workbench_size.get();

        let expected_state = if mode == "image" { "image" } else { "fallback" };
        let expected_label_source = if use_alt {
            "alt"
        } else if matches!(mode, "image" | "name-only") {
            "name"
        } else {
            "fallback"
        };
        let expected_size = match size {
            AvatarSize::Sm => "sm",
            AvatarSize::Md => "md",
            AvatarSize::Lg => "lg",
        };

        format!(
            "AvatarWorkbenchConfig {{\n  name: {},\n  src: {},\n  size: \"{expected_size}\",\n  alt: {},\n  class_name: {},\n  lang: {},\n  dir: {},\n  mode: \"{mode}\",\n  use_alt: {use_alt},\n  rtl: {rtl},\n  expected_state: \"{expected_state}\",\n  expected_label_source: \"{expected_label_source}\",\n}}",
            if matches!(mode, "image" | "name-only") {
                "Some(\"Ada Lovelace\")"
            } else {
                "None"
            },
            if mode == "image" {
                "Some(\"data:image/svg+xml,...\")"
            } else {
                "None"
            },
            if use_alt {
                "Some(\"Team collaborator\")"
            } else {
                "None"
            },
            if custom_class {
                "Some(\"docs-avatar-custom\")"
            } else {
                "None"
            },
            if rtl { "Some(\"ar\")" } else { "None" },
            if rtl { "Some(\"rtl\")" } else { "None" },
        )
    });

    view! {
        <ComponentPage
            title="Avatar"
            slug="avatar"
            group="Display"
            description="Avatar with image/error fallback, normalized labels, and baseline-style root state attrs + custom-class contract."
        >
            <Playground
                title="Hello World"
                code_signal=hello_code
                code_imports="use leptos::prelude::*;\nuse ui::Avatar;".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar />
                </div>
            </Playground>

            <Playground
                title="Interactive Playground (Props + State Preview)"
                description="Modify props live and inspect semantic state transitions without wiring internal state machines."
                code_signal=workbench_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};\nuse ui::color::area::A11yDirection;".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
                test_config_signal=workbench_config
                controls=move || {
                    view! {
                        <div class="docs-stack docs-stack--tight" data-slot="avatar-workbench-controls">
                            <div class="docs-search__label">"Render mode"</div>
                            <SegmentedControl
                                id_base="docs-avatar-workbench-mode".to_string()
                                options=workbench_mode_options.clone()
                                selected_index=workbench_mode_index
                                set_selected_index=set_workbench_mode_index
                                size=SegmentedControlSize::Sm
                                aria_label="Avatar render mode".to_string()
                            />

                            <div class="docs-search__label">"Size"</div>
                            <SegmentedControl
                                id_base="docs-avatar-workbench-size".to_string()
                                options=workbench_size_options.clone()
                                selected_index=workbench_size_index
                                set_selected_index=set_workbench_size_index
                                size=SegmentedControlSize::Sm
                                aria_label="Avatar size".to_string()
                            />

                            <Switch checked=workbench_use_alt set_checked=set_workbench_use_alt>
                                "Use alt label"
                            </Switch>
                            <Switch checked=workbench_custom_class set_checked=set_workbench_custom_class>
                                "Custom class"
                            </Switch>
                            <Switch checked=workbench_rtl set_checked=set_workbench_rtl>
                                "RTL direction"
                            </Switch>
                        </div>
                    }
                }
            >
                {move || {
                    let mode = workbench_mode.get();
                    let size = workbench_size.get();
                    let use_alt = workbench_use_alt.get();
                    let custom_class = workbench_custom_class.get();
                    let rtl = workbench_rtl.get();

                    let name = if matches!(mode, "image" | "name-only") {
                        "Ada Lovelace".to_string()
                    } else {
                        String::new()
                    };
                    let image_src = if mode == "image" {
                        into_owned_string(src)
                    } else {
                        String::new()
                    };
                    let alt = if use_alt {
                        "Team collaborator".to_string()
                    } else {
                        String::new()
                    };
                    let class_name = if custom_class {
                        "docs-avatar-custom".to_string()
                    } else {
                        String::new()
                    };
                    let lang = if rtl { "ar".to_string() } else { String::new() };
                    let dir = if rtl {
                        A11yDirection::Rtl
                    } else {
                        A11yDirection::Ltr
                    };

                    let expected_state = if mode == "image" { "image" } else { "fallback" };
                    let expected_label_source = if use_alt {
                        "alt"
                    } else if matches!(mode, "image" | "name-only") {
                        "name"
                    } else {
                        "fallback"
                    };
                    let expected_size = match size {
                        AvatarSize::Sm => "sm",
                        AvatarSize::Md => "md",
                        AvatarSize::Lg => "lg",
                    };

                    view! {
                        <div class="docs-stack" data-slot="avatar-workbench-preview">
                            <div class="docs-row">
                                <div class="docs-stack docs-stack--tight">
                                    <div class="docs-search__label">"Baseline"</div>
                                    <Avatar />
                                </div>
                                <div class="docs-stack docs-stack--tight" data-slot="avatar-workbench-configured">
                                    <div class="docs-search__label">"Configured"</div>
                                    <Avatar
                                        name=name
                                        src=image_src
                                        size=size
                                        alt=alt
                                        class_name=class_name
                                        lang=lang
                                        dir=dir
                                    />
                                </div>
                            </div>
                            <p class="ui-muted" data-slot="avatar-workbench-state">
                                {format!(
                                    "expected: state={expected_state}, label_source={expected_label_source}, size={expected_size}"
                                )}
                            </p>
                        </div>
                    }
                }}
            </Playground>



            <Playground
                title="Image + Fallback"
                code_signal=image_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=into_owned_string(src)
                        size=AvatarSize::Md
                    />
                    <Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
                    <Avatar name="Alan Turing".to_string() size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground
                title="Fallback Scenarios"
                description="Label source + fallback state matrix with stable semantic markers."
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=into_owned_string(src)
                        alt="Profile photo".to_string()
                        size=AvatarSize::Sm
                    />
                    <Avatar alt="Anonymous collaborator".to_string() size=AvatarSize::Sm />
                    <Avatar size=AvatarSize::Lg />
                </div>
            </Playground>

            <Playground
                title="Custom Class + Normalized Props"
                code_signal=custom_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="  Ada Lovelace  ".to_string()
                        alt="  Team lead  ".to_string()
                        size=AvatarSize::Lg
                        class_name="docs-avatar-custom".to_string()
                    />
                    <Avatar
                        alt="  Anonymous collaborator  ".to_string()
                        src="   ".to_string()
                        class_name="docs-avatar-custom".to_string()
                    />
                </div>
            </Playground>

            <Playground
                title="Controlled vs Uncontrolled (N/A)"
                description="Avatar has no internal controlled/uncontrolled axis; compare default usage and app-state-mapped props."
                code_signal=controlled_contrast_code
                code_imports="use leptos::prelude::*;\nuse ui::Avatar;".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar />
                    <Avatar name="Ada Lovelace".to_string() />
                </div>
            </Playground>

            <Playground
                title="Streaming Optional / Snapshot"
                description="Avatar is not a body-reader surface: streaming is optional and falls back to snapshot rendering."
                code_signal=stream_snapshot_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-stack docs-stack--tight">
                    <p class="ui-muted" data-slot="avatar-streaming-policy">
                        "Streaming Optional; fallback=snapshot."
                    </p>
                    <p class="ui-muted" data-slot="avatar-copy-ready-hint">
                        "Copy-ready snippets prepend imports automatically; source: components/avatar/src/view.rs."
                    </p>
                    <div class="docs-row">
                        <Avatar name="Snapshot User".to_string() size=AvatarSize::Md />
                        <Avatar alt="Fallback viewer".to_string() size=AvatarSize::Sm />
                    </div>
                </div>
            </Playground>



            <Playground
                title="State Matrix (Image / Name / Fallback)"
                description="Workbench 后的多参数对比：image/name-only/fallback。"
                code_signal=state_matrix_code
                code_imports="use leptos::prelude::*;\nuse ui::{Avatar, AvatarSize};".to_string()
                test_source_path="components/avatar/src/view.rs".to_string()
            >
                <div class="docs-row">
                    <Avatar
                        name="Ada Lovelace".to_string()
                        src=into_owned_string(src)
                        alt="Profile photo".to_string()
                        size=AvatarSize::Sm
                    />
                    <Avatar name="Grace Hopper".to_string() size=AvatarSize::Md />
                    <Avatar size=AvatarSize::Lg />
                </div>
            </Playground>

            <section class="docs-card docs-prose" data-slot="avatar-source-first">
                <h3>"Source-first / Copy-Paste Ready"</h3>
                <p>
                    "Playground copy action injects missing imports through "
                    <code>"apps/docs-app/src/playground.rs::compose_copy_ready_code"</code>
                    "."
                </p>
                <ul data-slot="avatar-source-prerequisites">
                    <li>
                        "Dependency prerequisite: enable "
                        <code>"component-avatar"</code>
                        " feature for package-mode consumption."
                    </li>
                    <li>
                        "Style prerequisite: use "
                        <code>"UiRoot"</code>
                        " with components CSS injection (or enable "
                        <code>"inject-css"</code>
                        " path) to avoid unstyled copy-paste output."
                    </li>
                </ul>
                <Snippet
                    text=source_first_code.get()
                    label="Copy avatar starter".to_string()
                    copyable=true
                    class_name="docs-avatar-source-copy".to_string()
                />
                <ul data-slot="avatar-source-paths">
                    <li><code>"components/avatar/src/mod.rs"</code></li>
                    <li><code>"components/avatar/src/logic.rs"</code></li>
                    <li><code>"components/avatar/src/view.rs"</code></li>
                    <li><code>"components/avatar/src/styles.rs"</code></li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-state-matrix">
                <h3>"State Matrix"</h3>
                <ul data-slot="avatar-state-rows">
                    <li><code>"data-state"</code>" = image | fallback"</li>
                    <li><code>"data-image / data-fallback"</code>" = true | (absent), derived from render mode"</li>
                    <li><code>"data-label-source"</code>" = alt | name | fallback"</li>
                    <li><code>"data-size"</code>" = sm | md | lg"</li>
                    <li><code>"control mode"</code>" = N/A (Avatar has no controlled/uncontrolled runtime axis)"</li>
                    <li><code>"disabled axis"</code>" = N/A (Avatar has no disabled prop in API)"</li>
                </ul>
            </section>

            <section class="docs-card docs-prose" data-slot="avatar-parameter-matrix">
                <h3>"Parameter Matrix"</h3>
                <ul data-slot="avatar-parameter-rows">
                    <li><code>"name / src / alt / class_name / lang: Option&lt;String&gt;"</code>" default = None; blank strings are normalized away by normalize_input/normalize_lang"</li>
                    <li><code>"size: AvatarSize"</code>" default = AvatarSize::Md"</li>
                    <li><code>"dir: Option&lt;A11yDirection&gt;"</code>" default = None (inherits locale direction/context)"</li>
                    <li><code>"label source priority"</code>" = alt -> name -> fallback"</li>
                    <li><code>"render mode"</code>" = image when src is present and no image error, else fallback"</li>
                </ul>
            </section>
        </ComponentPage>
    }
    .into_any()
}
